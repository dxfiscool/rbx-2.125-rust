// Auto-generated skeletons for rbx-script — filler EA-sorted asc (global holes)
// Filter: Lua|Script|Yield|lua (case-sensitive, lua lower) -> 5401 filtered, all stubbed (0 remaining)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x43a98..0x9fa10 | filtered 5401 done, script 11685->11785 total, global 79821->79921 covered, 5625 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::sync::LazyLock;

/// `rbx::signals` slot courts shared by the UIEvent `(bool,void*,UIEvent)`
/// and TextBox `(shared_ptr<TextBox>)` flavors below (IDA 0x45b74..0x4a908):
/// the decompiles are shape-identical across flavors (`connected` reads
/// `a1+12 != 0` at 0x45d64/0x4a904; empty-function dispatch throws
/// `bad_function_call` at 0x45e1a).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SlotConn {
    pub connected: bool,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SlotFn {
    pub armed: bool,
    pub calls: u32,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SlotList {
    pub slots: u32,
    pub removed: u32,
}
/// UIEvent-flavor slot static mutex (IDA 0x45fa4: guarded once-init at
/// 0x46000..0x46068).
static SLOT_UIEVENT_MUTEX: LazyLock<u32> = LazyLock::new(|| 1);
/// TextBox-flavor slot static mutex (IDA 0x4a540, same shape as 0x45fa0).
static SLOT_TEXTBOX_MUTEX: LazyLock<u32> = LazyLock::new(|| 1);
/// `StandardOutMessage`-flavor slot static mutex (IDA 0x65000..0x65004,
/// same shape as 0x45fa0).
static SLOT_STDOUT_MUTEX: LazyLock<u32> = LazyLock::new(|| 1);

/// `std::vector<void*>` observable state (IDA 0x62f08..0x63028): the
/// begin/end/capacity triple folds into the host `Vec`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PtrVec {
    pub items: Vec<u32>,
}

/// `FMOD::CodecMPEG` layer-3 decode courts (IDA 0x9b1f8..0x9eb14): the
/// bit-reader position, parsed side info, scalefactor bands, synth-filter
/// readiness, and produced-sample count. The Huffman/pow-table DSP folds
/// into the host; the models below preserve the observable zero/scale
/// structure (silence in, silence out; linear in the dequant scale).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MpegState {
    pub tables_ready: bool,
    pub main_data_begin: u32,
    pub private_bits: u32,
    pub scalefactors: [u32; 22],
    pub produced: u32,
}

/// `FMOD::MemoryTracker` byte counter (IDA 0x9fa10: `add` of 128 bytes).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FmodMem {
    pub bytes: u32,
}

// 0x45b74 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot() [0x45b74]")]
pub fn stub_0x45b74() {
    // IDA 0x45b74: UIEvent `callable_slot` D0 deleting dtor — same
    // vtable-reset/function-clear/release shape as D1 0x45aa0; drop glue
    // covers it — no-op.
}

// 0x45c4c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::disconnect(void)")]
pub fn stub_0x45c4c(slot: &mut SlotConn) {
    // IDA 0x45c4c: `slot::disconnect` — when the slot holds a connection
    // (a1+12 at 0x45c76), one-shots the slot mutex (0x45cb6), locks it,
    // and removes the slot from the signal list; the mutex/list glue
    // folds into the host.
    slot.connected = false;
}

// 0x45d5c — __ZNK3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::connected(void)const")]
pub fn stub_0x45d5c(slot: &SlotConn) -> bool {
    // IDA 0x45d5c: `slot::connected` answers `a1+12 != 0` (0x45d64).
    slot.connected
}

// 0x45d68 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
pub fn stub_0x45d68(f: &mut SlotFn) {
    // IDA 0x45d68: `callable<...,3>::call` forwards to the function
    // dispatch (0x45d94) — see `stub_0x45dc8`.
    stub_0x45dc8(f);
}

// 0x45d98 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
pub fn stub_0x45d98(f: &mut SlotFn) {
    // IDA 0x45d98: `Thn4` adjustor thunk shifts `this` by -4 and runs the
    // same `call` dispatch (cf. 0x3a7d8 shape).
    stub_0x45dc8(f);
}

// 0x45dc8 — __ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::operator()(bool,void *,RBX::UIEvent)const")]
pub fn stub_0x45dc8(f: &mut SlotFn) {
    // IDA 0x45dc8: `function3::operator()` throws `bad_function_call`
    // when the function is empty (0x45e1a..0x45e7e) and dispatches
    // otherwise; the callable body folds into the host.
    if !f.armed {
        panic!("bad_function_call");
    }
    f.calls += 1;
}

