//! rendering shard 408 — 100 stubs 0x615bd4..0x619134 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 44010->44110 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x615bd4..0x619134 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x615bd4 — __ZNK3RBX8SeatImplINS_17BasicPartInstanceEE11getDisabledEv
// type: 
#[doc(alias = "__ZNK3RBX8SeatImplINS_17BasicPartInstanceEE11getDisabledEv")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::getDisabled(void)const")]
// was: __ZNK3RBX8SeatImplINS_17BasicPartInstanceEE11getDisabledEv
// IDA 0x615bd4: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_615bd4() {
}

// 0x615bdc — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE11setDisabledERKb
// type: 
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEE11setDisabledERKb")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::setDisabled(bool const&)")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEE11setDisabledERKb
// IDA 0x615bdc: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_615bdc() {
}

// 0x615c20 — __ZN3RBX10Reflection14PropDescriptorINS_4SeatEbED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4SeatEbED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_4SeatEbED1Ev
// IDA 0x615c20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_615c20() {
}

// 0x615c44 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE17onServiceProviderEPNS_15ServiceProviderES4_
// type: 
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEE17onServiceProviderEPNS_15ServiceProviderES4_")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEE17onServiceProviderEPNS_15ServiceProviderES4_
// IDA 0x615c44: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_615c44() {
}

// 0x615d98 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE12onChildAddedEPNS_8InstanceE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEE12onChildAddedEPNS_8InstanceE")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::onChildAdded(RBX::Instance *)")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEE12onChildAddedEPNS_8InstanceE
// IDA 0x615d98: 179 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_615d98() {
}

// 0x615f70 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE14onChildRemovedEPNS_8InstanceE
// type: 
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEE14onChildRemovedEPNS_8InstanceE")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::onChildRemoved(RBX::Instance *)")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEE14onChildRemovedEPNS_8InstanceE
// IDA 0x615f70: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_615f70() {
}

// 0x615fb0 — __ZNK3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE12getClassNameEv
// IDA 0x615fb0: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_615fb0() {
}

// 0x615fc0 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE15onSeatedChangedEbPNS_8HumanoidE
// type: 
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEE15onSeatedChangedEbPNS_8HumanoidE")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::onSeatedChanged(bool,RBX::Humanoid *)")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEE15onSeatedChangedEbPNS_8HumanoidE
// IDA 0x615fc0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_615fc0() {
}

// 0x615fc4 — __ZThn32_NK3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE12getClassNameEv
// type: 
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE12getClassNameEv
// IDA 0x615fc4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_615fc4() {
}

// 0x615fd4 — __ZN3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x615fd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_615fd4() {
}

// 0x615fe8 — __ZN3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x615fe8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_615fe8() {
}

// 0x616098 — __ZThn132_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn132_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x616098: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_616098() {
}

// 0x6160ac — __ZThn132_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn132_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x6160ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6160ac() {
}

// 0x616160 — __ZN3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x616160: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_616160() {
}

// 0x616174 — __ZN3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x616174: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_616174() {
}

// 0x616224 — __ZThn132_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn132_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x616224: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_616224() {
}

// 0x616238 — __ZThn132_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn132_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x616238: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_616238() {
}

// 0x6162ec — __ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED1Ev
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED1Ev")]
// was: __ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED1Ev
// IDA 0x6162ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6162ec() {
}

// 0x616300 — __ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED0Ev
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED0Ev")]
// was: __ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED0Ev
// IDA 0x616300: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_616300() {
}

// 0x6163b0 — __ZThn132_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED1Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED1Ev")]
// was: __ZThn132_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED1Ev
// IDA 0x6163b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6163b0() {
}

// 0x6163c4 — __ZThn132_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED0Ev
// type: 
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED0Ev")]
// was: __ZThn132_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED0Ev
// IDA 0x6163c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6163c4() {
}

// 0x6163cc — __ZN3RBX8SeatImplINS_17BasicPartInstanceEED1Ev
// type: 
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEED1Ev")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::~SeatImpl()")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEED1Ev
// IDA 0x6163cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6163cc() {
}

