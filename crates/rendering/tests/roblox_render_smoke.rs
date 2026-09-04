use rbx_core::SharedPtr;
use rbx_rendering::generated as g;
use rbx_rendering::generated_141;
use rbx_rendering::movable;
use rbx_rendering::roblox_view::{
    self, FrameRateManager, FunctorOp, JobStats, PrepareOutcome, RenderDispatch, RenderJob,
    RenderMetricHost, RobloxView, SchedulerOp, StepInput,
};

fn datamodel() -> SharedPtr<rbx_datamodel::data_model::DataModel> {
    SharedPtr::new(rbx_datamodel::data_model::DataModel::default())
}

fn test_job() -> RenderJob {
    let dm = datamodel();
    let mut job = g::stub_3ecf0(0x1000, 0x2000, &dm);
    job.set_datamodel_live(true);
    job
}

#[test]
fn resume_builds_both_jobs_and_schedules() {
    let mut view = RobloxView::new(0x1000, 0x3000, 0x2000);
    let mut sink = Vec::new();
    g::stub_37378(&mut view, &datamodel(), &mut sink);
    assert!(view.render_job.is_some());
    assert!(view.view_update_job.is_some());
    assert_eq!(
        sink,
        vec![
            SchedulerOp::AddRenderJob,
            SchedulerOp::AddViewUpdateJob
        ]
    );
}

#[test]
fn stop_without_cleanup_sets_flag_and_drops_render_slot() {
    let mut view = RobloxView::new(0x1000, 0x3000, 0x2000);
    let mut sink = Vec::new();
    g::stub_37378(&mut view, &datamodel(), &mut sink);
    sink.clear();
    g::stub_37068(&mut view, false, &mut sink);
    assert!(view.render_job.is_none());
    assert!(view.view_update_job.is_none());
    assert_eq!(sink, vec![SchedulerOp::RemoveViewUpdateJob]);
}

#[test]
fn stop_with_cleanup_resets_both_slots() {
    let mut view = RobloxView::new(0x1000, 0x3000, 0x2000);
    let mut sink = Vec::new();
    g::stub_37378(&mut view, &datamodel(), &mut sink);
    sink.clear();
    g::stub_37068(&mut view, true, &mut sink);
    assert!(view.render_job.is_none());
    assert!(view.view_update_job.is_none());
    assert_eq!(
        sink,
        vec![
            SchedulerOp::RemoveRenderJob,
            SchedulerOp::RemoveViewUpdateJob
        ]
    );
}

#[test]
fn sleep_time_throttle_gate() {
    let mut job = test_job();
    g::stub_3f008(
        &mut job,
        &JobStats {
            throttle_enabled: true,
        },
        0.0,
    );
    assert!((job.sleep_time - 1.0 / 60.0).abs() < 1e-12);
    g::stub_3f008(
        &mut job,
        &JobStats {
            throttle_enabled: false,
        },
        0.0,
    );
    assert!(job.sleep_time.is_infinite());
}

#[test]
fn error_throttle_gate() {
    let mut job = test_job();
    g::stub_3f058(
        &mut job,
        &JobStats {
            throttle_enabled: true,
        },
        0.05,
    );
    assert!(job.has_error);
    g::stub_3f058(
        &mut job,
        &JobStats {
            throttle_enabled: false,
        },
        0.05,
    );
    assert!(!job.has_error && job.error_value == 0.0);
}

fn full_step_input() -> StepInput {
    StepInput {
        datamodel_present: true,
        view_can_step: true,
        render_no_dmlock: false,
        now_fast_sec: 10.0,
        now_sample: 11.0,
        camera_dt: Some(0.016),
    }
}