// 0x45eb0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::remove(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
pub fn stub_0x45eb0(list: &mut SlotList) {
    // IDA 0x45eb0: `signal::remove` asserts the item is alive
    // (`!intrusive_ptr_expired` at 0x45ec4..0x45efa), logs the removal
    // (0x45f18..0x45f26), and unlinks it from the list (0x45f2a..);
    // the assert/log/list glue folds into the host.
    list.slots = list.slots.saturating_sub(1);
    list.removed += 1;
}

// 0x45fa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x45fa0() -> u32 {
    // IDA 0x45fa0: UIEvent slot `safe_static_init_mutex` — see
    // `SLOT_UIEVENT_MUTEX`.
    *SLOT_UIEVENT_MUTEX
}

// 0x45fa4 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x45fa4() -> u32 {
    // IDA 0x45fa4: UIEvent slot `safe_static_do_get_mutex` one-shots the
    // static slot mutex behind a `__cxa_guard` (0x46000..0x4603e) and
    // answers it (0x46068). The opaque handle records once.
    *SLOT_UIEVENT_MUTEX
}

// 0x46094 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
pub fn stub_0x46094() {
    // IDA 0x46094: `callable<...,3>` D1 dtor — destroys the function and
    // releases the slot; drop glue covers it — no-op.
}

// 0x46168 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable() [0x46168]")]
pub fn stub_0x46168() {
    // IDA 0x46168: `callable<...,3>` D0 deleting dtor — same body as D1
    // plus the delete; drop glue covers it — no-op.
}

// 0x46240 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
pub fn stub_0x46240() {
    // IDA 0x46240: UIEvent `slot` D1 dtor — disconnects and destroys the
    // connection record; drop glue covers it — no-op.
}

// 0x462ec — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot() [0x462ec]")]
pub fn stub_0x462ec() {
    // IDA 0x462ec: UIEvent `slot` D0 deleting dtor — same body as D1 plus
    // the delete; drop glue covers it — no-op.
}

// 0x4639c — __ZN5boost9function3IvbPvN3RBX7UIEventEE13assign_to_ownERKS4_
// type: int(void)
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::assign_to_own(boost::function3<void,bool,void *,RBX::UIEvent> const&)")]
pub fn stub_0x4639c(dst: &mut SlotFn, src: &SlotFn) {
    // IDA 0x4639c: `function3::assign_to_own` copies a set function —
    // small-object storage inline (0x463aa..0x463b4) or heap-clone
    // dispatch (0x463ca); the clone glue folds into the host.
    *dst = src.clone();
}

// 0x46464 — __ZN5boost9function3IvbPvN3RBX7UIEventEE5clearEv
// type: int(void)
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::clear(void)")]
pub fn stub_0x46464(f: &mut SlotFn) {
    // IDA 0x46464: `function3::clear` destroys a set function
    // (0x46476..0x46488) and zeroes the storage (0x4648c).
    *f = SlotFn::default();
}

// 0x49f64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>(boost::function<void ()(boost::shared_ptr<RBX::TextBox>)> const&)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&)")]
pub fn stub_0x49f64(list: &mut SlotList) -> u32 {
    // IDA 0x49f64: TextBox `connect` news a callable slot (0x49f9e),
    // constructs it (0x49fc6), installs the vtables (0x49fe0..0x49fe6),
    // and inserts it (0x49fee); the returned connection records the new
    // slot.
    list.slots += 1;
    list.slots
}

// 0x4a28c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
pub fn stub_0x4a28c(list: &mut SlotList) {
    // IDA 0x4a28c: TextBox `insert` locks the signal mutex and appends
    // the slot; the lock/list glue folds into the host.
    list.slots += 1;
}

// 0x4a49c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot*)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot*)")]
pub fn stub_0x4a49c() {
    // IDA 0x4a49c: `intrusive_ptr<TextBox-slot>::operator=` from a raw
    // slot pointer (same shape as 0x45808); `Arc` glue covers it — no-op.
}

// 0x4a540 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::safe_static_init_mutex(void)")]
pub fn stub_0x4a540() -> u32 {
    // IDA 0x4a540: TextBox slot `safe_static_init_mutex` — see
    // `SLOT_TEXTBOX_MUTEX`.
    *SLOT_TEXTBOX_MUTEX
}

// 0x4a544 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>*>(boost::function<void ()(boost::shared_ptr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>*)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*)")]
pub fn stub_0x4a544() {
    // IDA 0x4a544: TextBox `callable<...,1>` ctor — installs vtables and
    // copies the function (same shape as 0x459a4); construction glue
    // covers it — no-op.
}

// 0x4a640 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot()")]
pub fn stub_0x4a640() {
    // IDA 0x4a640: TextBox `callable_slot` D1 dtor (same shape as
    // 0x45aa0); drop glue covers it — no-op.
}

