//! core wd_10i — 120 core stubs EA-sorted asc gap filler not yet in crates/core/src (global EA asc, next uncovered after 0x45fd6c).
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 120 not yet in crates/core/src (existing 25367 distinct, uncovered 60179 -> 60059 after batch).
//! Range: 0x45fd90..0x464230 | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// 0x45fd90 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_0x45fd90() -> ! {
    todo!("0x45fd90 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// 0x45fdd0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_0x45fdd0() -> ! {
    todo!("0x45fdd0 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// 0x45fdf0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
pub fn stub_0x45fdf0() -> ! {
    todo!("0x45fdf0 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// 0x460030 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13getIndexValueEPKNS0_13DescribedBaseE
pub fn stub_0x460030() -> ! {
    todo!("0x460030 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// 0x46004c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13setIndexValueEPNS0_13DescribedBaseEm
pub fn stub_0x46004c() -> ! {
    todo!("0x46004c RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// 0x460080 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12getEnumValueEPKNS0_13DescribedBaseE
pub fn stub_0x460080() -> ! {
    todo!("0x460080 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// 0x460088 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12setEnumValueEPNS0_13DescribedBaseEi
pub fn stub_0x460088() -> ! {
    todo!("0x460088 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// 0x4600d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11getEnumItemEPKNS0_13DescribedBaseE
pub fn stub_0x4600d4() -> ! {
    todo!("0x4600d4 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// 0x4600f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
pub fn stub_0x4600f4() -> ! {
    todo!("0x4600f4 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToIndex(RBX::DataModel::CreatorType)const")]
// 0x460128 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToIndexES3_
// type: int(void)
pub fn stub_0x460128() -> ! {
    todo!("0x460128 RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToIndex(RBX::DataModel::CreatorType)const")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// 0x460198 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
pub fn stub_0x460198() -> ! {
    todo!("0x460198 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// 0x4601d8 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
pub fn stub_0x4601d8() -> ! {
    todo!("0x4601d8 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isReadOnly(void)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// 0x4601dc — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
pub fn stub_0x4601dc() -> ! {
    todo!("0x4601dc RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x4601e0 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x4601e0() -> ! {
    todo!("0x4601e0 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::CreatorType const&)const")]
// 0x460200 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_0x460200() -> ! {
    todo!("0x460200 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::CreatorType const&)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::PropDescriptor<int (RBX::DataModel::*)(void)const,int>(char const*,char const*,int (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x460320 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0x460320() -> ! {
    todo!("0x460320 RBX::Reflection::PropDescriptor<RBX::DataModel,int>::PropDescriptor<int (RBX::DataModel::*)(void)const,int>(char const*,char const*,int (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::~PropDescriptor()")]
// 0x460430 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiED0Ev
pub fn stub_0x460430() -> ! {
    todo!("0x460430 RBX::Reflection::PropDescriptor<RBX::DataModel,int>::~PropDescriptor()")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// 0x460460 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE10isReadOnlyEv
pub fn stub_0x460460() -> ! {
    todo!("0x460460 RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isReadOnly(void)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// 0x460464 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
pub fn stub_0x460464() -> ! {
    todo!("0x460464 RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x460468 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x460468() -> ! {
    todo!("0x460468 RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// 0x460488 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
pub fn stub_0x460488() -> ! {
    todo!("0x460488 RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::Instance* (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x4605a8 — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_0x4605a8() -> ! {
    todo!("0x4605a8 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::Instance* (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::~RefPropDescriptor()")]
// 0x46064c — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEED0Ev
pub fn stub_0x46064c() -> ! {
    todo!("0x46064c RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::~RefPropDescriptor()")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::isReadOnly(void)const")]
// 0x46067c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10isReadOnlyEv
pub fn stub_0x46067c() -> ! {
    todo!("0x46067c RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::isReadOnly(void)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::isWriteOnly(void)const")]
// 0x46068c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11isWriteOnlyEv
pub fn stub_0x46068c() -> ! {
    todo!("0x46068c RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::isWriteOnly(void)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// 0x46069c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
pub fn stub_0x46069c() -> ! {
    todo!("0x46069c RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// 0x4606c4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x4606c4() -> ! {
    todo!("0x4606c4 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// 0x4607dc — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_0x4607dc() -> ! {
    todo!("0x4607dc RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// 0x4608a4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_0x4608a4() -> ! {
    todo!("0x4608a4 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

#[doc(alias = "RBX::Reflection::RefPropertyDescriptor::getDataSize(RBX::Reflection::DescribedBase const*)const")]
// 0x4608c8 — __ZNK3RBX10Reflection21RefPropertyDescriptor11getDataSizeEPKNS0_13DescribedBaseE
// type: _DWORD __fastcall(RBX::Reflection::RefPropertyDescriptor *__hidden this, const DescribedBase *)
pub fn stub_0x4608c8() -> ! {
    todo!("0x4608c8 RBX::Reflection::RefPropertyDescriptor::getDataSize(RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::RefPropertyDescriptor::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// 0x4608d0 — __ZNK3RBX10Reflection21RefPropertyDescriptor14getStringValueEPKNS0_13DescribedBaseE
// type: _DWORD __fastcall(RBX::Reflection::RefPropertyDescriptor *__hidden this, const DescribedBase *)
pub fn stub_0x4608d0() -> ! {
    todo!("0x4608d0 RBX::Reflection::RefPropertyDescriptor::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::RefPropertyDescriptor::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// 0x4608f8 — __ZNK3RBX10Reflection21RefPropertyDescriptor14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_0x4608f8() -> ! {
    todo!("0x4608f8 RBX::Reflection::RefPropertyDescriptor::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// 0x460908 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_0x460908() -> ! {
    todo!("0x460908 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// 0x4609dc — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_0x4609dc() -> ! {
    todo!("0x4609dc RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// 0x460a00 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
pub fn stub_0x460a00() -> ! {
    todo!("0x460a00 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x460a14 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
pub fn stub_0x460a14() -> ! {
    todo!("0x460a14 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x460a90 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
pub fn stub_0x460a90() -> ! {
    todo!("0x460a90 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x460ab0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0x460ab0() -> ! {
    todo!("0x460ab0 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x460b90 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
pub fn stub_0x460b90() -> ! {
    todo!("0x460b90 non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// 0x460b98 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
pub fn stub_0x460b98() -> ! {
    todo!("0x460b98 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::isReadOnly(void)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// 0x460b9c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
pub fn stub_0x460b9c() -> ! {
    todo!("0x460b9c RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x460ba0 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x460ba0() -> ! {
    todo!("0x460ba0 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")]
// 0x460bc0 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
pub fn stub_0x460bc0() -> ! {
    todo!("0x460bc0 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")
}

#[doc(alias = "RBX::Reflection::RefType<RBX::Instance *>::~RefType()")]
// 0x460ce0 — __ZN3RBX10Reflection7RefTypeIPNS_8InstanceEED1Ev
pub fn stub_0x460ce0() -> ! {
    todo!("0x460ce0 RBX::Reflection::RefType<RBX::Instance *>::~RefType()")
}

#[doc(alias = "RBX::Reflection::RefType<RBX::Instance *>::~RefType()")]
// 0x460ce8 — __ZN3RBX10Reflection7RefTypeIPNS_8InstanceEED0Ev
pub fn stub_0x460ce8() -> ! {
    todo!("0x460ce8 RBX::Reflection::RefType<RBX::Instance *>::~RefType()")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::RefPropDescriptor<RBX::Workspace* (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::Workspace* (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x460cec — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_0x460cec() -> ! {
    todo!("0x460cec RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::RefPropDescriptor<RBX::Workspace* (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::Workspace* (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

#[doc(alias = "RBX::Reflection::RefType<RBX::Workspace *>::singleton(void)")]
// 0x460d90 — __ZN3RBX10Reflection7RefTypeIPNS_9WorkspaceEE9singletonEv
pub fn stub_0x460d90() -> ! {
    todo!("0x460d90 RBX::Reflection::RefType<RBX::Workspace *>::singleton(void)")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::~RefPropDescriptor()")]
// 0x460e88 — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEED0Ev
pub fn stub_0x460e88() -> ! {
    todo!("0x460e88 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::~RefPropDescriptor()")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::isReadOnly(void)const")]
// 0x460eb8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10isReadOnlyEv
pub fn stub_0x460eb8() -> ! {
    todo!("0x460eb8 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::isReadOnly(void)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::isWriteOnly(void)const")]
// 0x460ec8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11isWriteOnlyEv
pub fn stub_0x460ec8() -> ! {
    todo!("0x460ec8 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::isWriteOnly(void)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// 0x460ed8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11equalValuesEPKNS0_13DescribedBaseES7_
pub fn stub_0x460ed8() -> ! {
    todo!("0x460ed8 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// 0x460f00 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x460f00() -> ! {
    todo!("0x460f00 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// 0x461018 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_0x461018() -> ! {
    todo!("0x461018 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// 0x4610e0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_0x4610e0() -> ! {
    todo!("0x4610e0 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// 0x461104 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_0x461104() -> ! {
    todo!("0x461104 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// 0x4611d8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_0x4611d8() -> ! {
    todo!("0x4611d8 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// 0x4611fc — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11getRefValueEPKNS0_13DescribedBaseE
pub fn stub_0x4611fc() -> ! {
    todo!("0x4611fc RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::getRefValue(RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x461210 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
pub fn stub_0x461210() -> ! {
    todo!("0x461210 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x46128c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
pub fn stub_0x46128c() -> ! {
    todo!("0x46128c RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x4612ac — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0x4612ac() -> ! {
    todo!("0x4612ac RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x46138c — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
pub fn stub_0x46138c() -> ! {
    todo!("0x46138c non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// 0x461394 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
pub fn stub_0x461394() -> ! {
    todo!("0x461394 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::isReadOnly(void)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// 0x461398 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
pub fn stub_0x461398() -> ! {
    todo!("0x461398 RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x46139c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x46139c() -> ! {
    todo!("0x46139c RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Workspace * const&)const")]
// 0x4613bc — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
pub fn stub_0x4613bc() -> ! {
    todo!("0x4613bc RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Workspace * const&)const")
}

#[doc(alias = "RBX::Reflection::RefType<RBX::Workspace *>::~RefType()")]
// 0x4614dc — __ZN3RBX10Reflection7RefTypeIPNS_9WorkspaceEED1Ev
pub fn stub_0x4614dc() -> ! {
    todo!("0x4614dc RBX::Reflection::RefType<RBX::Workspace *>::~RefType()")
}

#[doc(alias = "RBX::Reflection::Type::Type<RBX::Workspace *>(char const*,char const*,RBX::Workspace * *)")]
// 0x4614e0 — __ZN3RBX10Reflection4TypeC2IPNS_9WorkspaceEEEPKcS6_PT_
// type: int(void)
pub fn stub_0x4614e0() -> ! {
    todo!("0x4614e0 RBX::Reflection::Type::Type<RBX::Workspace *>(char const*,char const*,RBX::Workspace * *)")
}

#[doc(alias = "RBX::Reflection::RefType<RBX::Workspace *>::~RefType()")]
// 0x46158c — __ZN3RBX10Reflection7RefTypeIPNS_9WorkspaceEED0Ev
pub fn stub_0x46158c() -> ! {
    todo!("0x46158c RBX::Reflection::RefType<RBX::Workspace *>::~RefType()")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::BoundFuncDesc(void (RBX::DataModel::*)(RBX::DataModel::GearGenreSetting,int),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x461590 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EEC2EMS2_FvS3_iEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_0x461590() -> ! {
    todo!("0x461590 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::BoundFuncDesc(void (RBX::DataModel::*)(RBX::DataModel::GearGenreSetting,int),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x461758 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_
pub fn stub_0x461758() -> ! {
    todo!("0x461758 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::~BoundFuncDesc()")]
// 0x4617a4 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EED0Ev
pub fn stub_0x4617a4() -> ! {
    todo!("0x4617a4 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::~BoundFuncDesc()")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x461884 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_0x461884() -> ! {
    todo!("0x461884 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

#[doc(alias = "RBX::DataModel::GearGenreSetting RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearGenreSetting,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearGenreSetting> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearGenreSetting,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x4618d8 — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel16GearGenreSettingELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int(void)
pub fn stub_0x4618d8() -> ! {
    todo!("0x4618d8 RBX::DataModel::GearGenreSetting RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearGenreSetting,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearGenreSetting> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearGenreSetting,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearGenreSetting>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearGenreSetting &,boost::enable_if<boost::is_enum<RBX::DataModel::GearGenreSetting>,void>::type *)")]
// 0x461a68 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel16GearGenreSettingEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int(void)
pub fn stub_0x461a68() -> ! {
    todo!("0x461a68 bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearGenreSetting>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearGenreSetting &,boost::enable_if<boost::is_enum<RBX::DataModel::GearGenreSetting>,void>::type *)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::BoundFuncDesc(void (RBX::DataModel::*)(RBX::DataModel::Genre),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x461abc — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_0x461abc() -> ! {
    todo!("0x461abc RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::BoundFuncDesc(void (RBX::DataModel::*)(RBX::DataModel::Genre),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x461c34 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
pub fn stub_0x461c34() -> ! {
    todo!("0x461c34 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::~BoundFuncDesc()")]
// 0x461c64 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EED0Ev
pub fn stub_0x461c64() -> ! {
    todo!("0x461c64 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::~BoundFuncDesc()")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x461d38 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_0x461d38() -> ! {
    todo!("0x461d38 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

#[doc(alias = "RBX::DataModel::Genre RBX::Reflection::ArgHelper::getArg<RBX::DataModel::Genre,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::Genre> const&,boost::disable_if<boost::is_same<RBX::DataModel::Genre,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x461d6c — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel5GenreELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int(void)
pub fn stub_0x461d6c() -> ! {
    todo!("0x461d6c RBX::DataModel::Genre RBX::Reflection::ArgHelper::getArg<RBX::DataModel::Genre,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::Genre> const&,boost::disable_if<boost::is_same<RBX::DataModel::Genre,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::Genre>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::Genre &,boost::enable_if<boost::is_enum<RBX::DataModel::Genre>,void>::type *)")]
// 0x461efc — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel5GenreEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int(void)
pub fn stub_0x461efc() -> ! {
    todo!("0x461efc bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::Genre>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::Genre &,boost::enable_if<boost::is_enum<RBX::DataModel::Genre>,void>::type *)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::BoundFuncDesc(void (RBX::DataModel::*)(int,RBX::DataModel::CreatorType),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x461f50 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EEC2EMS2_FviS3_EPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_0x461f50() -> ! {
    todo!("0x461f50 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::BoundFuncDesc(void (RBX::DataModel::*)(int,RBX::DataModel::CreatorType),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x462118 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_
pub fn stub_0x462118() -> ! {
    todo!("0x462118 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::~BoundFuncDesc()")]
// 0x462164 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EED0Ev
pub fn stub_0x462164() -> ! {
    todo!("0x462164 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::~BoundFuncDesc()")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x462244 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_0x462244() -> ! {
    todo!("0x462244 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

#[doc(alias = "RBX::DataModel::CreatorType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::CreatorType,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::CreatorType> const&,boost::disable_if<boost::is_same<RBX::DataModel::CreatorType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x46229c — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel11CreatorTypeELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int(void)
pub fn stub_0x46229c() -> ! {
    todo!("0x46229c RBX::DataModel::CreatorType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::CreatorType,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::CreatorType> const&,boost::disable_if<boost::is_same<RBX::DataModel::CreatorType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<2,RBX::DataModel::CreatorType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::CreatorType &,boost::enable_if<boost::is_enum<RBX::DataModel::CreatorType>,void>::type *)")]
// 0x462430 — __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_9DataModel11CreatorTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int(void)
pub fn stub_0x462430() -> ! {
    todo!("0x462430 bool RBX::Reflection::ArgHelper::try_enum<2,RBX::DataModel::CreatorType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::CreatorType &,boost::enable_if<boost::is_enum<RBX::DataModel::CreatorType>,void>::type *)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::BoundFuncDesc(void (RBX::DataModel::*)(int,bool),char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x462484 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EEC2EMS2_FvibEPKcS8_S8_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_0x462484() -> ! {
    todo!("0x462484 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::BoundFuncDesc(void (RBX::DataModel::*)(int,bool),char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x462680 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
pub fn stub_0x462680() -> ! {
    todo!("0x462680 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::~BoundFuncDesc()")]
// 0x4626cc — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EED0Ev
pub fn stub_0x4626cc() -> ! {
    todo!("0x4626cc RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::~BoundFuncDesc()")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x4627ac — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_0x4627ac() -> ! {
    todo!("0x4627ac RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::getArg<bool,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x462800 — __ZN3RBX10Reflection9ArgHelper6getArgIbLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x462800() -> ! {
    todo!("0x462800 bool RBX::Reflection::ArgHelper::getArg<bool,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::BoundFuncDesc(void (RBX::DataModel::*)(double),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x4629a8 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EEC2EMS2_FvdEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_0x4629a8() -> ! {
    todo!("0x4629a8 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::BoundFuncDesc(void (RBX::DataModel::*)(double),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x462b20 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
pub fn stub_0x462b20() -> ! {
    todo!("0x462b20 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::~BoundFuncDesc()")]
// 0x462b50 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EED0Ev
pub fn stub_0x462b50() -> ! {
    todo!("0x462b50 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::~BoundFuncDesc()")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x462c24 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_0x462c24() -> ! {
    todo!("0x462c24 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::BoundFuncDesc(double (RBX::DataModel::*)(std::string,double),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x462c64 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EEC2EMS2_FdSsdEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_0x462c64() -> ! {
    todo!("0x462c64 RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::BoundFuncDesc(double (RBX::DataModel::*)(std::string,double),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x462e2c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
pub fn stub_0x462e2c() -> ! {
    todo!("0x462e2c RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::~BoundFuncDesc()")]
// 0x462e78 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EED0Ev
pub fn stub_0x462e78() -> ! {
    todo!("0x462e78 RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::~BoundFuncDesc()")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x462f54 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_0x462f54() -> ! {
    todo!("0x462f54 RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DataModel,double (RBX::DataModel::*)(std::string,double),std::string,double,double>::call(RBX::DataModel*,double (RBX::DataModel::*)(std::string,double),RBX::Reflection::Variant &,std::string const&,double const&)")]
// 0x4630b8 — __ZN3RBX10Reflection11Call2HelperINS_9DataModelEMS2_FdSsdESsddE4callEPS2_S4_RNS0_7VariantERKSsRKd
// type: int __fastcall(int, int, int, int, std::string *, int)
pub fn stub_0x4630b8() -> ! {
    todo!("0x4630b8 RBX::Reflection::Call2Helper<RBX::DataModel,double (RBX::DataModel::*)(std::string,double),std::string,double,double>::call(RBX::DataModel*,double (RBX::DataModel::*)(std::string,double),RBX::Reflection::Variant &,std::string const&,double const&)")
}

#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::BoundCallbackDesc<RBX::DataModel>(char const*,boost::function<bool ()(void)> RBX::DataModel::*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x463220 — __ZN3RBX10Reflection17BoundCallbackDescIFbvEEC2INS_9DataModelEEEPKcMT_N5boost8functionIS2_EENS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
pub fn stub_0x463220() -> ! {
    todo!("0x463220 RBX::Reflection::BoundCallbackDesc<bool ()(void)>::BoundCallbackDesc<RBX::DataModel>(char const*,boost::function<bool ()(void)> RBX::DataModel::*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)")]
// 0x4633a4 — __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EEC2ERNS0_15ClassDescriptorEPKcNS0_10Descriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
pub fn stub_0x4633a4() -> ! {
    todo!("0x4633a4 RBX::Reflection::CallbackDescImpl<bool ()(void),0>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)")
}

#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::~BoundCallbackDesc()")]
// 0x4634e4 — __ZN3RBX10Reflection17BoundCallbackDescIFbvEED0Ev
pub fn stub_0x4634e4() -> ! {
    todo!("0x4634e4 RBX::Reflection::BoundCallbackDesc<bool ()(void)>::~BoundCallbackDesc()")
}

#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::setGenericCallback(RBX::Reflection::DescribedBase *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)const")]
// 0x4635f0 — __ZNK3RBX10Reflection16CallbackDescImplIFbvELi0EE18setGenericCallbackEPNS0_13DescribedBaseEN5boost10shared_ptrINS6_8functionIFNS7_INS0_5TupleEEENS7_IKS9_EEEEEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
pub fn stub_0x4635f0() -> ! {
    todo!("0x4635f0 RBX::Reflection::CallbackDescImpl<bool ()(void),0>::setGenericCallback(RBX::Reflection::DescribedBase *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)const")
}

#[doc(alias = "RBX::Reflection::CallbackDesc<bool ()(void)>::clearCallback(RBX::Reflection::DescribedBase *)const")]
// 0x463730 — __ZNK3RBX10Reflection12CallbackDescIFbvEE13clearCallbackEPNS0_13DescribedBaseE
pub fn stub_0x463730() -> ! {
    todo!("0x463730 RBX::Reflection::CallbackDesc<bool ()(void)>::clearCallback(RBX::Reflection::DescribedBase *)const")
}

#[doc(alias = "boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list_av_1<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::type> boost::bind<bool,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
// 0x4637f0 — __ZN5boost4bindIbNS_10shared_ptrINS_8functionIFNS1_IN3RBX10Reflection5TupleEEENS1_IKS5_EEEEEEESB_EENS_3_bi6bind_tIT_PFSE_T0_ENSC_9list_av_1IT1_E4typeEEESH_SJ_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x4637f0() -> ! {
    todo!("0x4637f0 boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list_av_1<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::type> boost::bind<bool,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")
}

#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::callGeneric(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
// 0x463908 — __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EE11callGenericEN5boost10shared_ptrINS4_8functionIFNS5_INS0_5TupleEEENS5_IKS7_EEEEEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x463908() -> ! {
    todo!("0x463908 RBX::Reflection::CallbackDescImpl<bool ()(void),0>::callGeneric(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")
}

#[doc(alias = "boost::disable_if<boost::is_void<bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::callGeneric<bool>(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
// 0x463a5c — __ZN3RBX10Reflection12CallbackDescIFbvEE11callGenericIbEEN5boost10disable_ifINS5_7is_voidIT_EES8_E4typeENS5_10shared_ptrINS5_8functionIFNSC_INS0_5TupleEEENSC_IKSE_EEEEEEESF_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x463a5c() -> ! {
    todo!("0x463a5c boost::disable_if<boost::is_void<bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::callGeneric<bool>(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Reflection::Tuple>)")
}

#[doc(alias = "boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::convertResult<bool>(rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
// 0x463b98 — __ZN3RBX10Reflection12CallbackDescIFbvEE13convertResultIbEEN5boost10disable_ifINS5_7is_sameINS5_10shared_ptrIKNS0_5TupleEEET_EESC_E4typeENS8_IS9_EE
// type: int(void)
pub fn stub_0x463b98() -> ! {
    todo!("0x463b98 boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::convertResult<bool>(rbx_core::SharedPtr<RBX::Reflection::Tuple>)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
// 0x463ce8 — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC2IS3_EEPT_
pub fn stub_0x463ce8() -> ! {
    todo!("0x463ce8 rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::~sp_counted_impl_p()")]
// 0x463dc0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEED1Ev
pub fn stub_0x463dc0() -> ! {
    todo!("0x463dc0 boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::~sp_counted_impl_p()")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::dispose(void)")]
// 0x463dc8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE7disposeEv
pub fn stub_0x463dc8() -> ! {
    todo!("0x463dc8 boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::dispose(void)")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::get_untyped_deleter(void)")]
// 0x463e70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE19get_untyped_deleterEv
pub fn stub_0x463e70() -> ! {
    todo!("0x463e70 boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::get_untyped_deleter(void)")
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>::list1(boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>)")]
// 0x463e74 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEC2ESE_
pub fn stub_0x463e74() -> ! {
    todo!("0x463e74 boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>::list1(boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>)")
}

#[doc(alias = "__ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// 0x463f54 — __ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x463f54() -> ! {
    todo!("0x463f54 __ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// 0x464030 — __ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x464030() -> ! {
    todo!("0x464030 __ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>)")]
// 0x464110 — __ZN5boost9function0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x464110() -> ! {
    todo!("0x464110 void boost::function0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>)")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x464200 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
pub fn stub_0x464200() -> ! {
    todo!("0x464200 boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,bool>::invoke(boost::detail::function::function_buffer &)")]
// 0x46421c — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEbE6invokeERNS1_15function_bufferE
pub fn stub_0x46421c() -> ! {
    todo!("0x46421c boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,bool>::invoke(boost::detail::function::function_buffer &)")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &)const")]
// 0x464230 — __ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x464230() -> ! {
    todo!("0x464230 bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &)const")
}