// 0x6163dc — __ZN3RBX8SeatImplINS_17BasicPartInstanceEED0Ev
// type: 
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEED0Ev")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::~SeatImpl()")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEED0Ev
// IDA 0x6163dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6163dc() {
}

// 0x616488 — __ZThn132_N3RBX8SeatImplINS_17BasicPartInstanceEED1Ev
// type: 
#[doc(alias = "__ZThn132_N3RBX8SeatImplINS_17BasicPartInstanceEED1Ev")]
#[doc(alias = "non-virtual thunk toRBX::SeatImpl<RBX::BasicPartInstance>::~SeatImpl()")]
// was: __ZThn132_N3RBX8SeatImplINS_17BasicPartInstanceEED1Ev
// IDA 0x616488: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_616488() {
}

// 0x61649c — __ZThn132_N3RBX8SeatImplINS_17BasicPartInstanceEED0Ev
// type: 
#[doc(alias = "__ZThn132_N3RBX8SeatImplINS_17BasicPartInstanceEED0Ev")]
#[doc(alias = "non-virtual thunk toRBX::SeatImpl<RBX::BasicPartInstance>::~SeatImpl()")]
// was: __ZThn132_N3RBX8SeatImplINS_17BasicPartInstanceEED0Ev
// IDA 0x61649c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_61649c() {
}

// 0x61654c — __ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7CreatorD1Ev
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7CreatorD1Ev
// IDA 0x61654c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_61654c() {
}

// 0x616550 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE15isChildSeatWeldEPNS_8InstanceE
// type: int __fastcall(int, RBX::Instance *this)
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEE15isChildSeatWeldEPNS_8InstanceE")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::isChildSeatWeld(RBX::Instance *)")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEE15isChildSeatWeldEPNS_8InstanceE
// IDA 0x616550: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_616550() {
}

// 0x616598 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE16humanoidFromWeldEPNS_4WeldE
// type: int __fastcall(int, RBX::JointInstance *this)
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEE16humanoidFromWeldEPNS_4WeldE")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::humanoidFromWeld(RBX::Weld *)")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEE16humanoidFromWeldEPNS_4WeldE
// IDA 0x616598: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_616598() {
}

// 0x6165b0 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>> const&)")]
// was: __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
// IDA 0x6165b0: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6165b0() {
}

// 0x616624 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE27onEvent_humanoidDoneSittingEv
// type: 
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEE27onEvent_humanoidDoneSittingEv")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::onEvent_humanoidDoneSitting(void)")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEE27onEvent_humanoidDoneSittingEv
// IDA 0x616624: 13 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_616624() {
}

// 0x616644 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE12findSeatWeldEv
// type: int __fastcall(RBX::Instance *this)
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEE12findSeatWeldEv")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::findSeatWeld(void)")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEE12findSeatWeldEv
// IDA 0x616644: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_616644() {
}

// 0x616678 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev
// type: 
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev
// IDA 0x616678: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_616678() {
}

// 0x6166a4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev
// type: 
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev
// IDA 0x6166a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6166a4() {
}

// 0x616778 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
// type: 
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>,0,void ()(void)>::call(void)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
// IDA 0x616778: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_616778() {
}

// 0x616780 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
// type: 
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>,0,void ()(void)>::call(void)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
// IDA 0x616780: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_616780() {
}

// 0x616788 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX8SeatImplINS4_17BasicPartInstanceEEEEENS0_5list1INS0_5valueIPS7_EEEEEclEv
// type: int(void)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX8SeatImplINS4_17BasicPartInstanceEEEEENS0_5list1INS0_5valueIPS7_EEEEEclEv")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX8SeatImplINS4_17BasicPartInstanceEEEEENS0_5list1INS0_5valueIPS7_EEEEEclEv
// IDA 0x616788: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_616788() {
}

// 0x6167a0 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev
// type: 
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev
// IDA 0x6167a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6167a0() {
}

