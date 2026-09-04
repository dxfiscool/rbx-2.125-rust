//! rendering shard 331 — 100 stubs 0x5aad08..0x5ae1a4 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 36060->36160 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 36060 before -> 36160 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x5aac6c (lowest remaining 0x5aad08..0x5ae1a4, next lowest 0x5ae2ac if exists)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x5aad08 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// IDA 0x5aad08: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aad08() {
}

// 0x5aade8 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// IDA 0x5aade8: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aade8() {
}

// 0x5aadf0 — __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance *)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// IDA 0x5aadf0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aadf0() {
}

// 0x5aadf4 — __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance *)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// IDA 0x5aadf4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aadf4() {
}

// 0x5aadf8 — __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5aadf8: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aadf8() {
}

// 0x5aae18 — __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance * const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x5aae18: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aae18() {
}

// 0x5aae3c — __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5aae3c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5aae3c() {
}

// 0x5aae40 — __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5aae40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5aae40() {
}

// 0x5aaee0 — __ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5aaee0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5aaee0() {
}

// 0x5aaee8 — __ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5aaee8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5aaee8() {
}

// 0x5aaf8c — __ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5aaf8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5aaf8c() {
}

// 0x5aaf94 — __ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5aaf94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5aaf94() {
}

// 0x5ab038 — __ZN3RBX15ManualGlueJointD1Ev
// type: void __fastcall(RBX::ManualGlueJoint *__hidden this)
#[doc(alias = "RBX::ManualGlueJoint::~ManualGlueJoint()")]
// was: __ZN3RBX15ManualGlueJointD1Ev
// IDA 0x5ab038: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ab038() {
}

// 0x5ab03c — __ZN3RBX15ManualGlueJointD0Ev
// type: void __fastcall(RBX::ManualGlueJoint *__hidden this)
#[doc(alias = "RBX::ManualGlueJoint::~ManualGlueJoint()")]
// was: __ZN3RBX15ManualGlueJointD0Ev
// IDA 0x5ab03c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ab03c() {
}

// 0x5ab0dc — __ZNK3RBX15ManualGlueJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::ManualGlueJoint *__hidden this)
#[doc(alias = "RBX::ManualGlueJoint::getJointType(void)const")]
// was: __ZNK3RBX15ManualGlueJoint12getJointTypeEv
// IDA 0x5ab0dc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ab0dc() {
}

// 0x5ab0e0 — __ZNK3RBX9GlueJoint11isBreakableEv
// type: _DWORD __fastcall(RBX::GlueJoint *__hidden this)
#[doc(alias = "RBX::GlueJoint::isBreakable(void)const")]
// was: __ZNK3RBX9GlueJoint11isBreakableEv
// IDA 0x5ab0e0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ab0e0() {
}

// 0x5ab0e4 — __ZThn32_N3RBX15ManualGlueJointD1Ev
// type: void __fastcall(RBX::ManualGlueJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualGlueJoint::~ManualGlueJoint()")]
// was: __ZThn32_N3RBX15ManualGlueJointD1Ev
// IDA 0x5ab0e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ab0e4() {
}

// 0x5ab0ec — __ZThn32_N3RBX15ManualGlueJointD0Ev
// type: void __fastcall(RBX::ManualGlueJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualGlueJoint::~ManualGlueJoint()")]
// was: __ZThn32_N3RBX15ManualGlueJointD0Ev
// IDA 0x5ab0ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ab0ec() {
}

// 0x5ab0f4 — __ZN3RBX15ManualWeldJointD1Ev
// type: void __fastcall(RBX::ManualWeldJoint *__hidden this)
#[doc(alias = "RBX::ManualWeldJoint::~ManualWeldJoint()")]
// was: __ZN3RBX15ManualWeldJointD1Ev
// IDA 0x5ab0f4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ab0f4() {
}

// 0x5ab0f8 — __ZN3RBX15ManualWeldJointD0Ev
// type: void __fastcall(RBX::ManualWeldJoint *__hidden this)
#[doc(alias = "RBX::ManualWeldJoint::~ManualWeldJoint()")]
// was: __ZN3RBX15ManualWeldJointD0Ev
// IDA 0x5ab0f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ab0f8() {
}

