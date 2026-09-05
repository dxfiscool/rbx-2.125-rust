// Auto-generated skeletons for rbx-script — Lua/Luau/script namespaces (case-sensitive)
// Filter: Lua|Luau|script — 11911 filtered, 8773 remaining, batch +100
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: next 100 EA-sorted ascending after 0x19a30 (script max 0xf6fb4c, filler remaining)
// Range: 0x380a0..0x25dff4 | rbx_core::SharedPtr not boost::shared_ptr

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_110::{FunctorOp, JoinGameCallback};
use std::sync::LazyLock;

/// Opaque `signal<const PropertyDescriptor*>` static mutex handle (IDA
/// 0x3d5b0: guarded once-init at 0x3d60c..0x3d66a, same shape as the
/// UIEvent mutex).
static SIGNAL_PROPDESC_MUTEX: LazyLock<u32> = LazyLock::new(|| 1);
/// Opaque slot static mutex handle (IDA 0x3d938, thunk to the slot mutex).
static SLOT_PROPDESC_MUTEX: LazyLock<u32> = LazyLock::new(|| 1);

/// `signal<void(const PropertyDescriptor*)>` connection state (IDA
/// 0x3a278..0x4a150): slot allocation (0x3a290..0x3a2ce) and list insert
/// (0x3d2f4) fold into the count; removal unlinks with an expiry assert
/// (signal.h:261, 0x3d85c..0x3d890, folds into saturation); invocation runs
/// the bound member (0x3d830..0x3d844).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PropChangeSignal {
    pub slots: u32,
    pub next_id: u32,
    pub fired: u32,
}

/// FMOD codec/DSP description record (IDA 0x81074..0x107170): each getter
/// zeroes its static desc, stamps the name plus version 65792 and the
/// callback table, and answers it. Callbacks fold into the host; name and
/// version are observed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodecDescription {
    pub name: &'static str,
    pub version: u32,
}

/// FreeImage tag description slot (IDA 0x1c7470): the old string is freed
/// and the new one duplicated.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TagSlot {
    pub description: String,
}

impl PropChangeSignal {
    /// `connect` (IDA 0x3a278/0x46c18): allocates the slot and answers the
    /// connection id.
    pub fn connect(&mut self) -> u32 {
        self.slots += 1;
        self.next_id += 1;
        self.next_id
    }

    /// `insert` (IDA 0x3d2f4): links a slot under the mutex.
    pub fn insert(&mut self) {
        self.slots += 1;
    }

    /// `remove` (IDA 0x3d848): unlinks the slot.
    pub fn remove(&mut self) {
        self.slots = self.slots.saturating_sub(1);
    }

    /// Bind `operator()` (IDA 0x3d830): the virtual-aware member call.
    pub fn fire(&mut self) {
        self.fired += 1;
    }
}

// 0x380a0 — __ZN10RobloxView16onPlaceIDChangedEPKN3RBX10Reflection18PropertyDescriptorE
// type: _DWORD __fastcall(RobloxView *__hidden this, const PropertyDescriptor *)
#[doc(alias = "RobloxView::onPlaceIDChanged(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x380a0() {
    // IDA 0x380a0: `RobloxView::onPlaceIDChanged` has an empty body —
    // no-op.
}

// 0x3a278 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>> const&)")]
pub fn stub_0x3a278(sig: &mut PropChangeSignal) -> u32 {
    // IDA 0x3a278: `connect` for the `RobloxView` member bind — see
    // `PropChangeSignal::connect`.
    sig.connect()
}

// 0x3d2f4 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::insert(rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot *)")]
pub fn stub_0x3d2f4(sig: &mut PropChangeSignal) {
    // IDA 0x3d2f4: `insert` — see `PropChangeSignal::insert` (mutex
    // plumbing folds into the host).
    sig.insert();
}

// 0x3d508 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEEaSERKSC_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> const&)")]
pub fn stub_0x3d508() {
    // IDA 0x3d508: `intrusive_ptr<slot>::operator=` add-refs, swaps, and
    // releases (same shape as 0x3c0c8); `Arc` glue covers it — no-op.
}

