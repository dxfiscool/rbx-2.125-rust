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

/// `RBX::Reflection::GenericSlotWrapper` cutover for the MoveState 2-arg
/// signal (IDA 0x631b4c/0x631c68): the bound `mf2` target. The stored
/// `boost::function` folds into the closure; `execute2` packs its two
/// `MoveState` args into the call directly (cf. `fireEvent` at 0x631924).
#[derive(Clone)]
pub struct GenericSlotWrapper {
    pub callback: MoveState2Fn,
}
impl GenericSlotWrapper {
    pub fn new(callback: MoveState2Fn) -> Self {
        Self { callback }
    }
    pub fn execute(&self, a: i32, b: i32) {
        (self.callback)(a, b);
    }
}
/// `boost::_bi::bind_t<mf2<GenericSlotWrapper, MoveState, MoveState>>`
/// cutover (IDA 0x631b4c/0x632394): the bound target plus arg routing.
/// The `arg<1>/arg<2>` placeholders forward both call args to the target.
#[derive(Clone)]
pub struct BoundMoveStateSlot {
    pub target: SharedPtr<GenericSlotWrapper>,
}
/// `RBX::SkateboardPlatform::MoveState` items (IDA 0x627238 `EnumDesc::C2`:
/// `addPair` Stopped=0, Coasting=1, Pushing=2, Stopping=3, AirFree=4).
pub const MOVE_STATE_ITEMS: [(&str, i32); 5] = [
    ("Stopped", 0),
    ("Coasting", 1),
    ("Pushing", 2),
    ("Stopping", 3),
    ("AirFree", 4),
];
/// `EnumDesc<MoveState>::convertToString` lookup (IDA 0x633670).
pub fn move_state_to_string(value: i32) -> &'static str {
    MOVE_STATE_ITEMS
        .iter()
        .find(|&&(_, v)| v == value)
        .map(|&(name, _)| name)
        .unwrap_or_else(|| panic!("unknown MoveState {value} (IDA 0x633670)"))
}
/// `EnumDesc<MoveState>::convertToValue` lookup (IDA 0x633694/0x6339f8).
pub fn move_state_from_string(name: &str) -> Option<i32> {
    MOVE_STATE_ITEMS
        .iter()
        .find(|&&(n, _)| n == name)
        .map(|&(_, v)| v)
}
/// `RBX::Reflection::EnumPropDescriptor<SkateboardPlatform, MoveState>`
/// cutover (IDA 0x6332b0): name/category/attributes/permissions, the live
/// enum value and the item table. The getter/setter member-pointer pair
/// (+44) folds into direct field access; the `EnumDesc` singleton link
/// (+40/+48) folds into the owned table.
#[derive(Debug, Clone)]
pub struct MoveStateEnumProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: i32,
}
impl MoveStateEnumProp {
    pub fn new(
        name: &str,
        category: &str,
        initial: i32,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            value: initial,
        }
    }
    /// `EnumDesc<MoveState>::convertToIndex` (IDA 0x633a2c):
    /// `ReleaseAssert(value>=0)` (enumconverter.h:350), dense 0..5 maps
    /// to itself, anything else is -1.
    pub fn convert_to_index(value: i32) -> i32 {
        assert!(
            value >= 0,
            "value>=0 ../App/include/reflection/enumconverter.h:350 (IDA 0x633a2c)"
        );
        if (0..MOVE_STATE_ITEMS.len() as i32).contains(&value) {
            value
        } else {
            -1
        }
    }
}
/// `RBX::SkateboardPlatform` reflected state for the `GetSetImpl`
/// member-pointer pairs below (IDA 0x633ae4/0x633b04/0x633c70/0x633c94,
/// 0x633e00/0x633e20): the getter/setter member pointers fold into direct
/// field access; the `a2 - 36` described-adjust collapses.
#[derive(Debug, Clone, Default)]
pub struct SkateboardPlatformState {
    pub move_state: i32,
    pub flag: bool,
    pub count: i32,
}

// 0x0631b4c — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_18SkateboardPlatform9MoveStateES7_NS_10shared_ptrIS3_EENS_3argILi1EEENSA_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
// type: void __fastcall(_DWORD *, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_18SkateboardPlatform9MoveStateES7_NS_10shared_ptrIS3_EENS_3argILi1EEENSA_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_")]
pub fn stub_0631b4c(target: SharedPtr<GenericSlotWrapper>) -> BoundMoveStateSlot {
    // IDA 0x631b4c (`boost::bind` mf2<GenericSlotWrapper, MoveState,
    // MoveState>): copies the shared target into the `list3` buffer
    // (0x631bb2-0x631bd4, shared_count bumped) and returns the `bind_t`
    // triple (0x631bba-0x631bdc). The `arg<1>/arg<2>` placeholders ride
    // the call signature; `Box<dyn Fn>` carries the target instead.
    BoundMoveStateSlot { target }
}

