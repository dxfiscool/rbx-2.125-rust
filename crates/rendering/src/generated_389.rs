//! rendering shard 389 — 100 stubs 0x57654c..0x579878 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 41810->41910 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x57654c..0x579878 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x57654c — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>> const&)")]
// was: __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_
// IDA 0x57654c: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57654c() {
}

// 0x5765c0 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
// IDA 0x5765c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5765c0() {
}

// 0x5765ec — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
// IDA 0x5765ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5765ec() {
}

// 0x5766c0 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>,0,void ()(void)>::call(void)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// IDA 0x5766c0: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5766c0() {
}

// 0x5766c8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>,0,void ()(void)>::call(void)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// IDA 0x5766c8: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5766c8() {
}

// 0x5766d0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9HopperBinEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// type: int(void)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9HopperBinEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9HopperBinEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// IDA 0x5766d0: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5766d0() {
}

// 0x5766e8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev
// IDA 0x5766e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5766e8() {
}

// 0x576714 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev
// IDA 0x576714: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_576714() {
}

// 0x5767e8 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEEC2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEEC2Ev
// IDA 0x5767e8: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5767e8() {
}

// 0x576944 — __ZN3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x576944: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_576944() {
}

// 0x576a78 — __ZN3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x576a78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_576a78() {
}

// 0x576bbc — __ZThn32_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x576bbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_576bbc() {
}

// 0x576cec — __ZThn32_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x576cec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_576cec() {
}

// 0x576e30 — __ZThn36_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x576e30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_576e30() {
}

// 0x576f60 — __ZThn36_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x576f60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_576f60() {
}

// 0x5770a4 — __ZN3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5770a4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5770a4() {
}

// 0x5770a8 — __ZN3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5770a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5770a8() {
}

// 0x577148 — __ZThn32_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x577148: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_577148() {
}

// 0x577150 — __ZThn32_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x577150: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_577150() {
}

// 0x5771f4 — __ZThn36_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5771f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5771f4() {
}

// 0x5771fc — __ZThn36_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5771fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5771fc() {
}

// 0x5772a0 — __ZN3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5772a0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5772a0() {
}

// 0x5772a4 — __ZN3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5772a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5772a4() {
}

// 0x577344 — __ZThn32_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x577344: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_577344() {
}

// 0x57734c — __ZThn32_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x57734c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57734c() {
}

// 0x5773f0 — __ZThn36_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5773f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5773f0() {
}

// 0x5773f8 — __ZThn36_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5773f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5773f8() {
}

// 0x57749c — __ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsEC2IiMS2_FvRKSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsEC2IiMS2_FvRKSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::PropDescriptor<int,void (RBX::HopperBin::*)(std::string const&)>(char const*,char const*,int,void (RBX::HopperBin::*)(std::string const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsEC2IiMS2_FvRKSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x57749c: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57749c() {
}

// 0x5775a8 — __ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsED0Ev
// IDA 0x5775a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5775a8() {
}

// 0x5775d4 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE10isReadOnlyEv
// IDA 0x5775d4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5775d4() {
}

// 0x5775d8 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE11isWriteOnlyEv
// IDA 0x5775d8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5775d8() {
}

// 0x5775dc — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5775dc: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5775dc() {
}

// 0x5776fc — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8setValueEPNS0_13DescribedBaseES6_
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8setValueEPNS0_13DescribedBaseES6_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8setValueEPNS0_13DescribedBaseES6_
// IDA 0x5776fc: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5776fc() {
}

// 0x577720 — __ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::BoundFuncDesc(void (RBX::HopperBin::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x577720: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_577720() {
}

// 0x577824 — __ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EED0Ev
// IDA 0x577824: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_577824() {
}

// 0x5778d8 — __ZNK3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x5778d8: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5778d8() {
}

// 0x5778f8 — __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// IDA 0x5778f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5778f8() {
}

// 0x5779ac — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x5779ac: 198 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5779ac() {
}

// 0x577bb0 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x577bb0: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_577bb0() {
}

// 0x577c24 — __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x577c24: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_577c24() {
}

// 0x577c38 — __ZN3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEED0Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEED0Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEED0Ev
// IDA 0x577c38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_577c38() {
}

// 0x577cec — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x577cec: 204 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_577cec() {
}

// 0x577f00 — __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv
// IDA 0x577f00: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_577f00() {
}

// 0x577f08 — __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// IDA 0x577f08: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_577f08() {
}

// 0x577f10 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x577f10: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_577f10() {
}

// 0x577f84 — __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// IDA 0x577f84: 7 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_577f84() {
}

// 0x577f94 — __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x577f94: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_577f94() {
}

// 0x577fa8 — __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x577fa8: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_577fa8() {
}

// 0x57812c — __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev
// IDA 0x57812c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57812c() {
}

// 0x5781e0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// IDA 0x5781e0: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5781e0() {
}

// 0x578344 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// IDA 0x578344: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578344() {
}

// 0x5784a4 — __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x5784a4: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5784a4() {
}

// 0x5784b8 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9HopperBinEEEPKcS7_MT_bMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9HopperBinEEEPKcS7_MT_bMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HopperBin>(char const*,char const*,bool RBX::HopperBin::*,void (RBX::HopperBin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9HopperBinEEEPKcS7_MT_bMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE
// IDA 0x5784b8: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5784b8() {
}

// 0x57864c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE10isReadOnlyEv
// IDA 0x57864c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57864c() {
}

// 0x578650 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE11isWriteOnlyEv
// IDA 0x578650: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578650() {
}

// 0x578654 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x578654: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578654() {
}

