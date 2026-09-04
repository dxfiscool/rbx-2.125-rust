use rbx_rendering::generated_202 as g;
use rbx_core::SharedPtr;

#[test]
fn block_copy_dispose_single() {
    let src = vec![Some(SharedPtr::new(()))];
    let mut dst: Vec<g::BlockSlot> = vec![None];
    g::stub_0x18094(&mut dst, &src);
    assert!(dst[0].is_some());
    assert!(SharedPtr::strong_count(dst[0].as_ref().unwrap()) == 2);
    g::stub_0x1c740(&mut dst);
    assert!(dst[0].is_none());
}

#[test]
fn block_copy_dispose_triple() {
    let src = vec![Some(SharedPtr::new(())), Some(SharedPtr::new(())), None];
    let mut dst: Vec<g::BlockSlot> = vec![None, None, None];
    g::stub_0x1ae78(&mut dst, &src);
    assert!(dst[0].is_some() && dst[1].is_some() && dst[2].is_none());
    g::stub_0x1aea8(&mut dst);
    assert!(dst.iter().all(|s| s.is_none()));
}

#[test]
fn appirater_singleton_fast_path() {
    g::set_appirater_delegate(0xABCD);
    let a = g::stub_0x17f80(0x1000);
    assert_eq!(a.delegate, 0xABCD);
    let b = g::stub_0x17f80(0x2000);
    assert!(std::ptr::eq(a, b));
    assert_eq!(b.delegate, 0xABCD);
}

#[test]
fn top_most_descends_to_deepest() {
    let leaf = SharedPtr::new(g::TopViewController { child: None });
    let mid = SharedPtr::new(g::TopViewController { child: Some(leaf.clone()) });
    let root = SharedPtr::new(g::TopViewController { child: Some(mid.clone()) });
    let top = g::stub_0x1a124(root);
    assert!(SharedPtr::ptr_eq(&top, &leaf));
    let solo = g::stub_0x1a124(mid);
    assert!(SharedPtr::ptr_eq(&solo, &leaf));
}

#[test]
fn main_records_launch_and_returns_zero() {
    let rc = g::stub_0x1a768(2, vec!["app".to_string(), "arg".to_string()]);
    assert_eq!(rc, 0);
}

#[test]
fn global_ctors_run_idempotently() {
    g::stub_0x17c58();
    g::stub_0x1a5d0();
    g::stub_0x1a7d4();
    g::stub_0x1b308();
    g::stub_0x16e4c();
    g::stub_0x17c58();
    g::stub_0x16e4c();
}

#[test]
fn batch2_single_copy_dispose_pairs() {
    // Spot-check every single-capture batch-2 pair retains then releases.
    let src = vec![Some(SharedPtr::new(()))];
    let mut dst: Vec<g::BlockSlot> = vec![None];
    g::stub_0x1c874(&mut dst, &src);
    assert!(dst[0].is_some());
    g::stub_0x1c880(&mut dst);
    assert!(dst[0].is_none());
    g::stub_0x1e2d8(&mut dst, &src);
    g::stub_0x1e2e4(&mut dst);
    assert!(dst[0].is_none());
    g::stub_0x1ed30(&mut dst, &src);
    g::stub_0x1ed3c(&mut dst);
    g::stub_0x1ee84(&mut dst, &src);
    g::stub_0x1ee90(&mut dst);
    g::stub_0x1ee98(&mut dst, &src);
    g::stub_0x1eea4(&mut dst);
    g::stub_0x1efdc(&mut dst, &src);
    g::stub_0x1efe8(&mut dst);
    g::stub_0x1eff0(&mut dst, &src);
    g::stub_0x1effc(&mut dst);
    g::stub_0x1f480(&mut dst, &src);
    g::stub_0x1f48c(&mut dst);
    g::stub_0x1f494(&mut dst, &src);
    g::stub_0x1f4a0(&mut dst);
    g::stub_0x1f660(&mut dst, &src);
    g::stub_0x1f66c(&mut dst);
    g::stub_0x1f688(&mut dst, &src);
    g::stub_0x1f694(&mut dst);
    g::stub_0x1f69c(&mut dst, &src);
    g::stub_0x1f6a8(&mut dst);
    assert!(dst[0].is_none());
}