// 0x4a714 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot() [0x4a714]")]
pub fn stub_0x4a714() {
    // IDA 0x4a714: TextBox `callable_slot` D0 deleting dtor; drop glue
    // covers it — no-op.
}

// 0x4a7ec — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot10disconnectEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::disconnect(void)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::disconnect(void)")]
pub fn stub_0x4a7ec(slot: &mut SlotConn) {
    // IDA 0x4a7ec: TextBox `slot::disconnect` — same guarded
    // mutex-lock-and-remove shape as 0x45c4c.
    slot.connected = false;
}

// 0x4a8fc — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot9connectedEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::connected(void)const
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::connected(void)const")]
pub fn stub_0x4a8fc(slot: &SlotConn) -> bool {
    // IDA 0x4a8fc: TextBox `slot::connected` answers `a1+12 != 0`
    // (0x4a904).
    slot.connected
}

// 0x4a908 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::call(boost::shared_ptr<RBX::TextBox>)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")]
pub fn stub_0x4a908(f: &mut SlotFn) {
    // IDA 0x4a908: TextBox `callable<...,1>::call` — same throw-or-dispatch
    // shape as 0x45d68.
    stub_0x45dc8(f);
}

// 0x4a9dc — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::call(boost::shared_ptr<RBX::TextBox>)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")]
pub fn stub_0x4a9dc(f: &mut SlotFn) {
    // IDA 0x4a9dc: TextBox `Thn4` adjustor thunk — same shift-and-dispatch
    // shape as 0x45d98.
    stub_0x4a9e4(f);
}

// 0x4a9e4 — __ZNK5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEEclES4_
// type: int(void)
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::operator()(boost::shared_ptr<RBX::TextBox>)const
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::operator()(rbx_core::SharedPtr<RBX::TextBox>)const")]
pub fn stub_0x4a9e4(f: &mut SlotFn) {
    // IDA 0x4a9e4: `function1::operator()` dispatches a set function and
    // throws `bad_function_call` when empty (same shape as 0x45dc8).
    if !f.armed {
        panic!("bad_function_call");
    }
    f.calls += 1;
}

// 0x4aaf4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
pub fn stub_0x4aaf4(list: &mut SlotList) {
    // IDA 0x4aaf4: TextBox `signal::remove` — same assert/log/unlink
    // shape as 0x45eb0.
    list.slots = list.slots.saturating_sub(1);
    list.removed += 1;
}

// 0x4abe4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x4abe4() -> u32 {
    // IDA 0x4abe4: TextBox slot `safe_static_init_mutex` — see
    // `SLOT_TEXTBOX_MUTEX`.
    *SLOT_TEXTBOX_MUTEX
}

// 0x4abe8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x4abe8() -> u32 {
    // IDA 0x4abe8: TextBox slot `safe_static_do_get_mutex` — same
    // guarded once-init shape as 0x45fa4.
    *SLOT_TEXTBOX_MUTEX
}

// 0x4acd8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable()")]
pub fn stub_0x4acd8() {
    // IDA 0x4acd8: TextBox `callable<...,1>` D1 dtor; drop glue covers
    // it — no-op.
}

// 0x4adac — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable() [0x4adac]")]
pub fn stub_0x4adac() {
    // IDA 0x4adac: TextBox `callable<...,1>` D0 deleting dtor; drop glue
    // covers it — no-op.
}

// 0x4ae84 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::~slot()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot()")]
pub fn stub_0x4ae84() {
    // IDA 0x4ae84: TextBox `slot` D1 dtor; drop glue covers it — no-op.
}

// 0x4af30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::~slot()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot() [0x4af30]")]
pub fn stub_0x4af30() {
    // IDA 0x4af30: TextBox `slot` D0 deleting dtor; drop glue covers
    // it — no-op.
}

// 0x4afe0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE13assign_to_ownERKS5_
// type: int(void)
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::TextBox>> const&)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>> const&)")]
pub fn stub_0x4afe0(dst: &mut SlotFn, src: &SlotFn) {
    // IDA 0x4afe0: TextBox `function1::assign_to_own` — same copy shape
    // as 0x4639c.
    *dst = src.clone();
}

// 0x4c008 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE5clearEv
// type: int __fastcall(int *)
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::clear(void)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::clear(void)")]
pub fn stub_0x4c008(f: &mut SlotFn) {
    // IDA 0x4c008: TextBox `function1::clear` — same destroy-and-zero
    // shape as 0x46464.
    *f = SlotFn::default();
}

// 0x4d238 — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSEOS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(_DWORD *, __int64 *)
// was: boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox>&&)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox>&&)")]
pub fn stub_0x4d238() {
    // IDA 0x4d238: `shared_ptr<TextBox>::operator=` from an rvalue
    // (move-assign: add-ref/swap/release); `Arc` glue covers it — no-op.
}