// 0x0631c68 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_18SkateboardPlatform9MoveStateES4_EEvRKT_RKT0_
// demangled: void RBX::Reflection::GenericSlotWrapper::execute2<RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&)
// type: int __fastcall(int, int, int)
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&)")]
#[doc(alias = "__ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_18SkateboardPlatform9MoveStateES4_EEvRKT_RKT0_")]
pub fn stub_0631c68(wrapper: &GenericSlotWrapper, a: i32, b: i32) {
    // IDA 0x631c68 (`GenericSlotWrapper::execute2<MoveState, MoveState>`):
    // builds a 2-`Variant` vector tagged `Type::getSingleton<MoveState>`
    // (0x631cd4-0x631d18, values packed via `placement_any::operator=`)
    // and dispatches slot 8 (0x631d20); the vector dtor rides Drop
    // (0x631d30). The pack folds into the direct call.
    wrapper.execute(a, b);
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
pub fn stub_0631fc8(bound: &BoundMoveStateSlot) -> MoveState2Fn {
    // IDA 0x631fc8 (`function2::assign_to<bind_t>`): copies the `bind_t`
    // triple into locals (0x631fec-0x632000, shared_count bumped), stores
    // the `stored_vtable` (0x63203e-0x63204a) and delegates to the
    // `basic_vtable2::assign_to` below (0x63204c-0x632050). The vtable
    // folds into the closure; the copy folds into the capture clone.
    let target = SharedPtr::clone(&bound.target);
    Arc::new(move |a: i32, b: i32| target.execute(a, b))
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
pub fn stub_06320f4(dst: &mut MoveState2Fn, src: &MoveState2Fn) -> bool {
    // IDA 0x6320f4 (`basic_vtable2::assign_to`, functor overload): copies
    // the source triple (0x632114-0x63212a, shared_count bumped),
    // delegates to the `function_obj_tag` overload below
    // (0x63216c-0x632172), releases the temp (0x632176-0x63217e) and
    // returns 1 (0x632184). The buffer copy folds into the `Arc` clone.
    *dst = MoveState2Fn::clone(src);
    true
}

// 0x06321dc — __ZNK5boost6detail8function13basic_vtable2IvN3RBX18SkateboardPlatform9MoveStateES5_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_SF_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX18SkateboardPlatform9MoveStateES5_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_SF_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_06321dc(dst: &mut MoveState2Fn, src: &MoveState2Fn) -> bool {
    // IDA 0x6321dc (`basic_vtable2::assign_to`, `function_obj_tag`
    // overload): copies the triple (0x6321fc-0x63220c, shared_count
    // bumped), delegates to `assign_functor` below (0x632250-0x632254),
    // releases the temp (0x632258-0x632262) and returns 1 (0x632266).
    // Same clone discipline as 0x6320f4 above.
    *dst = MoveState2Fn::clone(src);
    true
}

// 0x06322c0 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX18SkateboardPlatform9MoveStateES5_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_SF_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// demangled: void boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX18SkateboardPlatform9MoveStateES5_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_SF_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_06322c0(dst: &mut MoveState2Fn, src: &MoveState2Fn) {
    // IDA 0x6322c0 (`basic_vtable2::assign_functor`, `false_type`):
    // `operator new(0x10)` (0x6322de-0x6322e4), copies the `bind_t`
    // triple plus its `shared_count` into the heap cell
    // (0x6322ea-0x632342) and stores the cell in the function buffer
    // (0x632346-0x63234a). The heap cell folds into the `Arc` clone.
    *dst = MoveState2Fn::clone(src);
}

// 0x0632394 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_18SkateboardPlatform9MoveStateESA_EENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSH_ILi2EEEEEEclIS8_S8_EEvRT_RT0_
// demangled: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState &,RBX::SkateboardPlatform::MoveState &)
// type: int __fastcall(int)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState &,RBX::SkateboardPlatform::MoveState &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_18SkateboardPlatform9MoveStateESA_EENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSH_ILi2EEEEEEclIS8_S8_EEvRT_RT0_")]
pub fn stub_0632394(bound: &BoundMoveStateSlot, a: i32, b: i32) {
    // IDA 0x632394 (`bind_t::operator()<MoveState, MoveState>`):
    // resolves the `mf2` member pointer against the stored target
    // (0x632394-0x6323a8, the `TST/ADD` member-pointer dance) and tail-
    // calls it with the two forwarded args (0x6323ac). The placeholders
    // forward both call args, so this is the direct dispatch.
    bound.target.execute(a, b);
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
pub fn stub_06332b0(
    name: &str,
    category: &str,
    initial: i32,
    attributes: u32,
    permissions: u32,
) -> MoveStateEnumProp {
    // IDA 0x6332b0 (`EnumPropDescriptor<SkateboardPlatform,
    // MoveState>::C2`): `PropertyDescriptor::C2` with the class descriptor
    // plus the `EnumDesc<MoveState>` singleton (0x6332d4-0x633342),
    // installs the `EnumPropDescriptor` vtable (0x633346-0x63336c),
    // links the singleton at +40/+48 (0x633366/0x633428), allocates the
    // `GetSetImpl` member triple (0x63338c-0x6333a0, 0x14 bytes at
    // 0x6333a8-0x6333c0) and clears the readonly/writeonly attribute bits
    // when the member reports readable/writable (0x6333c8-0x633428). The
    // member triple folds into direct field access; the attribute-bit
    // fixups fold into the readable/writable defaults below.
    MoveStateEnumProp::new(name, category, initial, attributes, permissions)
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
pub fn stub_0633490(prop: &MoveStateEnumProp) -> bool {
    // IDA 0x633490 (`EnumPropDescriptor::isReadOnly`): forwards to slot 0
    // of the bound member at +44 (0x633490-0x63349a). The member is the
    // `GetSetImpl` at 0x633adc, which returns 0 — readable.
    let _ = prop;
    false
}

// 0x06334a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::isWriteOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11isWriteOnlyEv")]
pub fn stub_06334a0(prop: &MoveStateEnumProp) -> bool {
    // IDA 0x6334a0 (`EnumPropDescriptor::isWriteOnly`): forwards to slot
    // 1 of the bound member at +44 (0x6334a0-0x6334aa). The member is the
    // `GetSetImpl` at 0x633ae0, which returns 0 — writable.
    let _ = prop;
    false
}

// 0x06334b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11equalValuesEPKNS0_13DescribedBaseES7_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_06334b0(prop: &MoveStateEnumProp, a: i32, b: i32) -> bool {
    // IDA 0x6334b0 (`EnumPropDescriptor::equalValues`): `getValue` via
    // slot 8 on both describeds (0x6334b6-0x6334ca) and compares
    // (0x6334cc-0x6334d4). The member-pointer dance folds into the args.
    let _ = prop;
    a == b
}

// 0x06334d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_06334d8(prop: &MoveStateEnumProp, value: i32) -> i32 {
    // IDA 0x6334d8 (`EnumPropDescriptor::getVariant`): `getValue` via
    // slot 68 (0x6334e0-0x6334e4), tags `Type::getSingleton<int>`
    // (0x6334e8-0x6334ec) and packs with `placement_any::operator=`
    // (0x6334f0-0x6334f4). The tag is always `int`; the payload is the
    // value. The described arg folds into `value` (cf. 0x633ae4).
    let _ = prop;
    value
}