#[test]
fn batch2_multi_capture_pairs() {
    let src3 = vec![Some(SharedPtr::new(())), Some(SharedPtr::new(())), Some(SharedPtr::new(()))];
    let mut dst3: Vec<g::BlockSlot> = vec![None, None, None];
    g::stub_0x1eb08(&mut dst3, &src3);
    assert!(dst3.iter().all(|s| s.is_some()));
    g::stub_0x1eb38(&mut dst3);
    assert!(dst3.iter().all(|s| s.is_none()));
    let src2 = vec![Some(SharedPtr::new(())), Some(SharedPtr::new(()))];
    let mut dst2: Vec<g::BlockSlot> = vec![None, None];
    g::stub_0x1ec44(&mut dst2, &src2);
    assert!(dst2.iter().all(|s| s.is_some()));
    g::stub_0x1ec68(&mut dst2);
    assert!(dst2.iter().all(|s| s.is_none()));
}

#[test]
fn batch2_login_singleton_and_global_ctor() {
    g::set_login_view_controller_shared(0x1234);
    assert_eq!(g::stub_0x1da5c(0x9999), 0x1234);
    g::set_login_view_controller_shared(0);
    assert_eq!(g::stub_0x1da5c(0x9999), 0);
    g::stub_0x1d870();
    g::stub_0x1d870();
}

#[test]
fn batch3_single_copy_dispose_pairs() {
    let src = vec![Some(SharedPtr::new(()))];
    let mut dst: Vec<g::BlockSlot> = vec![None];
    for (copy, dispose) in [
        (g::stub_0x1f82c as fn(&mut [g::BlockSlot], &[g::BlockSlot]), g::stub_0x1f838 as fn(&mut [g::BlockSlot])),
        (g::stub_0x1fa44, g::stub_0x1fa50),
        (g::stub_0x1fc90, g::stub_0x1fc9c),
        (g::stub_0x1fd24, g::stub_0x1fd30),
        (g::stub_0x20f08, g::stub_0x20f14),
        (g::stub_0x21adc, g::stub_0x21ae8),
        (g::stub_0x21b10, g::stub_0x21b1c),
        (g::stub_0x24a04, g::stub_0x24a10),
        (g::stub_0x253cc, g::stub_0x253d8),
    ] {
        copy(&mut dst, &src);
        assert!(dst[0].is_some());
        dispose(&mut dst);
        assert!(dst[0].is_none());
    }
}

#[test]
fn batch3_dual_copy_dispose_pairs() {
    let src = vec![Some(SharedPtr::new(())), Some(SharedPtr::new(()))];
    let mut dst: Vec<g::BlockSlot> = vec![None, None];
    for (copy, dispose) in [
        (g::stub_0x1fca4 as fn(&mut [g::BlockSlot], &[g::BlockSlot]), g::stub_0x1fcc8 as fn(&mut [g::BlockSlot])),
        (g::stub_0x1fce4, g::stub_0x1fd08),
        (g::stub_0x298a0, g::stub_0x298c4),
    ] {
        copy(&mut dst, &src);
        assert!(dst.iter().all(|s| s.is_some()));
        dispose(&mut dst);
        assert!(dst.iter().all(|s| s.is_none()));
    }
}

#[test]
fn batch3_global_ctors_and_place_launcher() {
    g::stub_0x202d0();
    g::stub_0x21c18();
    g::stub_0x24540();
    g::stub_0x202d0();
    let first: *const g::PlaceLauncher = g::stub_0x24974(0xabcd) as *const _;
    let second = g::stub_0x24974(0x1234) as *const _;
    assert_eq!(first, second);
    assert_eq!(unsafe { &*first }.class_token, 0xabcd);
    assert_eq!(g::stub_0x249d0(0x55).class_token, 0x55);
}

use rbx_rendering::generated_03 as g3;
use std::collections::HashMap;

struct Flag2d(bool);
impl g3::ShouldRender2d for Flag2d {
    fn should_render_2d(&self) -> bool {
        self.0
    }
}
struct Derived962d(Flag2d);
impl g3::Thunk96 for Derived962d {
    type Base = dyn g3::ShouldRender2d;
    fn adjusted_base(&self) -> &Self::Base {
        &self.0
    }
}

