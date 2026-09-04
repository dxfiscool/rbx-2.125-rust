// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel (exact RBX:: prefix), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x45db28..0x46068c | total filtered 9903, remaining 2496->2396 after batch (2193 before includes high-EA 248)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias
// Shard: 60 EA-sorted ascending gap after 0x45dafc (shard 58) / 0x45db28 start, distinct from shard 59 high-EA 0xf60c84..0xf628f4

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
// 0x45db28 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
pub use crate::instance::stub_0x45db28 as stub_45db28;
// 0x45db2c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x45db2c as stub_45db2c;
// 0x45db30 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45db30 as stub_45db30;
// 0x45db58 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub use crate::instance::stub_0x45db58 as stub_45db58;
// 0x45dc78 — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::~EventDesc()")]
pub use crate::instance::stub_0x45dc78 as stub_45dc78;
// 0x45dd2c — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub use crate::instance::stub_0x45dd2c as stub_45dd2c;
// 0x45df30 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub use crate::instance::stub_0x45df30 as stub_45df30;
// 0x45dfa4 — __ZNK3RBX10Reflection13EventDescBaseINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub use crate::instance::stub_0x45dfa4 as stub_45dfa4;
// 0x45e1f8 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EEC2EMS2_FbS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::BoundFuncDesc(bool (RBX::DataModel::*)(RBX::DataModel::GearType),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub use crate::instance::stub_0x45e1f8 as stub_45e1f8;
// 0x45e370 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub use crate::instance::stub_0x45e370 as stub_45e370;
// 0x45e3a0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x45e3a0 as stub_45e3a0;
// 0x45e474 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub use crate::instance::stub_0x45e474 as stub_45e474;
// 0x45e4b4 — __ZN3RBX10Reflection11Call1HelperINS_9DataModelEMS2_FbNS2_8GearTypeEES3_bE4callEPS2_S5_RNS0_7VariantERKS3_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::DataModel,bool (RBX::DataModel::*)(RBX::DataModel::GearType),RBX::DataModel::GearType,bool>::call(RBX::DataModel*,bool (RBX::DataModel::*)(RBX::DataModel::GearType),RBX::Reflection::Variant &,RBX::DataModel::GearType const&)")]
pub use crate::instance::stub_0x45e4b4 as stub_45e4b4;
// 0x45e4ec — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel8GearTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::DataModel::GearType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearType> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: RBX::DataModel::GearType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearType> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub use crate::instance::stub_0x45e4ec as stub_45e4ec;
// 0x45e67c — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel8GearTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearType &,boost::enable_if<boost::is_enum<RBX::DataModel::GearType>,void>::type *)")]
pub use crate::instance::stub_0x45e67c as stub_45e67c;
// 0x45e6d0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::EnumPropDescriptor<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub use crate::instance::stub_0x45e6d0 as stub_45e6d0;
// 0x45e87c — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::~EnumPropDescriptor()")]
pub use crate::instance::stub_0x45e87c as stub_45e87c;
// 0x45e8a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::isReadOnly(void)const")]
pub use crate::instance::stub_0x45e8a8 as stub_45e8a8;
// 0x45e8b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x45e8b8 as stub_45e8b8;
// 0x45e8c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45e8c8 as stub_45e8c8;
// 0x45e8f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub use crate::instance::stub_0x45e8f0 as stub_45e8f0;
// 0x45e914 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub use crate::instance::stub_0x45e914 as stub_45e914;
// 0x45ea60 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub use crate::instance::stub_0x45ea60 as stub_45ea60;
// 0x45ea88 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::hasStringValue(void)const")]
pub use crate::instance::stub_0x45ea88 as stub_45ea88;
// 0x45ea8c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45ea8c as stub_45ea8c;
// 0x45eab0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub use crate::instance::stub_0x45eab0 as stub_45eab0;
// 0x45eaf0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub use crate::instance::stub_0x45eaf0 as stub_45eaf0;
// 0x45eb10 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub use crate::instance::stub_0x45eb10 as stub_45eb10;
// 0x45ed50 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45ed50 as stub_45ed50;
// 0x45ed6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub use crate::instance::stub_0x45ed6c as stub_45ed6c;
// 0x45eda0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45eda0 as stub_45eda0;
// 0x45eda8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub use crate::instance::stub_0x45eda8 as stub_45eda8;
// 0x45edf4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45edf4 as stub_45edf4;
// 0x45ee14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub use crate::instance::stub_0x45ee14 as stub_45ee14;
// 0x45ee4c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToIndex(RBX::DataModel::GearGenreSetting)const")]
pub use crate::instance::stub_0x45ee4c as stub_45ee4c;
// 0x45eebc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub use crate::instance::stub_0x45eebc as stub_45eebc;
// 0x45ef00 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
pub use crate::instance::stub_0x45ef00 as stub_45ef00;
// 0x45ef04 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x45ef04 as stub_45ef04;
// 0x45ef08 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45ef08 as stub_45ef08;
// 0x45ef28 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::GearGenreSetting const&)const")]
pub use crate::instance::stub_0x45ef28 as stub_45ef28;
// 0x45f048 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::EnumPropDescriptor<RBX::DataModel::Genre (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::Genre (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub use crate::instance::stub_0x45f048 as stub_45f048;
// 0x45f1f4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::~EnumPropDescriptor()")]
pub use crate::instance::stub_0x45f1f4 as stub_45f1f4;
// 0x45f220 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::isReadOnly(void)const")]
pub use crate::instance::stub_0x45f220 as stub_45f220;
// 0x45f230 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x45f230 as stub_45f230;
// 0x45f240 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45f240 as stub_45f240;
// 0x45f268 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub use crate::instance::stub_0x45f268 as stub_45f268;
// 0x45f28c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub use crate::instance::stub_0x45f28c as stub_45f28c;
// 0x45f3d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub use crate::instance::stub_0x45f3d8 as stub_45f3d8;
// 0x45f3fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::hasStringValue(void)const")]
pub use crate::instance::stub_0x45f3fc as stub_45f3fc;
// 0x45f400 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45f400 as stub_45f400;
// 0x45f424 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub use crate::instance::stub_0x45f424 as stub_45f424;
// 0x45f464 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub use crate::instance::stub_0x45f464 as stub_45f464;
// 0x45f484 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub use crate::instance::stub_0x45f484 as stub_45f484;
// 0x45f6c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45f6c4 as stub_45f6c4;
// 0x45f6e0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub use crate::instance::stub_0x45f6e0 as stub_45f6e0;
// 0x45f714 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45f714 as stub_45f714;
// 0x45f71c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub use crate::instance::stub_0x45f71c as stub_45f71c;
// 0x45f768 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45f768 as stub_45f768;
// 0x45f788 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub use crate::instance::stub_0x45f788 as stub_45f788;
// 0x45f7bc — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToIndex(RBX::DataModel::Genre)const")]
pub use crate::instance::stub_0x45f7bc as stub_45f7bc;
// 0x45f82c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub use crate::instance::stub_0x45f82c as stub_45f82c;
// 0x45f86c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
pub use crate::instance::stub_0x45f86c as stub_45f86c;
// 0x45f870 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x45f870 as stub_45f870;
// 0x45f874 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45f874 as stub_45f874;
// 0x45f894 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::Genre const&)const")]
pub use crate::instance::stub_0x45f894 as stub_45f894;
// 0x45f9b4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::EnumPropDescriptor<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub use crate::instance::stub_0x45f9b4 as stub_45f9b4;
// 0x45fb60 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::~EnumPropDescriptor()")]
pub use crate::instance::stub_0x45fb60 as stub_45fb60;
// 0x45fb8c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::isReadOnly(void)const")]
pub use crate::instance::stub_0x45fb8c as stub_45fb8c;
// 0x45fb9c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x45fb9c as stub_45fb9c;
// 0x45fbac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45fbac as stub_45fbac;
// 0x45fbd4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub use crate::instance::stub_0x45fbd4 as stub_45fbd4;
// 0x45fbf8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub use crate::instance::stub_0x45fbf8 as stub_45fbf8;
// 0x45fd44 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub use crate::instance::stub_0x45fd44 as stub_45fd44;
// 0x45fd68 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::hasStringValue(void)const")]
pub use crate::instance::stub_0x45fd68 as stub_45fd68;
// 0x45fd6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x45fd6c as stub_45fd6c;
// 0x45fd90 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub use crate::instance::stub_0x45fd90 as stub_45fd90;
// 0x45fdd0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub use crate::instance::stub_0x45fdd0 as stub_45fdd0;
// 0x45fdf0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub use crate::instance::stub_0x45fdf0 as stub_45fdf0;
// 0x460030 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x460030 as stub_460030;
// 0x46004c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub use crate::instance::stub_0x46004c as stub_46004c;
// 0x460080 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x460080 as stub_460080;
// 0x460088 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub use crate::instance::stub_0x460088 as stub_460088;
// 0x4600d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x4600d4 as stub_4600d4;
// 0x4600f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub use crate::instance::stub_0x4600f4 as stub_4600f4;
// 0x460128 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToIndex(RBX::DataModel::CreatorType)const")]
pub use crate::instance::stub_0x460128 as stub_460128;
// 0x460198 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub use crate::instance::stub_0x460198 as stub_460198;
// 0x4601d8 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
pub use crate::instance::stub_0x4601d8 as stub_4601d8;
// 0x4601dc — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x4601dc as stub_4601dc;
// 0x4601e0 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x4601e0 as stub_4601e0;
// 0x460200 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::CreatorType const&)const")]
pub use crate::instance::stub_0x460200 as stub_460200;
// 0x460320 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::PropDescriptor<int (RBX::DataModel::*)(void)const,int>(char const*,char const*,int (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub use crate::instance::stub_0x460320 as stub_460320;
// 0x460430 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::~PropDescriptor()")]
pub use crate::instance::stub_0x460430 as stub_460430;
// 0x460460 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
pub use crate::instance::stub_0x460460 as stub_460460;
// 0x460464 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x460464 as stub_460464;
// 0x460468 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x460468 as stub_460468;
// 0x460488 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub use crate::instance::stub_0x460488 as stub_460488;
// 0x4605a8 — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::Instance* (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub use crate::instance::stub_0x4605a8 as stub_4605a8;
// 0x46064c — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::~RefPropDescriptor()")]
pub use crate::instance::stub_0x46064c as stub_46064c;
// 0x46067c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::isReadOnly(void)const")]
pub use crate::instance::stub_0x46067c as stub_46067c;
// 0x46068c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x46068c as stub_46068c;
