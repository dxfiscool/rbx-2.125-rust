//! audio generated_audio_wd_watchdog11 — 120 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 120 not in audio after 0x0630bf4 | rbx_core::SharedPtr not boost
//! Range 0x0630ca8..0x0637320 | existing 36089 -> 36209 distinct
//! Batch: 120 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use parking_lot::Mutex;
use rbx_core::SharedPtr;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };
// IDA 0x630ca8..0x632e50 host-seam signal model. Mirrors the
// DataModelSignal/Slot pair in datamodel (per-signal Mutex instead of the
// function-static one; same exclusion discipline).
/// The copied `boost::function<void(MoveState, MoveState)>` behind a slot.
pub type MoveState2Fn = Arc<dyn Fn(i32, i32) + Send + Sync>;
/// Slot payload of `rbx::signals::signal<void(MoveState, MoveState)>`: the
/// copied function plus the link flag (word `+0xC`, tested at IDA
/// 0x632b92/0x632bfc). Starts unlinked; `insert` links it.
pub struct MoveState2Slot {
    linked: AtomicBool,
    callback: MoveState2Fn,
}
impl MoveState2Slot {
    pub fn new(callback: MoveState2Fn) -> Self {
        Self { linked: AtomicBool::new(false), callback }
    }
    pub fn is_linked(&self) -> bool {
        self.linked.load(Ordering::SeqCst)
    }
    pub fn set_linked(&self, linked: bool) {
        self.linked.store(linked, Ordering::SeqCst);
    }
    pub fn call(&self, a: i32, b: i32) {
        (self.callback)(a, b);
    }
}
/// Dropping a slot unlinks it (function::clear plus the vtable reset).
impl Drop for MoveState2Slot {
    fn drop(&mut self) {
        self.set_linked(false);
    }
}
/// Rust model of `rbx::signals::signal<void(MoveState, MoveState)>`.
pub struct MoveState2Signal {
    slots: Mutex<Vec<SharedPtr<MoveState2Slot>>>,
}
impl MoveState2Signal {
    pub fn new() -> Self {
        Self { slots: Mutex::new(Vec::new()) }
    }
    pub fn insert(&self, slot: &SharedPtr<MoveState2Slot>) {
        let _static = MOVE_STATE2_SIGNAL_MUTEX.lock();
        slot.set_linked(true);
        self.slots.lock().push(SharedPtr::clone(slot));
    }
    /// Signal dispatch: snapshot the linked slots, then call each
    /// (the `callable::call` → stored-function path).
    pub fn emit(&self, a: i32, b: i32) {
        let live: Vec<SharedPtr<MoveState2Slot>> = {
            self.slots.lock().iter().filter(|s| s.is_linked()).map(SharedPtr::clone).collect()
        };
        for slot in live {
            slot.call(a, b);
        }
    }
    pub fn disconnect_all(&self) {
        let _static = MOVE_STATE2_SIGNAL_MUTEX.lock();
        let mut slots = self.slots.lock();
        for slot in slots.iter() {
            slot.set_linked(false);
        }
        slots.clear();
    }
    pub fn remove(&self, slot: &SharedPtr<MoveState2Slot>) {
        let _static = MOVE_STATE2_SIGNAL_MUTEX.lock();
        slot.set_linked(false);
        self.slots.lock().retain(|s| !SharedPtr::ptr_eq(s, slot));
    }
}
/// The copied `boost::function<void(shared, shared)>` behind a slot.
pub type SharedPairFn = Arc<dyn Fn(u32, u32) + Send + Sync>;
/// Slot payload of `rbx::signals::signal<void(shared, shared)>`; same
/// link-flag discipline as `MoveState2Slot`.
pub struct SharedPairSlot {
    linked: AtomicBool,
    callback: SharedPairFn,
}
impl SharedPairSlot {
    pub fn new(callback: SharedPairFn) -> Self {
        Self { linked: AtomicBool::new(false), callback }
    }
    pub fn is_linked(&self) -> bool {
        self.linked.load(Ordering::SeqCst)
    }
    pub fn set_linked(&self, linked: bool) {
        self.linked.store(linked, Ordering::SeqCst);
    }
    pub fn call(&self, a: u32, b: u32) {
        (self.callback)(a, b);
    }
}
impl Drop for SharedPairSlot {
    fn drop(&mut self) {
        self.set_linked(false);
    }
}
/// Rust model of `rbx::signals::signal<void(shared, shared)>`.
pub struct SharedPairSignal {
    slots: Mutex<Vec<SharedPtr<SharedPairSlot>>>,
}
impl SharedPairSignal {
    pub fn new() -> Self {
        Self { slots: Mutex::new(Vec::new()) }
    }
    pub fn insert(&self, slot: &SharedPtr<SharedPairSlot>) {
        let _static = SHARED_PAIR_SIGNAL_MUTEX.lock();
        slot.set_linked(true);
        self.slots.lock().push(SharedPtr::clone(slot));
    }
    pub fn emit(&self, a: u32, b: u32) {
        let live: Vec<SharedPtr<SharedPairSlot>> = {
            self.slots.lock().iter().filter(|s| s.is_linked()).map(SharedPtr::clone).collect()
        };
        for slot in live {
            slot.call(a, b);
        }
    }
    pub fn disconnect_all(&self) {
        let _static = SHARED_PAIR_SIGNAL_MUTEX.lock();
        let mut slots = self.slots.lock();
        for slot in slots.iter() {
            slot.set_linked(false);
        }
        slots.clear();
    }
}
/// The copied `boost::function<void(shared)>` behind a slot.
pub type Touched1Fn = Arc<dyn Fn(u32) + Send + Sync>;
/// Slot payload of `rbx::signals::signal<void(shared)>`; same link-flag
/// discipline as `MoveState2Slot`.
pub struct Touched1Slot {
    linked: AtomicBool,
    callback: Touched1Fn,
}
impl Touched1Slot {
    pub fn new(callback: Touched1Fn) -> Self {
        Self { linked: AtomicBool::new(false), callback }
    }
    pub fn is_linked(&self) -> bool {
        self.linked.load(Ordering::SeqCst)
    }
    pub fn set_linked(&self, linked: bool) {
        self.linked.store(linked, Ordering::SeqCst);
    }
    pub fn call(&self, a: u32) {
        (self.callback)(a);
    }
}
impl Drop for Touched1Slot {
    fn drop(&mut self) {
        self.set_linked(false);
    }
}
/// Rust model of `rbx::signals::signal<void(shared)>`.
pub struct Touched1Signal {
    slots: Mutex<Vec<SharedPtr<Touched1Slot>>>,
}
impl Touched1Signal {
    pub fn new() -> Self {
        Self { slots: Mutex::new(Vec::new()) }
    }
    pub fn insert(&self, slot: &SharedPtr<Touched1Slot>) {
        let _static = TOUCHED1_SIGNAL_MUTEX.lock();
        slot.set_linked(true);
        self.slots.lock().push(SharedPtr::clone(slot));
    }
    pub fn emit(&self, a: u32) {
        let live: Vec<SharedPtr<Touched1Slot>> = {
            self.slots.lock().iter().filter(|s| s.is_linked()).map(SharedPtr::clone).collect()
        };
        for slot in live {
            slot.call(a);
        }
    }
    pub fn disconnect_all(&self) {
        let _static = TOUCHED1_SIGNAL_MUTEX.lock();
        let mut slots = self.slots.lock();
        for slot in slots.iter() {
            slot.set_linked(false);
        }
        slots.clear();
    }
}
/// Host side of `EventDesc<SkateboardPlatform, void(MoveState, MoveState)>`
/// (IDA 0x63152c): the two declared signature argument names (each via
/// Name::declare + getSingleton<MoveState> + list hook at
/// 0x6315d4-0x63163a).
pub struct MoveStateEventDesc {
    pub arg_names: [String; 2],
}
/// Function-static mutex behind the MoveState 2-arg signal's static lock
/// (cf. `MOVE_STATE_SIGNAL_MUTEX` in generated_audio_wd_watchdog5).
static MOVE_STATE2_SIGNAL_MUTEX: Mutex<()> = Mutex::new(());
/// Function-static mutex behind the MoveState 2-arg *slot's* static lock
/// (see `stub_0632e50`); distinct from the signal mutex, same convention
/// as datamodel's SLOT_STATIC_MUTEX / SLOT_SLOT_STATIC_MUTEX pair.
static MOVE_STATE2_SLOT_MUTEX: Mutex<()> = Mutex::new(());
/// Function-static mutex behind the (shared, shared) signal's static lock.
static SHARED_PAIR_SIGNAL_MUTEX: Mutex<()> = Mutex::new(());
/// Function-static mutex behind the 1-arg touched signal's static lock.
static TOUCHED1_SIGNAL_MUTEX: Mutex<()> = Mutex::new(());

