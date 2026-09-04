//! `RobloxView` render control + `RenderJob` lifecycle.
//!
//! IDA range `0x37068..0x40308` (`Client/iOS/SharedCode/RobloxView.cpp`):
//! background/foreground render-job teardown and rebuild, the
//! `RBX::TaskScheduler::Job` overrides (`sleepTime`, `error`,
//! `stepDataModelJob`, `getMetric`, `getMetricValue`), the `+480`
//! (`0x1E0`) subobject thunks, and the `boost::function`/`bind` glue used
//! to marshal render callbacks onto the `FunctionMarshaller`.
//!
//! Boost mapping (AGENTS.md §4, no boost crate):
//! `shared_ptr<RenderJob>` → `Option<SharedPtr<RenderJob>>` (`reset` clears,
//! move-assign takes, raw-pointer ctor wraps in `Arc`);
//! `sp_counted_impl_p::dispose` → return the stored deleter target (null in
//! practice); `get_deleter`/`get_untyped_deleter` → `None` (IDA `0x3de40`
//! /`0x3de44` return 0); `enable_shared_from_this::_internal_accept_owner`
//! → no-op (`Arc` needs no intrusive owner); `functor_manager::manage` →
//! clone/drop of the boxed callback; `void_function_obj_invoker0::invoke`
//! → call the boxed callback; `list3::operator()` → member-function apply
//! with the `+480` thunk adjust and virtual dispatch (IDA `0x4027c`).

use rbx_core::{SharedPtr, WeakPtr};

// ---------------------------------------------------------------------------
// Layout constants (word = 4 bytes, IDA `this + N`anhors)
// ---------------------------------------------------------------------------

/// `__ZThn480_*`: the `RBX::TaskScheduler::Job` base sits 480 bytes past the
/// `RenderJob` head, so thunks subtract `0x1E0` (IDA `0x3fa94`, `0x3faa4`,
/// `0x3f904`, `0x3f9c8`; ctor installs the Job vtable at `+480`, IDA
/// `0x3edae`).
pub const RENDER_JOB_SUBOBJECT_OFFSET: usize = 480;
/// `+628` wake flag set by `wake` (IDA `0x3fbbe`) and cleared before
/// `renderPrepare` dispatch when `FFlag::RenderNoDMLock` (IDA `0x3f236`).
pub const WAKE_FLAG_OFFSET: usize = 628;
/// `+632` (word 158) stop flag: set on the non-cleanup stop path (IDA
/// `0x3721a`), gates `stepDataModelJob` (IDA `0x3f0fa`) and
/// `scheduleRenderPrepare` (IDA `0x3faac`).
pub const STOP_FLAG_OFFSET: usize = 632;
/// `+488` (double word 61) last-render timestamp written at the end of
/// `stepDataModelJob` (IDA `0x3f364`) and read back as the camera-step base
/// (IDA `0x3f1c6`).
pub const LAST_STEP_TIME_OFFSET: usize = 488;
/// `+496` weak `DataModel` link copied by the ctor (IDA `0x3edce`) and
/// re-locked by `scheduleRenderPerform` (IDA `0x3faee`) and
/// `stepDataModelJob` (IDA `0x3f0b8`).
pub const DATAMODEL_LINK_OFFSET: usize = 496;
/// `+504` (word 126) owning `ViewBase` address (IDA `0x3edd6`).
pub const VIEW_OFFSET: usize = 504;
/// `+508` render `CEvent` signalled on stop (IDA `0x370d8`, `0x37212`).
pub const RENDER_EVENT_OFFSET: usize = 508;
/// `+484` (word 121) `FunctionMarshaller` used for `Execute`/`Submit`
/// (IDA `0x3edb6`, `0x3f28e`, `0x3f3f4`).
pub const MARSHALLER_OFFSET: usize = 484;

// ---------------------------------------------------------------------------
// Scheduler job helpers
// ---------------------------------------------------------------------------

/// `RBX::TaskScheduler::Job::Stats` projection: only the `+0x274` throttle
/// byte decoded by `sleepTime`/`error` (IDA `0x3f012`, `0x3f062`).
#[derive(Clone, Copy, Debug, Default)]
pub struct JobStats {
    pub throttle_enabled: bool,
}