// 0x3d5b0 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x3d5b0() -> u32 {
    // IDA 0x3d5b0: `safe_static_do_get_mutex` — see
    // `SIGNAL_PROPDESC_MUTEX`.
    *SIGNAL_PROPDESC_MUTEX
}

// 0x3d6a8 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x3d6a8() {
    // IDA 0x3d6a8: `callable_slot` D1 dtor; drop glue covers it — no-op.
}

// 0x3d754 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>::~callable_slot() [0x3d754]")]
pub fn stub_0x3d754() {
    // IDA 0x3d754: `callable_slot` D0 dtor; drop glue covers it — no-op.
}

// 0x3d808 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1Iv10RobloxViewS7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x3d808(sig: &mut PropChangeSignal) {
    // IDA 0x3d808: `callable::call` forwards to the bind call (0x3d81a).
    stub_0x3d830(sig);
}

// 0x3d81c — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1Iv10RobloxViewS7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x3d81c(sig: &mut PropChangeSignal) {
    // IDA 0x3d81c: thn4 `call` adjusts `this` and forwards to `call`.
    stub_0x3d808(sig);
}

// 0x3d830 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1Iv10RobloxViewPKN3RBX10Reflection18PropertyDescriptorEEENS0_5list2INS0_5valueIPS4_EENS_3argILi1EEEEEEclIS9_EEvRT_
// type: int(void)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>::operator()<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* &)")]
pub fn stub_0x3d830(sig: &mut PropChangeSignal) {
    // IDA 0x3d830: bind `operator()` — see `PropChangeSignal::fire`.
    sig.fire();
}

// 0x3d848 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::remove(rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot *)")]
pub fn stub_0x3d848(sig: &mut PropChangeSignal) {
    // IDA 0x3d848: `remove` — see `PropChangeSignal::remove` (assert and
    // log at 0x3d85c..0x3d8be fold into the host).
    sig.remove();
}

// 0x3d938 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x3d938() -> u32 {
    // IDA 0x3d938: `slot::safe_static_init_mutex` — see
    // `SLOT_PROPDESC_MUTEX`.
    *SLOT_PROPDESC_MUTEX
}

// 0x3d940 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot::~slot()")]
pub fn stub_0x3d940() {
    // IDA 0x3d940: `slot` D0 dtor; drop glue covers it — no-op.
}

// 0x3d9f0 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1Iv10RobloxViewS7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_0x3d9f0() {
    // IDA 0x3d9f0: `callable` D1 dtor; drop glue covers it — no-op.
}

// 0x3da9c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1Iv10RobloxViewS7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable() [0x3da9c]")]
pub fn stub_0x3da9c() {
    // IDA 0x3da9c: `callable` D0 dtor; drop glue covers it — no-op.
}

// 0x46c18 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)")]
pub fn stub_0x46c18(sig: &mut PropChangeSignal) -> u32 {
    // IDA 0x46c18: `connect` for the ObjC `CharacterMove` bind flavor
    // (0x46c30..0x46c6e) — same slot record as 0x3a278.
    sig.connect()
}

// 0x46c8c — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEED1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x46c8c() {
    // IDA 0x46c8c: `callable_slot` D1 dtor; drop glue covers it — no-op.
}

// 0x46d38 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot() [0x46d38]")]
pub fn stub_0x46d38() {
    // IDA 0x46d38: `callable_slot` D0 dtor; drop glue covers it — no-op.
}

// 0x46de8 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x46de8(sig: &mut PropChangeSignal) {
    // IDA 0x46de8: ObjC-flavor `callable::call` invoking the bound method;
    // the closure call folds into the fire count.
    sig.fire();
}

// 0x46df8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x46df8(sig: &mut PropChangeSignal) {
    // IDA 0x46df8: thn4 `call` adjusts `this` and forwards.
    stub_0x46de8(sig);
}

// 0x46e08 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_0x46e08() {
    // IDA 0x46e08: `callable` D1 dtor; drop glue covers it — no-op.
}