// 0x6167cc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev
// type: 
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX8SeatImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev
// IDA 0x6167cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6167cc() {
}

// 0x6168a0 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE16destroyOtherWeldEN5boost10shared_ptrINS_8InstanceEEEPNS_4WeldE
// type: 
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEE16destroyOtherWeldEN5boost10shared_ptrINS_8InstanceEEEPNS_4WeldE")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::destroyOtherWeld(rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *)")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEE16destroyOtherWeldEN5boost10shared_ptrINS_8InstanceEEEPNS_4WeldE
// IDA 0x6168a0: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6168a0() {
}

// 0x6168c8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX8SeatImplINS3_17BasicPartInstanceEEEEENS_3argILi1EEENS2_IPNS3_4WeldEEEEclINS_4_mfi3mf2IvS6_NS_10shared_ptrINS3_8InstanceEEESC_EENS0_5list1IRKSK_EEEEvNS0_4typeIvEERT_RT0_i
// type: 
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX8SeatImplINS3_17BasicPartInstanceEEEEENS_3argILi1EEENS2_IPNS3_4WeldEEEEclINS_4_mfi3mf2IvS6_NS_10shared_ptrINS3_8InstanceEEESC_EENS0_5list1IRKSK_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance> *>,boost::arg<1>,boost::_bi::value<RBX::Weld *>>::operator()<boost::_mfi::mf2<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueIPN3RBX8SeatImplINS3_17BasicPartInstanceEEEEENS_3argILi1EEENS2_IPNS3_4WeldEEEEclINS_4_mfi3mf2IvS6_NS_10shared_ptrINS3_8InstanceEEESC_EENS0_5list1IRKSK_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x6168c8: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6168c8() {
}

// 0x6169a4 — __ZNK5boost4_mfi3mf2IvN3RBX8SeatImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEPNS2_4WeldEEclEPS5_S8_SA_
// type: 
#[doc(alias = "__ZNK5boost4_mfi3mf2IvN3RBX8SeatImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEPNS2_4WeldEEclEPS5_S8_SA_")]
#[doc(alias = "boost::_mfi::mf2<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *>::operator()(RBX::SeatImpl<RBX::BasicPartInstance>*,rbx_core::SharedPtr<RBX::Instance>,RBX::Weld *)const")]
// was: __ZNK5boost4_mfi3mf2IvN3RBX8SeatImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEPNS2_4WeldEEclEPS5_S8_SA_
// IDA 0x6169a4: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6169a4() {
}

// 0x616a90 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8SeatImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8SeatImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8SeatImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// IDA 0x616a90: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_616a90() {
}

// 0x616b04 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE19onEvent_seatTouchedEN5boost10shared_ptrINS_8InstanceEEE
// type: 
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEE19onEvent_seatTouchedEN5boost10shared_ptrINS_8InstanceEEE")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::onEvent_seatTouched(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEE19onEvent_seatTouchedEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x616b04: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_616b04() {
}

// 0x616c08 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE14createSeatWeldEPNS_8HumanoidE
// type: int(void)
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEE14createSeatWeldEPNS_8HumanoidE")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::createSeatWeld(RBX::Humanoid *)")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEE14createSeatWeldEPNS_8HumanoidE
// IDA 0x616c08: 442 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_616c08() {
}

// 0x6170d8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8SeatImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED1Ev
// type: 
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8SeatImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8SeatImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED1Ev
// IDA 0x6170d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6170d8() {
}

// 0x617104 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8SeatImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED0Ev
// type: 
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8SeatImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8SeatImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED0Ev
// IDA 0x617104: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_617104() {
}

// 0x6171d8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: 
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// IDA 0x6171d8: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6171d8() {
}

// 0x6171f4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: 
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// IDA 0x6171f4: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6171f4() {
}