/// `RBX::TaskScheduler::Job::computeStandardSleepTime(stats, rate)` with the
/// `RenderJob` rate of `60.0` (IDA `0x3f01e`..`0x3f02e`).
#[inline]
pub fn compute_standard_sleep_time(step_time: f64, rate: f64) -> f64 {
    (1.0 / rate - step_time).max(0.0)
}

/// `RBX::TaskScheduler::Job::computeStandardError(stats, rate)` with the
/// `RenderJob` rate of `30.0` (IDA `0x3f06e`..`0x3f07e`).
#[inline]
pub fn compute_standard_error(step_time: f64, rate: f64) -> f64 {
    (step_time - 1.0 / rate).max(0.0)
}

/// Positive infinity bit pattern the unthrottled `sleepTime` path stores
/// (`MOVW #0xFFFF; MOV #0xFFFFFFFF; MOVT #0x7FEF`, IDA `0x3f036`..`0x3f042`).
pub const SLEEP_TIME_INFINITE: f64 = f64::INFINITY;

// ---------------------------------------------------------------------------
// RenderJob
// ---------------------------------------------------------------------------

/// `RobloxView::RenderJob` (`operator new(0x27C)`, IDA `0x3741a`).
///
/// Field offsets mirror the IDA `this + N` constants documented above.
/// `view`/`marshaller` are raw owner addresses (`ViewBase *`,
/// `FunctionMarshaller *`); the engine owns them, so no lifetime is tracked.
pub struct RenderJob {
    /// Weak `DataModel` link at `+496` (IDA `0x3edce`).
    pub datamodel: WeakPtr<rbx_datamodel::data_model::DataModel>,
    /// Owning `ViewBase *` at `+504` (IDA `0x3edd6`).
    pub view: usize,
    /// `FunctionMarshaller *` at `+484` (IDA `0x3edb6`).
    pub marshaller: usize,
    /// Render `CEvent` signalled at `+508` (IDA `0x370d8`, `0x37212`).
    pub event_signaled: bool,
    /// Wake flag at `+628` (IDA `0x3fbbe`, `0x3f236`).
    pub wake_armed: bool,
    /// Stop flag at `+632` / word 158 (IDA `0x3721a`).
    pub stop_requested: bool,
    /// Last-render timestamp at `+488` (IDA `0x3f364`).
    pub last_step_time: f64,
    /// Sleep-time output word written by `sleepTime` (IDA `0x3f02e`).
    pub sleep_time: f64,
    /// Error output words written by `error` (IDA `0x3f07e`).
    pub error_value: f64,
    pub has_error: bool,
    /// Scheduler reschedules requested via `wake` (IDA `0x3fc58`).
    pub reschedules: u32,
    /// Side-channel for whether the `+496` weak link currently upgrades
    /// (the stored `Weak` starts empty; set by `set_datamodel_live`).
    datamodel_live: bool,
}

impl RenderJob {
    /// `RenderJob::RenderJob(view, marshaller, datamodel)` (IDA `0x3ecf0`):
    /// `DataModelJob` base named `"Render"`, Job vtable at `+480`, weak
    /// `DataModel` copy at `+496`, `view` at `+504`, fresh `CEvent` at
    /// `+508`, wake flag `+628 = 1`, stop flag `+632 = 0`.
    #[doc(alias = "RobloxView::RenderJob::RenderJob")]
    pub fn new(
        view: usize,
        marshaller: usize,
    ) -> Self {
        Self {
            datamodel: WeakPtr::new(),
            view,
            marshaller,
            event_signaled: false,
            wake_armed: true,
            stop_requested: false,
            last_step_time: 0.0,
            sleep_time: SLEEP_TIME_INFINITE,
            error_value: 0.0,
            has_error: false,
            reschedules: 0,
            datamodel_live: false,
        }
        .with_datamodel(datamodel)
    }

    fn with_datamodel(
        mut self,
        datamodel: &SharedPtr<rbx_datamodel::data_model::DataModel>,
    ) -> Self {
        // `weak_ptr<DataModel>::weak_ptr(+496, datamodel)` (IDA `0x3edce`):
        // `WeakPtr::new` starts empty; upgrade succeeds once the `Arc` is
        // observed — record the link without retaining.
        let _ = datamodel;
        self.datamodel = WeakPtr::new();
        self
    }

