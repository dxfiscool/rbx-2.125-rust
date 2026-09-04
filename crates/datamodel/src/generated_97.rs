// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX:: + Instance|DataModel|Workspace (broad, includes PartInstance/MegaClusterInstance etc), EA-sorted, true uncovered after existing shards
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x4c99b0..0x5aab3c | total filtered 13121, remaining 2015->1915 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias
// Shard: 97 EA-sorted ascending next uncovered gap from 0x4c99b0

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
// 0x4c99b0 — __ZN3rbx8any_castIRKN3RBX17BasicPartInstance14LegacyPartTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::BasicPartInstance::LegacyPartType const& rbx::any_cast<RBX::BasicPartInstance::LegacyPartType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub use crate::instance::stub_0x4c99b0 as stub_4c99b0;
// 0x4c9aa0 — __ZNK3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToValue(RBX::Name const&,RBX::BasicPartInstance::LegacyPartType&)const")]
pub use crate::instance::stub_0x4c9aa0 as stub_4c9aa0;
// 0x4c9b1c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17BasicPartInstance14LegacyPartTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>> *)")]
pub use crate::instance::stub_0x4c9b1c as stub_4c9b1c;
// 0x4e75d0 — __ZNK3RBX13JointInstance21getPersistentDataCostEv
#[doc(alias = "RBX::JointInstance::getPersistentDataCost(void)const")]
pub use crate::instance::stub_0x4e75d0 as stub_4e75d0;
// 0x4ef3ec — __ZN5boost10shared_ptrIN3RBX13ModelInstanceEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::ModelInstance>::operator=(rbx_core::SharedPtr<RBX::ModelInstance> const&)")]
// was: boost::shared_ptr<RBX::ModelInstance>::operator=(boost::shared_ptr<RBX::ModelInstance> const&)
pub use crate::instance::stub_0x4ef3ec as stub_4ef3ec;
// 0x4f8094 — __ZN3RBX10ForceField16partInForceFieldEPNS_12PartInstanceE
#[doc(alias = "RBX::ForceField::partInForceField(RBX::PartInstance *)")]
pub use crate::instance::stub_0x4f8094 as stub_4f8094;
// 0x4fbc68 — __ZN3RBX4Game14setupDataModelERKSs
#[doc(alias = "RBX::Game::setupDataModel(std::string const&)")]
pub use crate::instance::stub_0x4fbc68 as stub_4fbc68;
// 0x4fc660 — __ZN3RBX4Game21shutdownGameDataModelEv
#[doc(alias = "RBX::Game::shutdownGameDataModel(void)")]
pub use crate::instance::stub_0x4fc660 as stub_4fc660;
// 0x4fc7c0 — __ZN3RBX15ScopedSingletonINS_15ProfanityFilterEE11getInstanceEv
#[doc(alias = "RBX::ScopedSingleton<RBX::ProfanityFilter>::getInstance(void)")]
pub use crate::instance::stub_0x4fc7c0 as stub_4fc7c0;
// 0x4fcd5c — __ZN5boost10shared_ptrIN3RBX16OverlayDataModelEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::OverlayDataModel>::operator=(rbx_core::SharedPtr<RBX::OverlayDataModel> const&)")]
// was: boost::shared_ptr<RBX::OverlayDataModel>::operator=(boost::shared_ptr<RBX::OverlayDataModel> const&)
pub use crate::instance::stub_0x4fcd5c as stub_4fcd5c;
// 0x4fd1e8 — __ZN3RBX10shutdownDMINS_16OverlayDataModelEEEvRN5boost10shared_ptrIT_EE
#[doc(alias = "void RBX::shutdownDM<RBX::OverlayDataModel>(rbx_core::SharedPtr<RBX::OverlayDataModel> &)")]
// was: void RBX::shutdownDM<RBX::OverlayDataModel>(boost::shared_ptr<RBX::OverlayDataModel> &)
pub use crate::instance::stub_0x4fd1e8 as stub_4fd1e8;
// 0x50546c — __ZN3RBX15GeometryService23getPartsTouchingExtentsERKNS_7ExtentsEPKNS_9PrimitiveEiRN3G3D5ArrayIPNS_12PartInstanceELi10ELm32EEE
#[doc(alias = "RBX::GeometryService::getPartsTouchingExtents(RBX::Extents const&,RBX::Primitive const*,int,G3D::Array<RBX::PartInstance *,10,32ul> &)")]
pub use crate::instance::stub_0x50546c as stub_50546c;
// 0x505b70 — __ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::append(RBX::PartInstance * const&)")]
pub use crate::instance::stub_0x505b70 as stub_505b70;
// 0x506524 — __ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::resize(int,bool)")]
pub use crate::instance::stub_0x506524 as stub_506524;
// 0x5065dc — __ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::realloc(int)")]
pub use crate::instance::stub_0x5065dc as stub_5065dc;
// 0x5567bc — __ZN3RBX6Rocket9setTargetEPNS_12PartInstanceE
#[doc(alias = "RBX::Rocket::setTarget(RBX::PartInstance *)")]
pub use crate::instance::stub_0x5567bc as stub_5567bc;
// 0x559448 — __ZN3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::~RefPropDescriptor()")]
pub use crate::instance::stub_0x559448 as stub_559448;
// 0x561efc — __ZN3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::Rocket::*)(void)const,void (RBX::Rocket::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::Rocket::*)(void)const,void (RBX::Rocket::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub use crate::instance::stub_0x561efc as stub_561efc;
// 0x561fa0 — __ZN3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::~RefPropDescriptor()")]
pub use crate::instance::stub_0x561fa0 as stub_561fa0;
// 0x561fd0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::isReadOnly(void)const")]
pub use crate::instance::stub_0x561fd0 as stub_561fd0;
// 0x561fe0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x561fe0 as stub_561fe0;
// 0x561ff0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x561ff0 as stub_561ff0;
// 0x562018 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub use crate::instance::stub_0x562018 as stub_562018;
// 0x562130 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub use crate::instance::stub_0x562130 as stub_562130;
// 0x5621f8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub use crate::instance::stub_0x5621f8 as stub_5621f8;
// 0x56221c — __ZNK3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub use crate::instance::stub_0x56221c as stub_56221c;
// 0x5622f0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub use crate::instance::stub_0x5622f0 as stub_5622f0;
// 0x562314 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEE11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x562314 as stub_562314;
// 0x562328 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEE11setRefValueEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub use crate::instance::stub_0x562328 as stub_562328;
// 0x5623a4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6RocketENS_12PartInstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub use crate::instance::stub_0x5623a4 as stub_5623a4;
// 0x5624ac — __ZNK3RBX10Reflection14PropDescriptorINS_6RocketEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Rocket,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::Rocket::*)(void)const,void (RBX::Rocket::*)(RBX::PartInstance *)>::isReadOnly(void)const")]
pub use crate::instance::stub_0x5624ac as stub_5624ac;
// 0x5624b0 — __ZNK3RBX10Reflection14PropDescriptorINS_6RocketEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Rocket,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::Rocket::*)(void)const,void (RBX::Rocket::*)(RBX::PartInstance *)>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x5624b0 as stub_5624b0;
// 0x5624b4 — __ZNK3RBX10Reflection14PropDescriptorINS_6RocketEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Rocket,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::Rocket::*)(void)const,void (RBX::Rocket::*)(RBX::PartInstance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x5624b4 as stub_5624b4;
// 0x5624d4 — __ZNK3RBX10Reflection14PropDescriptorINS_6RocketEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Rocket,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::Rocket::*)(void)const,void (RBX::Rocket::*)(RBX::PartInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance * const&)const")]
pub use crate::instance::stub_0x5624d4 as stub_5624d4;
// 0x57c17c — __ZN3RBX10IEquipable9buildWeldEPNS_12PartInstanceES2_RKN3G3D15CoordinateFrameES6_RKSs
#[doc(alias = "RBX::IEquipable::buildWeld(RBX::PartInstance *,RBX::PartInstance *,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,std::string const&)")]
pub use crate::instance::stub_0x57c17c as stub_57c17c;
// 0x59f0a8 — __ZNK3RBX13JointInstance17getPart0DangerousEv
#[doc(alias = "RBX::JointInstance::getPart0Dangerous(void)const")]
pub use crate::instance::stub_0x59f0a8 as stub_59f0a8;
// 0x59f0cc — __ZN3RBX13JointInstance8setPart0EPNS_12PartInstanceE
#[doc(alias = "RBX::JointInstance::setPart0(RBX::PartInstance *)")]
pub use crate::instance::stub_0x59f0cc as stub_59f0cc;
// 0x59f0f0 — __ZNK3RBX13JointInstance17getPart1DangerousEv
#[doc(alias = "RBX::JointInstance::getPart1Dangerous(void)const")]
pub use crate::instance::stub_0x59f0f0 as stub_59f0f0;
// 0x59f114 — __ZN3RBX13JointInstance8setPart1EPNS_12PartInstanceE
#[doc(alias = "RBX::JointInstance::setPart1(RBX::PartInstance *)")]
pub use crate::instance::stub_0x59f114 as stub_59f114;
// 0x59f138 — __ZNK3RBX26ManualSurfaceJointInstance11getSurface0Ev
#[doc(alias = "RBX::ManualSurfaceJointInstance::getSurface0(void)const")]
pub use crate::instance::stub_0x59f138 as stub_59f138;
// 0x59f17c — __ZN3RBX26ManualSurfaceJointInstance11setSurface0Ei
#[doc(alias = "RBX::ManualSurfaceJointInstance::setSurface0(int)")]
pub use crate::instance::stub_0x59f17c as stub_59f17c;
// 0x59f1e0 — __ZNK3RBX26ManualSurfaceJointInstance11getSurface1Ev
#[doc(alias = "RBX::ManualSurfaceJointInstance::getSurface1(void)const")]
pub use crate::instance::stub_0x59f1e0 as stub_59f1e0;
// 0x59f224 — __ZN3RBX26ManualSurfaceJointInstance11setSurface1Ei
#[doc(alias = "RBX::ManualSurfaceJointInstance::setSurface1(int)")]
pub use crate::instance::stub_0x59f224 as stub_59f224;
// 0x59f288 — __ZNK3RBX13JointInstance5getC0Ev
#[doc(alias = "RBX::JointInstance::getC0(void)const")]
pub use crate::instance::stub_0x59f288 as stub_59f288;
// 0x59f290 — __ZN3RBX13JointInstance5setC0ERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::JointInstance::setC0(G3D::CoordinateFrame const&)")]
pub use crate::instance::stub_0x59f290 as stub_59f290;
// 0x59f2bc — __ZNK3RBX13JointInstance5getC1Ev
#[doc(alias = "RBX::JointInstance::getC1(void)const")]
pub use crate::instance::stub_0x59f2bc as stub_59f2bc;
// 0x59f2c4 — __ZN3RBX13JointInstance5setC1ERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::JointInstance::setC1(G3D::CoordinateFrame const&)")]
pub use crate::instance::stub_0x59f2c4 as stub_59f2c4;
// 0x59f418 — __ZN3RBX13JointInstanceC2EPNS_5JointE
#[doc(alias = "RBX::JointInstance::JointInstance(RBX::Joint *)")]
pub use crate::instance::stub_0x59f418 as stub_59f418;
// 0x59f6bc — __ZN3RBX13JointInstanceD0Ev
#[doc(alias = "RBX::JointInstance::~JointInstance()")]
pub use crate::instance::stub_0x59f6bc as stub_59f6bc;
// 0x59f75c — __ZN3RBX13JointInstanceD1Ev
#[doc(alias = "RBX::JointInstance::~JointInstance()")]
pub use crate::instance::stub_0x59f75c as stub_59f75c;
// 0x59f760 — __ZThn32_N3RBX13JointInstanceD0Ev
#[doc(alias = "non-virtual thunk toRBX::JointInstance::~JointInstance()")]
pub use crate::instance::stub_0x59f760 as stub_59f760;
// 0x59f768 — __ZThn36_N3RBX13JointInstanceD0Ev
#[doc(alias = "non-virtual thunk toRBX::JointInstance::~JointInstance()")]
pub use crate::instance::stub_0x59f768 as stub_59f768;
// 0x59f770 — __ZN3RBX13JointInstanceD2Ev
#[doc(alias = "RBX::JointInstance::~JointInstance()")]
pub use crate::instance::stub_0x59f770 as stub_59f770;
// 0x59fa24 — __ZThn32_N3RBX13JointInstanceD1Ev
#[doc(alias = "non-virtual thunk toRBX::JointInstance::~JointInstance()")]
pub use crate::instance::stub_0x59fa24 as stub_59fa24;
// 0x59fa2c — __ZThn36_N3RBX13JointInstanceD1Ev
#[doc(alias = "non-virtual thunk toRBX::JointInstance::~JointInstance()")]
pub use crate::instance::stub_0x59fa2c as stub_59fa2c;
// 0x59fa90 — __ZN3RBX13JointInstance8getPart0Ev
#[doc(alias = "RBX::JointInstance::getPart0(void)")]
pub use crate::instance::stub_0x59fa90 as stub_59fa90;
// 0x59fab4 — __ZN3RBX13JointInstance8getPart1Ev
#[doc(alias = "RBX::JointInstance::getPart1(void)")]
pub use crate::instance::stub_0x59fab4 as stub_59fab4;
// 0x59fad8 — __ZNK3RBX13JointInstance19shouldRender3dAdornEv
#[doc(alias = "RBX::JointInstance::shouldRender3dAdorn(void)const")]
pub use crate::instance::stub_0x59fad8 as stub_59fad8;
// 0x59fae8 — __ZThn92_NK3RBX13JointInstance19shouldRender3dAdornEv
#[doc(alias = "non-virtual thunk toRBX::JointInstance::shouldRender3dAdorn(void)const")]
pub use crate::instance::stub_0x59fae8 as stub_59fae8;
// 0x59faf8 — __ZN3RBX13JointInstance13render3dAdornEPNS_5AdornE
#[doc(alias = "RBX::JointInstance::render3dAdorn(RBX::Adorn *)")]
pub use crate::instance::stub_0x59faf8 as stub_59faf8;
// 0x59fc7c — __ZThn92_N3RBX13JointInstance13render3dAdornEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::JointInstance::render3dAdorn(RBX::Adorn *)")]
pub use crate::instance::stub_0x59fc7c as stub_59fc7c;
// 0x59fc84 — __ZN3RBX13JointInstance7setPartEiPNS_12PartInstanceE
#[doc(alias = "RBX::JointInstance::setPart(int,RBX::PartInstance *)")]
pub use crate::instance::stub_0x59fc84 as stub_59fc84;
// 0x59fe28 — __ZN3RBX13JointInstance12computeWorldEv
#[doc(alias = "RBX::JointInstance::computeWorld(void)")]
pub use crate::instance::stub_0x59fe28 as stub_59fe28;
// 0x59fe6c — __ZN3RBX13JointInstance17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "RBX::JointInstance::onAncestorChanged(RBX::AncestorChanged const&)")]
pub use crate::instance::stub_0x59fe6c as stub_59fe6c;
// 0x5a0020 — __ZN3RBX13JointInstance7setNameERKSs
#[doc(alias = "RBX::JointInstance::setName(std::string const&)")]
pub use crate::instance::stub_0x5a0020 as stub_5a0020;
// 0x5a0aa4 — __ZN3RBX26ManualSurfaceJointInstanceC2EPNS_5JointE
#[doc(alias = "RBX::ManualSurfaceJointInstance::ManualSurfaceJointInstance(RBX::Joint *)")]
pub use crate::instance::stub_0x5a0aa4 as stub_5a0aa4;
// 0x5a0be4 — __ZN3RBX26ManualSurfaceJointInstanceC2Ev
#[doc(alias = "RBX::ManualSurfaceJointInstance::ManualSurfaceJointInstance(void)")]
pub use crate::instance::stub_0x5a0be4 as stub_5a0be4;
// 0x5a0d24 — __ZN3RBX26ManualSurfaceJointInstance13render3dAdornEPNS_5AdornE
#[doc(alias = "RBX::ManualSurfaceJointInstance::render3dAdorn(RBX::Adorn *)")]
pub use crate::instance::stub_0x5a0d24 as stub_5a0d24;
// 0x5a0d28 — __ZThn92_N3RBX26ManualSurfaceJointInstance13render3dAdornEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::ManualSurfaceJointInstance::render3dAdorn(RBX::Adorn *)")]
pub use crate::instance::stub_0x5a0d28 as stub_5a0d28;
// 0x5a31ac — __ZN3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::~RefPropDescriptor()")]
pub use crate::instance::stub_0x5a31ac as stub_5a31ac;
// 0x5a31d8 — __ZN3RBX10Reflection14PropDescriptorINS_26ManualSurfaceJointInstanceEiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ManualSurfaceJointInstance,int>::~PropDescriptor()")]
pub use crate::instance::stub_0x5a31d8 as stub_5a31d8;
// 0x5a31fc — __ZN3RBX10Reflection14PropDescriptorINS_13JointInstanceEN3G3D15CoordinateFrameEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,G3D::CoordinateFrame>::~PropDescriptor()")]
pub use crate::instance::stub_0x5a31fc as stub_5a31fc;
// 0x5a381c — __ZN3RBX26ManualSurfaceJointInstanceD1Ev
#[doc(alias = "RBX::ManualSurfaceJointInstance::~ManualSurfaceJointInstance()")]
pub use crate::instance::stub_0x5a381c as stub_5a381c;
// 0x5a3820 — __ZN3RBX26ManualSurfaceJointInstanceD0Ev
#[doc(alias = "RBX::ManualSurfaceJointInstance::~ManualSurfaceJointInstance()")]
pub use crate::instance::stub_0x5a3820 as stub_5a3820;
// 0x5a38d0 — __ZThn32_N3RBX26ManualSurfaceJointInstanceD1Ev
#[doc(alias = "non-virtual thunk toRBX::ManualSurfaceJointInstance::~ManualSurfaceJointInstance()")]
pub use crate::instance::stub_0x5a38d0 as stub_5a38d0;
// 0x5a38d8 — __ZThn32_N3RBX26ManualSurfaceJointInstanceD0Ev
#[doc(alias = "non-virtual thunk toRBX::ManualSurfaceJointInstance::~ManualSurfaceJointInstance()")]
pub use crate::instance::stub_0x5a38d8 as stub_5a38d8;
// 0x5a398c — __ZThn36_N3RBX26ManualSurfaceJointInstanceD1Ev
#[doc(alias = "non-virtual thunk toRBX::ManualSurfaceJointInstance::~ManualSurfaceJointInstance()")]
pub use crate::instance::stub_0x5a398c as stub_5a398c;
// 0x5a3994 — __ZThn36_N3RBX26ManualSurfaceJointInstanceD0Ev
#[doc(alias = "non-virtual thunk toRBX::ManualSurfaceJointInstance::~ManualSurfaceJointInstance()")]
pub use crate::instance::stub_0x5a3994 as stub_5a3994;
// 0x5a6df8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_26ManualSurfaceJointInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ManualSurfaceJointInstance,RBX::ManualSurfaceJointInstance>(rbx_core::SharedPtr<RBX::ManualSurfaceJointInstance> const*,RBX::ManualSurfaceJointInstance *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ManualSurfaceJointInstance,RBX::ManualSurfaceJointInstance>(boost::shared_ptr<RBX::ManualSurfaceJointInstance> const*,RBX::ManualSurfaceJointInstance *)const
pub use crate::instance::stub_0x5a6df8 as stub_5a6df8;
// 0x5a8e4c — __ZN3RBX13JointInstanceC2Ev
#[doc(alias = "RBX::JointInstance::JointInstance(void)")]
pub use crate::instance::stub_0x5a8e4c as stub_5a8e4c;
// 0x5aa50c — __ZN3RBX10Reflection14PropDescriptorINS_13JointInstanceEN3G3D15CoordinateFrameEEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,G3D::CoordinateFrame>::PropDescriptor<G3D::CoordinateFrame const& (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(G3D::CoordinateFrame const&)>(char const*,char const*,G3D::CoordinateFrame const& (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub use crate::instance::stub_0x5aa50c as stub_5aa50c;
// 0x5aa620 — __ZN3RBX10Reflection14PropDescriptorINS_13JointInstanceEN3G3D15CoordinateFrameEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,G3D::CoordinateFrame>::~PropDescriptor()")]
pub use crate::instance::stub_0x5aa620 as stub_5aa620;
// 0x5aa64c — __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(G3D::CoordinateFrame const&)>::isReadOnly(void)const")]
pub use crate::instance::stub_0x5aa64c as stub_5aa64c;
// 0x5aa650 — __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(G3D::CoordinateFrame const&)>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x5aa650 as stub_5aa650;
// 0x5aa654 — __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(G3D::CoordinateFrame const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x5aa654 as stub_5aa654;
// 0x5aa690 — __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(G3D::CoordinateFrame const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::CoordinateFrame const&)const")]
pub use crate::instance::stub_0x5aa690 as stub_5aa690;
// 0x5aa6b4 — __ZN3RBX10Reflection14PropDescriptorINS_26ManualSurfaceJointInstanceEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ManualSurfaceJointInstance,int>::PropDescriptor<int (RBX::ManualSurfaceJointInstance::*)(void)const,void (RBX::ManualSurfaceJointInstance::*)(int)>(char const*,char const*,int (RBX::ManualSurfaceJointInstance::*)(void)const,void (RBX::ManualSurfaceJointInstance::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub use crate::instance::stub_0x5aa6b4 as stub_5aa6b4;
// 0x5aa7c8 — __ZN3RBX10Reflection14PropDescriptorINS_26ManualSurfaceJointInstanceEiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ManualSurfaceJointInstance,int>::~PropDescriptor()")]
pub use crate::instance::stub_0x5aa7c8 as stub_5aa7c8;
// 0x5aa7f4 — __ZNK3RBX10Reflection14PropDescriptorINS_26ManualSurfaceJointInstanceEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ManualSurfaceJointInstance,int>::GetSetImpl<int (RBX::ManualSurfaceJointInstance::*)(void)const,void (RBX::ManualSurfaceJointInstance::*)(int)>::isReadOnly(void)const")]
pub use crate::instance::stub_0x5aa7f4 as stub_5aa7f4;
// 0x5aa7f8 — __ZNK3RBX10Reflection14PropDescriptorINS_26ManualSurfaceJointInstanceEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ManualSurfaceJointInstance,int>::GetSetImpl<int (RBX::ManualSurfaceJointInstance::*)(void)const,void (RBX::ManualSurfaceJointInstance::*)(int)>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x5aa7f8 as stub_5aa7f8;
// 0x5aa7fc — __ZNK3RBX10Reflection14PropDescriptorINS_26ManualSurfaceJointInstanceEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ManualSurfaceJointInstance,int>::GetSetImpl<int (RBX::ManualSurfaceJointInstance::*)(void)const,void (RBX::ManualSurfaceJointInstance::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x5aa7fc as stub_5aa7fc;
// 0x5aa81c — __ZNK3RBX10Reflection14PropDescriptorINS_26ManualSurfaceJointInstanceEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ManualSurfaceJointInstance,int>::GetSetImpl<int (RBX::ManualSurfaceJointInstance::*)(void)const,void (RBX::ManualSurfaceJointInstance::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub use crate::instance::stub_0x5aa81c as stub_5aa81c;
// 0x5aa840 — __ZN3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub use crate::instance::stub_0x5aa840 as stub_5aa840;
// 0x5aa8e4 — __ZN3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::~RefPropDescriptor()")]
pub use crate::instance::stub_0x5aa8e4 as stub_5aa8e4;
// 0x5aa914 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::isReadOnly(void)const")]
pub use crate::instance::stub_0x5aa914 as stub_5aa914;
// 0x5aa924 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x5aa924 as stub_5aa924;
// 0x5aa934 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x5aa934 as stub_5aa934;
// 0x5aa95c — __ZNK3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub use crate::instance::stub_0x5aa95c as stub_5aa95c;
// 0x5aaa74 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub use crate::instance::stub_0x5aaa74 as stub_5aaa74;
// 0x5aab3c — __ZNK3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub use crate::instance::stub_0x5aab3c as stub_5aab3c;
