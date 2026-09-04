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