#[test]
fn batch4_handles_base_virtual_and_thunk() {
    assert!(g3::stub_3a8664(&Flag2d(true)));
    assert!(!g3::stub_3a8664(&Flag2d(false)));
    assert!(g3::stub_3a87dc(&Derived962d(Flag2d(true))));
    assert!(!g3::stub_3a87dc(&Derived962d(Flag2d(false))));
}

#[test]
fn batch4_billboard_render_cell_and_sorted_adorn() {
    let mut cell = g3::BillboardRenderCell::default();
    assert!(cell.render_fn.is_none());
    g3::stub_3c042c(&mut cell, Box::new(|_, _| {}));
    assert!(cell.render_fn.is_some());

    struct Host {
        enabled: bool,
        hit: bool,
    }
    impl g3::SortedAdornHost for Host {
        fn sorted_adorn_enabled(&self) -> bool {
            self.enabled
        }
        fn part_datamodel_hit(&self) -> bool {
            self.hit
        }
    }
    struct Derived(Host);
    impl g3::Thunk96 for Derived {
        type Base = dyn g3::SortedAdornHost;
        fn adjusted_base(&self) -> &Self::Base {
            &self.0
        }
    }
    assert!(g3::stub_3c04a8(&Host { enabled: true, hit: true }));
    assert!(!g3::stub_3c04a8(&Host { enabled: false, hit: true }));
    assert!(!g3::stub_3c04a8(&Host { enabled: true, hit: false }));
    assert!(g3::stub_3c066c(&Derived(Host { enabled: true, hit: true })));
    assert!(!g3::stub_3c066c(&Derived(Host { enabled: false, hit: true })));

    assert!(g3::stub_3f1c00());
    assert!(g3::stub_3f1c34());
}

struct TestWorld {
    items: HashMap<&'static str, SharedPtr<g3::GuiItemState>>,
    fps_flag: bool,
}
impl g3::RenderStatsWorld for TestWorld {
    fn find_gui_item(&self, name: &str) -> Option<SharedPtr<g3::GuiItemState>> {
        self.items.get(name).cloned()
    }
    fn debug_display_fps(&self) -> bool {
        self.fps_flag
    }
}

#[test]
fn batch4_render_stats_command_flow() {
    let cmd = g3::stub_3f6a90(0x777);
    assert_eq!(cmd.name, "RenderStats");
    assert_eq!(cmd.data_model, 0x777);
    let cmd1 = g3::stub_3f6a8c(0x777);
    assert_eq!(cmd1.name, "RenderStats");

    let mut items = HashMap::new();
    items.insert("RenderStats", SharedPtr::new(g3::GuiItemState::new(true)));
    items.insert("FPS", SharedPtr::new(g3::GuiItemState::new(false)));
    items.insert("StatsHud1", SharedPtr::new(g3::GuiItemState::new(true)));
    let world = TestWorld { items, fps_flag: true };

    assert!(g3::stub_3f6eb0(&cmd, &world));
    assert!(g3::stub_3f702c(&cmd, &world));
    g3::stub_3f6be8(&cmd, &world);
    assert!(!g3::stub_3f702c(&cmd, &world));
    assert!(world.items["FPS"].query_visible());

    let empty = TestWorld { items: HashMap::new(), fps_flag: false };
    assert!(!g3::stub_3f6eb0(&cmd, &empty));
    assert!(!g3::stub_3f702c(&cmd, &empty));
    g3::stub_3f6be8(&cmd, &empty);

    g3::stub_3fe43c(g3::stub_3f6a90(1));
    g3::stub_3fe440(Box::new(g3::stub_3f6a90(1)));
}