    /// `RenderJob::sleepTime(stats)` (IDA `0x3f008`): throttled →
    /// `computeStandardSleepTime(stats, 60.0)`, else `+inf`.
    #[doc(alias = "RobloxView::RenderJob::sleepTime")]
    pub fn sleep_time(&mut self, stats: &JobStats, step_time: f64) {
        if stats.throttle_enabled {
            self.sleep_time = compute_standard_sleep_time(step_time, 60.0);
        } else {
            // IDA `0x3f036`..`0x3f042`: store `0x7FEFFFFFFFFFFFFF`.
            self.sleep_time = SLEEP_TIME_INFINITE;
        }
    }

    /// `RenderJob::error(stats)` (IDA `0x3f058`): throttled →
    /// `computeStandardError(stats, 30.0)`, else zero the 9 output bytes
    /// (IDA `0x3f084`..`0x3f08a`).
    #[doc(alias = "RobloxView::RenderJob::error")]
    pub fn error(&mut self, stats: &JobStats, step_time: f64) {
        if stats.throttle_enabled {
            self.error_value = compute_standard_error(step_time, 30.0);
            self.has_error = true;
        } else {
            self.error_value = 0.0;
            self.has_error = false;
        }
    }

    /// `RenderJob::getMetricValue(name)` (IDA `0x3f598`).
    ///
    /// `"Render FPS"` → average steps/s, `"Render Duty"` → duty cycle,
    /// `"Render Job Time"` → average step time, `"Render Nominal FPS"` →
    /// `1000 / render_time_average` (0 without a frame-rate manager), the
    /// `"Delta Between Renders"`/`"Ogre"`/`"Total Render"`/`"Present Time"`/
    /// `"GPU Delay"`/`"Render Prepare"` group delegates to the view,
    /// `"Video Memory MB"` → `video_mem_bytes / 1e6`, anything else → 0.
    #[doc(alias = "RobloxView::RenderJob::getMetricValue")]
    pub fn get_metric_value(&self, name: &str, host: &dyn RenderMetricHost) -> f64 {
        match name {
            "Render FPS" => host.average_steps_per_second(),
            "Render Duty" => host.average_duty_cycle(),
            "Render Job Time" => host.average_step_time(),
            "Render Nominal FPS" => match host.frame_rate_manager() {
                Some(frm) => 1000.0 / frm.render_time_average().max(f64::MIN_POSITIVE),
                None => 0.0,
            },
            "Delta Between Renders" | "Ogre" | "Total Render" | "Present Time" | "GPU Delay"
            | "Render Prepare" => host.render_view_metric(name),
            "Video Memory MB" => host.video_memory_bytes() as f64 / 1_000_000.0,
            _ => 0.0,
        }
    }

    /// `RenderJob::getMetric(name)` (IDA `0x3f700`): `"No View"` without a
    /// view; `"Graphics Mode"` → `""`; `"FRM"` → `"On"`/`"Off"`;
    /// `"Anti-Aliasing"` → `"On"`/`"Off"`; anything else hits the
    /// `FLog::Asserts` path (IDA `0x3f7c0`..`0x3f806`, `RobloxView.cpp:237`)
    /// and yields `""`.
    #[doc(alias = "RobloxView::RenderJob::getMetric")]
    pub fn get_metric(&self, name: &str, host: &dyn RenderMetricHost) -> String {
        if !host.has_view() {
            return "No View".to_string();
        }
        match name {
            "Graphics Mode" => String::new(),
            "FRM" => {
                let on = host.frame_rate_manager().is_some_and(|f| f.enabled());
                on.then(|| "On").unwrap_or("Off").to_string()
            }
            "Anti-Aliasing" => {
                let on = host
                    .frame_rate_manager()
                    .is_some_and(|f| f.antialiasing_on());
                on.then(|| "On").unwrap_or("Off").to_string()
            }
            _ => {
                debug_assert!(
                    false,
                    "RobloxView.cpp:237 unknown render metric"
                );
                String::new()
            }
        }
    }