// 0x06334fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_06334fc(prop: &mut MoveStateEnumProp, value: i32) {
    // IDA 0x6334fc (`EnumPropDescriptor::setVariant`): `any_cast<int>`
    // on an int payload, else `Variant::convert<int>` on a copied
    // variant (0x633564-0x6335c8), then `setValue` via slot 72
    // (0x6335ca-0x6335e0). The variant forms fold into the `i32` arg.
    prop.value = value;
}

// 0x0633648 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE9copyValueEPKNS0_13DescribedBaseEPS5_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_0633648(dst: &mut MoveStateEnumProp, src: &MoveStateEnumProp) {
    // IDA 0x633648 (`EnumPropDescriptor::copyValue`): `getValue` into a
    // temp via slot 8, then `setValue` into the destination via slot 12
    // (same shape as the `TypedPropertyDescriptor` twin at 0x5f0c40).
    dst.value = src.value;
}

// 0x063366c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::hasStringValue(void)const
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14hasStringValueEv")]
pub fn stub_063366c() -> bool {
    // IDA 0x63366c (`EnumPropDescriptor::hasStringValue`): returns 1
    // (0x63366c-0x63366e) — enums always have a string form.
    true
}

// 0x0633670 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getStringValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0633670(prop: &MoveStateEnumProp) -> String {
    // IDA 0x633670 (`EnumPropDescriptor::getStringValue`): `getValue`
    // via slot 8, then `EnumDesc<MoveState>::convertToString` with the
    // +48 singleton (0x633670-0x633690).
    move_state_to_string(prop.value).to_owned()
}

// 0x0633694 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0633694(prop: &mut MoveStateEnumProp, name: &str) -> bool {
    // IDA 0x633694 (`EnumPropDescriptor::setStringValue`, string
    // overload): `Name::lookup` (0x633694-0x6336a0),
    // `EnumDesc<MoveState>::convertToValue` with the +48 singleton
    // (0x6336a4-0x6336ac); on success `setValue` via slot 12 and return
    // 1, else return 0 (0x6336ae-0x6336c8).
    match move_state_from_string(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x06336d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_06336d4(prop: &MoveStateEnumProp) -> (i32, i32) {
    // IDA 0x6336d4 (`EnumPropDescriptor::writeValue`): `getValue` via
    // slot 8, `clearValue` on the element, stores type tag 5 at +16 and
    // the value at +20, returns 5 (0x6336d4-0x6336f0).
    (5, prop.value)
}

// 0x06336f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_06336f4(prop: &mut MoveStateEnumProp, int_value: Option<i32>, text: Option<&str>) {
    // IDA 0x6336f4 (`EnumPropDescriptor::readValue`): `isXsiNil` returns
    // early (0x6336f8-0x633700); an int payload goes through
    // `setIntValue` (0x633708-0x633714); else a string payload goes
    // through `Name::lookup` + `convertToValue` + slot-12 `setValue`
    // (0x633716-0x633780), with `ReleaseAssert` diagnostics on bad
    // input (0x633782-0x6337e0). The XML element folds into the two
    // payload forms; xsi:nil folds into `None, None`.
    if let Some(v) = int_value {
        stub_0633a9c(prop, v);
    } else if let Some(name) = text {
        stub_0633694(prop, name);
    }
}

// 0x0633934 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0633934(prop: &MoveStateEnumProp) -> i32 {
    // IDA 0x633934 (`EnumPropDescriptor::getIndexValue`): `getValue` via
    // slot 8, then `EnumDesc<MoveState>::convertToIndex` (0x633934-
    // 0x633948).
    MoveStateEnumProp::convert_to_index(prop.value)
}

// 0x0633950 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0633950(prop: &mut MoveStateEnumProp, index: u32) -> bool {
    // IDA 0x633950 (`EnumPropDescriptor::setIndexValue`): bounds-checks
    // against the item count at +40 (0x633950-0x633958), loads the value
    // from the index table at +144 (0x63395c-0x633960) and `setValue`s
    // via slot 12, returning 1 — else returns 0 (0x633962-0x63396e).
    match MOVE_STATE_ITEMS.get(index as usize) {
        Some(&(_, v)) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x0633984 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0633984(prop: &MoveStateEnumProp) -> i32 {
    // IDA 0x633984 (`EnumPropDescriptor::getEnumValue`): `getValue` via
    // slot 8 (0x633984-0x63398a).
    prop.value
}

// 0x063398c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_063398c(prop: &mut MoveStateEnumProp, value: i32) -> bool {
    // IDA 0x63398c (`EnumPropDescriptor::setEnumValue`): `__find_if`
    // with `EnumDescriptor::equalValue` over the +28/+32 item range
    // (0x63398c-0x6339b0); on a hit `setValue`s via slot 12 and returns
    // 1, else returns 0 (0x6339b2-0x6339d2). The bind search folds into
    // the table scan.
    if MOVE_STATE_ITEMS.iter().any(|&(_, v)| v == value) {
        prop.value = value;
        true
    } else {
        false
    }
}

// 0x06339d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_06339d8(prop: &MoveStateEnumProp) -> (&'static str, i32) {
    // IDA 0x6339d8 (`EnumPropDescriptor::getEnumItem`): `getValue` via
    // slot 8, then `EnumDesc<MoveState>::convertToItem` with the +44
    // singleton link (0x6339d8-0x6339f0). The item folds into the
    // table entry.
    MOVE_STATE_ITEMS
        .iter()
        .find(|&&(_, v)| v == prop.value)
        .copied()
        .unwrap_or_else(|| panic!("unknown MoveState {} (IDA 0x6339d8)", prop.value))
}