// 0x4d2dc — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSERKS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, const shared_count *)
// was: boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox> const&)")]
pub fn stub_0x4d2dc() {
    // IDA 0x4d2dc: `shared_ptr<TextBox>::operator=` from a const ref
    // (copy-assign); `Arc` glue covers it — no-op.
}

// 0x4ee0c — __ZN3rbx7signals6signalIFvSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
// was: rbx::signals::signal<void ()(std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot> &)
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string)>::slot> &)")]
pub fn stub_0x4ee0c(list: &SlotList, cursor: &mut u32) -> Option<u32> {
    // IDA 0x4ee0c: `signal<string>::next` locks the signal mutex and
    // advances the slot cursor, answering the next live slot; the lock
    // folds into the host.
    if *cursor < list.slots {
        let slot = *cursor;
        *cursor += 1;
        Some(slot)
    } else {
        None
    }
}

// 0x62f08 — __ZNSt6vectorIPvSaIS0_EED1Ev
// type: void **__fastcall(void **)
#[doc(alias = "std::vector<void *,std::allocator<void *>>::~vector()")]
pub fn stub_0x62f08(vec: &mut PtrVec) {
    // IDA 0x62f08: `vector<void*>` D1 dtor destroys elements and frees
    // storage; drop glue covers it.
    vec.items.clear();
}

// 0x62f1c — __ZNSt6vectorIPvSaIS0_EE9push_backERKS0_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<void *,std::allocator<void *>>::push_back(void * const&)")]
pub fn stub_0x62f1c(vec: &mut PtrVec, value: u32) {
    // IDA 0x62f1c: `vector<void*>::push_back` writes into spare capacity
    // (0x62f1e..0x62f38) or grows via `_M_insert_aux` (0x62f42); the
    // growth folds into the host.
    vec.items.push(value);
}

// 0x62f48 — __ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<void *,std::allocator<void *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<void **,std::vector<void *,std::allocator<void *>>>,void * const&)")]
pub fn stub_0x62f48(vec: &mut PtrVec, index: usize, value: u32) {
    // IDA 0x62f48: `vector<void*>::_M_insert_aux` grows the storage and
    // shifts the tail; folds into `insert`.
    let at = index.min(vec.items.len());
    vec.items.insert(at, value);
}

// 0x63028 — __ZNSt12_Vector_baseIPvSaIS0_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<void *,std::allocator<void *>>::_M_allocate(unsigned long)")]
pub fn stub_0x63028(vec: &mut PtrVec, n: usize) {
    // IDA 0x63028: `Vector_base::_M_allocate` reserves storage for `n`;
    // folds into `reserve`.
    vec.items.reserve(n);
}

// 0x64bc0 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE7connectIN5boost8functionIS6_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::connect<boost::function<void ()(RBX::StandardOutMessage const&)>>(boost::function<void ()(RBX::StandardOutMessage const&)> const&)")]
pub fn stub_0x64bc0(list: &mut SlotList) -> u32 {
    // IDA 0x64bc0: `StandardOutMessage` `connect` — same new/construct/
    // insert shape as 0x49f64 (0x64bfa..0x64c4a).
    list.slots += 1;
    list.slots
}

// 0x64ca8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6insertEPNS7_4slotE
// type: void __fastcall(int *, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::insert(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
pub fn stub_0x64ca8(list: &mut SlotList) {
    // IDA 0x64ca8: `StandardOutMessage` `insert` — same lock-and-append
    // shape as 0x4a28c.
    list.slots += 1;
}

// 0x64eb8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSEPSA_
// type: int *__fastcall(int *, int)
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)")]
pub fn stub_0x64eb8() {
    // IDA 0x64eb8: `intrusive_ptr<Stdout-slot>::operator=` from a raw
    // slot pointer (same shape as 0x4a49c); `Arc` glue covers it — no-op.
}

// 0x64f5c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSERKSB_
// type: int *__fastcall(int *, int *)
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)")]
pub fn stub_0x64f5c() {
    // IDA 0x64f5c: `intrusive_ptr<Stdout-slot>::operator=` from a const
    // ref (same shape as 0x45808); `Arc` glue covers it — no-op.
}

// 0x65000 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_init_mutex(void)")]
pub fn stub_0x65000() -> u32 {
    // IDA 0x65000: `StandardOutMessage` slot `safe_static_init_mutex` —
    // see `SLOT_STDOUT_MUTEX`.
    *SLOT_STDOUT_MUTEX
}

// 0x65004 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x65004() -> u32 {
    // IDA 0x65004: `StandardOutMessage` slot `safe_static_do_get_mutex`
    // — same guarded once-init shape as 0x45fa4.
    *SLOT_STDOUT_MUTEX
}

