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
pub fn stub_5aad08() -> ! {
    todo!("0x5aad08 RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0x5aade8 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
pub fn stub_5aade8() -> ! {
    todo!("0x5aade8 non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0x5aadf0 — __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance *)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
pub fn stub_5aadf0() -> ! {
    todo!("0x5aadf0 RBX::Reflection::PropDescriptor<RBX::JointInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance *)>::isReadOnly(void)const")
}

// 0x5aadf4 — __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance *)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
pub fn stub_5aadf4() -> ! {
    todo!("0x5aadf4 RBX::Reflection::PropDescriptor<RBX::JointInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance *)>::isWriteOnly(void)const")
}

// 0x5aadf8 — __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_5aadf8() -> ! {
    todo!("0x5aadf8 RBX::Reflection::PropDescriptor<RBX::JointInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance *)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x5aae18 — __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance * const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
pub fn stub_5aae18() -> ! {
    todo!("0x5aae18 RBX::Reflection::PropDescriptor<RBX::JointInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance * const&)const")
}

// 0x5aae3c — __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5aae3c() -> ! {
    todo!("0x5aae3c __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5aae40 — __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5aae40() -> ! {
    todo!("0x5aae40 __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5aaee0 — __ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5aaee0() -> ! {
    todo!("0x5aaee0 __ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5aaee8 — __ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5aaee8() -> ! {
    todo!("0x5aaee8 __ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5aaf8c — __ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5aaf8c() -> ! {
    todo!("0x5aaf8c __ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5aaf94 — __ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5aaf94() -> ! {
    todo!("0x5aaf94 __ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5ab038 — __ZN3RBX15ManualGlueJointD1Ev
// type: void __fastcall(RBX::ManualGlueJoint *__hidden this)
#[doc(alias = "RBX::ManualGlueJoint::~ManualGlueJoint()")]
// was: __ZN3RBX15ManualGlueJointD1Ev
pub fn stub_5ab038() -> ! {
    todo!("0x5ab038 RBX::ManualGlueJoint::~ManualGlueJoint()")
}

// 0x5ab03c — __ZN3RBX15ManualGlueJointD0Ev
// type: void __fastcall(RBX::ManualGlueJoint *__hidden this)
#[doc(alias = "RBX::ManualGlueJoint::~ManualGlueJoint()")]
// was: __ZN3RBX15ManualGlueJointD0Ev
pub fn stub_5ab03c() -> ! {
    todo!("0x5ab03c RBX::ManualGlueJoint::~ManualGlueJoint()")
}

// 0x5ab0dc — __ZNK3RBX15ManualGlueJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::ManualGlueJoint *__hidden this)
#[doc(alias = "RBX::ManualGlueJoint::getJointType(void)const")]
// was: __ZNK3RBX15ManualGlueJoint12getJointTypeEv
pub fn stub_5ab0dc() -> ! {
    todo!("0x5ab0dc RBX::ManualGlueJoint::getJointType(void)const")
}

// 0x5ab0e0 — __ZNK3RBX9GlueJoint11isBreakableEv
// type: _DWORD __fastcall(RBX::GlueJoint *__hidden this)
#[doc(alias = "RBX::GlueJoint::isBreakable(void)const")]
// was: __ZNK3RBX9GlueJoint11isBreakableEv
pub fn stub_5ab0e0() -> ! {
    todo!("0x5ab0e0 RBX::GlueJoint::isBreakable(void)const")
}

// 0x5ab0e4 — __ZThn32_N3RBX15ManualGlueJointD1Ev
// type: void __fastcall(RBX::ManualGlueJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualGlueJoint::~ManualGlueJoint()")]
// was: __ZThn32_N3RBX15ManualGlueJointD1Ev
pub fn stub_5ab0e4() -> ! {
    todo!("0x5ab0e4 non-virtual thunk toRBX::ManualGlueJoint::~ManualGlueJoint()")
}

// 0x5ab0ec — __ZThn32_N3RBX15ManualGlueJointD0Ev
// type: void __fastcall(RBX::ManualGlueJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualGlueJoint::~ManualGlueJoint()")]
// was: __ZThn32_N3RBX15ManualGlueJointD0Ev
pub fn stub_5ab0ec() -> ! {
    todo!("0x5ab0ec non-virtual thunk toRBX::ManualGlueJoint::~ManualGlueJoint()")
}

// 0x5ab0f4 — __ZN3RBX15ManualWeldJointD1Ev
// type: void __fastcall(RBX::ManualWeldJoint *__hidden this)
#[doc(alias = "RBX::ManualWeldJoint::~ManualWeldJoint()")]
// was: __ZN3RBX15ManualWeldJointD1Ev
pub fn stub_5ab0f4() -> ! {
    todo!("0x5ab0f4 RBX::ManualWeldJoint::~ManualWeldJoint()")
}

// 0x5ab0f8 — __ZN3RBX15ManualWeldJointD0Ev
// type: void __fastcall(RBX::ManualWeldJoint *__hidden this)
#[doc(alias = "RBX::ManualWeldJoint::~ManualWeldJoint()")]
// was: __ZN3RBX15ManualWeldJointD0Ev
pub fn stub_5ab0f8() -> ! {
    todo!("0x5ab0f8 RBX::ManualWeldJoint::~ManualWeldJoint()")
}

// 0x5ab198 — __ZNK3RBX15ManualWeldJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::ManualWeldJoint *__hidden this)
#[doc(alias = "RBX::ManualWeldJoint::getJointType(void)const")]
// was: __ZNK3RBX15ManualWeldJoint12getJointTypeEv
pub fn stub_5ab198() -> ! {
    todo!("0x5ab198 RBX::ManualWeldJoint::getJointType(void)const")
}

// 0x5ab19c — __ZThn32_N3RBX15ManualWeldJointD1Ev
// type: void __fastcall(RBX::ManualWeldJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualWeldJoint::~ManualWeldJoint()")]
// was: __ZThn32_N3RBX15ManualWeldJointD1Ev
pub fn stub_5ab19c() -> ! {
    todo!("0x5ab19c non-virtual thunk toRBX::ManualWeldJoint::~ManualWeldJoint()")
}

// 0x5ab1a4 — __ZThn32_N3RBX15ManualWeldJointD0Ev
// type: void __fastcall(RBX::ManualWeldJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualWeldJoint::~ManualWeldJoint()")]
// was: __ZThn32_N3RBX15ManualWeldJointD0Ev
pub fn stub_5ab1a4() -> ! {
    todo!("0x5ab1a4 non-virtual thunk toRBX::ManualWeldJoint::~ManualWeldJoint()")
}

// 0x5ab1ac — __ZN3RBX9WeldJointD1Ev
// type: void __fastcall(RBX::WeldJoint *__hidden this)
#[doc(alias = "RBX::WeldJoint::~WeldJoint()")]
// was: __ZN3RBX9WeldJointD1Ev
pub fn stub_5ab1ac() -> ! {
    todo!("0x5ab1ac RBX::WeldJoint::~WeldJoint()")
}

// 0x5ab1b0 — __ZN3RBX9WeldJointD0Ev
// type: void __fastcall(RBX::WeldJoint *__hidden this)
#[doc(alias = "RBX::WeldJoint::~WeldJoint()")]
// was: __ZN3RBX9WeldJointD0Ev
pub fn stub_5ab1b0() -> ! {
    todo!("0x5ab1b0 RBX::WeldJoint::~WeldJoint()")
}

// 0x5ab250 — __ZNK3RBX9WeldJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::WeldJoint *__hidden this)
#[doc(alias = "RBX::WeldJoint::getJointType(void)const")]
// was: __ZNK3RBX9WeldJoint12getJointTypeEv
pub fn stub_5ab250() -> ! {
    todo!("0x5ab250 RBX::WeldJoint::getJointType(void)const")
}

// 0x5ab254 — __ZThn32_N3RBX9WeldJointD1Ev
// type: void __fastcall(RBX::WeldJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::WeldJoint::~WeldJoint()")]
// was: __ZThn32_N3RBX9WeldJointD1Ev
pub fn stub_5ab254() -> ! {
    todo!("0x5ab254 non-virtual thunk toRBX::WeldJoint::~WeldJoint()")
}

// 0x5ab25c — __ZThn32_N3RBX9WeldJointD0Ev
// type: void __fastcall(RBX::WeldJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::WeldJoint::~WeldJoint()")]
// was: __ZThn32_N3RBX9WeldJointD0Ev
pub fn stub_5ab25c() -> ! {
    todo!("0x5ab25c non-virtual thunk toRBX::WeldJoint::~WeldJoint()")
}

// 0x5ab264 — __ZN3RBX9SnapJointD1Ev
// type: void __fastcall(RBX::SnapJoint *__hidden this)
#[doc(alias = "RBX::SnapJoint::~SnapJoint()")]
// was: __ZN3RBX9SnapJointD1Ev
pub fn stub_5ab264() -> ! {
    todo!("0x5ab264 RBX::SnapJoint::~SnapJoint()")
}

// 0x5ab268 — __ZN3RBX9SnapJointD0Ev
// type: void __fastcall(RBX::SnapJoint *__hidden this)
#[doc(alias = "RBX::SnapJoint::~SnapJoint()")]
// was: __ZN3RBX9SnapJointD0Ev
pub fn stub_5ab268() -> ! {
    todo!("0x5ab268 RBX::SnapJoint::~SnapJoint()")
}

// 0x5ab308 — __ZNK3RBX9SnapJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::SnapJoint *__hidden this)
#[doc(alias = "RBX::SnapJoint::getJointType(void)const")]
// was: __ZNK3RBX9SnapJoint12getJointTypeEv
pub fn stub_5ab308() -> ! {
    todo!("0x5ab308 RBX::SnapJoint::getJointType(void)const")
}

// 0x5ab30c — __ZThn32_N3RBX9SnapJointD1Ev
// type: void __fastcall(RBX::SnapJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SnapJoint::~SnapJoint()")]
// was: __ZThn32_N3RBX9SnapJointD1Ev
pub fn stub_5ab30c() -> ! {
    todo!("0x5ab30c non-virtual thunk toRBX::SnapJoint::~SnapJoint()")
}

// 0x5ab314 — __ZThn32_N3RBX9SnapJointD0Ev
// type: void __fastcall(RBX::SnapJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SnapJoint::~SnapJoint()")]
// was: __ZThn32_N3RBX9SnapJointD0Ev
pub fn stub_5ab314() -> ! {
    todo!("0x5ab314 non-virtual thunk toRBX::SnapJoint::~SnapJoint()")
}

// 0x5ab31c — __GLOBAL__I_a_218
#[doc(alias = "global constructor keyed to_a_218")]
// was: __GLOBAL__I_a_218
pub fn stub_5ab31c() -> ! {
    todo!("0x5ab31c global constructor keyed to_a_218")
}

// 0x5abd48 — __ZN3RBX13JointsService24setJoinAfterMoveInstanceEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::JointsService::setJoinAfterMoveInstance(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX13JointsService24setJoinAfterMoveInstanceEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5abd48() -> ! {
    todo!("0x5abd48 RBX::JointsService::setJoinAfterMoveInstance(boost::shared_ptr<RBX::Instance>)")
}

// 0x5abe0c — __ZN3RBX13JointsService22setJoinAfterMoveTargetEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::JointsService::setJoinAfterMoveTarget(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX13JointsService22setJoinAfterMoveTargetEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5abe0c() -> ! {
    todo!("0x5abe0c RBX::JointsService::setJoinAfterMoveTarget(boost::shared_ptr<RBX::Instance>)")
}

// 0x5abed0 — __ZN3RBX13JointsService21showPermissibleJointsEv
// type: _DWORD __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::showPermissibleJoints(void)")]
// was: __ZN3RBX13JointsService21showPermissibleJointsEv
pub fn stub_5abed0() -> ! {
    todo!("0x5abed0 RBX::JointsService::showPermissibleJoints(void)")
}

// 0x5abf18 — __ZN3RBX13JointsService25createJoinAfterMoveJointsEv
// type: _DWORD __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::createJoinAfterMoveJoints(void)")]
// was: __ZN3RBX13JointsService25createJoinAfterMoveJointsEv
pub fn stub_5abf18() -> ! {
    todo!("0x5abf18 RBX::JointsService::createJoinAfterMoveJoints(void)")
}

// 0x5abf88 — __ZN3RBX13JointsService24clearJoinAfterMoveJointsEv
// type: _DWORD __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::clearJoinAfterMoveJoints(void)")]
// was: __ZN3RBX13JointsService24clearJoinAfterMoveJointsEv
pub fn stub_5abf88() -> ! {
    todo!("0x5abf88 RBX::JointsService::clearJoinAfterMoveJoints(void)")
}

// 0x5abfa0 — __ZN3RBX13JointsServiceC1Ev
// type: _DWORD __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::JointsService(void)")]
// was: __ZN3RBX13JointsServiceC1Ev
pub fn stub_5abfa0() -> ! {
    todo!("0x5abfa0 RBX::JointsService::JointsService(void)")
}

// 0x5abfa4 — __ZN3RBX13JointsServiceC2Ev
// type: _DWORD __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::JointsService(void)")]
// was: __ZN3RBX13JointsServiceC2Ev
pub fn stub_5abfa4() -> ! {
    todo!("0x5abfa4 RBX::JointsService::JointsService(void)")
}

// 0x5ac2e4 — __ZN3RBX13JointsService17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::JointsService *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::JointsService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX13JointsService17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_5ac2e4() -> ! {
    todo!("0x5ac2e4 RBX::JointsService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x5ac49c — __ZN3RBX13JointsService10onAutoJoinEPNS_5JointE
// type: _DWORD __fastcall(RBX::JointsService *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::JointsService::onAutoJoin(RBX::Joint *)")]
// was: __ZN3RBX13JointsService10onAutoJoinEPNS_5JointE
pub fn stub_5ac49c() -> ! {
    todo!("0x5ac49c RBX::JointsService::onAutoJoin(RBX::Joint *)")
}

// 0x5ac808 — __ZN3RBX13JointsService13onAutoDestroyEPNS_5JointE
// type: _DWORD __fastcall(RBX::JointsService *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::JointsService::onAutoDestroy(RBX::Joint *)")]
// was: __ZN3RBX13JointsService13onAutoDestroyEPNS_5JointE
pub fn stub_5ac808() -> ! {
    todo!("0x5ac808 RBX::JointsService::onAutoDestroy(RBX::Joint *)")
}

// 0x5ac8b8 — __ZN3RBX13JointsService20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::JointsService::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX13JointsService20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5ac8b8() -> ! {
    todo!("0x5ac8b8 RBX::JointsService::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)")
}

// 0x5ac8fc — __ZN3RBX13JointsService17onDescendantAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::JointsService *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::JointsService::onDescendantAdded(RBX::Instance *)")]
// was: __ZN3RBX13JointsService17onDescendantAddedEPNS_8InstanceE
pub fn stub_5ac8fc() -> ! {
    todo!("0x5ac8fc RBX::JointsService::onDescendantAdded(RBX::Instance *)")
}

// 0x5ac944 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
pub fn stub_5ac944() -> ! {
    todo!("0x5ac944 RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0x5aca50 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EED1Ev
pub fn stub_5aca50() -> ! {
    todo!("0x5aca50 RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::~BoundFuncDesc()")
}

// 0x5acaac — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Joint *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_5acaac() -> ! {
    todo!("0x5acaac rbx::signals::connection rbx::signals::signal<void ()(RBX::Joint *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>> const&)")
}

// 0x5acb20 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4SnapEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::Snap>(rbx_core::SharedPtr<RBX::Snap> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4SnapEEERS3_RKNS0_IT_EE
pub fn stub_5acb20() -> ! {
    todo!("0x5acb20 boost::shared_ptr<RBX::JointInstance>& boost::shared_ptr<RBX::JointInstance>::operator=<RBX::Snap>(boost::shared_ptr<RBX::Snap> const&)")
}

// 0x5acb54 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap,RBX::Joint *>(RBX::Joint *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
pub fn stub_5acb54() -> ! {
    todo!("0x5acb54 boost::shared_ptr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap,RBX::Joint *>(RBX::Joint *)")
}

// 0x5acc08 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4WeldEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::Weld>(rbx_core::SharedPtr<RBX::Weld> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4WeldEEERS3_RKNS0_IT_EE
pub fn stub_5acc08() -> ! {
    todo!("0x5acc08 boost::shared_ptr<RBX::JointInstance>& boost::shared_ptr<RBX::JointInstance>::operator=<RBX::Weld>(boost::shared_ptr<RBX::Weld> const&)")
}

// 0x5acc3c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4WeldEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Weld> RBX::Creatable<RBX::Instance>::create<RBX::Weld,RBX::Joint *>(RBX::Joint *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_4WeldEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
pub fn stub_5acc3c() -> ! {
    todo!("0x5acc3c boost::shared_ptr<RBX::Weld> RBX::Creatable<RBX::Instance>::create<RBX::Weld,RBX::Joint *>(RBX::Joint *)")
}

// 0x5accf0 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4GlueEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::Glue>(rbx_core::SharedPtr<RBX::Glue> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4GlueEEERS3_RKNS0_IT_EE
pub fn stub_5accf0() -> ! {
    todo!("0x5accf0 boost::shared_ptr<RBX::JointInstance>& boost::shared_ptr<RBX::JointInstance>::operator=<RBX::Glue>(boost::shared_ptr<RBX::Glue> const&)")
}

// 0x5acd24 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue,RBX::Joint *>(RBX::Joint *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
pub fn stub_5acd24() -> ! {
    todo!("0x5acd24 boost::shared_ptr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue,RBX::Joint *>(RBX::Joint *)")
}

// 0x5acdd8 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_6RotateEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::Rotate>(rbx_core::SharedPtr<RBX::Rotate> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_6RotateEEERS3_RKNS0_IT_EE
pub fn stub_5acdd8() -> ! {
    todo!("0x5acdd8 boost::shared_ptr<RBX::JointInstance>& boost::shared_ptr<RBX::JointInstance>::operator=<RBX::Rotate>(boost::shared_ptr<RBX::Rotate> const&)")
}

// 0x5ace0c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate,RBX::Joint *>(RBX::Joint *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
pub fn stub_5ace0c() -> ! {
    todo!("0x5ace0c boost::shared_ptr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate,RBX::Joint *>(RBX::Joint *)")
}

// 0x5acec0 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_7RotatePEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::RotateP>(rbx_core::SharedPtr<RBX::RotateP> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_7RotatePEEERS3_RKNS0_IT_EE
pub fn stub_5acec0() -> ! {
    todo!("0x5acec0 boost::shared_ptr<RBX::JointInstance>& boost::shared_ptr<RBX::JointInstance>::operator=<RBX::RotateP>(boost::shared_ptr<RBX::RotateP> const&)")
}

// 0x5acef4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP,RBX::Joint *>(RBX::Joint *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
pub fn stub_5acef4() -> ! {
    todo!("0x5acef4 boost::shared_ptr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP,RBX::Joint *>(RBX::Joint *)")
}

// 0x5acfa8 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_7RotateVEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::RotateV>(rbx_core::SharedPtr<RBX::RotateV> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_7RotateVEEERS3_RKNS0_IT_EE
pub fn stub_5acfa8() -> ! {
    todo!("0x5acfa8 boost::shared_ptr<RBX::JointInstance>& boost::shared_ptr<RBX::JointInstance>::operator=<RBX::RotateV>(boost::shared_ptr<RBX::RotateV> const&)")
}

// 0x5acfdc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV,RBX::Joint *>(RBX::Joint *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
pub fn stub_5acfdc() -> ! {
    todo!("0x5acfdc boost::shared_ptr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV,RBX::Joint *>(RBX::Joint *)")
}

// 0x5ad090 — __ZN5boost10shared_ptrIN3RBX10PVInstanceEEaSERKS3_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::PVInstance>::operator=(rbx_core::SharedPtr<RBX::PVInstance> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX10PVInstanceEEaSERKS3_
pub fn stub_5ad090() -> ! {
    todo!("0x5ad090 boost::shared_ptr<RBX::PVInstance>::operator=(boost::shared_ptr<RBX::PVInstance> const&)")
}

// 0x5ad0c8 — __ZN3RBX13JointsServiceD1Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::~JointsService()")]
// was: __ZN3RBX13JointsServiceD1Ev
pub fn stub_5ad0c8() -> ! {
    todo!("0x5ad0c8 RBX::JointsService::~JointsService()")
}

// 0x5ad0cc — __ZN3RBX13JointsServiceD0Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::~JointsService()")]
// was: __ZN3RBX13JointsServiceD0Ev
pub fn stub_5ad0cc() -> ! {
    todo!("0x5ad0cc RBX::JointsService::~JointsService()")
}

// 0x5ad16c — __ZNK3RBX13JointsService11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::JointsService *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::JointsService::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX13JointsService11askAddChildEPKNS_8InstanceE
pub fn stub_5ad16c() -> ! {
    todo!("0x5ad16c RBX::JointsService::askAddChild(RBX::Instance const*)const")
}

// 0x5ad1a8 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv
pub fn stub_5ad1a8() -> ! {
    todo!("0x5ad1a8 __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv")
}

// 0x5ad1d0 — __ZThn32_N3RBX13JointsServiceD1Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::JointsService::~JointsService()")]
// was: __ZThn32_N3RBX13JointsServiceD1Ev
pub fn stub_5ad1d0() -> ! {
    todo!("0x5ad1d0 non-virtual thunk toRBX::JointsService::~JointsService()")
}

// 0x5ad1d8 — __ZThn32_N3RBX13JointsServiceD0Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::JointsService::~JointsService()")]
// was: __ZThn32_N3RBX13JointsServiceD0Ev
pub fn stub_5ad1d8() -> ! {
    todo!("0x5ad1d8 non-virtual thunk toRBX::JointsService::~JointsService()")
}

// 0x5ad27c — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv
pub fn stub_5ad27c() -> ! {
    todo!("0x5ad27c __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv")
}

// 0x5ad2a4 — __ZThn36_N3RBX13JointsServiceD1Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::JointsService::~JointsService()")]
// was: __ZThn36_N3RBX13JointsServiceD1Ev
pub fn stub_5ad2a4() -> ! {
    todo!("0x5ad2a4 non-virtual thunk toRBX::JointsService::~JointsService()")
}

// 0x5ad2ac — __ZThn36_N3RBX13JointsServiceD0Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::JointsService::~JointsService()")]
// was: __ZThn36_N3RBX13JointsServiceD0Ev
pub fn stub_5ad2ac() -> ! {
    todo!("0x5ad2ac non-virtual thunk toRBX::JointsService::~JointsService()")
}

// 0x5ad350 — __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD1Ev
pub fn stub_5ad350() -> ! {
    todo!("0x5ad350 __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD1Ev")
}

// 0x5ad354 — __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD1Ev
pub fn stub_5ad354() -> ! {
    todo!("0x5ad354 __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD1Ev")
}

// 0x5ad358 — __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD1Ev
pub fn stub_5ad358() -> ! {
    todo!("0x5ad358 __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD1Ev")
}

// 0x5ad35c — __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD1Ev
pub fn stub_5ad35c() -> ! {
    todo!("0x5ad35c __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD1Ev")
}

// 0x5ad360 — __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD1Ev
pub fn stub_5ad360() -> ! {
    todo!("0x5ad360 __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD1Ev")
}

// 0x5ad364 — __ZN5boost20dynamic_pointer_castIN3RBX10PVInstanceENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::PVInstance> boost::dynamic_pointer_cast<RBX::PVInstance,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN5boost20dynamic_pointer_castIN3RBX10PVInstanceENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
pub fn stub_5ad364() -> ! {
    todo!("0x5ad364 boost::shared_ptr<RBX::PVInstance> boost::dynamic_pointer_cast<RBX::PVInstance,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)")
}

// 0x5ad3ac — __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD2Ev
pub fn stub_5ad3ac() -> ! {
    todo!("0x5ad3ac __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD2Ev")
}

// 0x5ad448 — __ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_5ad448() -> ! {
    todo!("0x5ad448 __ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x5ad4b4 — __ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator6createEv
pub fn stub_5ad4b4() -> ! {
    todo!("0x5ad4b4 __ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator6createEv")
}

// 0x5ad5f8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEEEN5boost10shared_ptrIT_EEv
pub fn stub_5ad5f8() -> ! {
    todo!("0x5ad5f8 boost::shared_ptr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV>(void)")
}

// 0x5ad6a8 — __ZN5boost10shared_ptrIN3RBX7RotateVEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateV>::shared_ptr<RBX::RotateV,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX7RotateVEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_5ad6a8() -> ! {
    todo!("0x5ad6a8 boost::shared_ptr<RBX::RotateV>::shared_ptr<RBX::RotateV,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x5ad770 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotateVES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RotateV,RBX::RotateV>(rbx_core::SharedPtr<RBX::RotateV> const*,RBX::RotateV *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotateVES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_5ad770() -> ! {
    todo!("0x5ad770 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RotateV,RBX::RotateV>(boost::shared_ptr<RBX::RotateV> const*,RBX::RotateV *)const")
}

// 0x5ad858 — __ZN5boost6detail12shared_countC2IPN3RBX7RotateVENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX7RotateVENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_5ad858() -> ! {
    todo!("0x5ad858 boost::detail::shared_count::shared_count<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x5ad960 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_5ad960() -> ! {
    todo!("0x5ad960 boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5ad964 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5ad964() -> ! {
    todo!("0x5ad964 boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5ad968 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5ad968() -> ! {
    todo!("0x5ad968 boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x5ad988 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5ad988() -> ! {
    todo!("0x5ad988 boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x5ad9a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5ad9a0() -> ! {
    todo!("0x5ad9a0 boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x5ad9a4 — __ZN3RBX4Name7declareILZNS_8sRotateVEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_8sRotateVEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_8sRotateVEEEERKS0_v
pub fn stub_5ad9a4() -> ! {
    todo!("0x5ad9a4 __ZN3RBX4Name7declareILZNS_8sRotateVEEEERKS0_v")
}

// 0x5ad9e8 — __ZN3RBX4Name13callDoDeclareILZNS_8sRotateVEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sRotateVEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_8sRotateVEEEEvv
pub fn stub_5ad9e8() -> ! {
    todo!("0x5ad9e8 __ZN3RBX4Name13callDoDeclareILZNS_8sRotateVEEEEvv")
}

// 0x5ad9ec — __ZN3RBX4Name9doDeclareILZNS_8sRotateVEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sRotateVEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_8sRotateVEEEERKS0_v
pub fn stub_5ad9ec() -> ! {
    todo!("0x5ad9ec __ZN3RBX4Name9doDeclareILZNS_8sRotateVEEEERKS0_v")
}

// 0x5adad0 — __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorC2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorC2Ev
pub fn stub_5adad0() -> ! {
    todo!("0x5adad0 __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorC2Ev")
}

// 0x5adcf8 — __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD2Ev
pub fn stub_5adcf8() -> ! {
    todo!("0x5adcf8 __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD2Ev")
}

// 0x5add94 — __ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_5add94() -> ! {
    todo!("0x5add94 __ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x5ade00 — __ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator6createEv
pub fn stub_5ade00() -> ! {
    todo!("0x5ade00 __ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator6createEv")
}

// 0x5adf44 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEEEN5boost10shared_ptrIT_EEv
pub fn stub_5adf44() -> ! {
    todo!("0x5adf44 boost::shared_ptr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP>(void)")
}

// 0x5adff4 — __ZN5boost10shared_ptrIN3RBX7RotatePEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateP>::shared_ptr<RBX::RotateP,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX7RotatePEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_5adff4() -> ! {
    todo!("0x5adff4 boost::shared_ptr<RBX::RotateP>::shared_ptr<RBX::RotateP,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x5ae0bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotatePES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RotateP,RBX::RotateP>(rbx_core::SharedPtr<RBX::RotateP> const*,RBX::RotateP *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotatePES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_5ae0bc() -> ! {
    todo!("0x5ae0bc void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RotateP,RBX::RotateP>(boost::shared_ptr<RBX::RotateP> const*,RBX::RotateP *)const")
}

// 0x5ae1a4 — __ZN5boost6detail12shared_countC2IPN3RBX7RotatePENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX7RotatePENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_5ae1a4() -> ! {
    todo!("0x5ae1a4 boost::detail::shared_count::shared_count<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter)")
}