#[test]
fn step_gates_and_dispatches() {
    let mut sink = Vec::new();
    let mut job = test_job();
    let mut no_dm = full_step_input();
    no_dm.datamodel_present = false;
    assert!(!g::stub_3f094(&mut job, &no_dm, &mut sink));
    job.stop_requested = true;
    assert!(!g::stub_3f094(&mut job, &full_step_input(), &mut sink));
    job.stop_requested = false;
    assert!(g::stub_3f094(&mut job, &full_step_input(), &mut sink));
    assert_eq!(
        sink,
        vec![RenderDispatch::Full {
            view: 0x1000,
            at: 10.0
        }]
    );
    assert_eq!(job.last_step_time, 11.0);

    sink.clear();
    let mut no_lock = full_step_input();
    no_lock.render_no_dmlock = true;
    assert!(g::stub_3f094(&mut job, &no_lock, &mut sink));
    assert_eq!(
        sink,
        vec![
            RenderDispatch::Prepare {
                view: 0x1000,
                at: 10.0
            },
            RenderDispatch::Perform {
                view: 0x1000,
                at: 10.0
            }
        ]
    );
    assert!(!job.wake_armed);
}

struct FakeMetrics {
    view: bool,
    frm: Option<FrameRateManager>,
}

impl RenderMetricHost for FakeMetrics {
    fn has_view(&self) -> bool {
        self.view
    }
    fn average_steps_per_second(&self) -> f64 {
        59.5
    }
    fn average_duty_cycle(&self) -> f64 {
        0.4
    }
    fn average_step_time(&self) -> f64 {
        0.016
    }
    fn frame_rate_manager(&self) -> Option<FrameRateManager> {
        self.frm
    }
    fn render_view_metric(&self, _name: &str) -> f64 {
        7.0
    }
    fn video_memory_bytes(&self) -> u32 {
        2_000_000
    }
}

#[test]
fn metric_value_dispatch() {
    let job = test_job();
    let host = FakeMetrics { view: true, frm: None };
    assert_eq!(g::stub_3f598(&job, "Render FPS", &host), 59.5);
    assert_eq!(g::stub_3f598(&job, "Render Duty", &host), 0.4);
    assert_eq!(g::stub_3f598(&job, "Render Job Time", &host), 0.016);
    assert_eq!(g::stub_3f598(&job, "Render Nominal FPS", &host), 0.0);
    assert_eq!(g::stub_3f598(&job, "Ogre", &host), 7.0);
    assert_eq!(g::stub_3f598(&job, "Video Memory MB", &host), 2.0);
    assert_eq!(g::stub_3f598(&job, "Nope", &host), 0.0);
    let frm_host = FakeMetrics {
        view: true,
        frm: Some(FrameRateManager {
            enabled: true,
            render_time_average: 20.0,
            antialiasing_mode: 1,
        }),
    };
    assert_eq!(g::stub_3f598(&job, "Render Nominal FPS", &frm_host), 50.0);
    assert_eq!(g::stub_3f700(&job, "FRM", &frm_host), "On");
    assert_eq!(g::stub_3f700(&job, "Anti-Aliasing", &frm_host), "On");
    assert_eq!(g::stub_3f700(&job, "Graphics Mode", &frm_host), "");
    assert_eq!(
        g::stub_3f700(
            &job,
            "Anything",
            &FakeMetrics { view: false, frm: None }
        ),
        "No View"
    );
    // Thunk wrappers (0x3fa94/0x3faa4) agree with the direct calls.
    assert_eq!(g::stub_3fa94(&job, "FRM", &frm_host), "On");
    assert_eq!(g::stub_3faa4(&job, "Render FPS", &frm_host), 59.5);
}

#[test]
fn prepare_gated_by_stop_flag() {
    let mut job = test_job();
    let mut target = test_job();
    target.wake_armed = true;
    let out = g::stub_3faac(&mut job, Some(&mut target), 0x1000);
    assert_eq!(out, PrepareOutcome::Prepared);
    assert!(!target.wake_armed);
    job.stop_requested = true;
    let out = g::stub_3faac(&mut job, None, 0x1000);
    assert_eq!(out, PrepareOutcome::Stopped);
}

#[test]
fn perform_and_wake() {
    let mut job = test_job();
    let mut target = test_job();
    target.wake_armed = true;
    g::stub_3fac4(&mut job, Some(&mut target), 0x1000, 3.0);
    assert!(!target.wake_armed);
    assert_eq!(job.reschedules, 1);
    job.set_datamodel_live(false);
    g::stub_3fb9c(&mut job);
    assert!(job.wake_armed);
    assert_eq!(job.reschedules, 1);
}