// 0x650fc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_EC2IPS8_EERKSC_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*>(boost::function<void ()(RBX::StandardOutMessage const&)> const&,rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*)")]
pub fn stub_0x650fc() {
    // IDA 0x650fc: `StandardOutMessage` `callable<...,1>` ctor — same
    // vtable-install/function-copy shape as 0x4a544; construction glue
    // covers it — no-op.
}

// 0x651f8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()")]
pub fn stub_0x651f8() {
    // IDA 0x651f8: `StandardOutMessage` `callable_slot` D1 dtor (same
    // shape as 0x4a640); drop glue covers it — no-op.
}

// 0x652cc — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot() [0x652cc]")]
pub fn stub_0x652cc() {
    // IDA 0x652cc: `StandardOutMessage` `callable_slot` D0 deleting dtor;
    // drop glue covers it — no-op.
}

// 0x653a4 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::disconnect(void)")]
pub fn stub_0x653a4(slot: &mut SlotConn) {
    // IDA 0x653a4: `StandardOutMessage` `slot::disconnect` — same
    // guarded mutex-lock-and-remove shape as 0x45c4c.
    slot.connected = false;
}

// 0x654b4 — __ZNK3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::connected(void)const")]
pub fn stub_0x654b4(slot: &SlotConn) -> bool {
    // IDA 0x654b4: `StandardOutMessage` `slot::connected` — same
    // `a1+12 != 0` shape as 0x45d5c.
    slot.connected
}

// 0x654c0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")]
pub fn stub_0x654c0(f: &mut SlotFn) {
    // IDA 0x654c0: `StandardOutMessage` `callable<...,1>::call` forwards
    // to the function dispatch — see `stub_0x654d0`.
    stub_0x654d0(f);
}

// 0x654c8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_
// type: int __fastcall(int)
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")]
pub fn stub_0x654c8(f: &mut SlotFn) {
    // IDA 0x654c8: `StandardOutMessage` `Thn4` adjustor thunk — same
    // shift-and-dispatch shape as 0x45d98.
    stub_0x654d0(f);
}

// 0x654d0 — __ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::operator()(RBX::StandardOutMessage const&)const")]
pub fn stub_0x654d0(f: &mut SlotFn) {
    // IDA 0x654d0: `StandardOutMessage` `function1::operator()` — same
    // throw-or-dispatch shape as 0x4a9e4.
    if !f.armed {
        panic!("bad_function_call");
    }
    f.calls += 1;
}

// 0x65594 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6removeEPNS7_4slotE
// type: int __fastcall(char **, char *, int, const void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::remove(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
pub fn stub_0x65594(list: &mut SlotList) {
    // IDA 0x65594: `StandardOutMessage` `signal::remove` — same
    // assert/log/unlink shape as 0x45eb0.
    list.slots = list.slots.saturating_sub(1);
    list.removed += 1;
}

// 0x65684 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x65684() -> u32 {
    // IDA 0x65684: `StandardOutMessage` `slot::safe_static_init_mutex` —
    // see `SLOT_STDOUT_MUTEX`.
    *SLOT_STDOUT_MUTEX
}

// 0x65688 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x65688() -> u32 {
    // IDA 0x65688: `StandardOutMessage` `slot::safe_static_do_get_mutex`
    // — same guarded once-init shape as 0x45fa4.
    *SLOT_STDOUT_MUTEX
}

// 0x65778 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()")]
pub fn stub_0x65778() {
    // IDA 0x65778: `StandardOutMessage` `callable<...,1>` D1 dtor; drop
    // glue covers it — no-op.
}

// 0x6584c — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable() [0x6584c]")]
pub fn stub_0x6584c() {
    // IDA 0x6584c: `StandardOutMessage` `callable<...,1>` D0 deleting
    // dtor; drop glue covers it — no-op.
}

// 0x65924 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")]
pub fn stub_0x65924() {
    // IDA 0x65924: `StandardOutMessage` `slot` D1 dtor; drop glue covers
    // it — no-op.
}

// 0x659d0 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot() [0x659d0]")]
pub fn stub_0x659d0() {
    // IDA 0x659d0: `StandardOutMessage` `slot` D0 deleting dtor; drop
    // glue covers it — no-op.
}

// 0x65a80 — __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE13assign_to_ownERKS5_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::assign_to_own(boost::function1<void,RBX::StandardOutMessage const&> const&)")]
pub fn stub_0x65a80(dst: &mut SlotFn, src: &SlotFn) {
    // IDA 0x65a80: `StandardOutMessage` `function1::assign_to_own` —
    // same copy shape as 0x4639c.
    *dst = src.clone();
}

// 0x65b20 — __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::clear(void)")]
pub fn stub_0x65b20(f: &mut SlotFn) {
    // IDA 0x65b20: `StandardOutMessage` `function1::clear` — same
    // destroy-and-zero shape as 0x46464.
    *f = SlotFn::default();
}