// 0x46eb4 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable() [0x46eb4]")]
pub fn stub_0x46eb4() {
    // IDA 0x46eb4: `callable` D0 dtor; drop glue covers it — no-op.
}

// 0x4a04c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_EC2IPS9_EERKSD_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*>(boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)> const&,rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*)")]
pub fn stub_0x4a04c() {
    // IDA 0x4a04c: `callable` ctor installs vtables (0x4a07e..0x4a0a6) and
    // assigns the function (0x4a0cc); slot-construction glue — no-op.
}

// 0x4a148 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x4a148(sig: &mut PropChangeSignal) {
    // IDA 0x4a148: function-flavor `call` runs the stored function
    // (folds into the fire count).
    sig.fire();
}

// 0x4a150 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x4a150(sig: &mut PropChangeSignal) {
    // IDA 0x4a150: thn4 `call` adjusts `this` and forwards.
    stub_0x4a148(sig);
}

// 0x4a158 — __ZNK5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEclES5_
// type: int(void)
#[doc(alias = "boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::operator()(RBX::Reflection::PropertyDescriptor const*)const")]
pub fn stub_0x4a158(cb: &Option<JoinGameCallback>) -> JoinGameCallback {
    // IDA 0x4a158: `function1::operator()` throws `bad_function_call` on an
    // empty function (0x4a1a6..0x4a1ea) and otherwise invokes through the
    // vtable (0x4a1b8, folds into the host). The invoked record is observed.
    match cb {
        Some(record) => record.clone(),
        None => panic!("boost::bad_function_call"),
    }
}

// 0x4a21c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKN3RBX10Reflection18PropertyDescriptorEENS3_5list3INS3_5valueIS6_EENSG_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x4a21c(_op: FunctorOp) -> &'static str {
    // IDA 0x4a21c: `functor_manager::manage` for the ObjC bind flavor —
    // same op dispatch as 0x3e030 (0x4a226..0x4a244) with its own bind
    // typeinfo (0x4a276..0x4a278).
    "bind_t<objc,propdesc>"
}

// 0x4a27c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKN3RBX10Reflection18PropertyDescriptorEENS3_5list3INS3_5valueIS6_EENSG_IS7_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Reflection::PropertyDescriptor const>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::PropertyDescriptor const)")]
pub fn stub_0x4a27c(sig: &mut PropChangeSignal) {
    // IDA 0x4a27c: invoker thunk forwarding to the ObjC bind call (same
    // shape as 0x3e090); the slot invocation folds into the fire count.
    sig.fire();
}

// 0x4bfdc — __ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::clear(void)")]
pub fn stub_0x4bfdc(cb: &mut Option<JoinGameCallback>) -> u32 {
    // IDA 0x4bfdc: `function1::clear` — same destroy-and-null shape as
    // 0x406e0 (0x4bfe2..0x4c004).
    *cb = None;
    0
}

// 0x4f470 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)")]
pub fn stub_0x4f470(sig: &mut PropChangeSignal) -> u32 {
    // IDA 0x4f470: `connect` for the `JumpButton` bind flavor — same slot
    // record as 0x3a278.
    sig.connect()
}

// 0x4f4e4 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEED1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x4f4e4() {
    // IDA 0x4f4e4: `callable_slot` D1 dtor; drop glue covers it — no-op.
}

// 0x4f590 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot() [0x4f590]")]
pub fn stub_0x4f590() {
    // IDA 0x4f590: `callable_slot` D0 dtor; drop glue covers it — no-op.
}

// 0x4f640 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x4f640(sig: &mut PropChangeSignal) {
    // IDA 0x4f640: JumpButton-flavor `callable::call`; the closure call
    // folds into the fire count.
    sig.fire();
}

// 0x4f650 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x4f650(sig: &mut PropChangeSignal) {
    // IDA 0x4f650: thn4 `call` adjusts `this` and forwards.
    stub_0x4f640(sig);
}

// 0x4f660 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_0x4f660() {
    // IDA 0x4f660: `callable` D1 dtor; drop glue covers it — no-op.
}