#[test]
fn shared_ptr_glue_counts() {
    let mut slot: Option<SharedPtr<parking_lot::Mutex<RenderJob>>> = None;
    let job = test_job();
    let mut src = Some(g::stub_3a0d4(job));
    assert_eq!(g::stub_3dd34(src.as_ref().unwrap()), 1);
    g::stub_3a030(&mut slot, &mut src);
    assert!(src.is_none() && slot.is_some());
    g::stub_39d7c(&mut slot);
    assert!(slot.is_none());
    assert_eq!(g::stub_3de30(), None);
    assert_eq!(g::stub_3de40(), None);
    assert_eq!(g::stub_3de44(), None);
    g::stub_3dc60();
    g::stub_3de28();
    g::stub_3de2c();
}

#[test]
fn dtor_and_thunk_shapes() {
    let mut job = test_job();
    job.event_signaled = true;
    g::stub_3ee80(&mut job);
    assert!(!job.event_signaled);
    job.event_signaled = true;
    g::stub_3f904(&mut job);
    assert!(!job.event_signaled);
    g::stub_3ef40(test_job());
    g::stub_3f9c8(test_job());
    assert_eq!(roblox_view::RENDER_JOB_SUBOBJECT_OFFSET, 480);
    let base = job.as_job_base();
    assert_eq!(
        base,
        (&job as *const RenderJob as usize).wrapping_add(480)
    );
}

#[test]
fn functor_bind_round_trip() {
    let bind = roblox_view::PerformBind {
        job: 0x1000,
        view: 0x2000,
        at: 1.5,
    };
    let mut dst = None;
    assert!(g::stub_40160(Some(bind), &mut dst, FunctorOp::Clone));
    assert!(dst.is_some());
    let mut job = test_job();
    job.wake_armed = false;
    job.set_datamodel_live(true);
    g::stub_401dc(dst.as_ref().unwrap(), &mut job);
    assert!(job.wake_armed);
    assert!(g::stub_40160(Some(bind), &mut dst, FunctorOp::Destroy));
    assert!(dst.is_none());

    let prepare = roblox_view::PrepareBind {
        job: 0x1000,
        view: 0x2000,
    };
    let mut pdst = None;
    assert!(g::stub_402a8(Some(prepare), &mut pdst, FunctorOp::Clone));
    let mut job2 = test_job();
    g::stub_40308(pdst.as_ref().unwrap(), &mut job2);
    assert!(!job2.wake_armed);
    let addr = g::stub_40270(0x2000, Some(&mut job2), 2.0);
    assert_eq!(addr, job2.as_job_base());
    let addr2 = g::stub_4027c(0x2000, None, 2.0);
    assert_eq!(addr2, 0x2000);
}

// ---- batch 2: Ogre value/codec/compositor types ----