// 0x0630ca8 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// demangled: RBX::Reflection::EventDescImpl<1,RBX::SkateboardPlatform,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
pub fn stub_0630ca8(
    signal: &Touched1Signal,
    valid: bool,
    callback: Touched1Fn,
) -> Option<SharedPtr<Touched1Slot>> {
    // IDA 0x630ca8 (EventDescImpl<1>::connectGeneric): GenericSlotWrapper
    // bind execute1 (0x630d20) + function ctor (0x630d2c) fold into the
    // callback; valid wrapper -> signal::connect (0x630d48) else a null
    // connection (0x630d52); function::clear (0x630d5a) plus the temp
    // releases (0x630d60-0x630d74) ride the dropped clones.
    if !valid {
        return None;
    }
    let slot = SharedPtr::new(Touched1Slot::new(callback));
    signal.insert(&slot);
    Some(slot)
}

// 0x0630dfc — __ZNK3RBX10Reflection13EventDescImplILi1ENS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// demangled: RBX::Reflection::EventDescImpl<1,RBX::SkateboardPlatform,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
pub fn stub_0630dfc(signal: &Touched1Signal, arg_count: usize, arg: u32) {
    // IDA 0x630dfc (EventDescImpl<1>::fireEvent): ReleaseAssert
    // args.size() == 1 (Event.h:320, 0x630e38-0x630ea8); any_cast the
    // shared arg + shared_count copy (0x630ecc-0x630ee6);
    // signal_with_args<1> dispatch (0x630ef2); release (0x630ef8-0x630f00
    // via drop). The -36 Described adjust (0x630eb2-0x630eb4) collapses —
    // host ids aren't pointers.
    debug_assert!(arg_count == 1, "args.size() == 1 Event.h:320");
    signal.emit(arg);
}