// 0x4f70c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable() [0x4f70c]")]
pub fn stub_0x4f70c() {
    // IDA 0x4f70c: `callable` D0 dtor; drop glue covers it — no-op.
}

// 0x81074 — __ZN4FMOD9CodecAIFF16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecAIFF *this)
#[doc(alias = "FMOD::CodecAIFF::getDescriptionEx(void)")]
pub fn stub_0x81074() -> CodecDescription {
    // IDA 0x81074: `CodecAIFF::getDescriptionEx` stamps "FMOD AIFF Codec"
    // plus version 65792 and callbacks (0x81090..0x810ec).
    CodecDescription { name: "FMOD AIFF Codec", version: 65792 }
}

// 0x815ec — __ZN4FMOD8CodecDLS16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecDLS *this)
#[doc(alias = "FMOD::CodecDLS::getDescriptionEx(void)")]
pub fn stub_0x815ec() -> CodecDescription {
    // IDA 0x815ec: `CodecDLS::getDescriptionEx` stamps "FMOD DLS Codec"
    // plus version 65792.
    CodecDescription { name: "FMOD DLS Codec", version: 65792 }
}

// 0x83320 — __ZN4FMOD9CodecFLAC16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecFLAC *this)
#[doc(alias = "FMOD::CodecFLAC::getDescriptionEx(void)")]
pub fn stub_0x83320() -> CodecDescription {
    // IDA 0x83320: `CodecFLAC::getDescriptionEx` stamps "FMOD FLAC Codec"
    // plus version 65792.
    CodecDescription { name: "FMOD FLAC Codec", version: 65792 }
}

// 0x834d4 — __ZN4FMOD8CodecFSB16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecFSB *this)
#[doc(alias = "FMOD::CodecFSB::getDescriptionEx(void)")]
pub fn stub_0x834d4() -> CodecDescription {
    // IDA 0x834d4: `CodecFSB::getDescriptionEx` stamps "FMOD FSB Codec"
    // plus version 65792.
    CodecDescription { name: "FMOD FSB Codec", version: 65792 }
}

// 0x88644 — __ZN4FMOD7CodecIT16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecIT *this)
#[doc(alias = "FMOD::CodecIT::getDescriptionEx(void)")]
pub fn stub_0x88644() -> CodecDescription {
    // IDA 0x88644: `CodecIT::getDescriptionEx` stamps "FMOD IT Codec" plus
    // version 65792.
    CodecDescription { name: "FMOD IT Codec", version: 65792 }
}

// 0x8f57c — __ZN4FMOD9CodecMIDI16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecMIDI *this)
#[doc(alias = "FMOD::CodecMIDI::getDescriptionEx(void)")]
pub fn stub_0x8f57c() -> CodecDescription {
    // IDA 0x8f57c: `CodecMIDI::getDescriptionEx` stamps "FMOD MIDI Codec"
    // plus version 65792.
    CodecDescription { name: "FMOD MIDI Codec", version: 65792 }
}

// 0x935c4 — __ZN4FMOD8CodecMOD16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecMOD *this)
#[doc(alias = "FMOD::CodecMOD::getDescriptionEx(void)")]
pub fn stub_0x935c4() -> CodecDescription {
    // IDA 0x935c4: `CodecMOD::getDescriptionEx` stamps "FMOD MOD Codec"
    // plus version 65792.
    CodecDescription { name: "FMOD MOD Codec", version: 65792 }
}

// 0x964e4 — __ZN4FMOD9CodecMPEG16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecMPEG *this)
#[doc(alias = "FMOD::CodecMPEG::getDescriptionEx(void)")]
pub fn stub_0x964e4() -> CodecDescription {
    // IDA 0x964e4: `CodecMPEG::getDescriptionEx` stamps "FMOD MPEG Codec"
    // plus version 65792.
    CodecDescription { name: "FMOD MPEG Codec", version: 65792 }
}

