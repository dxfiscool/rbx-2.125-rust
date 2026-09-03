// Auto-generated shard DJ — next 100 RBX::Reflection stubs — EA-sorted asc 0x45eda0..0x489720 (remaining 54 after)
// Source: ida/export.json filtered demangled contains RBX::Reflection (16171 total, 16017->16117 covered, 54 remaining)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr (was boost::shared_ptr)
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;

// 0x45eda0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_45eda0() -> ! {
    todo!("0x45eda0 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x45eda8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_45eda8() -> ! {
    todo!("0x45eda8 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x45edf4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_45edf4() -> ! {
    todo!("0x45edf4 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x45ee14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_45ee14() -> ! {
    todo!("0x45ee14 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x45ee4c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToIndex(RBX::DataModel::GearGenreSetting)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToIndexES3_")]
pub fn stub_45ee4c(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x45ee4c: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x45eebc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_45eebc() -> ! {
    todo!("0x45eebc RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x45ef00 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv")]
pub fn stub_45ef00() -> ! {
    todo!("0x45ef00 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::isReadOnly(void)const")
}

// 0x45ef04 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv")]
pub fn stub_45ef04() -> ! {
    todo!("0x45ef04 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")
}

// 0x45ef08 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_45ef08() -> ! {
    todo!("0x45ef08 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x45ef28 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::GearGenreSetting const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_45ef28() -> ! {
    todo!("0x45ef28 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::GearGenreSetting const&)const")
}

// 0x45f048 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::EnumPropDescriptor<RBX::DataModel::Genre (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::Genre (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_45f048() -> ! {
    todo!("0x45f048 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::EnumPropDescriptor<RBX::DataModel::Genre (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::Genre (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x45f1f4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEED0Ev")]
pub fn stub_45f1f4() {
    // IDA 0x45f1f4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x45f220 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10isReadOnlyEv")]
pub fn stub_45f220() -> ! {
    todo!("0x45f220 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::isReadOnly(void)const")
}

// 0x45f230 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11isWriteOnlyEv")]
pub fn stub_45f230() -> ! {
    todo!("0x45f230 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::isWriteOnly(void)const")
}

// 0x45f240 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_45f240() -> ! {
    todo!("0x45f240 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x45f268 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_45f268() -> ! {
    todo!("0x45f268 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x45f28c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_45f28c() -> ! {
    todo!("0x45f28c RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x45f3d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_45f3d8() -> ! {
    todo!("0x45f3d8 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x45f3fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14hasStringValueEv")]
pub fn stub_45f3fc() -> ! {
    todo!("0x45f3fc RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::hasStringValue(void)const")
}

// 0x45f400 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_45f400() -> ! {
    todo!("0x45f400 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x45f424 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_45f424() -> ! {
    todo!("0x45f424 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x45f464 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_45f464() -> ! {
    todo!("0x45f464 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x45f484 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_45f484() -> ! {
    todo!("0x45f484 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x45f6c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_45f6c4() -> ! {
    todo!("0x45f6c4 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x45f6e0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_45f6e0() -> ! {
    todo!("0x45f6e0 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x45f714 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_45f714() -> ! {
    todo!("0x45f714 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x45f71c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_45f71c() -> ! {
    todo!("0x45f71c RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x45f768 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_45f768() -> ! {
    todo!("0x45f768 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x45f788 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_45f788() -> ! {
    todo!("0x45f788 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x45f7bc — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToIndex(RBX::DataModel::Genre)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToIndexES3_")]
pub fn stub_45f7bc(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x45f7bc: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x45f82c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_45f82c() -> ! {
    todo!("0x45f82c RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x45f86c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv")]
pub fn stub_45f86c() -> ! {
    todo!("0x45f86c RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::isReadOnly(void)const")
}

// 0x45f870 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv")]
pub fn stub_45f870() -> ! {
    todo!("0x45f870 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")
}

// 0x45f874 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_45f874() -> ! {
    todo!("0x45f874 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x45f894 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::Genre const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_45f894() -> ! {
    todo!("0x45f894 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::Genre const&)const")
}

// 0x45f9b4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::EnumPropDescriptor<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_45f9b4() -> ! {
    todo!("0x45f9b4 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::EnumPropDescriptor<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x45fb60 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEED0Ev")]
pub fn stub_45fb60() {
    // IDA 0x45fb60: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x45fb8c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10isReadOnlyEv")]
pub fn stub_45fb8c() -> ! {
    todo!("0x45fb8c RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::isReadOnly(void)const")
}

// 0x45fb9c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11isWriteOnlyEv")]
pub fn stub_45fb9c() -> ! {
    todo!("0x45fb9c RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::isWriteOnly(void)const")
}

// 0x45fbac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_45fbac() -> ! {
    todo!("0x45fbac RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x45fbd4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_45fbd4() -> ! {
    todo!("0x45fbd4 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x45fbf8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_45fbf8() -> ! {
    todo!("0x45fbf8 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x45fd44 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_45fd44() -> ! {
    todo!("0x45fd44 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x45fd68 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14hasStringValueEv")]
pub fn stub_45fd68() -> ! {
    todo!("0x45fd68 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::hasStringValue(void)const")
}

// 0x45fd6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_45fd6c() -> ! {
    todo!("0x45fd6c RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x45fd90 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_45fd90() -> ! {
    todo!("0x45fd90 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x45fdd0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_45fdd0() -> ! {
    todo!("0x45fdd0 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x45fdf0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_45fdf0() -> ! {
    todo!("0x45fdf0 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x460030 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_460030() -> ! {
    todo!("0x460030 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x46004c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_46004c() -> ! {
    todo!("0x46004c RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x460080 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_460080() -> ! {
    todo!("0x460080 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x460088 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_460088() -> ! {
    todo!("0x460088 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x4600d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_4600d4() -> ! {
    todo!("0x4600d4 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x4600f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_4600f4() -> ! {
    todo!("0x4600f4 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x460128 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToIndex(RBX::DataModel::CreatorType)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToIndexES3_")]
pub fn stub_460128(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x460128: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x460198 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_460198() -> ! {
    todo!("0x460198 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x4601d8 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv")]
pub fn stub_4601d8() -> ! {
    todo!("0x4601d8 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isReadOnly(void)const")
}

// 0x4601dc — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv")]
pub fn stub_4601dc() -> ! {
    todo!("0x4601dc RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")
}

// 0x4601e0 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_4601e0() -> ! {
    todo!("0x4601e0 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x460200 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::CreatorType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_460200() -> ! {
    todo!("0x460200 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::CreatorType const&)const")
}

// 0x460320 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::PropDescriptor<int (RBX::DataModel::*)(void)const,int>(char const*,char const*,int (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_460320() -> ! {
    todo!("0x460320 RBX::Reflection::PropDescriptor<RBX::DataModel,int>::PropDescriptor<int (RBX::DataModel::*)(void)const,int>(char const*,char const*,int (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x460460 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE10isReadOnlyEv")]
pub fn stub_460460() -> ! {
    todo!("0x460460 RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isReadOnly(void)const")
}

// 0x460464 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE11isWriteOnlyEv")]
pub fn stub_460464() -> ! {
    todo!("0x460464 RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")
}

// 0x48867c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::setValue(RBX::Reflection::DescribedBase *,RBX::TaskScheduler::ThreadPoolConfig const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_48867c() -> ! {
    todo!("0x48867c RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::setValue(RBX::Reflection::DescribedBase *,RBX::TaskScheduler::ThreadPoolConfig const&)const")
}

// 0x4886a0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE13initSingletonEv")]
pub fn stub_4886a0() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0x4886a0: Singleton<EnumDesc<T>>::initSingleton -- thunk to doGetSingleton (decompiled 0x4a60b8). Rust: forward to the singleton.
    crate::generated::stub_0x4886a4()
}

// 0x4886a4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv")]
pub fn stub_4886a4() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0x4886a4: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x47a87c)
}

// 0x488794 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::BoundFuncDesc(void (RBX::DebugSettings::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_488794() -> ! {
    todo!("0x488794 RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::BoundFuncDesc(void (RBX::DebugSettings::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x48890c — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_48890c() -> ! {
    todo!("0x48890c RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x488a10 — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_488a10() -> ! {
    todo!("0x488a10 RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x488a48 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::BoundFuncDesc(void (RBX::DebugSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_488a48() -> ! {
    todo!("0x488a48 RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::BoundFuncDesc(void (RBX::DebugSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x488c00 — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_488c00() -> ! {
    todo!("0x488c00 RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x488c20 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::EnumPropDescriptor<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>(char const*,char const*,RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_488c20() -> ! {
    todo!("0x488c20 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::EnumPropDescriptor<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>(char const*,char const*,RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x488dd4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEED0Ev")]
pub fn stub_488dd4() {
    // IDA 0x488dd4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x488e00 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10isReadOnlyEv")]
pub fn stub_488e00() -> ! {
    todo!("0x488e00 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::isReadOnly(void)const")
}

// 0x488e10 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11isWriteOnlyEv")]
pub fn stub_488e10() -> ! {
    todo!("0x488e10 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::isWriteOnly(void)const")
}

// 0x488e20 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_488e20() -> ! {
    todo!("0x488e20 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x488e48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_488e48() -> ! {
    todo!("0x488e48 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x488e6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_488e6c() -> ! {
    todo!("0x488e6c RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x488fb8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_488fb8() -> ! {
    todo!("0x488fb8 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x488fdc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14hasStringValueEv")]
pub fn stub_488fdc() -> ! {
    todo!("0x488fdc RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::hasStringValue(void)const")
}

// 0x488fe0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_488fe0() -> ! {
    todo!("0x488fe0 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x489004 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_489004() -> ! {
    todo!("0x489004 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x489044 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_489044() -> ! {
    todo!("0x489044 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x489064 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_489064() -> ! {
    todo!("0x489064 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x4892a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_4892a4() -> ! {
    todo!("0x4892a4 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4892c0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_4892c0() -> ! {
    todo!("0x4892c0 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x4892f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_4892f4() -> ! {
    todo!("0x4892f4 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4892fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_4892fc() -> ! {
    todo!("0x4892fc RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x489348 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_489348() -> ! {
    todo!("0x489348 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x489368 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_489368() -> ! {
    todo!("0x489368 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x48939c — __ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToIndex(RBX::DebugSettings::ErrorReporting)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_")]
pub fn stub_48939c(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x48939c: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x48940c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_48940c() -> ! {
    todo!("0x48940c RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x48944c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
pub fn stub_48944c() -> ! {
    todo!("0x48944c RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::isReadOnly(void)const")
}

// 0x489450 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_489450() -> ! {
    todo!("0x489450 RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::isWriteOnly(void)const")
}

// 0x489454 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_489454() -> ! {
    todo!("0x489454 RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x489474 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::setValue(RBX::Reflection::DescribedBase *,RBX::DebugSettings::ErrorReporting const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_489474() -> ! {
    todo!("0x489474 RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::setValue(RBX::Reflection::DescribedBase *,RBX::DebugSettings::ErrorReporting const&)const")
}

// 0x489498 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE13initSingletonEv")]
pub fn stub_489498() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0x489498: Singleton<EnumDesc<T>>::initSingleton -- thunk to doGetSingleton (decompiled 0x4a60b8). Rust: forward to the singleton.
    crate::generated::stub_0x48949c()
}

// 0x48949c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv")]
pub fn stub_48949c() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0x48949c: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x47aee0)
}

// 0x48958c — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13DebugSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::DebugSettings>(char const*,char const*,bool RBX::DebugSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13DebugSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_48958c() -> ! {
    todo!("0x48958c RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::DebugSettings>(char const*,char const*,bool RBX::DebugSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x489720 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE10isReadOnlyEv")]
pub fn stub_489720() -> ! {
    todo!("0x489720 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::isReadOnly(void)const")
}