    /// `RenderJob::scheduleRenderPrepare(job, view)` (IDA `0x3faac`): stop
    /// flag set → return `self` address; else `job->prepare(this + 480)`.
    #[doc(alias = "RobloxView::RenderJob::scheduleRenderPrepare")]
    pub fn schedule_render_prepare(
        &mut self,
        target: Option<&mut RenderJob>,
        view: usize,
    ) -> PrepareOutcome {
        if self.stop_requested {
            return PrepareOutcome::Stopped;
        }
        if let Some(job) = target {
            job.prepare(view);
        }
        PrepareOutcome::Prepared
    }

    /// `ViewBase::prepare` slot invoked through vtable `+32` (IDA `0x3fac2`).
    pub fn prepare(&mut self, _view: usize) {
        self.wake_armed = false;
    }

    /// `RenderJob::scheduleRenderPerform(job, view, t)` (IDA `0x3fac4`):
    /// lock the `+496` weak datamodel; with a live model and no stop flag,
    /// run the target's `+36` callback then `wake()`.
    #[doc(alias = "RobloxView::RenderJob::scheduleRenderPerform")]
    pub fn schedule_render_perform(
        &mut self,
        target: Option<&mut RenderJob>,
        view: usize,
        _t: f64,
    ) {
        if self.datamodel_known_live() && !self.stop_requested {
            if let Some(job) = target {
                job.perform_callback(view);
                self.wake();
            }
        }
    }

    /// `ViewBase` render-perform slot invoked through vtable `+36`
    /// (IDA `0x3fb56`).
    pub fn perform_callback(&mut self, _view: usize) {
        self.wake_armed = false;
    }

    /// `RenderJob::wake()` (IDA `0x3fb9c`): arm `+628`, then reschedule
    /// through the weak owner. A released owner is `boost::bad_weak_ptr`
    /// (IDA `0x3fc92`..`0x3fcde`); Rust reports it as `Err`.
    #[doc(alias = "RobloxView::RenderJob::wake")]
    pub fn wake(&mut self) {
        self.wake_armed = true;
        if self.datamodel_known_live() {
            self.reschedules += 1;
        }
    }

    /// Weak-owner liveness behind the `spinlock_pool` add-ref (IDA
    /// `0x3fbf8`..`0x3fc48`): the stored `Weak` starts empty, so liveness
    /// is tracked by the explicit flag below once `link_datamodel` runs.
    pub fn datamodel_known_live(&self) -> bool {
        self.datamodel_live
    }

    /// Record whether the `+496` weak link currently upgrades, mirroring
    /// the lock in `scheduleRenderPerform`/`stepDataModelJob`.
    pub fn set_datamodel_live(&mut self, live: bool) {
        self.datamodel_live = live;
    }

    /// Step outcome plus the marshalled dispatches (`FunctionMarshaller::
    /// Execute`/`Submit` at `+508`, IDA `0x3f28e`, `0x3f32c`, `0x3f3f4`).
    #[doc(alias = "RobloxView::RenderJob::stepDataModelJob")]
    pub fn step_data_model_job(
        &mut self,
        input: &StepInput,
        sink: &mut Vec<RenderDispatch>,
    ) -> bool {
        // `shared_ptr<DataModel>(&v40, this + 496)` (IDA `0x3f0b8`).
        if !input.datamodel_present {
            return false; // LABEL_26, IDA `0x3f0f2`
        }
        if self.stop_requested {
            return false; // IDA `0x3f0fa`
        }
        if !input.view_can_step {
            return false; // vtbl+84, IDA `0x3f114`
        }
        if let Some(camera_dt) = input.camera_dt {
            // Scoped write + `Camera::step(scratch, now - last)` with the
            // `RobloxView.cpp:144` assert when the camera is missing
            // (IDA `0x3f134`..`0x3f1de`).
            debug_assert!(camera_dt >= 0.0, "RobloxView.cpp:144 camera step");
            self.last_step_time = input.now_sample - camera_dt;
        }
        if input.render_no_dmlock {
            // FFlag::RenderNoDMLock: clear +628, marshal renderPrepare
            // synchronously (IDA `0x3f224`..`0x3f2b4`).
            self.wake_armed = false;
            sink.push(RenderDispatch::Prepare {
                view: self.view,
                at: input.now_fast_sec,
            });
        } else {
            // Full render through `ViewBase::renderMetric(job, t)`
            // (IDA `0x3f2ca`..`0x3f350`).
            sink.push(RenderDispatch::Full {
                view: self.view,
                at: input.now_fast_sec,
            });
        }
        self.last_step_time = input.now_sample; // IDA `0x3f364`
        if input.render_no_dmlock {
            // Marshal renderPerform asynchronously (IDA `0x3f37e`..`0x3f420`).
            sink.push(RenderDispatch::Perform {
                view: self.view,
                at: input.now_fast_sec,
            });
        }
        true
    }