// 0x06339f8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_06339f8(prop: &mut MoveStateEnumProp, name: &str) -> bool {
    // IDA 0x6339f8 (`EnumPropDescriptor::setStringValue`, `Name`
    // overload): `EnumDesc<MoveState>::convertToValue` with the +48
    // singleton on the already-looked-up `Name` (0x6339f8-0x633a08); on
    // success `setValue`s via slot 12 and returns 1, else 0
    // (0x633a0a-0x633a22). Same table cutover as the string overload at
    // 0x633694; the `Name` form folds into `&str`.
    match move_state_from_string(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x0633a2c — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToIndex(RBX::SkateboardPlatform::MoveState)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToIndex(RBX::SkateboardPlatform::MoveState)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE14convertToIndexES3_")]
pub fn stub_0633a2c(value: i32) -> i32 {
    // IDA 0x633a2c (`EnumDesc<MoveState>::convertToIndex`): shared core
    // with `MoveStateEnumProp::convert_to_index` above.
    MoveStateEnumProp::convert_to_index(value)
}

// 0x0633a9c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0633a9c(prop: &mut MoveStateEnumProp, value: i32) -> bool {
    // IDA 0x633a9c (`EnumPropDescriptor::setIntValue`): rejects
    // negatives (0x633a9c-0x633aa2), indexes the +132 ordinal table with
    // bounds check (0x633aa4-0x633ab2); a -1 entry rejects, else
    // `setValue`s via slot 12 and returns 1 (0x633ab4-0x633acc).
    if value >= 0 && MoveStateEnumProp::convert_to_index(value) != -1 {
        prop.value = value;
        true
    } else {
        false
    }
}

// 0x0633adc — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE10isReadOnlyEv")]
pub fn stub_0633adc() -> bool {
    // IDA 0x633adc (`GetSetImpl<MoveState getter, MoveState
    // setter>::isReadOnly`): `MOVS R0, #0; BX LR` — always readable.
    false
}

// 0x0633ae0 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE11isWriteOnlyEv")]
pub fn stub_0633ae0() -> bool {
    // IDA 0x633ae0 (`GetSetImpl<MoveState getter, MoveState
    // setter>::isWriteOnly`): `MOVS R0, #0; BX LR` — always writable.
    false
}

// 0x0633ae4 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0633ae4(state: &SkateboardPlatformState) -> i32 {
    // IDA 0x633ae4 (`GetSetImpl::getValue`): null described reads the
    // member at offset 0, else `a2 - 36` (0x633ae4-0x633aea); resolves
    // the getter member pointer (+4/+8, virtual when the low bit is set,
    // 0x633aec-0x633afa) and tail-calls it (0x633afc). The member
    // pointer folds into the field.
    state.move_state
}

/// `RBX::Reflection::PropDescriptor<SkateboardPlatform, bool>` cutover
/// (IDA 0x633b28): name/category/attributes/permissions plus the live
/// value. The getter/setter member-pointer pair folds into direct field
/// access (same shape as `Prop<bool>` in reflection).
#[derive(Debug, Clone)]
pub struct SkateBoolProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: bool,
}
impl SkateBoolProp {
    pub fn new(
        name: &str,
        category: &str,
        initial: bool,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            value: initial,
        }
    }
}
/// `RBX::Reflection::PropDescriptor<SkateboardPlatform, int>` cutover
/// (IDA 0x633cb8): same shape as `SkateBoolProp` with an `i32` value.
#[derive(Debug, Clone)]
pub struct SkateIntProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: i32,
}
impl SkateIntProp {
    pub fn new(
        name: &str,
        category: &str,
        initial: i32,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            value: initial,
        }
    }
}
/// `RBX::Velocity` cutover (IDA 0x633e44): linear + angular parts; the
/// original passes one 6-float block (`a2[0..2]` linear, `a2[3..5]`
/// angular).
#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity {
    pub linear: [f32; 3],
    pub angular: [f32; 3],
}
/// `G3D::Matrix3` cutover (IDA 0x633e44): row-major 3x3, read from `a3`
/// as rows `[a3+0..12]`, `[a3+12..24]`, `[a3+24..36]` (0x633e64-0x633ea6).
#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix3 {
    pub rows: [[f32; 3]; 3],
}
/// `RBX::TextureId`/`RBX::ContentId` cutover (IDA 0x634660/0x636e0c):
/// the URL text plus the cached `Name` word (`+4`, from
/// `Name::getNullName` / `ContentId::fromAssets`). The cache folds into
/// the text; `operator!=` compares the text.
#[derive(Debug, Clone, Default)]
pub struct TextureId {
    pub url: String,
}
impl TextureId {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
        }
    }
}
/// `RBX::Sky` cutover (IDA 0x634660): six skybox faces at +92..+136
/// (words 23..34: up, lf, rt, bk, ft, dn), the +140 flag (init 1) and
/// the star count at +144 (word 36, init 3000). The `Instance`/`Described`
/// bases and the `setName("Sky")` registration fold away.
#[derive(Debug, Clone)]
pub struct SkyState {
    pub sky_up: TextureId,
    pub sky_lf: TextureId,
    pub sky_rt: TextureId,
    pub sky_bk: TextureId,
    pub sky_ft: TextureId,
    pub sky_dn: TextureId,
    pub flag_140: bool,
    pub num_stars: i32,
}
/// Default face set under `FFlag::PlatformSkyboxEnable` (IDA 0x634660).
pub const SKY_FACES_PLATFORM: [&str; 6] = [
    "textures/sky/sky512_up.tex",
    "textures/sky/sky512_lf.tex",
    "textures/sky/sky512_rt.tex",
    "textures/sky/sky512_bk.tex",
    "textures/sky/sky512_ft.tex",
    "textures/sky/sky512_dn.tex",
];
/// Default face set otherwise (IDA 0x634660).
pub const SKY_FACES_FALLBACK: [&str; 6] = [
    "sky/null_plainsky512_up.jpg",
    "sky/null_plainsky512_lf.jpg",
    "sky/null_plainsky512_rt.jpg",
    "sky/null_plainsky512_bk.jpg",
    "sky/null_plainsky512_ft.jpg",
    "sky/null_plainsky512_dn.jpg",
];
/// `RBX::Reflection::BoundProp<bool>` cutover for `Sky` (IDA 0x6368d0):
/// name/category plus the live value and the member offset (+8). The
/// `TypedPropertyDescriptor<bool>` base folds into the header fields.
#[derive(Debug, Clone)]
pub struct SkyBoolProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: bool,
}
impl SkyBoolProp {
    pub fn new(
        name: &str,
        category: &str,
        initial: bool,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            value: initial,
        }
    }
}
/// `RBX::Reflection::BoundProp<TextureId>` cutover for `Sky`
/// (IDA 0x636c50): same shape as `SkyBoolProp` with a `TextureId` value.
#[derive(Debug, Clone)]
pub struct SkyTextureProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: TextureId,
}
impl SkyTextureProp {
    pub fn new(
        name: &str,
        category: &str,
        initial: TextureId,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            value: initial,
        }
    }
}