// 0x1043dc — __ZN4FMOD8DSPDelay16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPDelay *__hidden this)
#[doc(alias = "FMOD::DSPDelay::getDescriptionEx(void)")]
pub fn stub_0x1043dc() -> CodecDescription {
    // IDA 0x1043dc: `DSPDelay::getDescriptionEx` stamps "FMOD Delay" plus
    // version 65792 (0x1043f8..0x104414).
    CodecDescription { name: "FMOD Delay", version: 65792 }
}

// 0x105e0c — __ZN4FMOD10DSPTremolo16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this)
#[doc(alias = "FMOD::DSPTremolo::getDescriptionEx(void)")]
pub fn stub_0x105e0c() -> CodecDescription {
    // IDA 0x105e0c: `DSPTremolo::getDescriptionEx` stamps "FMOD Tremolo"
    // plus version 65792.
    CodecDescription { name: "FMOD Tremolo", version: 65792 }
}

// 0x107170 — __ZN4FMOD15CodecAudioQueue16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this)
#[doc(alias = "FMOD::CodecAudioQueue::getDescriptionEx(void)")]
pub fn stub_0x107170() -> CodecDescription {
    // IDA 0x107170: `CodecAudioQueue::getDescriptionEx` stamps "FMOD Audio
    // Queue Codec" plus version 65792.
    CodecDescription { name: "FMOD Audio Queue Codec", version: 65792 }
}

// 0x1c44d4 — __ZN6TagLib17getTagDescriptionENS_7MDMODELEt
#[doc(alias = "TagLib::getTagDescription(TagLib::MDMODEL,unsigned short)")]
pub fn stub_0x1c44d4(desc: Option<&str>) -> Option<String> {
    // IDA 0x1c44d4: `getTagDescription` looks up the tag info (0x1c44e0,
    // folds into the input) and answers its description word (0x1c44e8) or
    // null (0x1c44ec).
    desc.map(str::to_owned)
}

// 0x1c7470 — _FreeImage_SetTagDescription
#[doc(alias = "_FreeImage_SetTagDescription")]
pub fn stub_0x1c7470(tag: &mut TagSlot, desc: Option<&str>) -> i32 {
    // IDA 0x1c7470: null tag or description answers 0 (0x1c7478..0x1c748c);
    // otherwise the old string is freed (0x1c74a0..0x1c74a4) and the new
    // one duplicated (0x1c74ac..0x1c74c0), answering 1.
    match desc {
        Some(text) => {
            tag.description = text.to_owned();
            1
        }
        None => 0,
    }
}

// 0x1cc694 — __ZL11Descriptionv_2
// type: _DWORD __fastcall()
#[doc(alias = "__ZL11Descriptionv_2")]
pub fn stub_0x1cc694() -> &'static str {
    // IDA 0x1cc694: `Description()` answers "Truevision Targa" (0x1cc69c).
    "Truevision Targa"
}

// 0x1f0b54 — _cid_get_postscript_name
#[doc(alias = "_cid_get_postscript_name")]
pub fn stub_0x1f0b54() -> ! {
    todo!("0x1f0b54 _cid_get_postscript_name")
}

