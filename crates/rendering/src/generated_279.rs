//! rendering shard 279 — 150 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15112/15112 complete, 30320->30470 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 30320 before -> 30470 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x3bdce4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEEC2IiMS2_FvS3_EEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::EnumPropDescriptor<int,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>(char const*,char const*,int,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEEC2IiMS2_FvS3_EEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_3bdce4() -> ! {
    todo!("0x3bdce4 RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::EnumPropDescriptor<int,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>(char const*,char const*,int,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}


// 0x3bde90 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE7SetImplIMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE7SetImplIMS2_FvS3_EE10isReadOnlyEv
pub fn stub_3bde90() -> ! {
    todo!("0x3bde90 RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isReadOnly(void)const")
}


// 0x3bde94 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE7SetImplIMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE7SetImplIMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_3bde94() -> ! {
    todo!("0x3bde94 RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isWriteOnly(void)const")
}


// 0x3bde98 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE7SetImplIMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE7SetImplIMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_3bde98() -> ! {
    todo!("0x3bde98 RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::getValue(RBX::Reflection::DescribedBase const*)const")
}


// 0x3bdfb8 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE7SetImplIMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::setValue(RBX::Reflection::DescribedBase *,RBX::BasicPartInstance::LegacyPartType const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE7SetImplIMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_3bdfb8() -> ! {
    todo!("0x3bdfb8 RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::setValue(RBX::Reflection::DescribedBase *,RBX::BasicPartInstance::LegacyPartType const&)const")
}