// 0x0633b04 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE8setValueEPNS0_13DescribedBaseES9_
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::SkateboardPlatform::MoveState const&)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::GetSetImpl<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::SkateboardPlatform::MoveState const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE8setValueEPNS0_13DescribedBaseES9_")]
pub fn stub_0633b04(state: &mut SkateboardPlatformState, value: i32) {
    // IDA 0x633b04 (`GetSetImpl::setValue`): null described writes at
    // offset 0, else `a2 - 36` (0x633b04-0x633b0c); resolves the setter
    // member pointer (+12/+16, virtual when the low bit is set,
    // 0x633b0e-0x633b20) and tail-calls it (0x633b22). The member
    // pointer folds into the field.
    state.move_state = value;
}

// 0x0633b28 — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::PropDescriptor<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>(char const*,char const*,bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::PropDescriptor<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>(char const*,char const*,bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0633b28(
    name: &str,
    category: &str,
    initial: bool,
    attributes: u32,
    permissions: u32,
) -> SkateBoolProp {
    // IDA 0x633b28 (`PropDescriptor<SkateboardPlatform, bool>::C2`):
    // allocates the `GetSetImpl` member triple (0x14 bytes,
    // 0x633b28-0x633b60), runs `TypedPropertyDescriptor<bool>::C2`
    // (0x633b62-0x633b90) and installs the `PropDescriptor` vtable
    // (0x633ba8). The member triple folds into direct field access.
    SkateBoolProp::new(name, category, initial, attributes, permissions)
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
pub fn stub_0633c68() -> bool {
    // IDA 0x633c68 (`GetSetImpl<bool getter, bool setter>::isReadOnly`):
    // `MOVS R0, #0; BX LR` — always readable.
    false
}

// 0x0633c6c — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_0633c6c() -> bool {
    // IDA 0x633c6c (`GetSetImpl<bool getter, bool setter>::isWriteOnly`):
    // `MOVS R0, #0; BX LR` — always writable.
    false
}

// 0x0633c70 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0633c70(state: &SkateboardPlatformState) -> bool {
    // IDA 0x633c70 (`GetSetImpl::getValue`): same member-pointer resolve
    // as 0x633ae4 above (null described reads at offset 0, else `a2 -
    // 36`; virtual when the low bit is set), tail-calling the getter.
    // The member pointer folds into the field.
    state.flag
}

// 0x0633c94 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0633c94(state: &mut SkateboardPlatformState, value: bool) {
    // IDA 0x633c94 (`GetSetImpl::setValue`): same member-pointer resolve
    // as 0x633b04 above, tail-calling the setter with `*a3`
    // (0x633c94-0x633cb2). The member pointer folds into the field.
    state.flag = value;
}

// 0x0633cb8 — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::PropDescriptor<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>(char const*,char const*,int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::PropDescriptor<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>(char const*,char const*,int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0633cb8(
    name: &str,
    category: &str,
    initial: i32,
    attributes: u32,
    permissions: u32,
) -> SkateIntProp {
    // IDA 0x633cb8 (`PropDescriptor<SkateboardPlatform, int>::C2`): same
    // member-triple + `TypedPropertyDescriptor<int>::C2` + vtable shape
    // as the bool twin at 0x633b28.
    SkateIntProp::new(name, category, initial, attributes, permissions)
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
pub fn stub_0633df8() -> bool {
    // IDA 0x633df8 (`GetSetImpl<int getter, int setter>::isReadOnly`):
    // `MOVS R0, #0; BX LR` — always readable.
    false
}

// 0x0633dfc — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv")]
pub fn stub_0633dfc() -> bool {
    // IDA 0x633dfc (`GetSetImpl<int getter, int setter>::isWriteOnly`):
    // `MOVS R0, #0; BX LR` — always writable.
    false
}

// 0x0633e00 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0633e00(state: &SkateboardPlatformState) -> i32 {
    // IDA 0x633e00 (`GetSetImpl::getValue`): same member-pointer resolve
    // as 0x633ae4 above (null described reads at offset 0, else `a2 -
    // 36`; virtual when the low bit is set, 0x633e00-0x633e1a),
    // tail-calling the getter (0x633e1c). The member pointer folds into
    // the field.
    state.count
}