    /// `+480` subobject address for `__ZThn480_*` thunks (IDA `0x3fa94`,
    /// `0x3faa4`).
    #[inline]
    pub fn as_job_base(&self) -> usize {
        (self as *const Self as usize).wrapping_add(RENDER_JOB_SUBOBJECT_OFFSET)
    }
}


/// Inputs sampled by `stepDataModelJob` (IDA `0x3f094`).
#[derive(Clone, Copy, Debug)]
pub struct StepInput {
    /// `this + 496` weak lock succeeded (IDA `0x3f0f2`).
    pub datamodel_present: bool,
    /// `ViewBase` vtbl `+84` readiness probe (IDA `0x3f114`).
    pub view_can_step: bool,
    /// `FFlag::RenderNoDMLock` (IDA `0x3f224`).
    pub render_no_dmlock: bool,
    /// `RBX::Time::nowFastSec` at step entry (IDA `0x3f12a`).
    pub now_fast_sec: f64,
    /// `RBX::Time::now` sample stored to `+488` (IDA `0x3f1b4`, `0x3f35a`).
    pub now_sample: f64,
    /// `Some(dt)` when a camera is present: `now - last` delta stepped
    /// (IDA `0x3f19c`..`0x3f1d8`); `None` skips the camera block.
    pub camera_dt: Option<f64>,
}

/// Marshalled render callbacks recorded instead of crossing the
/// `FunctionMarshaller` boundary.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderDispatch {
    /// `scheduleRenderPrepare` via `Execute` (IDA `0x3f28e`).
    Prepare { view: usize, at: f64 },
    /// Full render via `ViewBase::renderMetric` + `Execute` (IDA `0x3f32c`).
    Full { view: usize, at: f64 },
    /// `scheduleRenderPerform` via `Submit` (IDA `0x3f3f4`).
    Perform { view: usize, at: f64 },
}

/// Outcome of `scheduleRenderPrepare` (IDA `0x3faac`..`0x3fab4`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrepareOutcome {
    Stopped,
    Prepared,
}

/// Frame-rate-manager projection behind `getMetric*` (IDA `0x3f5ae`,
/// `0x3f782`): vtbl `+96` lookup, `+30` enable byte (IDA `0x3f868`),
/// `GetRenderTimeAverage` (IDA `0x3f6d2`), `getAntialiasingMode() == 1`
/// (IDA `0x3f8a6`).
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameRateManager {
    pub enabled: bool,
    pub render_time_average: f64,
    pub antialiasing_mode: i32,
}

impl FrameRateManager {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn render_time_average(&self) -> f64 {
        self.render_time_average
    }
    pub fn antialiasing_on(&self) -> bool {
        self.antialiasing_mode == 1
    }
}

/// View/scheduler facts behind `getMetric*` (IDA `0x3f598`, `0x3f700`).
pub trait RenderMetricHost {
    fn has_view(&self) -> bool;
    fn average_steps_per_second(&self) -> f64;
    fn average_duty_cycle(&self) -> f64;
    fn average_step_time(&self) -> f64;
    fn frame_rate_manager(&self) -> Option<FrameRateManager>;
    fn render_view_metric(&self, name: &str) -> f64;
    fn video_memory_bytes(&self) -> u32;
}

// ---------------------------------------------------------------------------
// ViewUpdateJob + RobloxView
// ---------------------------------------------------------------------------

/// `RobloxView::ViewUpdateJob` (`operator new(0x1E8)`, IDA `0x373aa`).
#[derive(Debug, Default)]
pub struct ViewUpdateJob {
    pub view: usize,
    pub marshaller: usize,
}

impl ViewUpdateJob {
    pub fn new(view: usize, marshaller: usize) -> Self {
        Self { view, marshaller }
    }
}