#[test]
fn batch4_mouse_cursor_branches() {
    use rbx_rendering::generated_03::MouseCursorContent;
    let base = g3::CursorDecision {
        mouse_lock_flag: true,
        settings_mode: 1,
        settings_feature_flag: true,
        local_player_present: true,
        adv_arrow_tool_enabled: false,
        server_present: true,
        workspace_cursor: 42,
    };
    assert!(matches!(g3::stub_4252ec(&base), MouseCursorContent::Assets("Textures/MouseLockedCursor.png")));
    let no_player = g3::CursorDecision { local_player_present: false, ..base };
    assert!(matches!(g3::stub_4252ec(&no_player), MouseCursorContent::Assets("Textures/ArrowCursor.png")));
    let adv = g3::CursorDecision {
        local_player_present: false,
        adv_arrow_tool_enabled: true,
        server_present: false,
        ..base
    };
    assert!(matches!(g3::stub_4252ec(&adv), MouseCursorContent::Assets("Textures/advCursor-default.png")));
    let off_path = g3::CursorDecision { mouse_lock_flag: false, ..base };
    assert!(matches!(g3::stub_4252ec(&off_path), MouseCursorContent::Assets("Textures/MouseLockedCursor.png")));
    let ws = g3::CursorDecision { mouse_lock_flag: false, local_player_present: false, ..base };
    assert!(matches!(g3::stub_4252ec(&ws), MouseCursorContent::Workspace(42)));
}

struct TestSvcHost {
    svc: Option<SharedPtr<g3::RenderHooksService>>,
    locked: bool,
    indexed: bool,
    published: usize,
    asserted: Vec<String>,
}
impl g3::RenderHooksServiceHost for TestSvcHost {
    fn find_render_hooks_service(&self) -> Option<SharedPtr<g3::RenderHooksService>> {
        self.svc.clone()
    }
    fn create_render_hooks_service(&mut self) -> SharedPtr<g3::RenderHooksService> {
        g3::stub_44e308()
    }
    fn lock_service_parent(&mut self, _svc: &SharedPtr<g3::RenderHooksService>) {
        self.locked = true;
    }
    fn init_service_class_index(&mut self) {
        self.indexed = true;
    }
    fn publish_render_hooks_service(&mut self, svc: &SharedPtr<g3::RenderHooksService>) {
        self.svc = Some(SharedPtr::clone(svc));
        self.published += 1;
    }
    fn debug_assert_service_registered(&self, name: &str) {
        assert_eq!(name, "RenderHooksService");
    }
}

#[test]
fn batch4_render_hooks_service_lifecycle() {
    let pre = g3::stub_44e308();
    let mut host = TestSvcHost { svc: Some(pre.clone()), locked: false, indexed: false, published: 0, asserted: vec![] };
    let found = g3::stub_435a28(&mut host).unwrap();
    assert!(SharedPtr::ptr_eq(&found, &pre));
    assert!(!host.locked && !host.indexed && host.published == 0);

    let mut host = TestSvcHost { svc: None, locked: false, indexed: false, published: 0, asserted: vec![] };
    let created = g3::stub_435a28(&mut host).unwrap();
    assert!(host.locked && host.indexed && host.published == 1);
    assert!(host.svc.as_ref().map(|s| SharedPtr::ptr_eq(s, &created)).unwrap());

    let mut dst = g3::stub_44e308();
    g3::stub_44e3b8(&mut dst, &created);
    assert_eq!(SharedPtr::strong_count(&dst), 3);

    assert_eq!(g3::stub_44e3ec(Some("RenderHooksService")), "RenderHooksService");
    assert_eq!(g3::stub_44e3ec(Some("RenderHooksService")), "RenderHooksService");
    assert_eq!(g3::stub_44e3ec(None), "");
    assert_eq!(g3::stub_44e430(), "RenderHooksService");
    assert_eq!(g3::stub_44e434(), "RenderHooksService");

    assert!(g3::stub_44e51c() != 0);
    assert_eq!(g3::stub_44e518(), g3::stub_44e51c());

    let via_box = g3::stub_44e5f4(Box::new(g3::RenderHooksService::new()));
    assert_eq!(SharedPtr::strong_count(&via_box), 1);

    let mut owner = None;
    g3::stub_44e6bc(&mut owner, &via_box);
    assert!(owner.as_ref().and_then(|w| w.upgrade()).is_some());
    g3::stub_44e6bc(&mut owner, &via_box);
    assert_eq!(SharedPtr::strong_count(&via_box), 1);
    let _ = host.asserted;
}