// 0x0633e20 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_0633e20(state: &mut SkateboardPlatformState, value: i32) {
    // IDA 0x633e20 (`GetSetImpl::setValue`): same member-pointer resolve
    // as 0x633b04 above, tail-calling the setter with the value
    // (0x633e20-0x633e40). The member pointer folds into the field.
    state.count = value;
}

// 0x0633e44 — __ZNK3RBX8Velocity8rotateByERKN3G3D7Matrix3E
// demangled: RBX::Velocity::rotateBy(G3D::Matrix3 const&)const
// type: int __fastcall(int result, __int32 *, int)
#[doc(alias = "RBX::Velocity::rotateBy(G3D::Matrix3 const&)const")]
#[doc(alias = "__ZNK3RBX8Velocity8rotateByERKN3G3D7Matrix3E")]
pub fn stub_0633e44(vel: &Velocity, mat: &Matrix3) -> Velocity {
    // IDA 0x633e44 (`RBX::Velocity::rotateBy`): two row-major
    // matrix-vector products — first over `a2[0..2]` (0x633e64-0x633ea6),
    // then over `a2[3..5]` (0x633eb0-0x633ef0) — stored to the result
    // block (0x633ef2-0x633f06). Each output row is the dot product of
    // the matrix row with the input vector.
    let mul = |v: &[f32; 3]| {
        [
            mat.rows[0][0] * v[0] + mat.rows[0][1] * v[1] + mat.rows[0][2] * v[2],
            mat.rows[1][0] * v[0] + mat.rows[1][1] * v[1] + mat.rows[1][2] * v[2],
            mat.rows[2][0] * v[0] + mat.rows[2][1] * v[1] + mat.rows[2][2] * v[2],
        ]
    };
    Velocity {
        linear: mul(&vel.linear),
        angular: mul(&vel.angular),
    }
}

// 0x0634630 — __ZN3RBX3Sky11setNumStarsEi
// demangled: RBX::Sky::setNumStars(int)
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
#[doc(alias = "RBX::Sky::setNumStars(int)")]
#[doc(alias = "__ZN3RBX3Sky11setNumStarsEi")]
pub fn stub_0634630(sky: &mut SkyState, value: i32) -> bool {
    // IDA 0x634630 (`RBX::Sky::setNumStars`): clamps to 0..5000
    // (0x634630-0x634644), returns early when unchanged (0x634646-
    // 0x63464a), else stores at +0x90 (word 36, 0x634654) and tail-calls
    // `raisePropertyChanged(prop_StarCount)` (0x63464c-0x63465c). The
    // raise folds into the changed flag.
    let clamped = value.clamp(0, 5000);
    if clamped == sky.num_stars {
        return false;
    }
    sky.num_stars = clamped;
    true
}

// 0x0634660 — __ZN3RBX3SkyC2Ev
// demangled: RBX::Sky::Sky(void)
// type: RBX::Instance *__fastcall(RBX::Sky *this)
#[doc(alias = "RBX::Sky::Sky(void)")]
#[doc(alias = "__ZN3RBX3SkyC2Ev")]
pub fn stub_0634660(platform_skybox_enable: bool) -> SkyState {
    // IDA 0x634660 (`RBX::Sky::Sky`): `Instance::C2` + vtable installs +
    // class-descriptor registration (0x63467c-0x634722); the six face
    // `TextureId`s at +92..+136 start empty with null names
    // (0x634722-0x634760), the +140 flag is set to 1 and the star count
    // at +144 (word 36) to 3000 (0x634760-0x634768); `setName("Sky")`
    // via slot 28 (0x63476a-0x634780); then the faces load
    // `textures/sky/sky512_{up,lf,rt,bk,ft,dn}.tex` under
    // `FFlag::PlatformSkyboxEnable`, else `sky/null_plainsky512_{...}.jpg`
    // (0x634782-0x635860, via `ContentId::fromAssets`). The flag is a
    // host-seam parameter; the name/registration fold away.
    let faces = if platform_skybox_enable {
        SKY_FACES_PLATFORM
    } else {
        SKY_FACES_FALLBACK
    };
    SkyState {
        sky_up: TextureId::new(faces[0]),
        sky_lf: TextureId::new(faces[1]),
        sky_rt: TextureId::new(faces[2]),
        sky_bk: TextureId::new(faces[3]),
        sky_ft: TextureId::new(faces[4]),
        sky_dn: TextureId::new(faces[5]),
        flag_140: true,
        num_stars: 3000,
    }
}