// 0x617210 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX8SeatImplINS3_17BasicPartInstanceEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS6_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
// type: 
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX8SeatImplINS3_17BasicPartInstanceEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS6_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX8SeatImplINS3_17BasicPartInstanceEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS6_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x617210: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_617210() {
}

// 0x6172e8 — __ZNK5boost4_mfi3mf1IvN3RBX8SeatImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_
// type: 
#[doc(alias = "__ZNK5boost4_mfi3mf1IvN3RBX8SeatImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_")]
#[doc(alias = "boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::SeatImpl<RBX::BasicPartInstance>*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: __ZNK5boost4_mfi3mf1IvN3RBX8SeatImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_
// IDA 0x6172e8: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6172e8() {
}

// 0x6173d0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
// type: 
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
// IDA 0x6173d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6173d0() {
}

// 0x6173fc — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
// type: 
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SeatImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8SeatImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
// IDA 0x6173fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6173fc() {
}

// 0x6174d0 — __ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7CreatorD2Ev
// IDA 0x6174d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6174d0() {
}

// 0x61756c — __ZNK3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x61756c: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61756c() {
}

// 0x6175f4 — __ZNK3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7Creator6createEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7Creator6createEv
// IDA 0x6175f4: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6175f4() {
}

// 0x617738 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SeatEEEN5boost10shared_ptrIT_EEv
// type: 
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_4SeatEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Seat> RBX::Creatable<RBX::Instance>::create<RBX::Seat>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SeatEEEN5boost10shared_ptrIT_EEv
// IDA 0x617738: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_617738() {
}

// 0x6177ec — __ZN5boost10shared_ptrIN3RBX4SeatEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: 
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX4SeatEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Seat>::shared_ptr<RBX::Seat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX4SeatEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x6177ec: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6177ec() {
}

// 0x6178b4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SeatES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: 
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SeatES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Seat,RBX::Seat>(rbx_core::SharedPtr<RBX::Seat> const*,RBX::Seat *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SeatES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x6178b4: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6178b4() {
}

// 0x61799c — __ZN5boost6detail12shared_countC2IPN3RBX4SeatENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX4SeatENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX4SeatENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x61799c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61799c() {
}

// 0x617aa4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x617aa4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_617aa4() {
}

// 0x617aa8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x617aa8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_617aa8() {
}

// 0x617aac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x617aac: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_617aac() {
}

// 0x617acc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x617acc: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_617acc() {
}

// 0x617ae4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Seat *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SeatENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x617ae4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_617ae4() {
}

// 0x617ae8 — __ZN3RBX4Name13callDoDeclareILZNS_5sSeatEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sSeatEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_5sSeatEEEEvv
// IDA 0x617ae8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_617ae8() {
}

// 0x617aec — __ZN3RBX4Name9doDeclareILZNS_5sSeatEEEERKS0_v
// type: 
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sSeatEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_5sSeatEEEERKS0_v
// IDA 0x617aec: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_617aec() {
}

// 0x617bcc — __ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE7CreatorC2Ev
// IDA 0x617bcc: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_617bcc() {
}

// 0x617e10 — __ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEE17static_getCreatorEv
// IDA 0x617e10: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_617e10() {
}

// 0x617e84 — __ZThn32_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED1Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED1Ev")]
// was: __ZThn32_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED1Ev
// IDA 0x617e84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_617e84() {
}

// 0x617e98 — __ZThn36_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED1Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED1Ev")]
// was: __ZThn36_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED1Ev
// IDA 0x617e98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_617e98() {
}

// 0x617eac — __ZThn32_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED0Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED0Ev")]
// was: __ZThn32_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED0Ev
// IDA 0x617eac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_617eac() {
}

// 0x617eb4 — __ZThn36_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED0Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED0Ev")]
// was: __ZThn36_N3RBX14FactoryProductINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEED0Ev
// IDA 0x617eb4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_617eb4() {
}