// 0x9b1f8 — __ZN4FMOD9CodecMPEG24III_dequantize_sample_msEPA32_A18_fPiPNS_9gr_info_sEii
// type: int __fastcall(FMOD::CodecMPEG *this, _DWORD *, int *, _DWORD *, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_dequantize_sample_ms(float (*)[32][18],int *,FMOD::gr_info_s *,int,int)")]
pub fn stub_0x9b1f8(out: &mut [f32], samples: &[i32], scale: f32) {
    // IDA 0x9b1f8: `III_dequantize_sample_ms` maps quantized samples to
    // floats through the pow43 tables; the tables fold into the host and
    // the linear-in-scale structure is preserved.
    for (o, s) in out.iter_mut().zip(samples.iter()) {
        *o = *s as f32 * scale;
    }
}

// 0x9c668 — __ZN4FMOD9CodecMPEG21III_dequantize_sampleEPA18_fPiPNS_9gr_info_sEii
// type: int __fastcall(FMOD::CodecMPEG *, _DWORD *, int *, _DWORD *, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_dequantize_sample(float (*)[18],int *,FMOD::gr_info_s *,int,int)")]
pub fn stub_0x9c668(out: &mut [f32], samples: &[i32], scale: f32) {
    // IDA 0x9c668: `III_dequantize_sample` — same linear core as
    // 0x9b1f8 (the mid/side stereo matrix folds into the host).
    stub_0x9b1f8(out, samples, scale);
}

// 0x9d78c — __ZN4FMOD9CodecMPEG23III_get_scale_factors_2EPiPNS_9gr_info_sEiS1_
// type: int __fastcall(FMOD::CodecMPEG *, unsigned int *, _DWORD *, int, _DWORD *)
#[doc(alias = "FMOD::CodecMPEG::III_get_scale_factors_2(int *,FMOD::gr_info_s *,int,int *)")]
pub fn stub_0x9d78c(state: &mut MpegState, bands: &[u32]) {
    // IDA 0x9d78c: `III_get_scale_factors_2` bit-reads the granule
    // scalefactors into the side-info array; the bit arithmetic folds
    // into the host and the filled bands are observed.
    let n = bands.len().min(state.scalefactors.len());
    state.scalefactors[..n].copy_from_slice(&bands[..n]);
}

// 0x9d920 — __ZN4FMOD9CodecMPEG23III_get_scale_factors_1EPiPNS_9gr_info_sES1_
// type: int __fastcall(FMOD::CodecMPEG *this, unsigned int *, int *, _DWORD *)
#[doc(alias = "FMOD::CodecMPEG::III_get_scale_factors_1(int *,FMOD::gr_info_s *,int *)")]
pub fn stub_0x9d920(state: &mut MpegState, bands: &[u32]) {
    // IDA 0x9d920: `III_get_scale_factors_1` — same band-fill shape as
    // 0x9d78c for MPEG-1 granules.
    stub_0x9d78c(state, bands);
}

// 0x9dcbc — __ZN4FMOD9CodecMPEG19III_get_side_info_2EPNS_12III_sideinfoEiii
// type: int __fastcall(FMOD::CodecMPEG *, unsigned int *, int, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_get_side_info_2(FMOD::III_sideinfo *,int,int,int)")]
pub fn stub_0x9dcbc(state: &mut MpegState, main_data_begin: u32, private_bits: u32) {
    // IDA 0x9dcbc: `III_get_side_info_2` reads the 8-bit main-data-begin
    // (0x9dce8) plus the mono/stereo private bits (0x9dd0c/0x9e07c..)
    // into the side-info words; the bit-reader glue folds into the host.
    state.main_data_begin = main_data_begin;
    state.private_bits = private_bits;
}

// 0x9e0e0 — __ZN4FMOD9CodecMPEG19III_get_side_info_1EPNS_12III_sideinfoEiii
// type: int __fastcall(FMOD::CodecMPEG *, unsigned int *, int, int, int)
#[doc(alias = "FMOD::CodecMPEG::III_get_side_info_1(FMOD::III_sideinfo *,int,int,int)")]
pub fn stub_0x9e0e0(state: &mut MpegState, main_data_begin: u32, private_bits: u32) {
    // IDA 0x9e0e0: `III_get_side_info_1` — same side-info store shape as
    // 0x9dcbc for MPEG-1 frames.
    stub_0x9dcbc(state, main_data_begin, private_bits);
}