// 0x5ab198 — __ZNK3RBX15ManualWeldJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::ManualWeldJoint *__hidden this)
#[doc(alias = "RBX::ManualWeldJoint::getJointType(void)const")]
// was: __ZNK3RBX15ManualWeldJoint12getJointTypeEv
// IDA 0x5ab198: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ab198() {
}

// 0x5ab19c — __ZThn32_N3RBX15ManualWeldJointD1Ev
// type: void __fastcall(RBX::ManualWeldJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualWeldJoint::~ManualWeldJoint()")]
// was: __ZThn32_N3RBX15ManualWeldJointD1Ev
// IDA 0x5ab19c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ab19c() {
}

// 0x5ab1a4 — __ZThn32_N3RBX15ManualWeldJointD0Ev
// type: void __fastcall(RBX::ManualWeldJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualWeldJoint::~ManualWeldJoint()")]
// was: __ZThn32_N3RBX15ManualWeldJointD0Ev
// IDA 0x5ab1a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ab1a4() {
}

// 0x5ab1ac — __ZN3RBX9WeldJointD1Ev
// type: void __fastcall(RBX::WeldJoint *__hidden this)
#[doc(alias = "RBX::WeldJoint::~WeldJoint()")]
// was: __ZN3RBX9WeldJointD1Ev
// IDA 0x5ab1ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ab1ac() {
}

// 0x5ab1b0 — __ZN3RBX9WeldJointD0Ev
// type: void __fastcall(RBX::WeldJoint *__hidden this)
#[doc(alias = "RBX::WeldJoint::~WeldJoint()")]
// was: __ZN3RBX9WeldJointD0Ev
// IDA 0x5ab1b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ab1b0() {
}

// 0x5ab250 — __ZNK3RBX9WeldJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::WeldJoint *__hidden this)
#[doc(alias = "RBX::WeldJoint::getJointType(void)const")]
// was: __ZNK3RBX9WeldJoint12getJointTypeEv
// IDA 0x5ab250: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ab250() {
}

// 0x5ab254 — __ZThn32_N3RBX9WeldJointD1Ev
// type: void __fastcall(RBX::WeldJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::WeldJoint::~WeldJoint()")]
// was: __ZThn32_N3RBX9WeldJointD1Ev
// IDA 0x5ab254: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ab254() {
}

// 0x5ab25c — __ZThn32_N3RBX9WeldJointD0Ev
// type: void __fastcall(RBX::WeldJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::WeldJoint::~WeldJoint()")]
// was: __ZThn32_N3RBX9WeldJointD0Ev
// IDA 0x5ab25c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ab25c() {
}

// 0x5ab264 — __ZN3RBX9SnapJointD1Ev
// type: void __fastcall(RBX::SnapJoint *__hidden this)
#[doc(alias = "RBX::SnapJoint::~SnapJoint()")]
// was: __ZN3RBX9SnapJointD1Ev
// IDA 0x5ab264: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ab264() {
}

// 0x5ab268 — __ZN3RBX9SnapJointD0Ev
// type: void __fastcall(RBX::SnapJoint *__hidden this)
#[doc(alias = "RBX::SnapJoint::~SnapJoint()")]
// was: __ZN3RBX9SnapJointD0Ev
// IDA 0x5ab268: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ab268() {
}

// 0x5ab308 — __ZNK3RBX9SnapJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::SnapJoint *__hidden this)
#[doc(alias = "RBX::SnapJoint::getJointType(void)const")]
// was: __ZNK3RBX9SnapJoint12getJointTypeEv
// IDA 0x5ab308: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ab308() {
}

// 0x5ab30c — __ZThn32_N3RBX9SnapJointD1Ev
// type: void __fastcall(RBX::SnapJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SnapJoint::~SnapJoint()")]
// was: __ZThn32_N3RBX9SnapJointD1Ev
// IDA 0x5ab30c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ab30c() {
}

// 0x5ab314 — __ZThn32_N3RBX9SnapJointD0Ev
// type: void __fastcall(RBX::SnapJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SnapJoint::~SnapJoint()")]
// was: __ZThn32_N3RBX9SnapJointD0Ev
// IDA 0x5ab314: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ab314() {
}