// 0x0635864 — __ZNK3RBX3Sky11getNumStarsEv
// demangled: RBX::Sky::getNumStars(void)const
// type: int __fastcall(RBX::Sky *this)
#[doc(alias = "RBX::Sky::getNumStars(void)const")]
#[doc(alias = "__ZNK3RBX3Sky11getNumStarsEv")]
pub fn stub_0635864(sky: &SkyState) -> i32 {
    // IDA 0x635864 (`RBX::Sky::getNumStars`): loads word 36 at +0x90
    // (0x635864-0x635868).
    sky.num_stars
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
pub fn stub_06360a4(platform_skybox_enable: bool) -> SharedPtr<SkyState> {
    // IDA 0x6360a4 (`Creatable<Instance>::create<Sky>`): `operator
    // new(0x94)` (0x6360c2-0x6360c4), `Sky::Sky` (0x6360fa-0x6360fc),
    // then the `shared_ptr<Sky>` ctor with the `Creatable::Deleter`
    // (0x636100-0x63610a). The raw allocation folds into the `Arc`;
    // the deleter folds into `Drop`.
    stub_0636154(stub_0634660(platform_skybox_enable))
}

// 0x0636154 — __ZN5boost10shared_ptrIN3RBX3SkyEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::Sky>::shared_ptr<RBX::Sky,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::Sky>::shared_ptr<RBX::Sky,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX3SkyEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0636154(sky: SkyState) -> SharedPtr<SkyState> {
    // IDA 0x636154 (`shared_ptr<Sky>::shared_ptr<Sky, Creatable
    // Deleter>`): stores the pointer (0x636170-0x636174), builds the
    // `shared_count` control block (0x63617a-0x63617c) and, when
    // non-null, wires the weak owner via `_internal_accept_owner`
    // (0x6361aa-0x6361be). `Arc::new` adopts the owner directly.
    SharedPtr::new(sky)
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
pub fn stub_06368d0(
    name: &str,
    category: &str,
    initial: bool,
    attributes: u32,
    permissions: u32,
) -> SkyBoolProp {
    // IDA 0x6368d0 (`BoundProp<bool>::BoundProp<Sky>`):
    // `TypedPropertyDescriptor<bool>::C2` with the class descriptor
    // (0x6368ec-0x636958), installs the `BoundProp` vtable
    // (0x636966-0x636976), allocates the `BoundPropGetSet<Sky>` member
    // cell (0x14 bytes, 0x63697e-0x6369a2) and links it at +40
    // (0x6369a4-0x6369c8), then clears the readonly/writeonly attribute
    // bits when the member reports readable/writable (0x6369ca-0x6369f0).
    // The member cell folds into direct field access.
    SkyBoolProp::new(name, category, initial, attributes, permissions)
}

/// `RBX::Reflection::PropDescriptor<Sky, int>` cutover (IDA 0x636ac4):
/// same shape as `SkateIntProp` for the `Sky` star-count member.
#[derive(Debug, Clone)]
pub struct SkyIntProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: i32,
}
impl SkyIntProp {
    pub fn new(
        name: &str,
        category: &str,
        initial: i32,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            value: initial,
        }
    }
}
/// `RBX::Smoke` cutover (IDA 0x637264/0x6372cc/0x637320): the `Color3`
/// at +0x64..+0x6c, the size at +0x70 and the opacity at +0x74. The
/// `Instance`/`Described` bases fold away. Initial values come from
/// `Smoke::Smoke`, so construction is explicit (no `Default`).
#[derive(Debug, Clone)]
pub struct SmokeState {
    pub color: [f32; 3],
    pub size: f32,
    pub opacity: f32,
}

// 0x0636a60 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv")]
pub fn stub_0636a60() -> bool {
    // IDA 0x636a60 (`BoundPropGetSet<Sky>::isReadOnly`): `MOVS R0, #0;
    // BX LR` — always readable.
    false
}

// 0x0636a64 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv")]
pub fn stub_0636a64() -> bool {
    // IDA 0x636a64 (`BoundPropGetSet<Sky>::isWriteOnly`): `MOVS R0, #0;
    // BX LR` — always writable.
    false
}

// 0x0636a68 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0636a68(sky: &SkyState) -> bool {
    // IDA 0x636a68 (`BoundPropGetSet<Sky>::getValue`): loads the member
    // offset at +8, adjusts the described (`R1 - 36` when non-null,
    // 0x636a68-0x636a6c) and returns the byte there (0x636a6e-0x636a70).
    // The member is the +140 flag (the only bool in `Sky::Sky`,
    // 0x634660); the offset folds into the field.
    sky.flag_140
}

// 0x0636a74 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0636a74(sky: &mut SkyState, value: bool) -> bool {
    // IDA 0x636a74 (`BoundPropGetSet<Sky>::setValue`): adjusts the
    // described (0x636a78-0x636a7e), returns early when the byte already
    // matches (0x636a86-0x636a8e), else stores (0x636a90), runs the
    // member hook when the +12/+16 pair is set (0x636a92-0x636ab2) and
    // tail-calls `raisePropertyChanged` (0x636ab6-0x636abe). The hook
    // and the raise fold into the changed flag.
    if sky.flag_140 == value {
        return false;
    }
    sky.flag_140 = value;
    true
}

// 0x0636ac4 — __ZN3RBX10Reflection14PropDescriptorINS_3SkyEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::Sky,int>::PropDescriptor<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>(char const*,char const*,int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::PropDescriptor<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>(char const*,char const*,int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_3SkyEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0636ac4(
    name: &str,
    category: &str,
    initial: i32,
    attributes: u32,
    permissions: u32,
) -> SkyIntProp {
    // IDA 0x636ac4 (`PropDescriptor<Sky, int>::C2`): allocates the
    // `GetSetImpl` member triple (0x14 bytes, 0x636af0-0x636b28), runs
    // the base `C2` (0x636bec-0x636c00) and installs the vtable
    // (0x636ba8-0x636bb8) — same shape as the Skateboard twin at
    // 0x633cb8. The member triple folds into direct field access.
    SkyIntProp::new(name, category, initial, attributes, permissions)
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
pub fn stub_0636c04() -> bool {
    // IDA 0x636c04 (`GetSetImpl<int getter, int setter>::isReadOnly`):
    // `MOVS R0, #0; BX LR` — always readable.
    false
}

// 0x0636c08 — __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv")]
pub fn stub_0636c08() -> bool {
    // IDA 0x636c08 (`GetSetImpl<int getter, int setter>::isWriteOnly`):
    // `MOVS R0, #0; BX LR` — always writable.
    false
}

// 0x0636c0c — __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0636c0c(sky: &SkyState) -> i32 {
    // IDA 0x636c0c (`GetSetImpl::getValue`): same member-pointer resolve
    // as the Skateboard twin at 0x633e00 (null described reads at
    // offset 0, else `a2 - 36`; virtual when the low bit is set),
    // tail-calling the getter. The member is the star-count getter
    // (cf. `getNumStars` at 0x635864); the pointer folds into the field.
    sky.num_stars
}

// 0x0636c2c — __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// demangled: RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_0636c2c(sky: &mut SkyState, value: i32) -> bool {
    // IDA 0x636c2c (`GetSetImpl::setValue`): same member-pointer resolve
    // as the Skateboard twin at 0x633e20, tail-calling the setter. The
    // member is the star-count setter (cf. `setNumStars` at 0x634630,
    // which clamps 0..5000 and raises); the pointer folds into it.
    stub_0634630(sky, value)
}