#[test]
fn colour_packing_matches_ida_shifts() {
    let red = movable::ColourValue {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    assert_eq!(g::stub_c6fee4(&red), 0xFF0000FF);
    assert_eq!(g::stub_c6ff3c(&red), 0xFFFF0000);
    assert_eq!(g::stub_c6ff94(&red), 0xFF0000FF);
    let white = movable::ColourValue {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    assert_eq!(g::stub_c6fee4(&white), 0xFFFFFFFF);
    assert!(g::stub_c6ffec(&red, &red));
    assert!(!g::stub_c6ffec(&red, &white));
    assert!(g::stub_c70040(&red, &white));
    assert!(!g::stub_c70040(&red, &red));
}

#[test]
fn fast_hash_properties() {
    assert_eq!(g::stub_c70150(&[], 0), 0);
    let a = g::stub_c70150(b"hello world", 0);
    assert_ne!(a, 0);
    assert_eq!(a, g::stub_c70150(b"hello world", 0));
    assert_ne!(a, g::stub_c70150(b"hello worlD", 0));
    assert_ne!(a, g::stub_c70150(b"hello world", 99));
    // All tail lengths hash without panicking.
    for len in 1..9usize {
        let v = g::stub_c70150(&vec![0xABu8; len], 0);
        assert_ne!(v, 0);
    }
}

#[test]
fn animable_base_always_throws() {
    let obj = movable::AnimableObject;
    assert_eq!(g::stub_c6e7d0(&obj), "");
    let err = g::stub_c6e7e4(&obj, "spin").unwrap_err();
    assert_eq!(err.number, 5);
    assert!(err.message.contains("spin"));
    assert_eq!(err.type_name, "ItemIdentityException");
}

#[test]
fn codec_registry_lookup_and_errors() {
    let mut reg = movable::CodecRegistry::default();
    reg.register(
        "DDS",
        movable::CodecEntry {
            extensions: vec!["dds".to_string()],
            magic_prefix: vec![b'D', b'D', b'S', b' '],
        },
    );
    assert_eq!(g::stub_c6f1f4(&reg), vec!["dds".to_string()]);
    assert_eq!(g::stub_c6f3a0(&reg, "dds"), Ok(0));
    assert_eq!(g::stub_c6f3a0(&reg, "DDS"), Ok(0));
    assert_eq!(g::stub_c6fbcc(&reg, b"DDS |"), Ok(0));
    let miss = g::stub_c6f3a0(&reg, "png").unwrap_err();
    assert!(miss.message.contains("dds"));
    assert!(g::stub_c6fbcc(&reg, b"nope").is_err());
}

struct MapResolver {
    handle: Option<usize>,
}

impl movable::MaterialResolver for MapResolver {
    fn load_material(&self, _name: &str) -> Option<usize> {
        self.handle
    }
}

#[test]
fn composition_pass_defaults_and_setters() {
    let mut pass = g::stub_c70228(0x5000);
    assert_eq!(pass.parent, 0x5000);
    assert_eq!(pass.pass_type, 3);
    assert_eq!((pass.first_render_queue, pass.last_render_queue), (0, 95));
    assert_eq!(pass.clear_buffers, 3);
    assert_eq!(pass.clear_depth, 1.0);
    assert_eq!(pass.inputs.len(), 16);
    g::stub_c706dc(&mut pass, 1);
    g::stub_c706e0(&mut pass, 42);
    g::stub_c708b8(&mut pass, 1);
    g::stub_c708bc(
        &mut pass,
        movable::ColourValue {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        },
    );
    g::stub_c709fc(&mut pass, 10);
    g::stub_c70a00(&mut pass, 90);
    g::stub_c70a04(&mut pass, "Default");
    g::stub_c70a10(&mut pass, 0.5);
    let resolver = MapResolver { handle: Some(77) };
    g::stub_c706e4(&mut pass, "BaseWhite", &resolver);
    g::stub_c708cc(&mut pass, 3, "rt0", 1);
    assert_eq!(pass.pass_type, 1);
    assert_eq!(pass.identifier, 42);
    assert_eq!(pass.clear_buffers, 1);
    assert_eq!(pass.clear_colour, movable::ColourValue {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0
    });
    assert_eq!((pass.first_render_queue, pass.last_render_queue), (10, 90));
    assert_eq!(pass.material_scheme, "Default");
    assert_eq!(pass.clear_depth, 0.5);
    assert_eq!(pass.material, Some(77));
    // IDA 0xc706e4 stores only the handle — the name is not retained.
    assert_eq!(pass.custom_type, "");
    assert_eq!(pass.inputs[3].name, "rt0");
    assert_eq!(pass.inputs[3].mrt_index, 1);
    let pass2 = g::stub_c70234(0x6000);
    assert_eq!(pass2.parent, 0x6000);
    g::stub_c70504();
    g::stub_c70510(pass);
    g::stub_c6f1f0();
    g::stub_c6f010();
    g::stub_c6f148(movable::OgreException::default());
    g::stub_c6f170(movable::OgreException::default());
    let with_any = movable::Renderable::default();
    assert_eq!(g::stub_c6eb08(&with_any).value, "");
    // generated_141's own stub shares the same setter.
    let mut direct = generated_141::CompositionPass::new(0);
    generated_141::stub_c708cc(&mut direct, 0, "x", 0);
    assert_eq!(direct.inputs[0].name, "x");
}

struct FakeSupport {
    techniques: usize,
    compiled: std::cell::Cell<usize>,
}

impl movable::PassMaterialSupport for FakeSupport {
    fn compile_material(&self, handle: usize) {
        self.compiled.set(self.compiled.get() + 1);
        let _ = handle;
    }
    fn supported_techniques(&self, _handle: usize) -> usize {
        self.techniques
    }
}

#[test]
fn stencil_quad_custom_and_support() {
    let mut pass = g::stub_c70234(0);
    assert_eq!(pass.stencil_func, 1);
    assert_eq!(pass.stencil_mask, u32::MAX);
    assert_eq!(pass.quad_corners, [-1.0, 1.0, 1.0, -1.0]);
    g::stub_c70a14(&mut pass, 7);
    g::stub_c70a18(&mut pass, true);
    g::stub_c70a20(&mut pass, 2);
    g::stub_c70a28(&mut pass, 128);
    g::stub_c70a30(&mut pass, 0xFF);
    g::stub_c70a38(&mut pass, 1);
    g::stub_c70a40(&mut pass, 2);
    g::stub_c70a48(&mut pass, 3);
    g::stub_c70a50(&mut pass, true);
    g::stub_c70a58(&mut pass, true, false);
    g::stub_c70a64(&mut pass, "custom");
    assert_eq!(pass.clear_stencil, 7);
    assert!(pass.stencil_check);
    assert_eq!(pass.stencil_func, 2);
    assert_eq!(pass.stencil_ref_value, 128);
    assert_eq!(pass.stencil_mask, 0xFF);
    assert_eq!(pass.stencil_fail_op, 1);
    assert_eq!(pass.stencil_depth_fail_op, 2);
    assert_eq!(pass.stencil_pass_op, 3);
    assert!(pass.stencil_two_sided);
    assert_eq!(pass.quad_far_corners, (true, false));
    assert_eq!(pass.custom_type, "custom");
    // Non-3 types pass without a material; type 3 needs compile + technique.
    pass.pass_type = 4;
    let no_mat = FakeSupport {
        techniques: 0,
        compiled: std::cell::Cell::new(0),
    };
    assert!(g::stub_c70a70(&pass, &no_mat));
    assert_eq!(no_mat.compiled.get(), 0);
    pass.pass_type = 3;
    assert!(!g::stub_c70a70(&pass, &no_mat));
    pass.material = Some(9);
    assert!(!g::stub_c70a70(&pass, &no_mat));
    let with_tech = FakeSupport {
        techniques: 2,
        compiled: std::cell::Cell::new(0),
    };
    assert!(g::stub_c70a70(&pass, &with_tech));
    assert_eq!(with_tech.compiled.get(), 1);
}

#[test]
fn target_pass_lifecycle() {
    let mut target = g::stub_c70ad8(0x9000, None);
    assert_eq!(target.parent, 0x9000);
    assert_eq!(target.material_scheme, "Default");
    assert_eq!(target.visibility_mask, u32::MAX);
    assert_eq!(target.lod_bias, 1.0);
    assert!(target.shadows_enabled);
    let rs = g::stub_c70ae4(0x9000, Some("GLSL"));
    assert_eq!(rs.material_scheme, "GLSL");
    g::stub_c70e08(&mut target, 2);
    g::stub_c70e0c(&mut target, "output");
    g::stub_c70e18(&mut target, true);
    g::stub_c70e1c(&mut target, 0xF0);
    g::stub_c70e20(&mut target, 0.5);
    g::stub_c70e24(&mut target, "Scheme");
    g::stub_c70e30(&mut target, false);
    assert_eq!(target.input_mode, 2);
    assert_eq!(target.output_name, "output");
    assert!(target.only_initial);
    assert_eq!(target.visibility_mask, 0xF0);
    assert_eq!(target.lod_bias, 0.5);
    assert_eq!(target.material_scheme, "Scheme");
    assert!(!target.shadows_enabled);
    let support = FakeSupport {
        techniques: 1,
        compiled: std::cell::Cell::new(0),
    };
    // Empty passes: AND over nothing is true (IDA 0xc70f38 break).
    assert!(g::stub_c70f2c(&target, &support));
    let idx = g::stub_c70e38(&mut target);
    assert_eq!(idx, 0);
    assert_eq!(target.passes.len(), 1);
    target.passes[0].pass_type = 4;
    assert!(g::stub_c70f2c(&target, &support));
    target.passes[0].pass_type = 3;
    assert!(!g::stub_c70f2c(&target, &support));
    g::stub_c70cb8();
    g::stub_c70cc4(target);
}

struct FakeTechSupport {
    inner: FakeSupport,
    max_mrt: u16,
    ok_formats: Vec<u32>,
}

impl movable::PassMaterialSupport for FakeTechSupport {
    fn compile_material(&self, handle: usize) {
        self.inner.compile_material(handle);
    }
    fn supported_techniques(&self, handle: usize) -> usize {
        self.inner.supported_techniques(handle)
    }
}

impl generated_141::TechniqueSupport for FakeTechSupport {
    fn max_mrt_buffers(&self) -> u16 {
        self.max_mrt
    }
    fn texture_format_ok(&self, format: u32, _srgb: bool) -> bool {
        self.ok_formats.contains(&format)
    }
}

fn tech_support(techs: usize, max_mrt: u16, formats: &[u32]) -> FakeTechSupport {
    FakeTechSupport {
        inner: FakeSupport {
            techniques: techs,
            compiled: std::cell::Cell::new(0),
        },
        max_mrt,
        ok_formats: formats.to_vec(),
    }
}

#[test]
fn technique_lifecycle_and_support() {
    let mut technique = g::stub_c71088(0x7000);
    assert_eq!(technique.parent, 0x7000);
    assert_eq!(technique.scheme_name, "");
    assert!(technique.texture_definitions().is_empty());
    assert!(technique.target_passes.is_empty());
    // Ctor-created output target pass carries the technique address.
    assert_eq!(g::stub_c71788(&technique).parent, &technique as *const _ as usize);
    g::stub_c718d0(&mut technique, "Mask");
    assert_eq!(technique.scheme_name, "Mask");
    let di = g::stub_c715e0(&mut technique, "rt");
    assert_eq!(di, 0);
    assert_eq!(g::stub_c71688(&technique).len(), 1);
    assert_eq!(g::stub_c71688(&technique)[0].name, "rt");
    assert_eq!(g::stub_c71688(&technique)[0].width_factor, 1.0);
    let ti = g::stub_c71694(&mut technique);
    assert_eq!(ti, 0);
    assert_eq!(technique.target_passes.len(), 1);
    // Output pass (type 3, no material) fails; flip it non-material.
    let support = tech_support(1, 4, &[7]);
    assert!(!g::stub_c7178c(&technique, false, &support));
    technique.output_target_pass.pass_type = 1;
    technique.target_passes[0].passes.push(generated_141::CompositionPass::new(0));
    technique.target_passes[0].passes[0].pass_type = 2;
    assert!(g::stub_c7178c(&technique, false, &support));
    // Format overflow fails even with valid passes.
    technique.texture_definitions[0].format_ids = vec![7, 7, 7, 7, 7];
    assert!(!g::stub_c7178c(&technique, false, &support));
    technique.texture_definitions[0].format_ids = vec![7];
    assert!(g::stub_c7178c(&technique, false, &support));
    technique.texture_definitions[0].format_ids = vec![9];
    assert!(!g::stub_c7178c(&technique, true, &support));
    g::stub_c71474(&mut technique);
    assert!(technique.texture_definitions().is_empty());
    let technique2 = g::stub_c71094(0x7001);
    assert_eq!(technique2.parent, 0x7001);
    g::stub_c712f0();
    g::stub_c712fc(technique);
    g::stub_c71260(technique2);
}