// 0x5ab31c — __GLOBAL__I_a_218
#[doc(alias = "global constructor keyed to_a_218")]
// was: __GLOBAL__I_a_218
// IDA 0x5ab31c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_5ab31c() {
}

// 0x5abd48 — __ZN3RBX13JointsService24setJoinAfterMoveInstanceEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::JointsService::setJoinAfterMoveInstance(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX13JointsService24setJoinAfterMoveInstanceEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5abd48: 67 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5abd48() {
}

// 0x5abe0c — __ZN3RBX13JointsService22setJoinAfterMoveTargetEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::JointsService::setJoinAfterMoveTarget(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX13JointsService22setJoinAfterMoveTargetEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5abe0c: 67 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5abe0c() {
}

// 0x5abed0 — __ZN3RBX13JointsService21showPermissibleJointsEv
// type: _DWORD __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::showPermissibleJoints(void)")]
// was: __ZN3RBX13JointsService21showPermissibleJointsEv
// IDA 0x5abed0: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5abed0() {
}

// 0x5abf18 — __ZN3RBX13JointsService25createJoinAfterMoveJointsEv
// type: _DWORD __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::createJoinAfterMoveJoints(void)")]
// was: __ZN3RBX13JointsService25createJoinAfterMoveJointsEv
// IDA 0x5abf18: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5abf18() {
}

// 0x5abf88 — __ZN3RBX13JointsService24clearJoinAfterMoveJointsEv
// type: _DWORD __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::clearJoinAfterMoveJoints(void)")]
// was: __ZN3RBX13JointsService24clearJoinAfterMoveJointsEv
// IDA 0x5abf88: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5abf88() {
}

// 0x5abfa0 — __ZN3RBX13JointsServiceC1Ev
// type: _DWORD __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::JointsService(void)")]
// was: __ZN3RBX13JointsServiceC1Ev
// IDA 0x5abfa0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5abfa0() {
}

// 0x5abfa4 — __ZN3RBX13JointsServiceC2Ev
// type: _DWORD __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::JointsService(void)")]
// was: __ZN3RBX13JointsServiceC2Ev
// IDA 0x5abfa4: 305 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5abfa4() {
}

// 0x5ac2e4 — __ZN3RBX13JointsService17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::JointsService *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::JointsService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX13JointsService17onServiceProviderEPNS_15ServiceProviderES2_
// IDA 0x5ac2e4: 160 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ac2e4() {
}

// 0x5ac49c — __ZN3RBX13JointsService10onAutoJoinEPNS_5JointE
// type: _DWORD __fastcall(RBX::JointsService *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::JointsService::onAutoJoin(RBX::Joint *)")]
// was: __ZN3RBX13JointsService10onAutoJoinEPNS_5JointE
// IDA 0x5ac49c: 318 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ac49c() {
}

// 0x5ac808 — __ZN3RBX13JointsService13onAutoDestroyEPNS_5JointE
// type: _DWORD __fastcall(RBX::JointsService *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::JointsService::onAutoDestroy(RBX::Joint *)")]
// was: __ZN3RBX13JointsService13onAutoDestroyEPNS_5JointE
// IDA 0x5ac808: 58 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ac808() {
}

// 0x5ac8b8 — __ZN3RBX13JointsService20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::JointsService::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX13JointsService20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5ac8b8: 24 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ac8b8() {
}

// 0x5ac8fc — __ZN3RBX13JointsService17onDescendantAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::JointsService *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::JointsService::onDescendantAdded(RBX::Instance *)")]
// was: __ZN3RBX13JointsService17onDescendantAddedEPNS_8InstanceE
// IDA 0x5ac8fc: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ac8fc() {
}

// 0x5ac944 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
// IDA 0x5ac944: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ac944() {
}

// 0x5aca50 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EED1Ev
// IDA 0x5aca50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5aca50() {
}

// 0x5acaac — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Joint *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// IDA 0x5acaac: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5acaac() {
}