// 0x3bdfdc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE11setIntValueEPNS0_13DescribedBaseEi
pub fn stub_3bdfdc() -> ! {
    todo!("0x3bdfdc RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}


// 0x3be01c — __ZThn32_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3be01c() -> ! {
    todo!("0x3be01c __ZThn32_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x3be030 — __ZThn32_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3be030() -> ! {
    todo!("0x3be030 __ZThn32_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x3be0e4 — __ZThn36_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3be0e4() -> ! {
    todo!("0x3be0e4 __ZThn36_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x3be0f8 — __ZThn36_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3be0f8() -> ! {
    todo!("0x3be0f8 __ZThn36_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x3be1ac — __ZN3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEC2Ev
// type: RBX::PartInstance *__fastcall(RBX::PartInstance *, int *)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEC2Ev")]
// was: __ZN3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEC2Ev
pub fn stub_3be1ac() -> ! {
    todo!("0x3be1ac __ZN3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEC2Ev")
}


// 0x3be228 — __ZThn32_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3be228() -> ! {
    todo!("0x3be228 __ZThn32_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x3be23c — __ZThn32_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3be23c() -> ! {
    todo!("0x3be23c __ZThn32_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x3be2f0 — __ZThn36_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3be2f0() -> ! {
    todo!("0x3be2f0 __ZThn36_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x3be304 — __ZThn36_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3be304() -> ! {
    todo!("0x3be304 __ZThn36_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x3be3b8 — __ZThn32_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev")]
// was: __ZThn32_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev
pub fn stub_3be3b8() -> ! {
    todo!("0x3be3b8 __ZThn32_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev")
}


// 0x3be3cc — __ZThn32_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev")]
// was: __ZThn32_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev
pub fn stub_3be3cc() -> ! {
    todo!("0x3be3cc __ZThn32_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev")
}


// 0x3be47c — __ZThn36_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev")]
// was: __ZThn36_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev
pub fn stub_3be47c() -> ! {
    todo!("0x3be47c __ZThn36_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev")
}


// 0x3be490 — __ZThn36_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev")]
// was: __ZThn36_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev
pub fn stub_3be490() -> ! {
    todo!("0x3be490 __ZThn36_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev")
}


// 0x3be540 — __ZN3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::EnumPropDescriptor<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>(char const*,char const*,RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_3be540() -> ! {
    todo!("0x3be540 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::EnumPropDescriptor<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>(char const*,char const*,RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}


// 0x3be6f4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEED0Ev
pub fn stub_3be6f4() -> ! {
    todo!("0x3be6f4 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::~EnumPropDescriptor()")
}


// 0x3be720 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10isReadOnlyEv
pub fn stub_3be720() -> ! {
    todo!("0x3be720 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::isReadOnly(void)const")
}


// 0x3be730 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE11isWriteOnlyEv
pub fn stub_3be730() -> ! {
    todo!("0x3be730 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::isWriteOnly(void)const")
}


// 0x3be740 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE11equalValuesEPKNS0_13DescribedBaseES8_
pub fn stub_3be740() -> ! {
    todo!("0x3be740 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}


// 0x3be768 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
pub fn stub_3be768() -> ! {
    todo!("0x3be768 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}


// 0x3be78c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_3be78c() -> ! {
    todo!("0x3be78c RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}


// 0x3be8d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE9copyValueEPKNS0_13DescribedBaseEPS6_
pub fn stub_3be8d8() -> ! {
    todo!("0x3be8d8 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}


// 0x3be8fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE14hasStringValueEv
pub fn stub_3be8fc() -> ! {
    todo!("0x3be8fc RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::hasStringValue(void)const")
}


// 0x3be900 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE14getStringValueEPKNS0_13DescribedBaseE
pub fn stub_3be900() -> ! {
    todo!("0x3be900 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}


// 0x3be924 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_3be924() -> ! {
    todo!("0x3be924 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}


// 0x3be964 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_3be964() -> ! {
    todo!("0x3be964 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}


// 0x3be984 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE13getIndexValueEPKNS0_13DescribedBaseE
pub fn stub_3be984() -> ! {
    todo!("0x3be984 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}


// 0x3be9a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE13setIndexValueEPNS0_13DescribedBaseEm
pub fn stub_3be9a0() -> ! {
    todo!("0x3be9a0 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}


// 0x3be9d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE12getEnumValueEPKNS0_13DescribedBaseE
pub fn stub_3be9d4() -> ! {
    todo!("0x3be9d4 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}


// 0x3be9dc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE12setEnumValueEPNS0_13DescribedBaseEi
pub fn stub_3be9dc() -> ! {
    todo!("0x3be9dc RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}


// 0x3bea28 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE11getEnumItemEPKNS0_13DescribedBaseE
pub fn stub_3bea28() -> ! {
    todo!("0x3bea28 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}


// 0x3bea48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
pub fn stub_3bea48() -> ! {
    todo!("0x3bea48 RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}


// 0x3bea7c — __ZNK3RBX10Reflection8EnumDescINS_12PartInstance10FormFactorEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor>::convertToIndex(RBX::PartInstance::FormFactor)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_12PartInstance10FormFactorEE14convertToIndexES3_
pub fn stub_3bea7c() -> ! {
    todo!("0x3bea7c RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor>::convertToIndex(RBX::PartInstance::FormFactor)const")
}


// 0x3beaec — __ZNK3RBX10Reflection14PropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::GetSetImpl<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
pub fn stub_3beaec() -> ! {
    todo!("0x3beaec RBX::Reflection::PropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::GetSetImpl<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>::isReadOnly(void)const")
}


// 0x3beaf0 — __ZNK3RBX10Reflection14PropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::GetSetImpl<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
pub fn stub_3beaf0() -> ! {
    todo!("0x3beaf0 RBX::Reflection::PropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::GetSetImpl<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>::isWriteOnly(void)const")
}


// 0x3beaf4 — __ZNK3RBX10Reflection14PropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::GetSetImpl<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_3beaf4() -> ! {
    todo!("0x3beaf4 RBX::Reflection::PropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::GetSetImpl<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>::getValue(RBX::Reflection::DescribedBase const*)const")
}


// 0x3beb14 — __ZNK3RBX10Reflection14PropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::GetSetImpl<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance::FormFactor const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
pub fn stub_3beb14() -> ! {
    todo!("0x3beb14 RBX::Reflection::PropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::GetSetImpl<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance::FormFactor const&)const")
}


// 0x3beb38 — __GLOBAL__I_a_162
#[doc(alias = "__GLOBAL__I_a_162")]
// was: __GLOBAL__I_a_162
pub fn stub_3beb38() -> ! {
    todo!("0x3beb38 `global constructor keyed to'_a_162")
}


// 0x3bf18c — __ZN3RBX10Reflection14PropDescriptorINS_9BevelMeshEfED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9BevelMeshEfED1Ev
pub fn stub_3bf18c() -> ! {
    todo!("0x3bf18c RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::~PropDescriptor()")
}


// 0x3bf254 — __ZNK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv
pub fn stub_3bf254() -> ! {
    todo!("0x3bf254 __ZNK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv")
}


// 0x3bf328 — __ZThn32_NK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv
pub fn stub_3bf328() -> ! {
    todo!("0x3bf328 __ZThn32_NK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv")
}


// 0x3bf3fc — __ZN3RBX4Name13callDoDeclareILZNS_10sBevelMeshEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sBevelMeshEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_10sBevelMeshEEEEvv
pub fn stub_3bf3fc() -> ! {
    todo!("0x3bf3fc __ZN3RBX4Name13callDoDeclareILZNS_10sBevelMeshEEEEvv")
}


// 0x3bf400 — __ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v
pub fn stub_3bf400() -> ! {
    todo!("0x3bf400 __ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v")
}


// 0x3bf4e0 — __ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3bf4e0() -> ! {
    todo!("0x3bf4e0 __ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x3bf4e4 — __ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3bf4e4() -> ! {
    todo!("0x3bf4e4 __ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x3bf584 — __ZThn32_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3bf584() -> ! {
    todo!("0x3bf584 __ZThn32_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x3bf58c — __ZThn32_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3bf58c() -> ! {
    todo!("0x3bf58c __ZThn32_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x3bf630 — __ZThn36_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3bf630() -> ! {
    todo!("0x3bf630 __ZThn36_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x3bf638 — __ZThn36_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3bf638() -> ! {
    todo!("0x3bf638 __ZThn36_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x3bf6dc — __ZN3RBX10Reflection14PropDescriptorINS_9BevelMeshEfEC2IMS2_KFKfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::PropDescriptor<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>(char const*,char const*,float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9BevelMeshEfEC2IMS2_KFKfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_3bf6dc() -> ! {
    todo!("0x3bf6dc RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::PropDescriptor<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>(char const*,char const*,float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}


// 0x3bf7f0 — __ZN3RBX10Reflection14PropDescriptorINS_9BevelMeshEfED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9BevelMeshEfED0Ev
pub fn stub_3bf7f0() -> ! {
    todo!("0x3bf7f0 RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::~PropDescriptor()")
}


// 0x3bf81c — __ZNK3RBX10Reflection14PropDescriptorINS_9BevelMeshEfE10GetSetImplIMS2_KFKfvEMS2_FvfEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::GetSetImpl<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9BevelMeshEfE10GetSetImplIMS2_KFKfvEMS2_FvfEE10isReadOnlyEv
pub fn stub_3bf81c() -> ! {
    todo!("0x3bf81c RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::GetSetImpl<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>::isReadOnly(void)const")
}


// 0x3bf820 — __ZNK3RBX10Reflection14PropDescriptorINS_9BevelMeshEfE10GetSetImplIMS2_KFKfvEMS2_FvfEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::GetSetImpl<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9BevelMeshEfE10GetSetImplIMS2_KFKfvEMS2_FvfEE11isWriteOnlyEv
pub fn stub_3bf820() -> ! {
    todo!("0x3bf820 RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::GetSetImpl<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>::isWriteOnly(void)const")
}


// 0x3bf824 — __ZNK3RBX10Reflection14PropDescriptorINS_9BevelMeshEfE10GetSetImplIMS2_KFKfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::GetSetImpl<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9BevelMeshEfE10GetSetImplIMS2_KFKfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_3bf824() -> ! {
    todo!("0x3bf824 RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::GetSetImpl<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")
}


// 0x3bf844 — __ZNK3RBX10Reflection14PropDescriptorINS_9BevelMeshEfE10GetSetImplIMS2_KFKfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERS5_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::GetSetImpl<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9BevelMeshEfE10GetSetImplIMS2_KFKfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERS5_
pub fn stub_3bf844() -> ! {
    todo!("0x3bf844 RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::GetSetImpl<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}


// 0x3bf868 — __GLOBAL__I_a_163
#[doc(alias = "__GLOBAL__I_a_163")]
// was: __GLOBAL__I_a_163
pub fn stub_3bf868() -> ! {
    todo!("0x3bf868 `global constructor keyed to'_a_163")
}


// 0x3bffc0 — __ZN3RBX12BillboardGui19setPlayerToHideFromEPNS_8InstanceE
// type: void __fastcall(RBX::BillboardGui *this, RBX::Instance *, int, int)
#[doc(alias = "RBX::BillboardGui::setPlayerToHideFrom(RBX::Instance *)")]
// was: __ZN3RBX12BillboardGui19setPlayerToHideFromEPNS_8InstanceE
pub fn stub_3bffc0() -> ! {
    todo!("0x3bffc0 RBX::BillboardGui::setPlayerToHideFrom(RBX::Instance *)")
}


// 0x3c0434 — __ZNK3RBX12BillboardGui12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::BillboardGui *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::BillboardGui::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX12BillboardGui12askSetParentEPKNS_8InstanceE
pub fn stub_3c0434() -> ! {
    todo!("0x3c0434 RBX::BillboardGui::askSetParent(RBX::Instance const*)const")
}


// 0x3c057c — __ZNK3RBX12BillboardGui7getPartEv
// type: void __fastcall(RBX::BillboardGui *this, int)
#[doc(alias = "RBX::BillboardGui::getPart(void)const")]
// was: __ZNK3RBX12BillboardGui7getPartEv
pub fn stub_3c057c() -> ! {
    todo!("0x3c057c RBX::BillboardGui::getPart(void)const")
}


// 0x3c0f7c — __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEED1Ev
pub fn stub_3c0f7c() -> ! {
    todo!("0x3c0f7c RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::~RefPropDescriptor()")
}


// 0x3c0ff0 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EED1Ev
pub fn stub_3c0ff0() -> ! {
    todo!("0x3c0ff0 RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::~PropDescriptor()")
}


// 0x3c101c — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbED1Ev
pub fn stub_3c101c() -> ! {
    todo!("0x3c101c RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::~PropDescriptor()")
}


// 0x3c1eec — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::PropDescriptor<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>(char const*,char const*,bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_3c1eec() -> ! {
    todo!("0x3c1eec RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::PropDescriptor<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>(char const*,char const*,bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}


// 0x3c2000 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbED0Ev
pub fn stub_3c2000() -> ! {
    todo!("0x3c2000 RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::~PropDescriptor()")
}


// 0x3c202c — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::GetSetImpl<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
pub fn stub_3c202c() -> ! {
    todo!("0x3c202c RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::GetSetImpl<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>::isReadOnly(void)const")
}


// 0x3c2030 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::GetSetImpl<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
pub fn stub_3c2030() -> ! {
    todo!("0x3c2030 RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::GetSetImpl<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>::isWriteOnly(void)const")
}


// 0x3c2034 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::GetSetImpl<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_3c2034() -> ! {
    todo!("0x3c2034 RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::GetSetImpl<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}


// 0x3c2058 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::GetSetImpl<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
pub fn stub_3c2058() -> ! {
    todo!("0x3c2058 RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::GetSetImpl<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}


// 0x3c207c — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::PropDescriptor<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>(char const*,char const*,RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_3c207c() -> ! {
    todo!("0x3c207c RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::PropDescriptor<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>(char const*,char const*,RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}


// 0x3c2190 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EED0Ev
pub fn stub_3c2190() -> ! {
    todo!("0x3c2190 RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::~PropDescriptor()")
}


// 0x3c21bc — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_3c21bc() -> ! {
    todo!("0x3c21bc RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>::isReadOnly(void)const")
}


// 0x3c21c0 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_3c21c0() -> ! {
    todo!("0x3c21c0 RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>::isWriteOnly(void)const")
}


// 0x3c21c4 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_3c21c4() -> ! {
    todo!("0x3c21c4 RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>::getValue(RBX::Reflection::DescribedBase const*)const")
}


// 0x3c21ec — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>::setValue(RBX::Reflection::DescribedBase *,RBX::UDim2 const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_3c21ec() -> ! {
    todo!("0x3c21ec RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::UDim2)>::setValue(RBX::Reflection::DescribedBase *,RBX::UDim2 const&)const")
}


// 0x3c2560 — __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance*)>(char const*,char const*,RBX::Instance* (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_3c2560() -> ! {
    todo!("0x3c2560 RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance*)>(char const*,char const*,RBX::Instance* (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}


// 0x3c2604 — __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEED0Ev
pub fn stub_3c2604() -> ! {
    todo!("0x3c2604 RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::~RefPropDescriptor()")
}


// 0x3c2634 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10isReadOnlyEv
pub fn stub_3c2634() -> ! {
    todo!("0x3c2634 RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::isReadOnly(void)const")
}


// 0x3c2644 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11isWriteOnlyEv
pub fn stub_3c2644() -> ! {
    todo!("0x3c2644 RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::isWriteOnly(void)const")
}


// 0x3c2654 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
pub fn stub_3c2654() -> ! {
    todo!("0x3c2654 RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}


// 0x3c267c — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void __fastcall(int, int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
pub fn stub_3c267c() -> ! {
    todo!("0x3c267c RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}


// 0x3c2794 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_3c2794() -> ! {
    todo!("0x3c2794 RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}


// 0x3c285c — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_3c285c() -> ! {
    todo!("0x3c285c RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}


// 0x3c2880 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_3c2880() -> ! {
    todo!("0x3c2880 RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}


// 0x3c2954 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_3c2954() -> ! {
    todo!("0x3c2954 RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}


// 0x3c2978 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
pub fn stub_3c2978() -> ! {
    todo!("0x3c2978 RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")
}


// 0x3c298c — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
pub fn stub_3c298c() -> ! {
    todo!("0x3c298c RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")
}


// 0x3c2a08 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
pub fn stub_3c2a08() -> ! {
    todo!("0x3c2a08 RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")
}


// 0x3c2a28 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: void __fastcall(int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
pub fn stub_3c2a28() -> ! {
    todo!("0x3c2a28 RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}


// 0x3c2b08 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int)
#[doc(alias = "__ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
// was: __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
pub fn stub_3c2b08() -> ! {
    todo!("0x3c2b08 `non-virtual thunk to'RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}


// 0x3c2b10 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
pub fn stub_3c2b10() -> ! {
    todo!("0x3c2b10 RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::isReadOnly(void)const")
}


// 0x3c2b14 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
pub fn stub_3c2b14() -> ! {
    todo!("0x3c2b14 RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::isWriteOnly(void)const")
}


// 0x3c2b18 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_3c2b18() -> ! {
    todo!("0x3c2b18 RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::getValue(RBX::Reflection::DescribedBase const*)const")
}


// 0x3c2b38 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
pub fn stub_3c2b38() -> ! {
    todo!("0x3c2b38 RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")
}


// 0x3c2d40 — __GLOBAL__I_a_164
#[doc(alias = "__GLOBAL__I_a_164")]
// was: __GLOBAL__I_a_164
pub fn stub_3c2d40() -> ! {
    todo!("0x3c2d40 `global constructor keyed to'_a_164")
}


// 0x3c333c — __GLOBAL__I_a_165
#[doc(alias = "__GLOBAL__I_a_165")]
// was: __GLOBAL__I_a_165
pub fn stub_3c333c() -> ! {
    todo!("0x3c333c `global constructor keyed to'_a_165")
}


// 0x3c39ac — __ZNK3RBX6Camera33getCameraSubjectInstanceDangerousEv
// type: int __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::getCameraSubjectInstanceDangerous(void)const")]
// was: __ZNK3RBX6Camera33getCameraSubjectInstanceDangerousEv
pub fn stub_3c39ac() -> ! {
    todo!("0x3c39ac RBX::Camera::getCameraSubjectInstanceDangerous(void)const")
}


// 0x3c39b4 — __ZN3RBX6Camera16setCameraSubjectEPNS_8InstanceE
// type: void __fastcall(shared_count *this, RBX::Instance *)
#[doc(alias = "RBX::Camera::setCameraSubject(RBX::Instance *)")]
// was: __ZN3RBX6Camera16setCameraSubjectEPNS_8InstanceE
pub fn stub_3c39b4() -> ! {
    todo!("0x3c39b4 RBX::Camera::setCameraSubject(RBX::Instance *)")
}


// 0x3ca77c — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToItem(RBX::Camera::CameraPanMode const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE13convertToItemERKS3_
pub fn stub_3ca77c() -> ! {
    todo!("0x3ca77c RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToItem(RBX::Camera::CameraPanMode const&)const")
}


// 0x3ca848 — __ZN3rbx8any_castIRKN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Camera::CameraPanMode const& rbx::any_cast<RBX::Camera::CameraPanMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_3ca848() -> ! {
    todo!("0x3ca848 RBX::Camera::CameraPanMode const& rbx::any_cast<RBX::Camera::CameraPanMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}


// 0x3ca938 — __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToValue(RBX::Name const&,RBX::Camera::CameraPanMode&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEE14convertToValueERKNS_4NameERS3_
pub fn stub_3ca938() -> ! {
    todo!("0x3ca938 RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::convertToValue(RBX::Name const&,RBX::Camera::CameraPanMode&)const")
}


// 0x3ca9b4 — __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_6Camera13CameraPanModeEED2Ev
pub fn stub_3ca9b4() -> ! {
    todo!("0x3ca9b4 RBX::Reflection::EnumDesc<RBX::Camera::CameraPanMode>::~EnumDesc()")
}


// 0x3cab88 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToString(RBX::Camera::CameraMode const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE15convertToStringERKS3_
pub fn stub_3cab88() -> ! {
    todo!("0x3cab88 RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToString(RBX::Camera::CameraMode const&)const")
}


// 0x3cad28 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraMode>(RBX::Camera::CameraMode const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraModeEEERS3_RKT_
pub fn stub_3cad28() -> ! {
    todo!("0x3cad28 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraMode>(RBX::Camera::CameraMode const&)")
}


// 0x3cad78 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraMode>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE9singletonEv
pub fn stub_3cad78() -> ! {
    todo!("0x3cad78 rbx::implementation::typed_holder<RBX::Camera::CameraMode>::singleton(void)")
}


// 0x3cade4 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraMode>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE14construct_funcEPKcPc
pub fn stub_3cade4() -> ! {
    todo!("0x3cade4 rbx::implementation::typed_holder<RBX::Camera::CameraMode>::construct_func(char const*,char *)")
}


// 0x3cadf0 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraMode>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE13destruct_funcEPc
pub fn stub_3cadf0() -> ! {
    todo!("0x3cadf0 rbx::implementation::typed_holder<RBX::Camera::CameraMode>::destruct_func(char *)")
}


// 0x3cadf4 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToItem(RBX::Camera::CameraMode const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE13convertToItemERKS3_
pub fn stub_3cadf4() -> ! {
    todo!("0x3cadf4 RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToItem(RBX::Camera::CameraMode const&)const")
}


// 0x3caec0 — __ZN3rbx8any_castIRKN3RBX6Camera10CameraModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Camera::CameraMode const& rbx::any_cast<RBX::Camera::CameraMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX6Camera10CameraModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_3caec0() -> ! {
    todo!("0x3caec0 RBX::Camera::CameraMode const& rbx::any_cast<RBX::Camera::CameraMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}


// 0x3cafb0 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToValue(RBX::Name const&,RBX::Camera::CameraMode&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraModeEE14convertToValueERKNS_4NameERS3_
pub fn stub_3cafb0() -> ! {
    todo!("0x3cafb0 RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::convertToValue(RBX::Name const&,RBX::Camera::CameraMode&)const")
}


// 0x3cb02c — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraModeEED2Ev
pub fn stub_3cb02c() -> ! {
    todo!("0x3cb02c RBX::Reflection::EnumDesc<RBX::Camera::CameraMode>::~EnumDesc()")
}


// 0x3cb200 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToString(RBX::Camera::CameraType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE15convertToStringERKS3_
pub fn stub_3cb200() -> ! {
    todo!("0x3cb200 RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToString(RBX::Camera::CameraType const&)const")
}


// 0x3cb3a0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraType>(RBX::Camera::CameraType const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraTypeEEERS3_RKT_
pub fn stub_3cb3a0() -> ! {
    todo!("0x3cb3a0 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraType>(RBX::Camera::CameraType const&)")
}


// 0x3cb3f0 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraType>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE9singletonEv
pub fn stub_3cb3f0() -> ! {
    todo!("0x3cb3f0 rbx::implementation::typed_holder<RBX::Camera::CameraType>::singleton(void)")
}


// 0x3cb45c — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraType>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE14construct_funcEPKcPc
pub fn stub_3cb45c() -> ! {
    todo!("0x3cb45c rbx::implementation::typed_holder<RBX::Camera::CameraType>::construct_func(char const*,char *)")
}


// 0x3cb468 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraType>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE13destruct_funcEPc
pub fn stub_3cb468() -> ! {
    todo!("0x3cb468 rbx::implementation::typed_holder<RBX::Camera::CameraType>::destruct_func(char *)")
}


// 0x3cb46c — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToItem(RBX::Camera::CameraType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE13convertToItemERKS3_
pub fn stub_3cb46c() -> ! {
    todo!("0x3cb46c RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToItem(RBX::Camera::CameraType const&)const")
}


// 0x3cb538 — __ZN3rbx8any_castIRKN3RBX6Camera10CameraTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Camera::CameraType const& rbx::any_cast<RBX::Camera::CameraType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX6Camera10CameraTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_3cb538() -> ! {
    todo!("0x3cb538 RBX::Camera::CameraType const& rbx::any_cast<RBX::Camera::CameraType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}


// 0x3cb628 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToValue(RBX::Name const&,RBX::Camera::CameraType&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE14convertToValueERKNS_4NameERS3_
pub fn stub_3cb628() -> ! {
    todo!("0x3cb628 RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToValue(RBX::Name const&,RBX::Camera::CameraType&)const")
}


// 0x3cb6a4 — __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEED2Ev
pub fn stub_3cb6a4() -> ! {
    todo!("0x3cb6a4 RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::~EnumDesc()")
}


// 0x3cb878 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E17static_getCreatorEv
pub fn stub_3cb878() -> ! {
    todo!("0x3cb878 __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E17static_getCreatorEv")
}


// 0x3cb8ec — __ZN3RBX10Reflection9DescribedINS_8HumanoidELZNS_9sHumanoidEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sHumanoidEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8HumanoidELZNS_9sHumanoidEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sHumanoidEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_8HumanoidELZNS_9sHumanoidEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sHumanoidEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_3cb8ec() -> ! {
    todo!("0x3cb8ec __ZN3RBX10Reflection9DescribedINS_8HumanoidELZNS_9sHumanoidEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sHumanoidEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}


// 0x3cbf50 — __ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3cbf50() -> ! {
    todo!("0x3cbf50 __ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x3cbf54 — __ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3cbf54() -> ! {
    todo!("0x3cbf54 __ZN3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x3cbff4 — __ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3cbff4() -> ! {
    todo!("0x3cbff4 __ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x3cbffc — __ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3cbffc() -> ! {
    todo!("0x3cbffc __ZThn32_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x3cc0a0 — __ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3cc0a0() -> ! {
    todo!("0x3cc0a0 __ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x3cc0a8 — __ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3cc0a8() -> ! {
    todo!("0x3cc0a8 __ZThn36_N3RBX10Reflection9DescribedINS_6CameraELZNS_7sCameraEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sCameraEES4_EELNS0_15ClassDescriptor13FunctionalityE25ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x3cc14c — __ZN3rbx8any_castIN3RBX6Camera13CameraPanModeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::Camera::CameraPanMode * rbx::any_cast<RBX::Camera::CameraPanMode,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: __ZN3rbx8any_castIN3RBX6Camera13CameraPanModeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_3cc14c() -> ! {
    todo!("0x3cc14c RBX::Camera::CameraPanMode * rbx::any_cast<RBX::Camera::CameraPanMode,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}


// 0x3cc1a4 — __ZN3rbx8any_castIRN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Camera::CameraPanMode & rbx::any_cast<RBX::Camera::CameraPanMode &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_3cc1a4() -> ! {
    todo!("0x3cc1a4 RBX::Camera::CameraPanMode & rbx::any_cast<RBX::Camera::CameraPanMode &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}


// 0x3cc294 — __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::resize(unsigned long,RBX::Camera::CameraPanMode)")]
// was: __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE6resizeEmS2_
pub fn stub_3cc294() -> ! {
    todo!("0x3cc294 std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::resize(unsigned long,RBX::Camera::CameraPanMode)")
}


// 0x3cc2c8 — __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::push_back(RBX::Camera::CameraPanMode const&)")]
// was: __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE9push_backERKS2_
pub fn stub_3cc2c8() -> ! {
    todo!("0x3cc2c8 std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::push_back(RBX::Camera::CameraPanMode const&)")
}


// 0x3cc2f0 — __ZNSt3mapIPKN3RBX4NameENS0_6Camera13CameraPanModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraPanMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_6Camera13CameraPanModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_3cc2f0() -> ! {
    todo!("0x3cc2f0 std::map<RBX::Name const*,RBX::Camera::CameraPanMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::operator[](RBX::Name const* const&)")
}


// 0x3cc348 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_3cc348() -> ! {
    todo!("0x3cc348 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")
}


// 0x3cc3fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_3cc3fc() -> ! {
    todo!("0x3cc3fc std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")
}


// 0x3cc454 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_3cc454() -> ! {
    todo!("0x3cc454 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")
}


// 0x3cc4bc — __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraPanMode*,std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>>,RBX::Camera::CameraPanMode const&)")]
// was: __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_3cc4bc() -> ! {
    todo!("0x3cc4bc std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraPanMode*,std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>>,RBX::Camera::CameraPanMode const&)")
}


// 0x3cc5a0 — __ZNSt12_Vector_baseIN3RBX6Camera13CameraPanModeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX6Camera13CameraPanModeESaIS2_EE11_M_allocateEm
pub fn stub_3cc5a0() -> ! {
    todo!("0x3cc5a0 std::_Vector_base<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_allocate(unsigned long)")
}


// 0x3cc5b8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera13CameraPanModeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Camera::CameraPanMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *>(RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera13CameraPanModeES6_EET0_T_S8_S7_
pub fn stub_3cc5b8() -> ! {
    todo!("0x3cc5b8 RBX::Camera::CameraPanMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *>(RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *)")
}


// 0x3cc5f4 — __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraPanMode*,std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>>,unsigned long,RBX::Camera::CameraPanMode const&)")]
// was: __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_3cc5f4() -> ! {
    todo!("0x3cc5f4 std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraPanMode*,std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>>,unsigned long,RBX::Camera::CameraPanMode const&)")
}


// 0x3cc784 — __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::resize(unsigned long,RBX::Camera::CameraMode)")]
// was: __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE6resizeEmS2_
pub fn stub_3cc784() -> ! {
    todo!("0x3cc784 std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::resize(unsigned long,RBX::Camera::CameraMode)")
}


// 0x3cc7b8 — __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::push_back(RBX::Camera::CameraMode const&)")]
// was: __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE9push_backERKS2_
pub fn stub_3cc7b8() -> ! {
    todo!("0x3cc7b8 std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::push_back(RBX::Camera::CameraMode const&)")
}


// 0x3cc7e0 — __ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_3cc7e0() -> ! {
    todo!("0x3cc7e0 std::map<RBX::Name const*,RBX::Camera::CameraMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::operator[](RBX::Name const* const&)")
}


// 0x3cc838 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_3cc838() -> ! {
    todo!("0x3cc838 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")
}


// 0x3cc8ec — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_3cc8ec() -> ! {
    todo!("0x3cc8ec std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")
}


// 0x3cc944 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_3cc944() -> ! {
    todo!("0x3cc944 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")
}