// 0x258c78 — __ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x258c78() -> ! {
    todo!("0x258c78 __ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x258c7c — __ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x258c7c() -> ! {
    todo!("0x258c7c __ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x258d1c — __ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x258d1c() -> ! {
    todo!("0x258d1c __ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x258d24 — __ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x258d24() -> ! {
    todo!("0x258d24 __ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x258dc8 — __ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x258dc8() -> ! {
    todo!("0x258dc8 __ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x258dd0 — __ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x258dd0() -> ! {
    todo!("0x258dd0 __ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x2594ac — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_11HttpServiceEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HttpService>(char const*,char const*,bool RBX::HttpService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x2594ac() -> ! {
    todo!("0x2594ac RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HttpService>(char const*,char const*,bool RBX::HttpService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x2596a0 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EEC2EMS2_FSsSI_EPKcSO_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::BoundFuncDesc(std::string (RBX::HttpService::*)(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x2596a0() -> ! {
    todo!("0x2596a0 RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::BoundFuncDesc(std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x259984 — __ZNK3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x259984() -> ! {
    todo!("0x259984 RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x259bfc — __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISJ_EEPNS3_10disable_ifINS3_7is_sameISJ_NS4_IKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(sp_counted_base **, int, const shared_count **)
#[doc(alias = "rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x259bfc() -> ! {
    todo!("0x259bfc boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::ArgHelper::getArg<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&,boost::disable_if<boost::is_same<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x259dbc — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EEC2EMS2_FSI_SsEPKcSO_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x259dbc() -> ! {
    todo!("0x259dbc RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x25a030 — __ZNK3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x25a030() -> ! {
    todo!("0x25a030 RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x25a958 — __ZN3RBX10Reflection9ArgHelper6getArgINS_11HttpService15HttpContentTypeELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int (__fastcall ***__fastcall(int (__fastcall ***)(_DWORD), int))(_DWORD)
#[doc(alias = "RBX::HttpService::HttpContentType RBX::Reflection::ArgHelper::getArg<RBX::HttpService::HttpContentType,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::HttpService::HttpContentType> const&,boost::disable_if<boost::is_same<RBX::HttpService::HttpContentType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x25a958() -> ! {
    todo!("0x25a958 RBX::HttpService::HttpContentType RBX::Reflection::ArgHelper::getArg<RBX::HttpService::HttpContentType,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::HttpService::HttpContentType> const&,boost::disable_if<boost::is_same<RBX::HttpService::HttpContentType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x25aaec — __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_11HttpService15HttpContentTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int __fastcall(int, _DWORD *, int, int)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<3,RBX::HttpService::HttpContentType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::HttpService::HttpContentType &,boost::enable_if<boost::is_enum<RBX::HttpService::HttpContentType>,void>::type *)")]
pub fn stub_0x25aaec() -> ! {
    todo!("0x25aaec bool RBX::Reflection::ArgHelper::try_enum<3,RBX::HttpService::HttpContentType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::HttpService::HttpContentType &,boost::enable_if<boost::is_enum<RBX::HttpService::HttpContentType>,void>::type *)")
}

// 0x25c0cc — __ZN3RBX10Reflection14PropDescriptorINS_5LightEbED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,bool>::~PropDescriptor()")]
pub fn stub_0x25c0cc() -> ! {
    todo!("0x25c0cc RBX::Reflection::PropDescriptor<RBX::Light,bool>::~PropDescriptor()")
}

// 0x25c100 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()")]
pub fn stub_0x25c100() -> ! {
    todo!("0x25c100 RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()")
}

// 0x25c128 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEfED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,float>::~PropDescriptor()")]
pub fn stub_0x25c128() -> ! {
    todo!("0x25c128 RBX::Reflection::PropDescriptor<RBX::Light,float>::~PropDescriptor()")
}

// 0x25c150 — __ZN3RBX10Reflection14PropDescriptorINS_10PointLightEfED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PointLight,float>::~PropDescriptor()")]
pub fn stub_0x25c150() -> ! {
    todo!("0x25c150 RBX::Reflection::PropDescriptor<RBX::PointLight,float>::~PropDescriptor()")
}

// 0x25c178 — __ZN3RBX10Reflection14PropDescriptorINS_9SpotLightEfED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SpotLight,float>::~PropDescriptor()")]
pub fn stub_0x25c178() -> ! {
    todo!("0x25c178 RBX::Reflection::PropDescriptor<RBX::SpotLight,float>::~PropDescriptor()")
}

// 0x25c1ac — __ZN3RBX10Reflection18EnumPropDescriptorINS_9SpotLightENS_8NormalIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SpotLight,RBX::NormalId>::~EnumPropDescriptor()")]
pub fn stub_0x25c1ac() -> ! {
    todo!("0x25c1ac RBX::Reflection::EnumPropDescriptor<RBX::SpotLight,RBX::NormalId>::~EnumPropDescriptor()")
}

// 0x25d6ac — __ZN3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Light *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25d6ac() -> ! {
    todo!("0x25d6ac __ZN3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25d6b0 — __ZN3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Light *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25d6b0() -> ! {
    todo!("0x25d6b0 __ZN3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25d750 — __ZThn32_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25d750() -> ! {
    todo!("0x25d750 __ZThn32_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25d758 — __ZThn32_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25d758() -> ! {
    todo!("0x25d758 __ZThn32_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25d7fc — __ZThn36_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25d7fc() -> ! {
    todo!("0x25d7fc __ZThn36_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25d804 — __ZThn36_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25d804() -> ! {
    todo!("0x25d804 __ZThn36_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25d8a8 — __ZThn92_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25d8a8() -> ! {
    todo!("0x25d8a8 __ZThn92_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25d8b0 — __ZThn92_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25d8b0() -> ! {
    todo!("0x25d8b0 __ZThn92_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25d954 — __ZN3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Light *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25d954() -> ! {
    todo!("0x25d954 __ZN3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25d958 — __ZN3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Light *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25d958() -> ! {
    todo!("0x25d958 __ZN3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25d9f8 — __ZThn32_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25d9f8() -> ! {
    todo!("0x25d9f8 __ZThn32_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25da00 — __ZThn32_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25da00() -> ! {
    todo!("0x25da00 __ZThn32_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25daa4 — __ZThn36_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25daa4() -> ! {
    todo!("0x25daa4 __ZThn36_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25daac — __ZThn36_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25daac() -> ! {
    todo!("0x25daac __ZThn36_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25db50 — __ZThn92_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25db50() -> ! {
    todo!("0x25db50 __ZThn92_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25db58 — __ZThn92_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25db58() -> ! {
    todo!("0x25db58 __ZThn92_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25dbfc — __ZN3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25dbfc() -> ! {
    todo!("0x25dbfc __ZN3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25dc00 — __ZN3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25dc00() -> ! {
    todo!("0x25dc00 __ZN3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25dca0 — __ZThn32_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25dca0() -> ! {
    todo!("0x25dca0 __ZThn32_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25dca8 — __ZThn32_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25dca8() -> ! {
    todo!("0x25dca8 __ZThn32_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25dd4c — __ZThn36_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25dd4c() -> ! {
    todo!("0x25dd4c __ZThn36_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25dd54 — __ZThn36_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25dd54() -> ! {
    todo!("0x25dd54 __ZThn36_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25ddf8 — __ZN3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25ddf8() -> ! {
    todo!("0x25ddf8 __ZN3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25ddfc — __ZN3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25ddfc() -> ! {
    todo!("0x25ddfc __ZN3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25de9c — __ZThn32_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25de9c() -> ! {
    todo!("0x25de9c __ZThn32_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25dea4 — __ZThn32_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25dea4() -> ! {
    todo!("0x25dea4 __ZThn32_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25df48 — __ZThn36_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25df48() -> ! {
    todo!("0x25df48 __ZThn36_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x25df50 — __ZThn36_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25df50() -> ! {
    todo!("0x25df50 __ZThn36_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x25dff4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9SpotLightENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SpotLight,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(RBX::NormalId)>(char const*,char const*,RBX::NormalId (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(RBX::NormalId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x25dff4() -> ! {
    todo!("0x25dff4 RBX::Reflection::EnumPropDescriptor<RBX::SpotLight,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(RBX::NormalId)>(char const*,char const*,RBX::NormalId (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(RBX::NormalId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

#[cfg(test)]
mod prop_signal_batch_tests {
    use super::*;

    #[test]
    fn connect_insert_remove() {
        let mut sig = PropChangeSignal::default();
        assert_eq!(stub_0x3a278(&mut sig), 1);
        assert_eq!(stub_0x46c18(&mut sig), 2);
        assert_eq!(sig.slots, 2);
        stub_0x3d2f4(&mut sig);
        assert_eq!(sig.slots, 3);
        stub_0x3d848(&mut sig);
        assert_eq!(sig.slots, 2);
        stub_0x3d848(&mut sig);
        stub_0x3d848(&mut sig);
        stub_0x3d848(&mut sig);
        assert_eq!(sig.slots, 0);
    }

    #[test]
    fn call_chains_fire_once() {
        let mut sig = PropChangeSignal::default();
        stub_0x3d830(&mut sig);
        assert_eq!(sig.fired, 1);
        stub_0x3d808(&mut sig);
        assert_eq!(sig.fired, 2);
        stub_0x3d81c(&mut sig);
        assert_eq!(sig.fired, 3);
        stub_0x46de8(&mut sig);
        assert_eq!(sig.fired, 4);
        stub_0x46df8(&mut sig);
        assert_eq!(sig.fired, 5);
        stub_0x4a148(&mut sig);
        assert_eq!(sig.fired, 6);
        stub_0x4a150(&mut sig);
        assert_eq!(sig.fired, 7);
    }

    #[test]
    fn static_mutexes() {
        assert_eq!(stub_0x3d5b0(), *SIGNAL_PROPDESC_MUTEX);
        assert_eq!(stub_0x3d938(), *SLOT_PROPDESC_MUTEX);
    }

    #[test]
    fn glue_noops() {
        stub_0x380a0();
        stub_0x3d508();
        stub_0x3d6a8();
        stub_0x3d754();
        stub_0x3d940();
        stub_0x3d9f0();
        stub_0x3da9c();
        stub_0x46c8c();
        stub_0x46d38();
        stub_0x46e08();
        stub_0x46eb4();
        stub_0x4a04c();
    }
}

#[cfg(test)]
mod functor_codec_batch_tests {
    use super::*;

    #[test]
    fn function_call_and_clear() {
        let cb = JoinGameCallback { place_id: 5, url: String::new(), game_live: true, with_request: false };
        assert_eq!(stub_0x4a158(&Some(cb.clone())), cb);
        let mut slot = Some(cb);
        assert_eq!(stub_0x4bfdc(&mut slot), 0);
        assert_eq!(slot, None);
    }

    #[test]
    #[should_panic(expected = "bad_function_call")]
    fn empty_call_throws() {
        stub_0x4a158(&None);
    }

    #[test]
    fn objc_functor_answers_type() {
        assert_eq!(stub_0x4a21c(FunctorOp::GetType), "bind_t<objc,propdesc>");
        let mut sig = PropChangeSignal::default();
        assert_eq!(stub_0x4f470(&mut sig), 1);
        assert_eq!(sig.fired, 0);
        stub_0x4f4e4();
        stub_0x4f590();
        stub_0x4f640(&mut sig);
        stub_0x4f650(&mut sig);
        stub_0x4f660();
        stub_0x4f70c();
        assert_eq!(sig.fired, 2);
    }

    #[test]
    fn codec_descriptions() {
        for (got, name) in [
            (stub_0x81074(), "FMOD AIFF Codec"),
            (stub_0x815ec(), "FMOD DLS Codec"),
            (stub_0x83320(), "FMOD FLAC Codec"),
            (stub_0x834d4(), "FMOD FSB Codec"),
            (stub_0x88644(), "FMOD IT Codec"),
            (stub_0x8f57c(), "FMOD MIDI Codec"),
            (stub_0x935c4(), "FMOD MOD Codec"),
            (stub_0x964e4(), "FMOD MPEG Codec"),
            (stub_0x1043dc(), "FMOD Delay"),
            (stub_0x105e0c(), "FMOD Tremolo"),
            (stub_0x107170(), "FMOD Audio Queue Codec"),
        ] {
            assert_eq!(got, CodecDescription { name, version: 65792 });
        }
        assert_eq!(stub_0x1cc694(), "Truevision Targa");
    }

    #[test]
    fn tag_description_flow() {
        assert_eq!(stub_0x1c44d4(Some("ID3v2")), Some("ID3v2".to_owned()));
        assert_eq!(stub_0x1c44d4(None), None);
        let mut tag = TagSlot::default();
        assert_eq!(stub_0x1c7470(&mut tag, None), 0);
        assert_eq!(tag.description, "");
        assert_eq!(stub_0x1c7470(&mut tag, Some("genre")), 1);
        assert_eq!(tag.description, "genre");
        assert_eq!(stub_0x1c7470(&mut tag, Some("title")), 1);
        assert_eq!(tag.description, "title");
    }
}