// 0x0630f5c — __ZNK3RBX10Reflection13EventDescBaseINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// demangled: RBX::Reflection::EventDescBase<RBX::SkateboardPlatform,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*>::disconnectAll(RBX::Reflection::EventSource *)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0630f5c(signal: &Touched1Signal, present: bool) {
    // IDA 0x630f5c (EventDescBase<1>::disconnectAll): null source -> out
    // (0x630f60); else signal::disconnectAll on the member signal
    // (0x630f68).
    if present {
        signal.disconnect_all();
    }
}

// 0x0630f70 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0630f70() {
    // IDA 0x0630f70: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x0631160 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_ED0Ev
// demangled: RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*>::~EventDesc()
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_ED0Ev")]
pub fn stub_0631160() {
    // IDA 0x0631160: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0631214 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// demangled: RBX::Reflection::EventDescImpl<2,RBX::SkateboardPlatform,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
pub fn stub_0631214(
    signal: &SharedPairSignal,
    valid: bool,
    callback: SharedPairFn,
) -> Option<SharedPtr<SharedPairSlot>> {
    // IDA 0x631214 (EventDescImpl<2, shared, shared>::connectGeneric):
    // same bind/function/connect-or-null shape as the 1-arg twin 0x630ca8.
    if !valid {
        return None;
    }
    let slot = SharedPtr::new(SharedPairSlot::new(callback));
    signal.insert(&slot);
    Some(slot)
}

// 0x0631368 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// demangled: RBX::Reflection::EventDescImpl<2,RBX::SkateboardPlatform,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
// type: void __fastcall(int, int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
pub fn stub_0631368(signal: &SharedPairSignal, arg_count: usize, a: u32, b: u32) {
    // IDA 0x631368 (EventDescImpl<2, shared, shared>::fireEvent): same
    // assert-size/cast/copy/dispatch shape as the 1-arg twin 0x630dfc with
    // two shared args.
    debug_assert!(arg_count == 2, "args.size() == 2 Event.h:320");
    signal.emit(a, b);
}

// 0x0631518 — __ZNK3RBX10Reflection13EventDescBaseINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// demangled: RBX::Reflection::EventDescBase<RBX::SkateboardPlatform,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*>::disconnectAll(RBX::Reflection::EventSource *)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0631518(signal: &SharedPairSignal, present: bool) {
    // IDA 0x631518 (EventDescBase<2, shared>::disconnectAll): null source
    // -> out; else signal::disconnectAll; same shape as 0x630f5c.
    if present {
        signal.disconnect_all();
    }
}

// 0x063152c — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::EventDesc(rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::EventDesc(rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_063152c(first: &str, second: &str, declare: impl Fn(&str) -> String) -> MoveStateEventDesc {
    // IDA 0x63152c (EventDesc<MoveState, MoveState> C2): EventDescriptor
    // base (0x631586); member-offset word (0x6315aa); vtable install
    // (0x6315ae); per argument: Name::declare + getSingleton<MoveState> +
    // Signature Item + list hook (0x6315d4-0x63163a, twice).
    MoveStateEventDesc { arg_names: [declare(first), declare(second)] }
}

// 0x063171c — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_ED0Ev
// demangled: RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::~EventDesc()
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_ED0Ev")]
pub fn stub_063171c() {
    // IDA 0x063171c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06317d0 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// demangled: RBX::Reflection::EventDescImpl<2,RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_06317d0(
    signal: &MoveState2Signal,
    valid: bool,
    callback: MoveState2Fn,
) -> Option<SharedPtr<MoveState2Slot>> {
    // IDA 0x6317d0 (EventDescImpl<2, MoveState>::connectGeneric): same
    // bind/function/connect-or-null shape as the twins 0x630ca8/0x631214.
    if !valid {
        return None;
    }
    let slot = SharedPtr::new(MoveState2Slot::new(callback));
    signal.insert(&slot);
    Some(slot)
}

// 0x0631924 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// demangled: RBX::Reflection::EventDescImpl<2,RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
pub fn stub_0631924(signal: &MoveState2Signal, arg_count: usize, a: i32, b: i32) {
    // IDA 0x631924 (EventDescImpl<2, MoveState>::fireEvent): same
    // assert-size/cast/copy/dispatch shape as the twins 0x630dfc/0x631368.
    debug_assert!(arg_count == 2, "args.size() == 2 Event.h:320");
    signal.emit(a, b);
}

// 0x06319c0 — __ZNK3RBX10Reflection13EventDescBaseINS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
// demangled: RBX::Reflection::EventDescBase<RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::disconnectAll(RBX::Reflection::EventSource *)const
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_06319c0(signal: &MoveState2Signal) {
    // IDA 0x6319c0 (EventDescBase<2, MoveState>::disconnectAll): the -36
    // member-signal adjust (0x6319c4-0x6319c6) collapses — host member
    // access needs no adjustment; unconditional signal::disconnectAll
    // (0x6319cc-0x6319d2).
    signal.disconnect_all();
}

// 0x06319d4 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE13disconnectAllEv
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::disconnectAll(void)
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE13disconnectAllEv")]
pub fn stub_06319d4(signal: &MoveState2Signal) {
    // IDA 0x6319d4 (signal::disconnectAll): while the head slot is
    // non-null (0x6319fa/0x631aca): call_once init + lock the static mutex
    // (0x631a3a-0x631a64), clear each link word walking to null
    // (0x631a7c-0x631a96), unlock (0x631aa0-0x631aa8), release temps
    // (0x631ab4-0x631ac4). `disconnect_all` holds the same lock + clear
    // discipline.
    signal.disconnect_all();
}

