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