// 0x0636c50 — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_3SkyEEEPKcS8_MT_S2_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Sky>(char const*,char const*,RBX::TextureId RBX::Sky::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Sky>(char const*,char const*,RBX::TextureId RBX::Sky::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_3SkyEEEPKcS8_MT_S2_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0636c50(
    name: &str,
    category: &str,
    initial: TextureId,
    attributes: u32,
    permissions: u32,
) -> SkyTextureProp {
    // IDA 0x636c50 (`BoundProp<TextureId>::BoundProp<Sky>`): same
    // `TypedPropertyDescriptor<TextureId>::C2` + vtable + member-cell
    // shape as the bool twin at 0x6368d0 (0x636c50-0x636d90). The
    // member cell folds into direct field access.
    SkyTextureProp::new(name, category, initial, attributes, permissions)
}

// 0x0636de0 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv
// demangled: RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv")]
pub fn stub_0636de0() -> bool {
    // IDA 0x636de0 (`BoundPropGetSet<Sky>::isReadOnly`): `MOVS R0, #0;
    // BX LR` — always readable.
    false
}

// 0x0636de4 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv
// demangled: RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv")]
pub fn stub_0636de4() -> bool {
    // IDA 0x636de4 (`BoundPropGetSet<Sky>::isWriteOnly`): `MOVS R0, #0;
    // BX LR` — always writable.
    false
}

// 0x0636de8 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0636de8(face: &TextureId) -> TextureId {
    // IDA 0x636de8 (`BoundPropGetSet<Sky>::getValue`): adjusts the
    // described (`R2 - 36` when non-null, 0x636dee-0x636df2), adds the
    // +8 member offset (0x636df6-0x636dfa) and copy-constructs the
    // `TextureId` out (string copy at 0x636dfe-0x636e00 plus the `Name`
    // word at 0x636e04-0x636e06). The described/offset resolve folds
    // into the face arg; the `Name` word folds into the text.
    face.clone()
}

// 0x0636e0c — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKS2_
// demangled: RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKS2_")]
pub fn stub_0636e0c(face: &mut TextureId, value: &TextureId) -> bool {
    // IDA 0x636e0c (`BoundPropGetSet<Sky>::setValue`): adjusts the
    // described (0x636e16-0x636e1e), returns early when
    // `operator!=(ContentId, ContentId)` is false (0x636e22-0x636e36),
    // else assigns the string plus the `Name` word (0x636e38-0x636e48),
    // runs the member hook when the +12/+16 pair is set (0x636e4a-
    // 0x636e68) and tail-calls `raisePropertyChanged` (0x636e6e-
    // 0x636e7a). The hook and the raise fold into the changed flag.
    if face.url == value.url {
        return false;
    }
    face.url = value.url.clone();
    true
}

// 0x0637264 — __ZN3RBX5Smoke8setColorEN3G3D6Color3E
// demangled: RBX::Smoke::setColor(G3D::Color3)
#[doc(alias = "RBX::Smoke::setColor(G3D::Color3)")]
#[doc(alias = "__ZN3RBX5Smoke8setColorEN3G3D6Color3E")]
pub fn stub_0637264(smoke: &mut SmokeState, color: [f32; 3]) -> bool {
    // IDA 0x637264 (`RBX::Smoke::setColor`): compares the `Color3` at
    // +0x64/+0x68/+0x6c component-wise (0x637264-0x6372a8), returning
    // early when all match; else stores all three (0x6372ac-0x6372c0)
    // and tail-calls `raisePropertyChanged` (0x6372c2-0x6372c6). The
    // raise folds into the changed flag.
    if smoke.color == color {
        return false;
    }
    smoke.color = color;
    true
}

// 0x06372cc — __ZN3RBX5Smoke9setSizeUiEf
// demangled: RBX::Smoke::setSizeUi(float)
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "RBX::Smoke::setSizeUi(float)")]
#[doc(alias = "__ZN3RBX5Smoke9setSizeUiEf")]
pub fn stub_06372cc(smoke: &mut SmokeState, value: f32) -> bool {
    // IDA 0x6372cc (`RBX::Smoke::setSizeUi`): clamps below-or-at 0.1 up
    // to 0.1 and above 100.0 down via `VMIN` (0x6372cc-0x6372e2); when
    // the clamped value differs from +0x70 it tail-calls `setSize`
    // (0x6372e6-0x6372f8, which stores and raises); when it differs
    // from the raw input it tail-calls `raisePropertyChanged` for the
    // Ui prop (0x6372fc-0x637314). Both raises fold into the flag.
    let clamped = if value > 0.1 { value.min(100.0) } else { 0.1 };
    let mut changed = false;
    if clamped != smoke.size {
        smoke.size = clamped;
        changed = true;
    }
    if clamped != value {
        changed = true;
    }
    changed
}

// 0x0637320 — __ZN3RBX5Smoke12setOpacityUiEf
// demangled: RBX::Smoke::setOpacityUi(float)
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "RBX::Smoke::setOpacityUi(float)")]
#[doc(alias = "__ZN3RBX5Smoke12setOpacityUiEf")]
pub fn stub_0637320(smoke: &mut SmokeState, value: f32) -> bool {
    // IDA 0x637320 (`RBX::Smoke::setOpacityUi`): same clamp-and-raise
    // shape as `setSizeUi` at 0x6372cc, with bounds 0.0/1.0 over +0x74
    // via `setOpacity` (0x637320-0x637368).
    let clamped = if value > 0.0 { value.min(1.0) } else { 0.0 };
    let mut changed = false;
    if clamped != smoke.opacity {
        smoke.opacity = clamped;
        changed = true;
    }
    if clamped != value {
        changed = true;
    }
    changed
}