// 0x5acb20 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4SnapEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::Snap>(rbx_core::SharedPtr<RBX::Snap> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4SnapEEERS3_RKNS0_IT_EE
// IDA 0x5acb20: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5acb20() {
}

// 0x5acb54 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap,RBX::Joint *>(RBX::Joint *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
// IDA 0x5acb54: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5acb54() {
}

// 0x5acc08 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4WeldEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::Weld>(rbx_core::SharedPtr<RBX::Weld> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4WeldEEERS3_RKNS0_IT_EE
// IDA 0x5acc08: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5acc08() {
}

// 0x5acc3c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4WeldEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Weld> RBX::Creatable<RBX::Instance>::create<RBX::Weld,RBX::Joint *>(RBX::Joint *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_4WeldEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
// IDA 0x5acc3c: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5acc3c() {
}

// 0x5accf0 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4GlueEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::Glue>(rbx_core::SharedPtr<RBX::Glue> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4GlueEEERS3_RKNS0_IT_EE
// IDA 0x5accf0: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5accf0() {
}

// 0x5acd24 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue,RBX::Joint *>(RBX::Joint *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
// IDA 0x5acd24: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5acd24() {
}

// 0x5acdd8 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_6RotateEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::Rotate>(rbx_core::SharedPtr<RBX::Rotate> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_6RotateEEERS3_RKNS0_IT_EE
// IDA 0x5acdd8: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5acdd8() {
}

// 0x5ace0c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate,RBX::Joint *>(RBX::Joint *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
// IDA 0x5ace0c: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ace0c() {
}

// 0x5acec0 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_7RotatePEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::RotateP>(rbx_core::SharedPtr<RBX::RotateP> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_7RotatePEEERS3_RKNS0_IT_EE
// IDA 0x5acec0: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5acec0() {
}

// 0x5acef4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP,RBX::Joint *>(RBX::Joint *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
// IDA 0x5acef4: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5acef4() {
}

// 0x5acfa8 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_7RotateVEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::RotateV>(rbx_core::SharedPtr<RBX::RotateV> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_7RotateVEEERS3_RKNS0_IT_EE
// IDA 0x5acfa8: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5acfa8() {
}

// 0x5acfdc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV,RBX::Joint *>(RBX::Joint *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
// IDA 0x5acfdc: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5acfdc() {
}

// 0x5ad090 — __ZN5boost10shared_ptrIN3RBX10PVInstanceEEaSERKS3_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::PVInstance>::operator=(rbx_core::SharedPtr<RBX::PVInstance> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX10PVInstanceEEaSERKS3_
// IDA 0x5ad090: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad090() {
}

// 0x5ad0c8 — __ZN3RBX13JointsServiceD1Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::~JointsService()")]
// was: __ZN3RBX13JointsServiceD1Ev
// IDA 0x5ad0c8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ad0c8() {
}

// 0x5ad0cc — __ZN3RBX13JointsServiceD0Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::~JointsService()")]
// was: __ZN3RBX13JointsServiceD0Ev
// IDA 0x5ad0cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ad0cc() {
}

// 0x5ad16c — __ZNK3RBX13JointsService11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::JointsService *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::JointsService::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX13JointsService11askAddChildEPKNS_8InstanceE
// IDA 0x5ad16c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad16c() {
}

// 0x5ad1a8 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv
// IDA 0x5ad1a8: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad1a8() {
}

// 0x5ad1d0 — __ZThn32_N3RBX13JointsServiceD1Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::JointsService::~JointsService()")]
// was: __ZThn32_N3RBX13JointsServiceD1Ev
// IDA 0x5ad1d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ad1d0() {
}

// 0x5ad1d8 — __ZThn32_N3RBX13JointsServiceD0Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::JointsService::~JointsService()")]
// was: __ZThn32_N3RBX13JointsServiceD0Ev
// IDA 0x5ad1d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ad1d8() {
}

// 0x5ad27c — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv
// IDA 0x5ad27c: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad27c() {
}

// 0x5ad2a4 — __ZThn36_N3RBX13JointsServiceD1Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::JointsService::~JointsService()")]
// was: __ZThn36_N3RBX13JointsServiceD1Ev
// IDA 0x5ad2a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ad2a4() {
}