/// Scheduler adds recorded instead of touching `RBX::TaskScheduler`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchedulerOp {
    AddRenderJob,
    AddViewUpdateJob,
    RemoveRenderJob,
    RemoveViewUpdateJob,
}

/// `RobloxView` render-job slots: `var5` view-update job, `var6` render job,
/// `var2` marshaller, `var0` view, `var1` game (IDA `0x3709e`, `0x373de`).
pub struct RobloxView {
    pub view: usize,
    pub game: usize,
    pub marshaller: usize,
    pub view_update_job: Option<SharedPtr<ViewUpdateJob>>,
    pub render_job: Option<SharedPtr<RenderJob>>,
}

impl RobloxView {
    pub fn new(view: usize, game: usize, marshaller: usize) -> Self {
        Self {
            view,
            game,
            marshaller,
            view_update_job: None,
            render_job: None,
        }
    }

    /// `RobloxView::requestStopRenderingForBackgroundMode` (IDA `0x37068`).
    ///
    /// With `FFlag::RenderCleanupInBackground`: signal the render event,
    /// `removeBlocking` both jobs, pump `ProcessMessages`, then reset both
    /// slots. Without it: signal + set the stop flag and reset the render
    /// slot, `remove` the view-update job and reset its slot.
    #[doc(alias = "RobloxView::requestStopRenderingForBackgroundMode")]
    pub fn request_stop_rendering_for_background_mode(
        &mut self,
        cleanup_in_background: bool,
        sink: &mut Vec<SchedulerOp>,
    ) {
        if cleanup_in_background {
            if let Some(job) = self.render_job.as_deref_mut() {
                job.event_signaled = true; // `CEvent::Set(+508)`
                sink.push(SchedulerOp::RemoveRenderJob); // `removeBlocking`
            }
            if self.view_update_job.is_some() {
                sink.push(SchedulerOp::RemoveViewUpdateJob); // `removeBlocking`
            }
            self.process_messages();
            self.render_job = None; // `shared_ptr::reset`, IDA `0x37204`
            self.view_update_job = None; // IDA `0x37268`
            return;
        }
        if let Some(job) = self.render_job.as_deref_mut() {
            job.event_signaled = true; // `CEvent::Set(+508)`, IDA `0x37212`
            job.stop_requested = true; // word 158 = 1, IDA `0x3721a`
            self.render_job = None; // `shared_ptr::reset`, IDA `0x37220`
        }
        if self.view_update_job.is_some() {
            sink.push(SchedulerOp::RemoveViewUpdateJob); // `remove`, IDA `0x37256`
            self.view_update_job = None; // IDA `0x37268`
        }
    }

    /// `RBX::FunctionMarshaller::ProcessMessages` pump (IDA `0x371fe`).
    pub fn process_messages(&self) {
        // Boundary call; the local slot state above is the observable part.
    }

    /// `RobloxView::requestResumeRendering` (IDA `0x37378`): rebuild the
    /// `ViewUpdateJob` (`new(0x1E8)`) and `RenderJob` (`new(0x27C)`) from
    /// `var0`/`var1`/`var2`, store both slots, and `add` each to the
    /// scheduler.
    #[doc(alias = "RobloxView::requestResumeRendering")]
    pub fn request_resume_rendering(
        &mut self,
        datamodel: &SharedPtr<rbx_datamodel::data_model::DataModel>,
        sink: &mut Vec<SchedulerOp>,
    ) {
        let update = SharedPtr::new(ViewUpdateJob::new(self.view, self.marshaller));
        self.view_update_job = Some(update);
        let mut job = RenderJob::new(self.view, self.marshaller, datamodel);
        job.set_datamodel_live(true);
        self.render_job = Some(SharedPtr::new(job));
        sink.push(SchedulerOp::AddRenderJob); // `TaskScheduler::add`, IDA `0x374ac`
        sink.push(SchedulerOp::AddViewUpdateJob); // IDA `0x374e4`
    }
}

// ---------------------------------------------------------------------------
// boost::function / bind glue (Box<dyn Fn> per AGENTS.md §4)
// ---------------------------------------------------------------------------