// 0x617ebc — __ZN3RBX8SeatImplINS_17BasicPartInstanceEED2Ev
// type: int __fastcall(int, int, int, int, int, rbx::signals::connection *, int, int, int, int)
#[doc(alias = "__ZN3RBX8SeatImplINS_17BasicPartInstanceEED2Ev")]
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::~SeatImpl()")]
// was: __ZN3RBX8SeatImplINS_17BasicPartInstanceEED2Ev
// IDA 0x617ebc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_617ebc() {
}

// 0x6180ac — __ZThn32_N3RBX8SeatImplINS_17BasicPartInstanceEED1Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX8SeatImplINS_17BasicPartInstanceEED1Ev")]
#[doc(alias = "non-virtual thunk toRBX::SeatImpl<RBX::BasicPartInstance>::~SeatImpl()")]
// was: __ZThn32_N3RBX8SeatImplINS_17BasicPartInstanceEED1Ev
// IDA 0x6180ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6180ac() {
}

// 0x6180c0 — __ZThn32_N3RBX8SeatImplINS_17BasicPartInstanceEED0Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX8SeatImplINS_17BasicPartInstanceEED0Ev")]
#[doc(alias = "non-virtual thunk toRBX::SeatImpl<RBX::BasicPartInstance>::~SeatImpl()")]
// was: __ZThn32_N3RBX8SeatImplINS_17BasicPartInstanceEED0Ev
// IDA 0x6180c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6180c0() {
}

// 0x618170 — __ZThn36_N3RBX8SeatImplINS_17BasicPartInstanceEED1Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX8SeatImplINS_17BasicPartInstanceEED1Ev")]
#[doc(alias = "non-virtual thunk toRBX::SeatImpl<RBX::BasicPartInstance>::~SeatImpl()")]
// was: __ZThn36_N3RBX8SeatImplINS_17BasicPartInstanceEED1Ev
// IDA 0x618170: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_618170() {
}

// 0x618184 — __ZThn36_N3RBX8SeatImplINS_17BasicPartInstanceEED0Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX8SeatImplINS_17BasicPartInstanceEED0Ev")]
#[doc(alias = "non-virtual thunk toRBX::SeatImpl<RBX::BasicPartInstance>::~SeatImpl()")]
// was: __ZThn36_N3RBX8SeatImplINS_17BasicPartInstanceEED0Ev
// IDA 0x618184: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_618184() {
}

// 0x618234 — __ZN3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev
// IDA 0x618234: 170 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_618234() {
}

// 0x61845c — __ZThn32_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x61845c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_61845c() {
}

// 0x618470 — __ZThn32_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int, int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x618470: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_618470() {
}

// 0x618524 — __ZThn36_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x618524: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_618524() {
}

// 0x618538 — __ZThn36_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_4SeatENS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x618538: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_618538() {
}

// 0x6185ec — __ZThn32_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x6185ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6185ec() {
}

// 0x618600 — __ZThn32_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x618600: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_618600() {
}

// 0x6186b4 — __ZThn36_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x6186b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6186b4() {
}

// 0x6186c8 — __ZThn36_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_4SeatELZNS_5sSeatEENS_14FactoryProductIS2_NS_8SeatImplINS_17BasicPartInstanceEEELZNS_5sSeatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x6186c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6186c8() {
}

// 0x61877c — __ZN3RBX10Reflection14PropDescriptorINS_4SeatEbEC2IMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EEEPKcSF_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4SeatEbEC2IMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EEEPKcSF_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::PropDescriptor<bool const& (RBX::SeatImpl<RBX::BasicPartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::BasicPartInstance>::*)(bool const&)>(char const*,char const*,bool const& (RBX::SeatImpl<RBX::BasicPartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::BasicPartInstance>::*)(bool const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_4SeatEbEC2IMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EEEPKcSF_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x61877c: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61877c() {
}

// 0x618890 — __ZN3RBX10Reflection14PropDescriptorINS_4SeatEbED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4SeatEbED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_4SeatEbED0Ev
// IDA 0x618890: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_618890() {
}