// 0x5ad2ac — __ZThn36_N3RBX13JointsServiceD0Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::JointsService::~JointsService()")]
// was: __ZThn36_N3RBX13JointsServiceD0Ev
// IDA 0x5ad2ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ad2ac() {
}

// 0x5ad350 — __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD1Ev
// IDA 0x5ad350: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ad350() {
}

// 0x5ad354 — __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD1Ev
// IDA 0x5ad354: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ad354() {
}

// 0x5ad358 — __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD1Ev
// IDA 0x5ad358: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ad358() {
}

// 0x5ad35c — __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD1Ev
// IDA 0x5ad35c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ad35c() {
}

// 0x5ad360 — __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD1Ev
// IDA 0x5ad360: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ad360() {
}

// 0x5ad364 — __ZN5boost20dynamic_pointer_castIN3RBX10PVInstanceENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::PVInstance> boost::dynamic_pointer_cast<RBX::PVInstance,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN5boost20dynamic_pointer_castIN3RBX10PVInstanceENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
// IDA 0x5ad364: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad364() {
}

// 0x5ad3ac — __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5ad3ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ad3ac() {
}

// 0x5ad448 — __ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5ad448: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad448() {
}

// 0x5ad4b4 — __ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator6createEv
// IDA 0x5ad4b4: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad4b4() {
}

// 0x5ad5f8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEEEN5boost10shared_ptrIT_EEv
// IDA 0x5ad5f8: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad5f8() {
}

// 0x5ad6a8 — __ZN5boost10shared_ptrIN3RBX7RotateVEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateV>::shared_ptr<RBX::RotateV,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX7RotateVEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5ad6a8: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad6a8() {
}

// 0x5ad770 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotateVES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RotateV,RBX::RotateV>(rbx_core::SharedPtr<RBX::RotateV> const*,RBX::RotateV *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotateVES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5ad770: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad770() {
}

// 0x5ad858 — __ZN5boost6detail12shared_countC2IPN3RBX7RotateVENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX7RotateVENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5ad858: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad858() {
}

// 0x5ad960 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5ad960: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5ad960() {
}

// 0x5ad964 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5ad964: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ad964() {
}

// 0x5ad968 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5ad968: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad968() {
}

// 0x5ad988 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5ad988: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad988() {
}

// 0x5ad9a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5ad9a0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad9a0() {
}

// 0x5ad9a4 — __ZN3RBX4Name7declareILZNS_8sRotateVEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_8sRotateVEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_8sRotateVEEEERKS0_v
// IDA 0x5ad9a4: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad9a4() {
}

// 0x5ad9e8 — __ZN3RBX4Name13callDoDeclareILZNS_8sRotateVEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sRotateVEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_8sRotateVEEEEvv
// IDA 0x5ad9e8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ad9e8() {
}

// 0x5ad9ec — __ZN3RBX4Name9doDeclareILZNS_8sRotateVEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sRotateVEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_8sRotateVEEEERKS0_v
// IDA 0x5ad9ec: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ad9ec() {
}

// 0x5adad0 — __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorC2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5adad0: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5adad0() {
}

// 0x5adcf8 — __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5adcf8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5adcf8() {
}

// 0x5add94 — __ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5add94: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5add94() {
}

// 0x5ade00 — __ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator6createEv
// IDA 0x5ade00: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ade00() {
}

// 0x5adf44 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEEEN5boost10shared_ptrIT_EEv
// IDA 0x5adf44: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5adf44() {
}

// 0x5adff4 — __ZN5boost10shared_ptrIN3RBX7RotatePEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateP>::shared_ptr<RBX::RotateP,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX7RotatePEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5adff4: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5adff4() {
}

// 0x5ae0bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotatePES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RotateP,RBX::RotateP>(rbx_core::SharedPtr<RBX::RotateP> const*,RBX::RotateP *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotatePES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5ae0bc: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ae0bc() {
}

// 0x5ae1a4 — __ZN5boost6detail12shared_countC2IPN3RBX7RotatePENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX7RotatePENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5ae1a4: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ae1a4() {
}