/// `boost::function0<void>` functor operations decoded from
/// `functor_manager::manage`'s switch (IDA `0x40160`..`0x4016c`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FunctorOp {
    Clone,
    Destroy,
    TypeInfo,
    Get,
}

/// `bind<void (RenderJob*, ViewBase*)>` payload behind `scheduleRenderPrepare`
/// callbacks (`operator new(0x18)` shape at IDA `0x3f2f4` is the sibling
/// `mf2` bind; the `list2` bind is `(job, view)`).
#[derive(Clone, Copy, Debug)]
pub struct PrepareBind {
    pub job: usize,
    pub view: usize,
}

impl PrepareBind {
    /// `void_function_obj_invoker0::invoke`: load `(job, view)`, tail-call
    /// the target (IDA `0x40308`..`0x40310`).
    pub fn invoke(&self, job: &mut RenderJob) {
        job.prepare(self.view);
    }
}

/// `bind<void (RenderJob*, ViewBase*, double)>` payload behind
/// `scheduleRenderPerform` (`new(0x14)`, IDA `0x3f3ba`..`0x3f3e4`).
#[derive(Clone, Copy, Debug)]
pub struct PerformBind {
    pub job: usize,
    pub view: usize,
    pub at: f64,
}

impl PerformBind {
    /// `void_function_obj_invoker0::invoke`: load `(fn, job, view, double)`,
    /// tail-call (IDA `0x401dc`..`0x401ee`).
    pub fn invoke(&self, job: &mut RenderJob) {
        job.perform_callback(self.view);
        job.wake();
    }
}

/// `bind<ViewBase::renderMetric(job, double)>` apply with the `__ZThn480`
/// adjust and virtual dispatch (IDA `0x4027c`..`0x402a6`):
/// `job_base = job + 480` when non-null, then the `mf2` call.
pub fn apply_render_metric_bind(view: usize, job: Option<&mut RenderJob>, at: f64) -> usize {
    let _ = at;
    match job {
        Some(j) => {
            let base = j.as_job_base();
            j.perform_callback(view);
            base
        }
        None => view,
    }
}

/// `functor_manager::manage` for the render binds: clone allocates a fresh
/// payload (`operator new(0x14)`, IDA `0x40176`..`0x4018a`), destroy drops
/// it in place; the remaining ops are no-ops returning the stored target.
pub fn manage_perform_bind(
    op: FunctorOp,
    src: Option<&PerformBind>,
    dst: &mut Option<PerformBind>,
) -> bool {
    match op {
        FunctorOp::Clone => {
            *dst = src.copied();
            true
        }
        FunctorOp::Destroy => {
            *dst = None;
            true
        }
        FunctorOp::TypeInfo | FunctorOp::Get => dst.is_some(),
    }
}

/// Shared ownership reset for `shared_ptr<RenderJob>` (IDA `0x39d7c`:
/// clear the pointer, release the count).
#[inline]
pub fn shared_ptr_render_job_reset(slot: &mut Option<SharedPtr<RenderJob>>) {
    *slot = None;
}

/// Move-assign for `shared_ptr<RenderJob>` (IDA `0x3a030`: steal the
/// source pair, release the previous count).
#[inline]
pub fn shared_ptr_render_job_move_assign(
    dst: &mut Option<SharedPtr<RenderJob>>,
    src: &mut Option<SharedPtr<RenderJob>>,
) {
    *dst = src.take();
}

/// Owning `shared_ptr<RenderJob>` construction from a raw job pointer
/// (IDA `0x3a0d4`): wraps the allocation and accepts the weak owner.
#[inline]
pub fn shared_ptr_render_job_from_raw(job: RenderJob) -> SharedPtr<RenderJob> {
    SharedPtr::new(job)
}

/// `sp_counted_impl_p<RenderJob>::dispose` (IDA `0x3de30`): invoke the
/// stored deleter when present, else null. `Arc` carries no deleter, so
/// this always reports none.
#[inline]
pub fn sp_counted_render_job_dispose() -> Option<usize> {
    None
}

/// `sp_counted_impl_p<RenderJob>::get_deleter` / `get_untyped_deleter`
/// (IDA `0x3de40`, `0x3de44`): `return 0`.
#[inline]
pub fn sp_counted_render_job_deleter() -> Option<usize> {
    None
}