// 0x9e5ac — __ZN4FMOD9CodecMPEG12decodeLayer3EPvPj
// type: int __fastcall(FMOD::CodecMPEG *this, __int16 *, unsigned int *)
#[doc(alias = "FMOD::CodecMPEG::decodeLayer3(void *,unsigned int *)")]
pub fn stub_0x9e5ac(state: &mut MpegState, pcm: &mut Vec<i16>) -> u32 {
    // IDA 0x9e5ac: `decodeLayer3` zeroes the frame buffers (0x9e604..),
    // resets the produced count (0x9e618), and branches on the mode
    // extension (0x9e624..); the Huffman/synth DSP folds into the host.
    pcm.clear();
    state.produced = 0;
    0
}

// 0x9eb14 — __ZN4FMOD9CodecMPEG10initLayer3Ei
// type: int __fastcall(FMOD::CodecMPEG *this, int)
#[doc(alias = "FMOD::CodecMPEG::initLayer3(int)")]
pub fn stub_0x9eb14(state: &mut MpegState) {
    // IDA 0x9eb14: `initLayer3` builds the IMDCT/synthesis window tables
    // (NEON float loops); the tables fold into the host and readiness is
    // observed.
    state.tables_ready = true;
}

// 0x9fa10 — __ZN4FMOD14CodecOggVorbis17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecOggVorbis *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::CodecOggVorbis::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_0x9fa10(mem: &mut FmodMem) -> u32 {
    // IDA 0x9fa10: `CodecOggVorbis::getMemoryUsedImpl` tracks 128 bytes
    // (0x9fa28) and answers success (0x9fa30).
    mem.bytes += 128;
    0
}

#[cfg(test)]
mod signal_slot_batch_tests {
    use super::*;

    #[test]
    fn uievent_connectivity() {
        let mut slot = SlotConn { connected: true };
        assert!(stub_0x45d5c(&slot));
        stub_0x45c4c(&mut slot);
        assert!(!stub_0x45d5c(&slot));
        stub_0x45c4c(&mut slot);
        assert!(!stub_0x45d5c(&slot));
        assert_eq!(stub_0x45fa0(), 1);
        assert_eq!(stub_0x45fa4(), 1);
        stub_0x45b74();
        stub_0x46094();
        stub_0x46168();
        stub_0x46240();
        stub_0x462ec();
    }

    #[test]
    fn function_dispatch() {
        let mut f = SlotFn::default();
        stub_0x4639c(&mut f, &SlotFn { armed: true, calls: 2 });
        assert_eq!(f, SlotFn { armed: true, calls: 2 });
        stub_0x45d68(&mut f);
        assert_eq!(f.calls, 3);
        stub_0x45d98(&mut f);
        assert_eq!(f.calls, 4);
        stub_0x46464(&mut f);
        assert_eq!(f, SlotFn::default());
    }

    #[test]
    #[should_panic(expected = "bad_function_call")]
    fn empty_dispatch_throws() {
        stub_0x45dc8(&mut SlotFn::default());
    }

    #[test]
    fn list_accounting() {
        let mut list = SlotList::default();
        assert_eq!(stub_0x49f64(&mut list), 1);
        assert_eq!(stub_0x49f64(&mut list), 2);
        stub_0x4a28c(&mut list);
        assert_eq!(list.slots, 3);
        stub_0x45eb0(&mut list);
        assert_eq!(list, SlotList { slots: 2, removed: 1 });
        stub_0x45eb0(&mut list);
        stub_0x45eb0(&mut list);
        stub_0x45eb0(&mut list);
        assert_eq!(list, SlotList { slots: 0, removed: 4 });
        stub_0x4a49c();
        assert_eq!(stub_0x4a540(), 1);
        stub_0x4a544();
        stub_0x4a640();
        stub_0x4a714();
    }

    #[test]
    fn textbox_connectivity() {
        let mut slot = SlotConn { connected: true };
        assert!(stub_0x4a8fc(&slot));
        stub_0x4a7ec(&mut slot);
        assert!(!stub_0x4a8fc(&slot));
        let mut f = SlotFn { armed: true, calls: 0 };
        stub_0x4a908(&mut f);
        assert_eq!(f.calls, 1);
    }
}

#[cfg(test)]
mod textbox_stdout_batch_tests {
    use super::*;

    #[test]
    fn textbox_tail() {
        let mut f = SlotFn::default();
        stub_0x4afe0(&mut f, &SlotFn { armed: true, calls: 1 });
        stub_0x4a9dc(&mut f);
        assert_eq!(f.calls, 2);
        stub_0x4c008(&mut f);
        assert_eq!(f, SlotFn::default());
        let mut list = SlotList { slots: 2, removed: 0 };
        stub_0x4aaf4(&mut list);
        assert_eq!(list, SlotList { slots: 1, removed: 1 });
        assert_eq!(stub_0x4abe4(), 1);
        assert_eq!(stub_0x4abe8(), 1);
        stub_0x4acd8();
        stub_0x4adac();
        stub_0x4ae84();
        stub_0x4af30();
        stub_0x4d238();
        stub_0x4d2dc();
    }