// 0x0631b4c — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_18SkateboardPlatform9MoveStateES7_NS_10shared_ptrIS3_EENS_3argILi1EEENSA_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
// type: void __fastcall(_DWORD *, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_18SkateboardPlatform9MoveStateES7_NS_10shared_ptrIS3_EENS_3argILi1EEENSA_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_")]
pub fn stub_0631b4c() -> ! {
    todo!("0x0631b4c boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")
}

// 0x0631c68 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_18SkateboardPlatform9MoveStateES4_EEvRKT_RKT0_
// demangled: void RBX::Reflection::GenericSlotWrapper::execute2<RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&)
// type: int __fastcall(int, int, int)
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&)")]
#[doc(alias = "__ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_18SkateboardPlatform9MoveStateES4_EEvRKT_RKT0_")]
pub fn stub_0631c68() -> ! {
    todo!("0x0631c68 void RBX::Reflection::GenericSlotWrapper::execute2<RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&)")
}

// 0x0631dd0 — __ZN5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_E5clearEv
// demangled: boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::clear(void)
// type: int __fastcall(int *)
#[doc(alias = "boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::clear(void)")]
#[doc(alias = "__ZN5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_E5clearEv")]
pub fn stub_0631dd0() {
    // IDA 0x0631dd0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

// 0x0631fc8 — __ZN5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_SD_EENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_
// demangled: void boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
#[doc(alias = "__ZN5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_SD_EENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_")]
pub fn stub_0631fc8() -> ! {
    todo!("0x0631fc8 void boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0x06320c0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_18SkateboardPlatform9MoveStateESD_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_18SkateboardPlatform9MoveStateESD_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")]
pub fn stub_06320c0() {
    // IDA 0x06320c0: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x06320dc — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_18SkateboardPlatform9MoveStateESD_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEvSB_SB_E6invokeERNS1_15function_bufferESB_SB_
// demangled: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::invoke(boost::detail::function::function_buffer &,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)
// type: int()
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::invoke(boost::detail::function::function_buffer &,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_18SkateboardPlatform9MoveStateESD_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEvSB_SB_E6invokeERNS1_15function_bufferESB_SB_")]
pub fn stub_06320dc() {
    // IDA 0x06320dc: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

// 0x06320f4 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX18SkateboardPlatform9MoveStateES5_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_SF_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX18SkateboardPlatform9MoveStateES5_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_SF_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_06320f4() -> ! {
    todo!("0x06320f4 bool boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")
}

// 0x06321dc — __ZNK5boost6detail8function13basic_vtable2IvN3RBX18SkateboardPlatform9MoveStateES5_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_SF_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX18SkateboardPlatform9MoveStateES5_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_SF_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_06321dc() -> ! {
    todo!("0x06321dc bool boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x06322c0 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX18SkateboardPlatform9MoveStateES5_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_SF_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// demangled: void boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX18SkateboardPlatform9MoveStateES5_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_SF_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_06322c0() -> ! {
    todo!("0x06322c0 void boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x0632394 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_18SkateboardPlatform9MoveStateESA_EENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSH_ILi2EEEEEEclIS8_S8_EEvRT_RT0_
// demangled: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState &,RBX::SkateboardPlatform::MoveState &)
// type: int __fastcall(int)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState &,RBX::SkateboardPlatform::MoveState &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_18SkateboardPlatform9MoveStateESA_EENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSH_ILi2EEEEEEclIS8_S8_EEvRT_RT0_")]
pub fn stub_0632394() -> ! {
    todo!("0x0632394 void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState &,RBX::SkateboardPlatform::MoveState &)")
}

// 0x06323b0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_18SkateboardPlatform9MoveStateESD_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_18SkateboardPlatform9MoveStateESD_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_06323b0() {
    // IDA 0x06323b0: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x0632508 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::connect<boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>>(boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> const&)
// type: void __fastcall(int, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::connect<boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>>(boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")]
pub fn stub_0632508(signal: &MoveState2Signal, callback: MoveState2Fn) -> SharedPtr<MoveState2Slot> {
    // IDA 0x632508 (signal::connect<function>): callable new(32)
    // (0x632542) + callable ctor (0x63256a) + vtable installs
    // (0x632584-0x63258a) fold into the slot; insert under the static lock
    // (0x632592); connection <= slot with add_weak_ref (0x63259a-0x6325a4,
    // the returned clone).
    let slot = SharedPtr::new(MoveState2Slot::new(callback));
    signal.insert(&slot);
    slot
}

// 0x06325fc — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE6insertEPNS6_4slotE
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::insert(rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot *)
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::insert(rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE6insertEPNS6_4slotE")]
pub fn stub_06325fc(signal: &MoveState2Signal, slot: &SharedPtr<MoveState2Slot>) {
    // IDA 0x6325fc (signal::insert): ReleaseAssert item non-null
    // (signal.h:290, 0x63263a-0x6326a6 — collapses, SharedPtr can't spell
    // null); call_once init + lock the static mutex (0x6326a6-0x6326da);
    // link the slot (0x6326e2-0x632778). Same shape as datamodel 0x4b164.
    signal.insert(slot);
}

// 0x0632808 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES6_EE4slotEEaSEPS9_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot>::operator=(rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot*)
// type: int *__fastcall(int *, int)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot>::operator=(rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES6_EE4slotEEaSEPS9_")]
pub fn stub_0632808<T>(dst: &mut SharedPtr<T>, src: &SharedPtr<T>) {
    // IDA 0x632808: add_ref the new (0x632816), swap in, release the old
    // (0x632822-0x632828); same sequence as datamodel 0x4b374.
    // was: boost::intrusive_ptr<...MoveState...slot>::operator=(...).
    let old = std::mem::replace(dst, SharedPtr::clone(src));
    drop(old);
}

// 0x063282c — __ZN3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>*>(boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> const&,rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>*)
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>*>(boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> const&,rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_")]
pub fn stub_063282c(callback: MoveState2Fn) -> SharedPtr<MoveState2Slot> {
    // IDA 0x63282c (callable ctor): zero the link word (0x63285e), store
    // the signal back-pointer (0x63286e), install the callable/function
    // vtables (0x632874-0x63287a), assign_to_own the function copy
    // (0x6328ac). Construction returns the retained, unlinked slot (starts
    // unlinked, like the zeroed link word); same shape as datamodel
    // 0x4b5b8.
    SharedPtr::new(MoveState2Slot::new(callback))
}

// 0x0632928 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE13callable_slotIN5boost8functionIS5_EEED1Ev
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::callable_slot<boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>>::~callable_slot()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::callable_slot<boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE13callable_slotIN5boost8functionIS5_EEED1Ev")]
pub fn stub_0632928() {
    // IDA 0x0632928: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0632a38 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE13callable_slotIN5boost8functionIS5_EEED0Ev
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::callable_slot<boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>>::~callable_slot()
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::callable_slot<boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE13callable_slotIN5boost8functionIS5_EEED0Ev")]
pub fn stub_0632a38() {
    // IDA 0x0632a38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0632b68 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slot10disconnectEv
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot::disconnect(void)
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slot10disconnectEv")]
pub fn stub_0632b68(signal: &MoveState2Signal, slot: &SharedPtr<MoveState2Slot>) {
    // IDA 0x632b68 (slot::disconnect): null link (+0xC) returns early
    // (0x632b92); else call_once init + lock the slot static mutex
    // (0x632bd2-0x632bf4), re-test the link and signal->remove(slot) +
    // clear it (0x632bf8-0x632c06), unlock (0x632c0e-0x632c18; the landing
    // pad unlocks on unwind, which RAII guards reproduce). Same shape as
    // datamodel 0x4b860.
    if !slot.is_linked() {
        return;
    }
    let _static = stub_0632e50().lock();
    if slot.is_linked() {
        signal.remove(slot);
    }
}

// 0x0632c78 — __ZNK3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slot9connectedEv
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot::connected(void)const
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slot9connectedEv")]
pub fn stub_0632c78(slot: &SharedPtr<MoveState2Slot>) -> bool {
    // IDA 0x632c78 (slot::connected): the link word (+0xC) is nonzero
    // exactly when insert linked the slot (0x632c80); same shape as
    // datamodel 0x4b970.
    slot.is_linked()
}

// 0x0632c84 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_S5_
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::call(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::call(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_S5_")]
pub fn stub_0632c84(slot: &SharedPtr<MoveState2Slot>, a: i32, b: i32) {
    // IDA 0x632c84 (callable::call): function2::operator()(slot + 16, a,
    // b); the link word is not consulted here, only the stored function —
    // same shape as datamodel 0x4b97c.
    slot.call(a, b);
}

// 0x0632c8c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_S5_
// demangled: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::call(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)
// type: int()
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::call(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_S5_")]
pub fn stub_0632c8c() {
    // IDA 0x0632c8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0632c94 — __ZNK5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_EclES3_S3_
// demangled: boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::operator()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)const
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::operator()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)const")]
#[doc(alias = "__ZNK5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_EclES3_S3_")]
pub fn stub_0632c94(callback: &MoveState2Fn, a: i32, b: i32) {
    // IDA 0x632c94 (function2::operator()): dispatches via the stored
    // invoker ((vtable & ~1) + 4 at 0x632cf8); the landing pads unwind
    // through RAII guards (0x632d40-0x632d4e). An empty function throws
    // bad_function_call — Arc-held closures are never empty. Same shape as
    // datamodel 0x4b98c.
    callback(a, b);
}

// 0x0632d5c — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE6removeEPNS6_4slotE
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::remove(rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot *)
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::remove(rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE6removeEPNS6_4slotE")]
pub fn stub_0632d5c(signal: &MoveState2Signal, slot: &SharedPtr<MoveState2Slot>) {
    // IDA 0x632d5c (signal::remove): ReleaseAsserts the slot is not
    // expired, locks the static mutex, unlinks the node and clears its
    // link word. Expired slots cannot be spelled as SharedPtr, so only the
    // live path is modelled. Same shape as datamodel 0x4ba50.
    signal.remove(slot);
}

// 0x0632e4c — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slot22safe_static_init_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot::safe_static_init_mutex(void)
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slot22safe_static_init_mutexEv")]
pub fn stub_0632e4c() {
    // IDA 0x632e4c: single call into safe_static_do_get_mutex (0x632e50)
    // — the one-time init trampoline; same shape as datamodel 0x4bb40 and
    // stub_062bdf0 in generated_audio_wd_watchdog5.
    let _ = stub_0632e50();
}

// 0x0632e50 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slot24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot::safe_static_do_get_mutex(void)
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0632e50() -> &'static Mutex<()> {
    // IDA 0x632e50: returns the function-static slot mutex via the
    // __cxa_guard_acquire dance (0x632eac-0x632f14); same shape as
    // datamodel 0x4bb44 and stub_062bdf4 in generated_audio_wd_watchdog5.
    &MOVE_STATE2_SLOT_MUTEX
}

// 0x0632f40 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_ED1Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::~callable()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_ED1Ev")]
pub fn stub_0632f40() {
    // IDA 0x0632f40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0633050 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_ED0Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::~callable()
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_ED0Ev")]
pub fn stub_0633050() {
    // IDA 0x0633050: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0633180 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slotD1Ev
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot::~slot()
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slotD1Ev")]
pub fn stub_0633180() {
    // IDA 0x0633180: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06331ac — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slotD0Ev
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot::~slot()
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slotD0Ev")]
pub fn stub_06331ac() {
    // IDA 0x06331ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0633280 — __ZN5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_E13assign_to_ownERKS4_
// demangled: boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to_own(boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState> const&)
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to_own(boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState> const&)")]
#[doc(alias = "__ZN5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_E13assign_to_ownERKS4_")]
pub fn stub_0633280() {
    // IDA 0x0633280: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

// 0x06332b0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEEC2IMS2_KFS3_vEMS2_FvRKS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::EnumPropDescriptor<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>(char const*,char const*,RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::EnumPropDescriptor<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>(char const*,char const*,RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEEC2IMS2_KFS3_vEMS2_FvRKS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_06332b0() -> ! {
    todo!("0x06332b0 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::EnumPropDescriptor<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>(char const*,char const*,RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x0633464 — __ZN3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEED0Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::~EnumPropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEED0Ev")]
pub fn stub_0633464() {
    // IDA 0x0633464: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0633490 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10isReadOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::isReadOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10isReadOnlyEv")]
pub fn stub_0633490() -> ! {
    todo!("0x0633490 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::isReadOnly(void)const")
}

// 0x06334a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::isWriteOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11isWriteOnlyEv")]
pub fn stub_06334a0() -> ! {
    todo!("0x06334a0 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::isWriteOnly(void)const")
}

// 0x06334b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11equalValuesEPKNS0_13DescribedBaseES7_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_06334b0() -> ! {
    todo!("0x06334b0 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x06334d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_06334d8() -> ! {
    todo!("0x06334d8 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x06334fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_06334fc() -> ! {
    todo!("0x06334fc RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x0633648 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE9copyValueEPKNS0_13DescribedBaseEPS5_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_0633648() -> ! {
    todo!("0x0633648 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x063366c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::hasStringValue(void)const
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14hasStringValueEv")]
pub fn stub_063366c() -> ! {
    todo!("0x063366c RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::hasStringValue(void)const")
}

// 0x0633670 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getStringValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0633670() -> ! {
    todo!("0x0633670 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x0633694 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0633694() -> ! {
    todo!("0x0633694 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x06336d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_06336d4() -> ! {
    todo!("0x06336d4 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x06336f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_06336f4() -> ! {
    todo!("0x06336f4 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x0633934 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0633934() -> ! {
    todo!("0x0633934 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x0633950 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0633950() -> ! {
    todo!("0x0633950 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x0633984 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0633984() -> ! {
    todo!("0x0633984 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x063398c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_063398c() -> ! {
    todo!("0x063398c RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x06339d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_06339d8() -> ! {
    todo!("0x06339d8 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x06339f8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_06339f8() -> ! {
    todo!("0x06339f8 RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x0633a2c — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToIndex(RBX::SkateboardPlatform::MoveState)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToIndex(RBX::SkateboardPlatform::MoveState)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE14convertToIndexES3_")]
pub fn stub_0633a2c() -> ! {
    todo!("0x0633a2c RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToIndex(RBX::SkateboardPlatform::MoveState)const")
}

// 0x0633a9c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0633a9c() -> ! {
    todo!("0x0633a9c RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x0633adc — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE10isReadOnlyEv")]
pub fn stub_0633adc() -> ! {
    todo!("0x0633adc RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::isReadOnly(void)const")
}

// 0x0633ae0 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE11isWriteOnlyEv")]
pub fn stub_0633ae0() -> ! {
    todo!("0x0633ae0 RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::isWriteOnly(void)const")
}

// 0x0633ae4 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0633ae4() -> ! {
    todo!("0x0633ae4 RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x0633b04 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE8setValueEPNS0_13DescribedBaseES9_
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::SkateboardPlatform::MoveState const&)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::SkateboardPlatform::MoveState const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE8setValueEPNS0_13DescribedBaseES9_")]
pub fn stub_0633b04() -> ! {
    todo!("0x0633b04 RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::SkateboardPlatform::MoveState const&)const")
}

// 0x0633b28 — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::PropDescriptor<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>(char const*,char const*,bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::PropDescriptor<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>(char const*,char const*,bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0633b28() -> ! {
    todo!("0x0633b28 RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::PropDescriptor<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>(char const*,char const*,bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x0633c3c — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::~PropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbED0Ev")]
pub fn stub_0633c3c() {
    // IDA 0x0633c3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0633c68 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
pub fn stub_0633c68() -> ! {
    todo!("0x0633c68 RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::isReadOnly(void)const")
}

// 0x0633c6c — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_0633c6c() -> ! {
    todo!("0x0633c6c RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::isWriteOnly(void)const")
}

// 0x0633c70 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0633c70() -> ! {
    todo!("0x0633c70 RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x0633c94 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0633c94() -> ! {
    todo!("0x0633c94 RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x0633cb8 — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::PropDescriptor<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>(char const*,char const*,int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::PropDescriptor<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>(char const*,char const*,int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0633cb8() -> ! {
    todo!("0x0633cb8 RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::PropDescriptor<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>(char const*,char const*,int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x0633dcc — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::~PropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiED0Ev")]
pub fn stub_0633dcc() {
    // IDA 0x0633dcc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0633df8 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv")]
pub fn stub_0633df8() -> ! {
    todo!("0x0633df8 RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::isReadOnly(void)const")
}

// 0x0633dfc — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv")]
pub fn stub_0633dfc() -> ! {
    todo!("0x0633dfc RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::isWriteOnly(void)const")
}

// 0x0633e00 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0633e00() -> ! {
    todo!("0x0633e00 RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x0633e20 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_0633e20() -> ! {
    todo!("0x0633e20 RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x0633e44 — __ZNK3RBX8Velocity8rotateByERKN3G3D7Matrix3E
// demangled: RBX::Velocity::rotateBy(G3D::Matrix3 const&)const
// type: int __fastcall(int result, __int32 *, int)
#[doc(alias = "RBX::Velocity::rotateBy(G3D::Matrix3 const&)const")]
#[doc(alias = "__ZNK3RBX8Velocity8rotateByERKN3G3D7Matrix3E")]
pub fn stub_0633e44() -> ! {
    todo!("0x0633e44 RBX::Velocity::rotateBy(G3D::Matrix3 const&)const")
}

// 0x0634630 — __ZN3RBX3Sky11setNumStarsEi
// demangled: RBX::Sky::setNumStars(int)
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
#[doc(alias = "RBX::Sky::setNumStars(int)")]
#[doc(alias = "__ZN3RBX3Sky11setNumStarsEi")]
pub fn stub_0634630() -> ! {
    todo!("0x0634630 RBX::Sky::setNumStars(int)")
}

// 0x0634660 — __ZN3RBX3SkyC2Ev
// demangled: RBX::Sky::Sky(void)
// type: RBX::Instance *__fastcall(RBX::Sky *this)
#[doc(alias = "RBX::Sky::Sky(void)")]
#[doc(alias = "__ZN3RBX3SkyC2Ev")]
pub fn stub_0634660() -> ! {
    todo!("0x0634660 RBX::Sky::Sky(void)")
}

// 0x0635864 — __ZNK3RBX3Sky11getNumStarsEv
// demangled: RBX::Sky::getNumStars(void)const
// type: int __fastcall(RBX::Sky *this)
#[doc(alias = "RBX::Sky::getNumStars(void)const")]
#[doc(alias = "__ZNK3RBX3Sky11getNumStarsEv")]
pub fn stub_0635864() -> ! {
    todo!("0x0635864 RBX::Sky::getNumStars(void)const")
}

// 0x063586c — __ZN3RBX10Reflection14PropDescriptorINS_3SkyEiED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Sky,int>::~PropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_3SkyEiED1Ev")]
pub fn stub_063586c() {
    // IDA 0x063586c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0635894 — __ZN3RBX3SkyD1Ev
// demangled: RBX::Sky::~Sky()
// type: void __fastcall(RBX::Sky *__hidden this)
#[doc(alias = "RBX::Sky::~Sky()")]
#[doc(alias = "__ZN3RBX3SkyD1Ev")]
pub fn stub_0635894() {
    // IDA 0x0635894: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06358f8 — __ZN3RBX3SkyD0Ev
// demangled: RBX::Sky::~Sky()
// type: void __fastcall(RBX::Sky *__hidden this)
#[doc(alias = "RBX::Sky::~Sky()")]
#[doc(alias = "__ZN3RBX3SkyD0Ev")]
pub fn stub_06358f8() {
    // IDA 0x06358f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0635a04 — __ZThn32_N3RBX3SkyD1Ev
// demangled: non-virtual thunk toRBX::Sky::~Sky()
// type: void __fastcall(RBX::Sky *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Sky::~Sky()")]
#[doc(alias = "__ZThn32_N3RBX3SkyD1Ev")]
pub fn stub_0635a04() {
    // IDA 0x0635a04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0635a70 — __ZThn32_N3RBX3SkyD0Ev
// demangled: non-virtual thunk toRBX::Sky::~Sky()
// type: void __fastcall(RBX::Sky *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Sky::~Sky()")]
#[doc(alias = "__ZThn32_N3RBX3SkyD0Ev")]
pub fn stub_0635a70() {
    // IDA 0x0635a70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0635b7c — __ZThn36_N3RBX3SkyD1Ev
// demangled: non-virtual thunk toRBX::Sky::~Sky()
// type: void __fastcall(RBX::Sky *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Sky::~Sky()")]
#[doc(alias = "__ZThn36_N3RBX3SkyD1Ev")]
pub fn stub_0635b7c() {
    // IDA 0x0635b7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0635be8 — __ZThn36_N3RBX3SkyD0Ev
// demangled: non-virtual thunk toRBX::Sky::~Sky()
// type: void __fastcall(RBX::Sky *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Sky::~Sky()")]
#[doc(alias = "__ZThn36_N3RBX3SkyD0Ev")]
pub fn stub_0635be8() {
    // IDA 0x0635be8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06360a4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_3SkyEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::Sky> RBX::Creatable<RBX::Instance>::create<RBX::Sky>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Sky> RBX::Creatable<RBX::Instance>::create<RBX::Sky>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_3SkyEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_06360a4() -> ! {
    todo!("0x06360a4 boost::shared_ptr<RBX::Sky> RBX::Creatable<RBX::Instance>::create<RBX::Sky>(void)")
}

// 0x0636154 — __ZN5boost10shared_ptrIN3RBX3SkyEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::Sky>::shared_ptr<RBX::Sky,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::Sky>::shared_ptr<RBX::Sky,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX3SkyEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0636154() -> ! {
    todo!("0x0636154 boost::shared_ptr<RBX::Sky>::shared_ptr<RBX::Sky,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x063621c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_3SkyES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Sky,RBX::Sky>(boost::shared_ptr<RBX::Sky> const*,RBX::Sky *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Sky,RBX::Sky>(rbx_core::SharedPtr<RBX::Sky> const*,RBX::Sky *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_3SkyES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_063621c() {
    // IDA 0x063621c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x0636344 — __ZN5boost6detail12shared_countC2IPN3RBX3SkyENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX3SkyENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0636344() {
    // IDA 0x0636344: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x063644c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_063644c() {
    // IDA 0x063644c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0636450 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0636450() {
    // IDA 0x0636450: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0636454 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0636454() {
    // IDA 0x0636454: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x0636474 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0636474() {
    // IDA 0x0636474: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x063648c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_063648c() {
    // IDA 0x063648c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x06368d0 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_3SkyEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Sky>(char const*,char const*,bool RBX::Sky::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Sky>(char const*,char const*,bool RBX::Sky::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_3SkyEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_06368d0() -> ! {
    todo!("0x06368d0 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Sky>(char const*,char const*,bool RBX::Sky::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x0636a60 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv")]
pub fn stub_0636a60() -> ! {
    todo!("0x0636a60 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isReadOnly(void)const")
}

// 0x0636a64 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv")]
pub fn stub_0636a64() -> ! {
    todo!("0x0636a64 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isWriteOnly(void)const")
}

// 0x0636a68 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0636a68() -> ! {
    todo!("0x0636a68 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x0636a74 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0636a74() -> ! {
    todo!("0x0636a74 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x0636ac4 — __ZN3RBX10Reflection14PropDescriptorINS_3SkyEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::Sky,int>::PropDescriptor<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>(char const*,char const*,int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::PropDescriptor<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>(char const*,char const*,int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_3SkyEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0636ac4() -> ! {
    todo!("0x0636ac4 RBX::Reflection::PropDescriptor<RBX::Sky,int>::PropDescriptor<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>(char const*,char const*,int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x0636bd8 — __ZN3RBX10Reflection14PropDescriptorINS_3SkyEiED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Sky,int>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_3SkyEiED0Ev")]
pub fn stub_0636bd8() {
    // IDA 0x0636bd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0636c04 — __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv")]
pub fn stub_0636c04() -> ! {
    todo!("0x0636c04 RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::isReadOnly(void)const")
}

// 0x0636c08 — __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv")]
pub fn stub_0636c08() -> ! {
    todo!("0x0636c08 RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::isWriteOnly(void)const")
}

// 0x0636c0c — __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0636c0c() -> ! {
    todo!("0x0636c0c RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x0636c2c — __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// demangled: RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_0636c2c() -> ! {
    todo!("0x0636c2c RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x0636c50 — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_3SkyEEEPKcS8_MT_S2_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Sky>(char const*,char const*,RBX::TextureId RBX::Sky::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Sky>(char const*,char const*,RBX::TextureId RBX::Sky::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_3SkyEEEPKcS8_MT_S2_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0636c50() -> ! {
    todo!("0x0636c50 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Sky>(char const*,char const*,RBX::TextureId RBX::Sky::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x0636de0 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv
// demangled: RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv")]
pub fn stub_0636de0() -> ! {
    todo!("0x0636de0 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isReadOnly(void)const")
}

// 0x0636de4 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv
// demangled: RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv")]
pub fn stub_0636de4() -> ! {
    todo!("0x0636de4 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isWriteOnly(void)const")
}

// 0x0636de8 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0636de8() -> ! {
    todo!("0x0636de8 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x0636e0c — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKS2_
// demangled: RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKS2_")]
pub fn stub_0636e0c() -> ! {
    todo!("0x0636e0c RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")
}

// 0x0637264 — __ZN3RBX5Smoke8setColorEN3G3D6Color3E
// demangled: RBX::Smoke::setColor(G3D::Color3)
#[doc(alias = "RBX::Smoke::setColor(G3D::Color3)")]
#[doc(alias = "__ZN3RBX5Smoke8setColorEN3G3D6Color3E")]
pub fn stub_0637264() -> ! {
    todo!("0x0637264 RBX::Smoke::setColor(G3D::Color3)")
}

// 0x06372cc — __ZN3RBX5Smoke9setSizeUiEf
// demangled: RBX::Smoke::setSizeUi(float)
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "RBX::Smoke::setSizeUi(float)")]
#[doc(alias = "__ZN3RBX5Smoke9setSizeUiEf")]
pub fn stub_06372cc() -> ! {
    todo!("0x06372cc RBX::Smoke::setSizeUi(float)")
}

// 0x0637320 — __ZN3RBX5Smoke12setOpacityUiEf
// demangled: RBX::Smoke::setOpacityUi(float)
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "RBX::Smoke::setOpacityUi(float)")]
#[doc(alias = "__ZN3RBX5Smoke12setOpacityUiEf")]
pub fn stub_0637320() -> ! {
    todo!("0x0637320 RBX::Smoke::setOpacityUi(float)")
}
