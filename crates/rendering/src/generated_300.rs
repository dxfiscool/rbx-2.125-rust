//! rendering shard 300 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 32540->32640 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 32540 before -> 32640 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0xf6fb4c (lowest remaining 0x3e7940..0x4228cc)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x3e7940 — __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_3e7940() -> ! {
    todo!("0x3e7940 RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x3e7968 — __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: void __fastcall(int, int, const std::string *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_3e7968() -> ! {
    todo!("0x3e7968 RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")
}

// 0x3e7ab0 — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_8ClothingEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Clothing>(char const*,char const*,RBX::TextureId RBX::Clothing::*,void (RBX::Clothing::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_8ClothingEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
pub fn stub_3e7ab0() -> ! {
    todo!("0x3e7ab0 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Clothing>(char const*,char const*,RBX::TextureId RBX::Clothing::*,void (RBX::Clothing::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x3e7c44 — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// was: __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EED0Ev
pub fn stub_3e7c44() -> ! {
    todo!("0x3e7c44 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::~BoundProp()")
}

// 0x3e7c70 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Clothing>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE10isReadOnlyEv
pub fn stub_3e7c70() -> ! {
    todo!("0x3e7c70 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Clothing>::isReadOnly(void)const")
}

// 0x3e7c74 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Clothing>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE11isWriteOnlyEv
pub fn stub_3e7c74() -> ! {
    todo!("0x3e7c74 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Clothing>::isWriteOnly(void)const")
}

// 0x3e7c78 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(std::string *this, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Clothing>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_3e7c78() -> ! {
    todo!("0x3e7c78 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Clothing>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x3e7c9c — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE8setValueEPNS0_13DescribedBaseERKS2_
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Clothing>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE8setValueEPNS0_13DescribedBaseERKS2_
pub fn stub_3e7c9c() -> ! {
    todo!("0x3e7c9c RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Clothing>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")
}

// 0x3e7d10 — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_12ShirtGraphicEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ShirtGraphic>(char const*,char const*,RBX::TextureId RBX::ShirtGraphic::*,void (RBX::ShirtGraphic::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_12ShirtGraphicEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
pub fn stub_3e7d10() -> ! {
    todo!("0x3e7d10 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ShirtGraphic>(char const*,char const*,RBX::TextureId RBX::ShirtGraphic::*,void (RBX::ShirtGraphic::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x3e7ea4 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ShirtGraphic>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE10isReadOnlyEv
pub fn stub_3e7ea4() -> ! {
    todo!("0x3e7ea4 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ShirtGraphic>::isReadOnly(void)const")
}

// 0x3e7ea8 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ShirtGraphic>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE11isWriteOnlyEv
pub fn stub_3e7ea8() -> ! {
    todo!("0x3e7ea8 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ShirtGraphic>::isWriteOnly(void)const")
}

// 0x3e7eac — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(std::string *this, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ShirtGraphic>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_3e7eac() -> ! {
    todo!("0x3e7eac RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ShirtGraphic>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x3e7ed0 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE8setValueEPNS0_13DescribedBaseERKS2_
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ShirtGraphic>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE8setValueEPNS0_13DescribedBaseERKS2_
pub fn stub_3e7ed0() -> ! {
    todo!("0x3e7ed0 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ShirtGraphic>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")
}

// 0x3e7f44 — __GLOBAL__I_a_168
#[doc(alias = "global constructor keyed to_a_168")]
// was: __GLOBAL__I_a_168
pub fn stub_3e7f44() -> ! {
    todo!("0x3e7f44 global constructor keyed to_a_168")
}

// 0x3e8668 — __ZN3RBX13CharacterMesh11setBodyPartENS0_8BodyPartE
// type: RBX::Instance *__fastcall(RBX::Instance *result, int)
#[doc(alias = "RBX::CharacterMesh::setBodyPart(RBX::CharacterMesh::BodyPart)")]
// was: __ZN3RBX13CharacterMesh11setBodyPartENS0_8BodyPartE
pub fn stub_3e8668() -> ! {
    todo!("0x3e8668 RBX::CharacterMesh::setBodyPart(RBX::CharacterMesh::BodyPart)")
}

// 0x3e8684 — __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEEC1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEEC1Ev
pub fn stub_3e8684() -> ! {
    todo!("0x3e8684 RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::EnumDesc(void)")
}

// 0x3e8688 — __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEEC2Ev
pub fn stub_3e8688() -> ! {
    todo!("0x3e8688 RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::EnumDesc(void)")
}

// 0x3e88a4 — __ZN3RBX13CharacterMeshC2Ev
// type: RBX::Instance *__fastcall(RBX::CharacterMesh *this)
#[doc(alias = "RBX::CharacterMesh::CharacterMesh(void)")]
// was: __ZN3RBX13CharacterMeshC2Ev
pub fn stub_3e88a4() -> ! {
    todo!("0x3e88a4 RBX::CharacterMesh::CharacterMesh(void)")
}

// 0x3e8b1c — __ZN3RBX13CharacterMesh17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
#[doc(alias = "RBX::CharacterMesh::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX13CharacterMesh17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_3e8b1c() -> ! {
    todo!("0x3e8b1c RBX::CharacterMesh::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x3e8b20 — __ZN3RBX13CharacterMesh13applyByMyselfEPNS_8HumanoidE
// type: RBX::PartInstance *__fastcall(RBX::CharacterMesh *this, RBX::Humanoid *)
#[doc(alias = "RBX::CharacterMesh::applyByMyself(RBX::Humanoid *)")]
// was: __ZN3RBX13CharacterMesh13applyByMyselfEPNS_8HumanoidE
pub fn stub_3e8b20() -> ! {
    todo!("0x3e8b20 RBX::CharacterMesh::applyByMyself(RBX::Humanoid *)")
}

// 0x3e8b74 — __ZNK3RBX13CharacterMesh16getBaseTextureIdEv
// type: void __fastcall(RBX::CharacterMesh *this, int)
#[doc(alias = "RBX::CharacterMesh::getBaseTextureId(void)const")]
// was: __ZNK3RBX13CharacterMesh16getBaseTextureIdEv
pub fn stub_3e8b74() -> ! {
    todo!("0x3e8b74 RBX::CharacterMesh::getBaseTextureId(void)const")
}

// 0x3e8cb0 — __ZNK3RBX13CharacterMesh19getOverlayTextureIdEv
// type: void __fastcall(RBX::CharacterMesh *this, int)
#[doc(alias = "RBX::CharacterMesh::getOverlayTextureId(void)const")]
// was: __ZNK3RBX13CharacterMesh19getOverlayTextureIdEv
pub fn stub_3e8cb0() -> ! {
    todo!("0x3e8cb0 RBX::CharacterMesh::getOverlayTextureId(void)const")
}

// 0x3e8dec — __ZNK3RBX13CharacterMesh9getMeshIdEv
// type: void __fastcall(RBX::CharacterMesh *this, int)
#[doc(alias = "RBX::CharacterMesh::getMeshId(void)const")]
// was: __ZNK3RBX13CharacterMesh9getMeshIdEv
pub fn stub_3e8dec() -> ! {
    todo!("0x3e8dec RBX::CharacterMesh::getMeshId(void)const")
}

// 0x3e8f28 — __ZNK3RBX13CharacterMesh11getBodyPartEv
// type: int __fastcall(RBX::CharacterMesh *this)
#[doc(alias = "RBX::CharacterMesh::getBodyPart(void)const")]
// was: __ZNK3RBX13CharacterMesh11getBodyPartEv
pub fn stub_3e8f28() -> ! {
    todo!("0x3e8f28 RBX::CharacterMesh::getBodyPart(void)const")
}

// 0x3e8f2c — __ZN3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEED1Ev
pub fn stub_3e8f2c() -> ! {
    todo!("0x3e8f2c RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::~EnumPropDescriptor()")
}

// 0x3e8f50 — __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::addPair(RBX::CharacterMesh::BodyPart,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE7addPairES3_PKc
pub fn stub_3e8f50() -> ! {
    todo!("0x3e8f50 RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::addPair(RBX::CharacterMesh::BodyPart,char const*)")
}

// 0x3e92b0 — __ZN3RBX13CharacterMeshD1Ev
// type: void __fastcall(RBX::CharacterMesh *__hidden this)
#[doc(alias = "RBX::CharacterMesh::~CharacterMesh()")]
// was: __ZN3RBX13CharacterMeshD1Ev
pub fn stub_3e92b0() -> ! {
    todo!("0x3e92b0 RBX::CharacterMesh::~CharacterMesh()")
}

// 0x3e92b4 — __ZN3RBX13CharacterMeshD0Ev
// type: void __fastcall(RBX::CharacterMesh *__hidden this)
#[doc(alias = "RBX::CharacterMesh::~CharacterMesh()")]
// was: __ZN3RBX13CharacterMeshD0Ev
pub fn stub_3e92b4() -> ! {
    todo!("0x3e92b4 RBX::CharacterMesh::~CharacterMesh()")
}

// 0x3e9354 — __ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE12getClassNameEv
pub fn stub_3e9354() -> ! {
    todo!("0x3e9354 __ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE12getClassNameEv")
}

// 0x3e9364 — __ZThn32_N3RBX13CharacterMeshD1Ev
// type: void __fastcall(RBX::CharacterMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CharacterMesh::~CharacterMesh()")]
// was: __ZThn32_N3RBX13CharacterMeshD1Ev
pub fn stub_3e9364() -> ! {
    todo!("0x3e9364 non-virtual thunk toRBX::CharacterMesh::~CharacterMesh()")
}

// 0x3e936c — __ZThn32_N3RBX13CharacterMeshD0Ev
// type: void __fastcall(RBX::CharacterMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CharacterMesh::~CharacterMesh()")]
// was: __ZThn32_N3RBX13CharacterMeshD0Ev
pub fn stub_3e936c() -> ! {
    todo!("0x3e936c non-virtual thunk toRBX::CharacterMesh::~CharacterMesh()")
}

// 0x3e9410 — __ZThn32_NK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE12getClassNameEv
pub fn stub_3e9410() -> ! {
    todo!("0x3e9410 __ZThn32_NK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE12getClassNameEv")
}

// 0x3e9420 — __ZThn36_N3RBX13CharacterMeshD1Ev
// type: void __fastcall(RBX::CharacterMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CharacterMesh::~CharacterMesh()")]
// was: __ZThn36_N3RBX13CharacterMeshD1Ev
pub fn stub_3e9420() -> ! {
    todo!("0x3e9420 non-virtual thunk toRBX::CharacterMesh::~CharacterMesh()")
}

// 0x3e9428 — __ZThn36_N3RBX13CharacterMeshD0Ev
// type: void __fastcall(RBX::CharacterMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CharacterMesh::~CharacterMesh()")]
// was: __ZThn36_N3RBX13CharacterMeshD0Ev
pub fn stub_3e9428() -> ! {
    todo!("0x3e9428 non-virtual thunk toRBX::CharacterMesh::~CharacterMesh()")
}

// 0x3e94cc — __ZThn92_N3RBX13CharacterMeshD1Ev
// type: void __fastcall(RBX::CharacterMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CharacterMesh::~CharacterMesh()")]
// was: __ZThn92_N3RBX13CharacterMeshD1Ev
pub fn stub_3e94cc() -> ! {
    todo!("0x3e94cc non-virtual thunk toRBX::CharacterMesh::~CharacterMesh()")
}

// 0x3e94d4 — __ZThn92_N3RBX13CharacterMeshD0Ev
// type: void __fastcall(RBX::CharacterMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CharacterMesh::~CharacterMesh()")]
// was: __ZThn92_N3RBX13CharacterMeshD0Ev
pub fn stub_3e94d4() -> ! {
    todo!("0x3e94d4 non-virtual thunk toRBX::CharacterMesh::~CharacterMesh()")
}

// 0x3e9578 — __ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorD1Ev
pub fn stub_3e9578() -> ! {
    todo!("0x3e9578 __ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorD1Ev")
}

// 0x3e957c — __ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorD2Ev
pub fn stub_3e957c() -> ! {
    todo!("0x3e957c __ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorD2Ev")
}

// 0x3e9618 — __ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_3e9618() -> ! {
    todo!("0x3e9618 __ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x3e96a0 — __ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7Creator6createEv
pub fn stub_3e96a0() -> ! {
    todo!("0x3e96a0 __ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7Creator6createEv")
}

// 0x3e97e4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13CharacterMeshEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "boost::shared_ptr<RBX::CharacterMesh> RBX::Creatable<RBX::Instance>::create<RBX::CharacterMesh>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_13CharacterMeshEEEN5boost10shared_ptrIT_EEv
pub fn stub_3e97e4() -> ! {
    todo!("0x3e97e4 boost::shared_ptr<RBX::CharacterMesh> RBX::Creatable<RBX::Instance>::create<RBX::CharacterMesh>(void)")
}

// 0x3e9894 — __ZN5boost10shared_ptrIN3RBX13CharacterMeshEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::CharacterMesh>::shared_ptr<RBX::CharacterMesh,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13CharacterMeshEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_3e9894() -> ! {
    todo!("0x3e9894 boost::shared_ptr<RBX::CharacterMesh>::shared_ptr<RBX::CharacterMesh,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3e995c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13CharacterMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CharacterMesh,RBX::CharacterMesh>(boost::shared_ptr<RBX::CharacterMesh> const*,RBX::CharacterMesh *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13CharacterMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_3e995c() -> ! {
    todo!("0x3e995c void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CharacterMesh,RBX::CharacterMesh>(boost::shared_ptr<RBX::CharacterMesh> const*,RBX::CharacterMesh *)const")
}

// 0x3e9a44 — __ZN5boost6detail12shared_countC2IPN3RBX13CharacterMeshENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13CharacterMeshENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_3e9a44() -> ! {
    todo!("0x3e9a44 boost::detail::shared_count::shared_count<RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3e9b4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13CharacterMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13CharacterMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_3e9b4c() -> ! {
    todo!("0x3e9b4c boost::detail::sp_counted_impl_pd<RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3e9b50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13CharacterMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13CharacterMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_3e9b50() -> ! {
    todo!("0x3e9b50 boost::detail::sp_counted_impl_pd<RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3e9b54 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13CharacterMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13CharacterMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_3e9b54() -> ! {
    todo!("0x3e9b54 boost::detail::sp_counted_impl_pd<RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3e9b74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13CharacterMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13CharacterMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_3e9b74() -> ! {
    todo!("0x3e9b74 boost::detail::sp_counted_impl_pd<RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3e9b8c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13CharacterMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13CharacterMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_3e9b8c() -> ! {
    todo!("0x3e9b8c boost::detail::sp_counted_impl_pd<RBX::CharacterMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3e9b90 — __ZN3RBX4Name13callDoDeclareILZNS_14sCharacterMeshEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sCharacterMeshEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sCharacterMeshEEEEvv
pub fn stub_3e9b90() -> ! {
    todo!("0x3e9b90 __ZN3RBX4Name13callDoDeclareILZNS_14sCharacterMeshEEEEvv")
}

// 0x41d2d0 — __ZN3RBX9DataModel12setCreatorIDEiNS0_11CreatorTypeE
// type: int __fastcall(RBX::Instance *, int, int)
#[doc(alias = "RBX::DataModel::setCreatorID(int,RBX::DataModel::CreatorType)")]
// was: __ZN3RBX9DataModel12setCreatorIDEiNS0_11CreatorTypeE
pub fn stub_41d2d0() -> ! {
    todo!("0x41d2d0 RBX::DataModel::setCreatorID(int,RBX::DataModel::CreatorType)")
}

// 0x41d320 — __ZN3RBX9DataModel8setGenreENS0_5GenreE
// type: RBX::Instance *__fastcall(RBX::Instance *result, int)
#[doc(alias = "RBX::DataModel::setGenre(RBX::DataModel::Genre)")]
// was: __ZN3RBX9DataModel8setGenreENS0_5GenreE
pub fn stub_41d320() -> ! {
    todo!("0x41d320 RBX::DataModel::setGenre(RBX::DataModel::Genre)")
}

// 0x41d340 — __ZN3RBX9DataModel7setGearENS0_16GearGenreSettingEi
// type: int __fastcall(RBX::Instance *, int, int)
#[doc(alias = "RBX::DataModel::setGear(RBX::DataModel::GearGenreSetting,int)")]
// was: __ZN3RBX9DataModel7setGearENS0_16GearGenreSettingEi
pub fn stub_41d340() -> ! {
    todo!("0x41d340 RBX::DataModel::setGear(RBX::DataModel::GearGenreSetting,int)")
}

// 0x41d384 — __ZNK3RBX9DataModel21getLightingDeprecatedEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getLightingDeprecated(void)const")]
// was: __ZNK3RBX9DataModel21getLightingDeprecatedEv
pub fn stub_41d384() -> ! {
    todo!("0x41d384 RBX::DataModel::getLightingDeprecated(void)const")
}

// 0x41d390 — __ZN3RBX9DataModel17isGearTypeAllowedENS0_8GearTypeE
// type: bool __fastcall(int, char)
#[doc(alias = "RBX::DataModel::isGearTypeAllowed(RBX::DataModel::GearType)")]
// was: __ZN3RBX9DataModel17isGearTypeAllowedENS0_8GearTypeE
pub fn stub_41d390() -> ! {
    todo!("0x41d390 RBX::DataModel::isGearTypeAllowed(RBX::DataModel::GearType)")
}

// 0x41d3a4 — __ZN3RBX9DataModel20setScreenshotSEOInfoESs
// type: int __fastcall(int)
#[doc(alias = "RBX::DataModel::setScreenshotSEOInfo(std::string)")]
// was: __ZN3RBX9DataModel20setScreenshotSEOInfoESs
pub fn stub_41d3a4() -> ! {
    todo!("0x41d3a4 RBX::DataModel::setScreenshotSEOInfo(std::string)")
}

// 0x41d3ac — __ZN3RBX9DataModel15setVideoSEOInfoESs
// type: int __fastcall(int)
#[doc(alias = "RBX::DataModel::setVideoSEOInfo(std::string)")]
// was: __ZN3RBX9DataModel15setVideoSEOInfoESs
pub fn stub_41d3ac() -> ! {
    todo!("0x41d3ac RBX::DataModel::setVideoSEOInfo(std::string)")
}

// 0x41d3b4 — __ZN3RBX9DataModel13addCustomStatESsSs
// type: int __fastcall(int, const std::string *, const std::string *)
#[doc(alias = "RBX::DataModel::addCustomStat(std::string,std::string)")]
// was: __ZN3RBX9DataModel13addCustomStatESsSs
pub fn stub_41d3b4() -> ! {
    todo!("0x41d3b4 RBX::DataModel::addCustomStat(std::string,std::string)")
}

// 0x41d3bc — __ZN3RBX9DataModel16removeCustomStatESs
// type: int __fastcall(int, const std::string *)
#[doc(alias = "RBX::DataModel::removeCustomStat(std::string)")]
// was: __ZN3RBX9DataModel16removeCustomStatESs
pub fn stub_41d3bc() -> ! {
    todo!("0x41d3bc RBX::DataModel::removeCustomStat(std::string)")
}

// 0x41d3c4 — __ZN3RBX9DataModel18writeStatsSettingsEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::writeStatsSettings(void)")]
// was: __ZN3RBX9DataModel18writeStatsSettingsEv
pub fn stub_41d3c4() -> ! {
    todo!("0x41d3c4 RBX::DataModel::writeStatsSettings(void)")
}

// 0x41d3d0 — __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEEC2Ev
pub fn stub_41d3d0() -> ! {
    todo!("0x41d3d0 RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::EnumDesc(void)")
}

// 0x41d590 — __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEEC2Ev
pub fn stub_41d590() -> ! {
    todo!("0x41d590 RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::EnumDesc(void)")
}

// 0x41d864 — __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEEC2Ev
pub fn stub_41d864() -> ! {
    todo!("0x41d864 RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::EnumDesc(void)")
}

// 0x41da24 — __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEEC2Ev
pub fn stub_41da24() -> ! {
    todo!("0x41da24 RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::EnumDesc(void)")
}

// 0x41dc84 — __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEEC2Ev
pub fn stub_41dc84() -> ! {
    todo!("0x41dc84 RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::EnumDesc(void)")
}

// 0x41de60 — __ZN3RBX15StringConverterINS_9DataModel11CreatorTypeEE14convertToValueERKSsRS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::DataModel::CreatorType>::convertToValue(std::string const&,RBX::DataModel::CreatorType&)")]
// was: __ZN3RBX15StringConverterINS_9DataModel11CreatorTypeEE14convertToValueERKSsRS2_
pub fn stub_41de60() -> ! {
    todo!("0x41de60 RBX::StringConverter<RBX::DataModel::CreatorType>::convertToValue(std::string const&,RBX::DataModel::CreatorType&)")
}

// 0x41deac — __ZN3RBX15StringConverterINS_9DataModel5GenreEE14convertToValueERKSsRS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::DataModel::Genre>::convertToValue(std::string const&,RBX::DataModel::Genre&)")]
// was: __ZN3RBX15StringConverterINS_9DataModel5GenreEE14convertToValueERKSsRS2_
pub fn stub_41deac() -> ! {
    todo!("0x41deac RBX::StringConverter<RBX::DataModel::Genre>::convertToValue(std::string const&,RBX::DataModel::Genre&)")
}

// 0x41def8 — __ZN3RBX15StringConverterINS_9DataModel16GearGenreSettingEE14convertToValueERKSsRS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::DataModel::GearGenreSetting>::convertToValue(std::string const&,RBX::DataModel::GearGenreSetting&)")]
// was: __ZN3RBX15StringConverterINS_9DataModel16GearGenreSettingEE14convertToValueERKSsRS2_
pub fn stub_41def8() -> ! {
    todo!("0x41def8 RBX::StringConverter<RBX::DataModel::GearGenreSetting>::convertToValue(std::string const&,RBX::DataModel::GearGenreSetting&)")
}

// 0x41df44 — __ZN3RBX15StringConverterINS_9DataModel8GearTypeEE14convertToValueERKSsRS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::DataModel::GearType>::convertToValue(std::string const&,RBX::DataModel::GearType&)")]
// was: __ZN3RBX15StringConverterINS_9DataModel8GearTypeEE14convertToValueERKSsRS2_
pub fn stub_41df44() -> ! {
    todo!("0x41df44 RBX::StringConverter<RBX::DataModel::GearType>::convertToValue(std::string const&,RBX::DataModel::GearType&)")
}

// 0x41df90 — __ZN3RBX15StringConverterINS_8Instance10SaveFilterEE14convertToValueERKSsRS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::Instance::SaveFilter>::convertToValue(std::string const&,RBX::Instance::SaveFilter&)")]
// was: __ZN3RBX15StringConverterINS_8Instance10SaveFilterEE14convertToValueERKSsRS2_
pub fn stub_41df90() -> ! {
    todo!("0x41df90 RBX::StringConverter<RBX::Instance::SaveFilter>::convertToValue(std::string const&,RBX::Instance::SaveFilter&)")
}

// 0x41dfdc — __ZN3RBX9DataModel7canSaveEPKNS_8InstanceE
// type: bool __fastcall(RBX::DataModel *this, const RBX::Instance *)
#[doc(alias = "RBX::DataModel::canSave(RBX::Instance const*)")]
// was: __ZN3RBX9DataModel7canSaveEPKNS_8InstanceE
pub fn stub_41dfdc() -> ! {
    todo!("0x41dfdc RBX::DataModel::canSave(RBX::Instance const*)")
}

// 0x41e040 — __ZN3RBX9DataModel15serverSavePlaceENS_8Instance10SaveFilterEN5boost8functionIFvNS3_10shared_ptrIKNS_10Reflection5TupleEEEEEENS4_IFvSsEEE
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::DataModel::serverSavePlace(RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX9DataModel15serverSavePlaceENS_8Instance10SaveFilterEN5boost8functionIFvNS3_10shared_ptrIKNS_10Reflection5TupleEEEEEENS4_IFvSsEEE
pub fn stub_41e040() -> ! {
    todo!("0x41e040 RBX::DataModel::serverSavePlace(RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0x41e51c — __ZN3RBX9DataModel14savePlaceAsyncENS_8Instance10SaveFilterEN5boost8functionIFvNS3_10shared_ptrIKNS_10Reflection5TupleEEEEEENS4_IFvSsEEE
// type: void __fastcall(int *, int, _BOOL4, int)
#[doc(alias = "RBX::DataModel::savePlaceAsync(RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX9DataModel14savePlaceAsyncENS_8Instance10SaveFilterEN5boost8functionIFvNS3_10shared_ptrIKNS_10Reflection5TupleEEEEEENS4_IFvSsEEE
pub fn stub_41e51c() -> ! {
    todo!("0x41e51c RBX::DataModel::savePlaceAsync(RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0x41e84c — __ZN3RBX9DataModel24getSyncronizationArbiterEv
// type: RBX::DataModel *__fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getSyncronizationArbiter(void)")]
// was: __ZN3RBX9DataModel24getSyncronizationArbiterEv
pub fn stub_41e84c() -> ! {
    todo!("0x41e84c RBX::DataModel::getSyncronizationArbiter(void)")
}

// 0x41e860 — __ZThn184_N3RBX9DataModel24getSyncronizationArbiterEv
// type: char *__fastcall(RBX::DataModel *this)
#[doc(alias = "non-virtual thunk toRBX::DataModel::getSyncronizationArbiter(void)")]
// was: __ZThn184_N3RBX9DataModel24getSyncronizationArbiterEv
pub fn stub_41e860() -> ! {
    todo!("0x41e860 non-virtual thunk toRBX::DataModel::getSyncronizationArbiter(void)")
}

// 0x41e878 — __ZN3RBX9DataModel16doDataModelSetupEN5boost10shared_ptrIS0_EEb
// type: void __fastcall(const shared_count *, char, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, void *, void *, void *, char, char, char, int, int, int, int)
#[doc(alias = "RBX::DataModel::doDataModelSetup(boost::shared_ptr<RBX::DataModel>,bool)")]
// was: __ZN3RBX9DataModel16doDataModelSetupEN5boost10shared_ptrIS0_EEb
pub fn stub_41e878() -> ! {
    todo!("0x41e878 RBX::DataModel::doDataModelSetup(boost::shared_ptr<RBX::DataModel>,bool)")
}

// 0x41ede0 — __ZN3RBX9DataModel10LegacyLockC1EN5boost10shared_ptrIS0_EENS_12DataModelJob8TaskTypeE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::DataModel::LegacyLock::LegacyLock(boost::shared_ptr<RBX::DataModel>,RBX::DataModelJob::TaskType)")]
// was: __ZN3RBX9DataModel10LegacyLockC1EN5boost10shared_ptrIS0_EENS_12DataModelJob8TaskTypeE
pub fn stub_41ede0() -> ! {
    todo!("0x41ede0 RBX::DataModel::LegacyLock::LegacyLock(boost::shared_ptr<RBX::DataModel>,RBX::DataModelJob::TaskType)")
}

// 0x41ede4 — __ZN3RBX9DataModel18initializeContentsEb
// type: void __fastcall(RBX::DataModel *this, boost::detail::sp_counted_base *)
#[doc(alias = "RBX::DataModel::initializeContents(bool)")]
// was: __ZN3RBX9DataModel18initializeContentsEb
pub fn stub_41ede4() -> ! {
    todo!("0x41ede4 RBX::DataModel::initializeContents(bool)")
}

// 0x41f220 — __ZN3RBX9DataModel10LegacyLockD1Ev
// type: void __fastcall(RBX::DataModel::LegacyLock *__hidden this)
#[doc(alias = "RBX::DataModel::LegacyLock::~LegacyLock()")]
// was: __ZN3RBX9DataModel10LegacyLockD1Ev
pub fn stub_41f220() -> ! {
    todo!("0x41f220 RBX::DataModel::LegacyLock::~LegacyLock()")
}

// 0x41f230 — __ZN3RBX9DataModel15createDataModelEbPNS_4VerbEPS0_
// type: void __fastcall(RBX::DataModel *this, const char *, RBX::Verb *, RBX::DataModel *)
#[doc(alias = "RBX::DataModel::createDataModel(bool,RBX::Verb *,RBX::DataModel*)")]
// was: __ZN3RBX9DataModel15createDataModelEbPNS_4VerbEPS0_
pub fn stub_41f230() -> ! {
    todo!("0x41f230 RBX::DataModel::createDataModel(bool,RBX::Verb *,RBX::DataModel*)")
}

// 0x41f360 — __ZN3RBX9DataModel40onlyJobsLeftForThisArbiterAreGenericJobsEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::onlyJobsLeftForThisArbiterAreGenericJobs(void)")]
// was: __ZN3RBX9DataModel40onlyJobsLeftForThisArbiterAreGenericJobsEv
pub fn stub_41f360() -> ! {
    todo!("0x41f360 RBX::DataModel::onlyJobsLeftForThisArbiterAreGenericJobs(void)")
}

// 0x41f518 — __ZN3RBX9DataModel16doCloseDataModelEN5boost10shared_ptrIS0_EE
// type: void __fastcall(const char **, int, int, const void *, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::DataModel::doCloseDataModel(boost::shared_ptr<RBX::DataModel>)")]
// was: __ZN3RBX9DataModel16doCloseDataModelEN5boost10shared_ptrIS0_EE
pub fn stub_41f518() -> ! {
    todo!("0x41f518 RBX::DataModel::doCloseDataModel(boost::shared_ptr<RBX::DataModel>)")
}

// 0x41fba0 — __ZN3RBX9DataModel14closeDataModelEN5boost10shared_ptrIS0_EEb
// type: void __fastcall(int, int, int, const void *, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int)
#[doc(alias = "RBX::DataModel::closeDataModel(boost::shared_ptr<RBX::DataModel>,bool)")]
// was: __ZN3RBX9DataModel14closeDataModelEN5boost10shared_ptrIS0_EEb
pub fn stub_41fba0() -> ! {
    todo!("0x41fba0 RBX::DataModel::closeDataModel(boost::shared_ptr<RBX::DataModel>,bool)")
}

// 0x41fd78 — __ZN3RBX9DataModelC2EPNS_4VerbEPS0_
// type: int __fastcall(RBX::DataModel *this, RBX::Verb *, RBX::DataModel *)
#[doc(alias = "RBX::DataModel::DataModel(RBX::Verb *,RBX::DataModel*)")]
// was: __ZN3RBX9DataModelC2EPNS_4VerbEPS0_
pub fn stub_41fd78() -> ! {
    todo!("0x41fd78 RBX::DataModel::DataModel(RBX::Verb *,RBX::DataModel*)")
}

// 0x420c38 — __ZN3RBXL19registerNewImageAPIEv
// type: void __fastcall(RBX *this)
#[doc(alias = "RBX::registerNewImageAPI(void)")]
// was: __ZN3RBXL19registerNewImageAPIEv
pub fn stub_420c38() -> ! {
    todo!("0x420c38 RBX::registerNewImageAPI(void)")
}

// 0x420fec — __ZN3RBX9DataModel15onRunTransitionENS_13RunTransitionE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::DataModel::onRunTransition(RBX::RunTransition)")]
// was: __ZN3RBX9DataModel15onRunTransitionENS_13RunTransitionE
pub fn stub_420fec() -> ! {
    todo!("0x420fec RBX::DataModel::onRunTransition(RBX::RunTransition)")
}

// 0x4210d8 — __ZN3RBX9DataModel15loadCoreScriptsEv
// type: void __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::loadCoreScripts(void)")]
// was: __ZN3RBX9DataModel15loadCoreScriptsEv
pub fn stub_4210d8() -> ! {
    todo!("0x4210d8 RBX::DataModel::loadCoreScripts(void)")
}

// 0x421b80 — __ZN3RBX9DataModelD0Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "RBX::DataModel::~DataModel()")]
// was: __ZN3RBX9DataModelD0Ev
pub fn stub_421b80() -> ! {
    todo!("0x421b80 RBX::DataModel::~DataModel()")
}

// 0x421c20 — __ZN3RBX9DataModelD1Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "RBX::DataModel::~DataModel()")]
// was: __ZN3RBX9DataModelD1Ev
pub fn stub_421c20() -> ! {
    todo!("0x421c20 RBX::DataModel::~DataModel()")
}

// 0x421c24 — __ZThn32_N3RBX9DataModelD0Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel()")]
// was: __ZThn32_N3RBX9DataModelD0Ev
pub fn stub_421c24() -> ! {
    todo!("0x421c24 non-virtual thunk toRBX::DataModel::~DataModel()")
}

// 0x421c2c — __ZThn36_N3RBX9DataModelD0Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel()")]
// was: __ZThn36_N3RBX9DataModelD0Ev
pub fn stub_421c2c() -> ! {
    todo!("0x421c2c non-virtual thunk toRBX::DataModel::~DataModel()")
}

// 0x421c34 — __ZThn144_N3RBX9DataModelD0Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel()")]
// was: __ZThn144_N3RBX9DataModelD0Ev
pub fn stub_421c34() -> ! {
    todo!("0x421c34 non-virtual thunk toRBX::DataModel::~DataModel()")
}

// 0x421c3c — __ZThn180_N3RBX9DataModelD0Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel()")]
// was: __ZThn180_N3RBX9DataModelD0Ev
pub fn stub_421c3c() -> ! {
    todo!("0x421c3c non-virtual thunk toRBX::DataModel::~DataModel()")
}

// 0x421c44 — __ZThn184_N3RBX9DataModelD0Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel()")]
// was: __ZThn184_N3RBX9DataModelD0Ev
pub fn stub_421c44() -> ! {
    todo!("0x421c44 non-virtual thunk toRBX::DataModel::~DataModel()")
}

// 0x421c4c — __ZN3RBX9DataModelD2Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "RBX::DataModel::~DataModel()")]
// was: __ZN3RBX9DataModelD2Ev
pub fn stub_421c4c() -> ! {
    todo!("0x421c4c RBX::DataModel::~DataModel()")
}

// 0x4228ac — __ZThn32_N3RBX9DataModelD1Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel()")]
// was: __ZThn32_N3RBX9DataModelD1Ev
pub fn stub_4228ac() -> ! {
    todo!("0x4228ac non-virtual thunk toRBX::DataModel::~DataModel()")
}

// 0x4228b4 — __ZThn36_N3RBX9DataModelD1Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel()")]
// was: __ZThn36_N3RBX9DataModelD1Ev
pub fn stub_4228b4() -> ! {
    todo!("0x4228b4 non-virtual thunk toRBX::DataModel::~DataModel()")
}

// 0x4228bc — __ZThn144_N3RBX9DataModelD1Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel()")]
// was: __ZThn144_N3RBX9DataModelD1Ev
pub fn stub_4228bc() -> ! {
    todo!("0x4228bc non-virtual thunk toRBX::DataModel::~DataModel()")
}

// 0x4228c4 — __ZThn180_N3RBX9DataModelD1Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel()")]
// was: __ZThn180_N3RBX9DataModelD1Ev
pub fn stub_4228c4() -> ! {
    todo!("0x4228c4 non-virtual thunk toRBX::DataModel::~DataModel()")
}

// 0x4228cc — __ZThn184_N3RBX9DataModelD1Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel()")]
// was: __ZThn184_N3RBX9DataModelD1Ev
pub fn stub_4228cc() -> ! {
    todo!("0x4228cc non-virtual thunk toRBX::DataModel::~DataModel()")
}