// 0x578660 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8setValueEPNS0_13DescribedBaseERKb")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x578660: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578660() {
}

// 0x5786b0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::EnumPropDescriptor<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>(char const*,char const*,RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5786b0: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5786b0() {
}

// 0x578864 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEED0Ev
// IDA 0x578864: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_578864() {
}

// 0x578890 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10isReadOnlyEv
// IDA 0x578890: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578890() {
}

// 0x5788a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11isWriteOnlyEv
// IDA 0x5788a0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5788a0() {
}

// 0x5788b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11equalValuesEPKNS0_13DescribedBaseES7_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x5788b0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5788b0() {
}

// 0x5788d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x5788d8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5788d8() {
}

// 0x5788fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x5788fc: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5788fc() {
}

// 0x578a48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x578a48: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578a48() {
}

// 0x578a6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14hasStringValueEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14hasStringValueEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14hasStringValueEv
// IDA 0x578a6c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578a6c() {
}

// 0x578a70 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14getStringValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x578a70: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578a70() {
}

// 0x578a94 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x578a94: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578a94() {
}

// 0x578ad4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x578ad4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578ad4() {
}

// 0x578af4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x578af4: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578af4() {
}

// 0x578d34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE13getIndexValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x578d34: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578d34() {
}

// 0x578d50 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE13setIndexValueEPNS0_13DescribedBaseEm")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x578d50: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578d50() {
}

// 0x578d84 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE12getEnumValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x578d84: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578d84() {
}

// 0x578d8c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE12setEnumValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x578d8c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578d8c() {
}

// 0x578dd8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11getEnumItemEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x578dd8: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578dd8() {
}

// 0x578df8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x578df8: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578df8() {
}

// 0x578e2c — __ZNK3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE14convertToIndexES3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::convertToIndex(RBX::HopperBin::BinType)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE14convertToIndexES3_
// IDA 0x578e2c: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578e2c() {
}

// 0x578e9c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11setIntValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x578e9c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578e9c() {
}

// 0x578edc — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x578edc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578edc() {
}

// 0x578ee0 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x578ee0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578ee0() {
}

// 0x578ee4 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x578ee4: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578ee4() {
}

// 0x578f04 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::setValue(RBX::Reflection::DescribedBase *,RBX::HopperBin::BinType const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x578f04: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578f04() {
}

// 0x578f28 — __ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEEC2IMS2_KFKS3_vEMS2_FvRS6_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEEC2IMS2_KFKS3_vEMS2_FvRS6_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::PropDescriptor<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>(char const*,char const*,RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEEC2IMS2_KFKS3_vEMS2_FvRS6_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x578f28: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_578f28() {
}

// 0x57903c — __ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEED0Ev
// IDA 0x57903c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57903c() {
}

// 0x579068 — __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE10isReadOnlyEv
// IDA 0x579068: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_579068() {
}

// 0x57906c — __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE11isWriteOnlyEv
// IDA 0x57906c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57906c() {
}

// 0x579070 — __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x579070: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_579070() {
}

// 0x579098 — __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE8setValueEPNS0_13DescribedBaseES9_
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE8setValueEPNS0_13DescribedBaseES9_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE8setValueEPNS0_13DescribedBaseES9_
// IDA 0x579098: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_579098() {
}

// 0x5790bc — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "__ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE6resizeEmS2_")]
#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::resize(unsigned long,RBX::HopperBin::BinType)")]
// was: __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE6resizeEmS2_
// IDA 0x5790bc: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5790bc() {
}

// 0x5790f0 — __ZNSt3mapIPKN3RBX4NameENS0_9HopperBin7BinTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_9HopperBin7BinTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
#[doc(alias = "std::map<RBX::Name const*,RBX::HopperBin::BinType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_9HopperBin7BinTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x5790f0: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5790f0() {
}

// 0x579148 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x579148: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_579148() {
}

// 0x5791fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x5791fc: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5791fc() {
}

// 0x579254 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x579254: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_579254() {
}

// 0x5792bc — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "__ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,unsigned long,RBX::HopperBin::BinType const&)")]
// was: __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x5792bc: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5792bc() {
}

// 0x57944c — __ZNSt12_Vector_baseIN3RBX9HopperBin7BinTypeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9HopperBin7BinTypeESaIS2_EE11_M_allocateEm")]
#[doc(alias = "std::_Vector_base<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX9HopperBin7BinTypeESaIS2_EE11_M_allocateEm
// IDA 0x57944c: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_57944c() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x579464 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9HopperBin7BinTypeES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9HopperBin7BinTypeES6_EET0_T_S8_S7_")]
#[doc(alias = "RBX::HopperBin::BinType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HopperBin::BinType *,RBX::HopperBin::BinType *>(RBX::HopperBin::BinType *,RBX::HopperBin::BinType *,RBX::HopperBin::BinType *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9HopperBin7BinTypeES6_EET0_T_S8_S7_
// IDA 0x579464: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_579464() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x5794a0 — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "__ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE9push_backERKS2_")]
#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::push_back(RBX::HopperBin::BinType const&)")]
// was: __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE9push_backERKS2_
// IDA 0x5794a0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_5794a0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x5794c8 — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "__ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,RBX::HopperBin::BinType const&)")]
// was: __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x5794c8: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_5794c8() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x5795ac — __ZN3RBX9HopperBinD2Ev
// type: void __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZN3RBX9HopperBinD2Ev")]
#[doc(alias = "RBX::HopperBin::~HopperBin()")]
// was: __ZN3RBX9HopperBinD2Ev
// IDA 0x5795ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5795ac() {
}

// 0x579878 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEED2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEED2Ev
// IDA 0x579878: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_579878() {
}