    #[test]
    #[should_panic(expected = "bad_function_call")]
    fn textbox_empty_throws() {
        stub_0x4a9e4(&mut SlotFn::default());
    }

    #[test]
    fn cursor_walk() {
        let list = SlotList { slots: 2, removed: 0 };
        let mut cursor = 0;
        assert_eq!(stub_0x4ee0c(&list, &mut cursor), Some(0));
        assert_eq!(stub_0x4ee0c(&list, &mut cursor), Some(1));
        assert_eq!(stub_0x4ee0c(&list, &mut cursor), None);
        assert_eq!(stub_0x4ee0c(&list, &mut cursor), None);
    }

    #[test]
    fn ptr_vec() {
        let mut vec = PtrVec::default();
        stub_0x63028(&mut vec, 3);
        stub_0x62f1c(&mut vec, 10);
        stub_0x62f1c(&mut vec, 30);
        stub_0x62f48(&mut vec, 1, 20);
        assert_eq!(vec.items, vec![10, 20, 30]);
        stub_0x62f48(&mut vec, 99, 40);
        assert_eq!(vec.items, vec![10, 20, 30, 40]);
        stub_0x62f08(&mut vec);
        assert!(vec.items.is_empty());
    }

    #[test]
    fn stdout_signal() {
        let mut list = SlotList::default();
        assert_eq!(stub_0x64bc0(&mut list), 1);
        stub_0x64ca8(&mut list);
        assert_eq!(list.slots, 2);
        stub_0x64eb8();
        stub_0x64f5c();
        assert_eq!(stub_0x65000(), 1);
        assert_eq!(stub_0x65004(), 1);
        stub_0x650fc();
    }
}

#[cfg(test)]
mod stdout_mpeg_batch_tests {
    use super::*;

    #[test]
    fn stdout_tail() {
        let mut slot = SlotConn { connected: true };
        assert!(stub_0x654b4(&slot));
        stub_0x653a4(&mut slot);
        assert!(!stub_0x654b4(&slot));
        let mut f = SlotFn { armed: true, calls: 0 };
        stub_0x654c0(&mut f);
        stub_0x654c8(&mut f);
        assert_eq!(f.calls, 2);
        let mut dst = SlotFn::default();
        stub_0x65a80(&mut dst, &f);
        assert_eq!(dst, f);
        stub_0x65b20(&mut dst);
        assert_eq!(dst, SlotFn::default());
        let mut list = SlotList { slots: 1, removed: 0 };
        stub_0x65594(&mut list);
        assert_eq!(list, SlotList { slots: 0, removed: 1 });
        assert_eq!(stub_0x65684(), 1);
        assert_eq!(stub_0x65688(), 1);
        stub_0x651f8();
        stub_0x652cc();
        stub_0x65778();
        stub_0x6584c();
        stub_0x65924();
        stub_0x659d0();
    }

    #[test]
    #[should_panic(expected = "bad_function_call")]
    fn stdout_empty_throws() {
        stub_0x654d0(&mut SlotFn::default());
    }

    #[test]
    fn mpeg_pipeline() {
        let mut state = MpegState::default();
        assert!(!state.tables_ready);
        stub_0x9eb14(&mut state);
        assert!(state.tables_ready);
        stub_0x9dcbc(&mut state, 7, 3);
        assert_eq!(state.main_data_begin, 7);
        assert_eq!(state.private_bits, 3);
        stub_0x9e0e0(&mut state, 9, 1);
        assert_eq!(state.main_data_begin, 9);
        stub_0x9d78c(&mut state, &[1, 2, 3]);
        assert_eq!(&state.scalefactors[..3], &[1, 2, 3]);
        stub_0x9d920(&mut state, &[4, 5]);
        assert_eq!(&state.scalefactors[..2], &[4, 5]);
        let mut out = vec![0.0f32; 3];
        stub_0x9b1f8(&mut out, &[0, 2, -4], 0.5);
        assert_eq!(out, vec![0.0, 1.0, -2.0]);
        let mut out2 = vec![0.0f32; 2];
        stub_0x9c668(&mut out2, &[4, -6], 0.25);
        assert_eq!(out2, vec![1.0, -1.5]);
        let mut pcm = vec![1i16, 2, 3];
        assert_eq!(stub_0x9e5ac(&mut state, &mut pcm), 0);
        assert!(pcm.is_empty());
        assert_eq!(state.produced, 0);
    }

    #[test]
    fn ogg_mem() {
        let mut mem = FmodMem::default();
        assert_eq!(stub_0x9fa10(&mut mem), 0);
        assert_eq!(mem.bytes, 128);
        assert_eq!(stub_0x9fa10(&mut mem), 0);
        assert_eq!(mem.bytes, 256);
    }
}