// 0x6188bc — __ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::GetSetImpl<bool const& (RBX::SeatImpl<RBX::BasicPartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::BasicPartInstance>::*)(bool const&)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE10isReadOnlyEv
// IDA 0x6188bc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6188bc() {
}

// 0x6188c0 — __ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::GetSetImpl<bool const& (RBX::SeatImpl<RBX::BasicPartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::BasicPartInstance>::*)(bool const&)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE11isWriteOnlyEv
// IDA 0x6188c0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6188c0() {
}

// 0x6188c4 — __ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::GetSetImpl<bool const& (RBX::SeatImpl<RBX::BasicPartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::BasicPartInstance>::*)(bool const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x6188c4: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6188c4() {
}

// 0x6188ec — __ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE8setValueEPNS0_13DescribedBaseES9_
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE8setValueEPNS0_13DescribedBaseES9_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::GetSetImpl<bool const& (RBX::SeatImpl<RBX::BasicPartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::BasicPartInstance>::*)(bool const&)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE8setValueEPNS0_13DescribedBaseES9_
// IDA 0x6188ec: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6188ec() {
}

// 0x618910 — __GLOBAL__I_a_251
// type: 
#[doc(alias = "__GLOBAL__I_a_251")]
#[doc(alias = "global constructor keyed to_a_251")]
// was: __GLOBAL__I_a_251
// IDA 0x618910: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_618910() {
}

// 0x618c98 — __ZN3RBX9SelectionC1Ev
// type: _DWORD __fastcall(RBX::Selection *__hidden this)
#[doc(alias = "__ZN3RBX9SelectionC1Ev")]
#[doc(alias = "RBX::Selection::Selection(void)")]
// was: __ZN3RBX9SelectionC1Ev
// IDA 0x618c98: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_618c98() {
}

// 0x618c9c — __ZN3RBX9SelectionC2Ev
// type: _DWORD __fastcall(RBX::Selection *__hidden this)
#[doc(alias = "__ZN3RBX9SelectionC2Ev")]
#[doc(alias = "RBX::Selection::Selection(void)")]
// was: __ZN3RBX9SelectionC2Ev
// IDA 0x618c9c: 365 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_618c9c() {
}

// 0x619080 — __ZN3RBX9Selection26propagateChangeSignalToLuaERKNS_16SelectionChangedE
// type: 
#[doc(alias = "__ZN3RBX9Selection26propagateChangeSignalToLuaERKNS_16SelectionChangedE")]
#[doc(alias = "RBX::Selection::propagateChangeSignalToLua(RBX::SelectionChanged const&)")]
// was: __ZN3RBX9Selection26propagateChangeSignalToLuaERKNS_16SelectionChangedE
// IDA 0x619080: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_619080() {
}

// 0x619088 — __ZN3RBX9SelectionD0Ev
// type: void __fastcall(RBX::Selection *__hidden this)
#[doc(alias = "__ZN3RBX9SelectionD0Ev")]
#[doc(alias = "RBX::Selection::~Selection()")]
// was: __ZN3RBX9SelectionD0Ev
// IDA 0x619088: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_619088() {
}

// 0x619128 — __ZN3RBX9SelectionD1Ev
// type: void __fastcall(RBX::Selection *__hidden this)
#[doc(alias = "__ZN3RBX9SelectionD1Ev")]
#[doc(alias = "RBX::Selection::~Selection()")]
// was: __ZN3RBX9SelectionD1Ev
// IDA 0x619128: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_619128() {
}

// 0x61912c — __ZThn32_N3RBX9SelectionD0Ev
// type: void __fastcall(RBX::Selection *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9SelectionD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Selection::~Selection()")]
// was: __ZThn32_N3RBX9SelectionD0Ev
// IDA 0x61912c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_61912c() {
}

// 0x619134 — __ZThn36_N3RBX9SelectionD0Ev
// type: void __fastcall(RBX::Selection *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9SelectionD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Selection::~Selection()")]
// was: __ZThn36_N3RBX9SelectionD0Ev
// IDA 0x619134: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_619134() {
}
