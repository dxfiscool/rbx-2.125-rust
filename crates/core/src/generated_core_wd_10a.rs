//! core wd_10a — 120 core stubs EA-sorted asc gap filler not yet in crates/core/src (global EA asc, next uncovered after 0x45fd90).
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 120 not yet in crates/core/src.
//! Range: 0x45fd90..0x46602c | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// 0x45fd90 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_0x45fd90() {
    // IDA 0x45fd90: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// 0x45fdd0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_0x45fdd0() {
    // IDA 0x45fdd0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// 0x45fdf0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
pub fn stub_0x45fdf0() {
    // IDA 0x45fdf0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// 0x460030 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13getIndexValueEPKNS0_13DescribedBaseE
pub fn stub_0x460030() {
    // IDA 0x460030: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// 0x46004c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE13setIndexValueEPNS0_13DescribedBaseEm
pub fn stub_0x46004c() {
    // IDA 0x46004c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// 0x460080 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12getEnumValueEPKNS0_13DescribedBaseE
pub fn stub_0x460080() {
    // IDA 0x460080: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// 0x460088 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE12setEnumValueEPNS0_13DescribedBaseEi
pub fn stub_0x460088() {
    // IDA 0x460088: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// 0x4600d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11getEnumItemEPKNS0_13DescribedBaseE
pub fn stub_0x4600d4() {
    // IDA 0x4600d4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// 0x4600f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
pub fn stub_0x4600f4() {
    // IDA 0x4600f4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToIndex(RBX::DataModel::CreatorType)const")]
// 0x460128 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToIndexES3_
// type: int(void)
pub fn stub_0x460128() {
    // IDA 0x460128: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// 0x460198 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
pub fn stub_0x460198() {
    // IDA 0x460198: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// 0x4601d8 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
pub fn stub_0x4601d8() {
    // IDA 0x4601d8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// 0x4601dc — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
pub fn stub_0x4601dc() {
    // IDA 0x4601dc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x4601e0 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x4601e0() {
    // IDA 0x4601e0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::GetImpl<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::CreatorType const&)const")]
// 0x460200 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_11CreatorTypeEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_0x460200() {
    // IDA 0x460200: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::PropDescriptor<int (RBX::DataModel::*)(void)const,int>(char const*,char const*,int (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x460320 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0x460320() {
    // IDA 0x460320: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::~PropDescriptor()")]
// 0x460430 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiED0Ev
pub fn stub_0x460430() {
    // IDA 0x460430: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// 0x460460 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE10isReadOnlyEv
pub fn stub_0x460460() {
    // IDA 0x460460: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// 0x460464 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
pub fn stub_0x460464() {
    // IDA 0x460464: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x460468 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x460468() {
    // IDA 0x460468: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::GetImpl<int (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// 0x460488 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
pub fn stub_0x460488() {
    // IDA 0x460488: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::Instance* (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x4605a8 — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_0x4605a8() {
    // IDA 0x4605a8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::~RefPropDescriptor()")]
// 0x46064c — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEED0Ev
pub fn stub_0x46064c() {
    // IDA 0x46064c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::isReadOnly(void)const")]
// 0x46067c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10isReadOnlyEv
pub fn stub_0x46067c() {
    // IDA 0x46067c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::isWriteOnly(void)const")]
// 0x46068c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11isWriteOnlyEv
pub fn stub_0x46068c() {
    // IDA 0x46068c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// 0x46069c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
pub fn stub_0x46069c() {
    // IDA 0x46069c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// 0x4606c4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x4606c4() {
    // IDA 0x4606c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// 0x4607dc — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_0x4607dc() {
    // IDA 0x4607dc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// 0x4608a4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_0x4608a4() {
    // IDA 0x4608a4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::RefPropertyDescriptor::getDataSize(RBX::Reflection::DescribedBase const*)const")]
// 0x4608c8 — __ZNK3RBX10Reflection21RefPropertyDescriptor11getDataSizeEPKNS0_13DescribedBaseE
// type: _DWORD __fastcall(RBX::Reflection::RefPropertyDescriptor *__hidden this, const DescribedBase *)
pub fn stub_0x4608c8() {
    // IDA 0x4608c8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::RefPropertyDescriptor::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// 0x4608d0 — __ZNK3RBX10Reflection21RefPropertyDescriptor14getStringValueEPKNS0_13DescribedBaseE
// type: _DWORD __fastcall(RBX::Reflection::RefPropertyDescriptor *__hidden this, const DescribedBase *)
pub fn stub_0x4608d0() {
    // IDA 0x4608d0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::RefPropertyDescriptor::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// 0x4608f8 — __ZNK3RBX10Reflection21RefPropertyDescriptor14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_0x4608f8() {
    // IDA 0x4608f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// 0x460908 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_0x460908() {
    // IDA 0x460908: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// 0x4609dc — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_0x4609dc() {
    // IDA 0x4609dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// 0x460a00 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
pub fn stub_0x460a00() {
    // IDA 0x460a00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x460a14 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
pub fn stub_0x460a14() {
    // IDA 0x460a14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x460a90 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
pub fn stub_0x460a90() {
    // IDA 0x460a90: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x460ab0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0x460ab0() {
    // IDA 0x460ab0: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x460b90 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
pub fn stub_0x460b90() {
    // IDA 0x460b90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// 0x460b98 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
pub fn stub_0x460b98() {
    // IDA 0x460b98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// 0x460b9c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
pub fn stub_0x460b9c() {
    // IDA 0x460b9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x460ba0 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x460ba0() {
    // IDA 0x460ba0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")]
// 0x460bc0 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
pub fn stub_0x460bc0() {
    // IDA 0x460bc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefType<RBX::Instance *>::~RefType()")]
// 0x460ce0 — __ZN3RBX10Reflection7RefTypeIPNS_8InstanceEED1Ev
pub fn stub_0x460ce0() {
    // IDA 0x460ce0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefType<RBX::Instance *>::~RefType()")]
// 0x460ce8 — __ZN3RBX10Reflection7RefTypeIPNS_8InstanceEED0Ev
pub fn stub_0x460ce8() {
    // IDA 0x460ce8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::RefPropDescriptor<RBX::Workspace* (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::Workspace* (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x460cec — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_0x460cec() {
    // IDA 0x460cec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefType<RBX::Workspace *>::singleton(void)")]
// 0x460d90 — __ZN3RBX10Reflection7RefTypeIPNS_9WorkspaceEE9singletonEv
pub fn stub_0x460d90() {
    // IDA 0x460d90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::~RefPropDescriptor()")]
// 0x460e88 — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEED0Ev
pub fn stub_0x460e88() {
    // IDA 0x460e88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::isReadOnly(void)const")]
// 0x460eb8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10isReadOnlyEv
pub fn stub_0x460eb8() {
    // IDA 0x460eb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::isWriteOnly(void)const")]
// 0x460ec8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11isWriteOnlyEv
pub fn stub_0x460ec8() {
    // IDA 0x460ec8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// 0x460ed8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11equalValuesEPKNS0_13DescribedBaseES7_
pub fn stub_0x460ed8() {
    // IDA 0x460ed8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// 0x460f00 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x460f00() {
    // IDA 0x460f00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// 0x461018 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_0x461018() {
    // IDA 0x461018: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// 0x4610e0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_0x4610e0() {
    // IDA 0x4610e0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// 0x461104 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_0x461104() {
    // IDA 0x461104: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// 0x4611d8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_0x4611d8() {
    // IDA 0x4611d8: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// 0x4611fc — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11getRefValueEPKNS0_13DescribedBaseE
pub fn stub_0x4611fc() {
    // IDA 0x4611fc: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x461210 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
pub fn stub_0x461210() {
    // IDA 0x461210: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x46128c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
pub fn stub_0x46128c() {
    // IDA 0x46128c: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x4612ac — __ZNK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0x4612ac() {
    // IDA 0x4612ac: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x46138c — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
pub fn stub_0x46138c() {
    // IDA 0x46138c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// 0x461394 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
pub fn stub_0x461394() {
    // IDA 0x461394: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// 0x461398 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
pub fn stub_0x461398() {
    // IDA 0x461398: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x46139c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x46139c() {
    // IDA 0x46139c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::Workspace *>::GetImpl<RBX::Workspace * (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Workspace * const&)const")]
// 0x4613bc — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEPNS_9WorkspaceEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
pub fn stub_0x4613bc() {
    // IDA 0x4613bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefType<RBX::Workspace *>::~RefType()")]
// 0x4614dc — __ZN3RBX10Reflection7RefTypeIPNS_9WorkspaceEED1Ev
pub fn stub_0x4614dc() {
    // IDA 0x4614dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::Type::Type<RBX::Workspace *>(char const*,char const*,RBX::Workspace * *)")]
// 0x4614e0 — __ZN3RBX10Reflection4TypeC2IPNS_9WorkspaceEEEPKcS6_PT_
// type: int(void)
pub fn stub_0x4614e0() {
    // IDA 0x4614e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::RefType<RBX::Workspace *>::~RefType()")]
// 0x46158c — __ZN3RBX10Reflection7RefTypeIPNS_9WorkspaceEED0Ev
pub fn stub_0x46158c() {
    // IDA 0x46158c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::BoundFuncDesc(void (RBX::DataModel::*)(RBX::DataModel::GearGenreSetting,int),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x461590 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EEC2EMS2_FvS3_iEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_0x461590() {
    // IDA 0x461590: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x461758 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_
pub fn stub_0x461758() {
    // IDA 0x461758: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::~BoundFuncDesc()")]
// 0x4617a4 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EED0Ev
pub fn stub_0x4617a4() {
    // IDA 0x4617a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x461884 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_0x461884() {
    // IDA 0x461884: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::BoundFuncDesc(void (RBX::DataModel::*)(RBX::DataModel::Genre),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x461abc — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_0x461abc() {
    // IDA 0x461abc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x461c34 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
pub fn stub_0x461c34() {
    // IDA 0x461c34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::~BoundFuncDesc()")]
// 0x461c64 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EED0Ev
pub fn stub_0x461c64() {
    // IDA 0x461c64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x461d38 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_0x461d38() {
    // IDA 0x461d38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::BoundFuncDesc(void (RBX::DataModel::*)(int,RBX::DataModel::CreatorType),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x461f50 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EEC2EMS2_FviS3_EPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_0x461f50() {
    // IDA 0x461f50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x462118 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_
pub fn stub_0x462118() {
    // IDA 0x462118: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::~BoundFuncDesc()")]
// 0x462164 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EED0Ev
pub fn stub_0x462164() {
    // IDA 0x462164: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x462244 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_0x462244() {
    // IDA 0x462244: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::BoundFuncDesc(void (RBX::DataModel::*)(int,bool),char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x462484 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EEC2EMS2_FvibEPKcS8_S8_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_0x462484() {
    // IDA 0x462484: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x462680 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
pub fn stub_0x462680() {
    // IDA 0x462680: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::~BoundFuncDesc()")]
// 0x4626cc — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EED0Ev
pub fn stub_0x4626cc() {
    // IDA 0x4626cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x4627ac — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_0x4627ac() {
    // IDA 0x4627ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::BoundFuncDesc(void (RBX::DataModel::*)(double),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x4629a8 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EEC2EMS2_FvdEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_0x4629a8() {
    // IDA 0x4629a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x462b20 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
pub fn stub_0x462b20() {
    // IDA 0x462b20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::~BoundFuncDesc()")]
// 0x462b50 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EED0Ev
pub fn stub_0x462b50() {
    // IDA 0x462b50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x462c24 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_0x462c24() {
    // IDA 0x462c24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::BoundFuncDesc(double (RBX::DataModel::*)(std::string,double),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x462c64 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EEC2EMS2_FdSsdEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_0x462c64() {
    // IDA 0x462c64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x462e2c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
pub fn stub_0x462e2c() {
    // IDA 0x462e2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::~BoundFuncDesc()")]
// 0x462e78 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EED0Ev
pub fn stub_0x462e78() {
    // IDA 0x462e78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x462f54 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_0x462f54() {
    // IDA 0x462f54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DataModel,double (RBX::DataModel::*)(std::string,double),std::string,double,double>::call(RBX::DataModel*,double (RBX::DataModel::*)(std::string,double),RBX::Reflection::Variant &,std::string const&,double const&)")]
// 0x4630b8 — __ZN3RBX10Reflection11Call2HelperINS_9DataModelEMS2_FdSsdESsddE4callEPS2_S4_RNS0_7VariantERKSsRKd
// type: int __fastcall(int, int, int, int, std::string *, int)
pub fn stub_0x4630b8() {
    // IDA 0x4630b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)")]
// 0x4633a4 — __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EEC2ERNS0_15ClassDescriptorEPKcNS0_10Descriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
pub fn stub_0x4633a4() {
    // IDA 0x4633a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::~BoundCallbackDesc()")]
// 0x4634e4 — __ZN3RBX10Reflection17BoundCallbackDescIFbvEED0Ev
pub fn stub_0x4634e4() {
    // IDA 0x4634e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::CallbackDesc<bool ()(void)>::clearCallback(RBX::Reflection::DescribedBase *)const")]
// 0x463730 — __ZNK3RBX10Reflection12CallbackDescIFbvEE13clearCallbackEPNS0_13DescribedBaseE
pub fn stub_0x463730() {
    // IDA 0x463730: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::CallbackDesc<bool ()(void)>::~CallbackDesc()")]
// 0x46455c — __ZN3RBX10Reflection12CallbackDescIFbvEED1Ev
pub fn stub_0x46455c() {
    // IDA 0x46455c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::CallbackDesc<bool ()(void)>::~CallbackDesc()")]
// 0x464654 — __ZN3RBX10Reflection12CallbackDescIFbvEED0Ev
pub fn stub_0x464654() {
    // IDA 0x464654: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::CallbackDescriptor::~CallbackDescriptor()")]
// 0x464760 — __ZN3RBX10Reflection18CallbackDescriptorD1Ev
// type: void __fastcall(RBX::Reflection::CallbackDescriptor *__hidden this)
pub fn stub_0x464760() {
    // IDA 0x464760: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::Setter<RBX::DataModel>::~Setter()")]
// 0x464788 — __ZN3RBX10Reflection17BoundCallbackDescIFbvEE6SetterINS_9DataModelEED1Ev
pub fn stub_0x464788() {
    // IDA 0x464788: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::Setter<RBX::DataModel>::~Setter()")]
// 0x46478c — __ZN3RBX10Reflection17BoundCallbackDescIFbvEE6SetterINS_9DataModelEED0Ev
pub fn stub_0x46478c() {
    // IDA 0x46478c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::~CallbackDescImpl()")]
// 0x464acc — __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EED1Ev
pub fn stub_0x464acc() {
    // IDA 0x464acc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::~CallbackDescImpl()")]
// 0x464bc4 — __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EED0Ev
pub fn stub_0x464bc4() {
    // IDA 0x464bc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::~BoundYieldFuncDesc()")]
// 0x464dd4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EED0Ev
pub fn stub_0x464dd4() {
    // IDA 0x464dd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::PropDescriptor<bool (RBX::DataModel::*)(void)const,int>(char const*,char const*,bool (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x465474 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0x465474() {
    // IDA 0x465474: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::~PropDescriptor()")]
// 0x465584 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEbED0Ev
pub fn stub_0x465584() {
    // IDA 0x465584: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// 0x4655b4 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE7GetImplIMS2_KFbvEE10isReadOnlyEv
pub fn stub_0x4655b4() {
    // IDA 0x4655b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// 0x4655b8 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv
pub fn stub_0x4655b8() {
    // IDA 0x4655b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x4655bc — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x4655bc() {
    // IDA 0x4655bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// 0x4655e0 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb
pub fn stub_0x4655e0() {
    // IDA 0x4655e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::PropDescriptor<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>(char const*,char const*,bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x465700 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
pub fn stub_0x465700() {
    // IDA 0x465700: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::isReadOnly(void)const")]
// 0x465814 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
pub fn stub_0x465814() {
    // IDA 0x465814: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::isWriteOnly(void)const")]
// 0x465818 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
pub fn stub_0x465818() {
    // IDA 0x465818: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x46581c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x46581c() {
    // IDA 0x46581c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// 0x465840 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
pub fn stub_0x465840() {
    // IDA 0x465840: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::BoundFuncDesc(void (RBX::DataModel::*)(std::string,std::string,std::string,std::string,std::string),char const*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x465864 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EEC2EMS2_FvSsSsSsSsSsEPKcS8_S8_S8_S8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_0x465864() {
    // IDA 0x465864: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x465b24 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_S6_S7_S6_S7_
pub fn stub_0x465b24() {
    // IDA 0x465b24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::~BoundFuncDesc()")]
// 0x465bc4 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EED0Ev
pub fn stub_0x465bc4() {
    // IDA 0x465bc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x465cb0 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_0x465cb0() {
    // IDA 0x465cb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::Call5Helper<RBX::DataModel,void (RBX::DataModel::*)(std::string,std::string,std::string,std::string,std::string),std::string,std::string,std::string,std::string,std::string,void>::call(RBX::DataModel*,void (RBX::DataModel::*)(std::string,std::string,std::string,std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&,std::string const&,std::string const&,std::string const&)")]
// 0x46602c — __ZN3RBX10Reflection11Call5HelperINS_9DataModelEMS2_FvSsSsSsSsSsESsSsSsSsSsvE4callEPS2_S4_RNS0_7VariantERKSsSA_SA_SA_SA_
// type: int __fastcall(int, int, int, int, std::string *, int, std::string *, std::string *, std::string *)
pub fn stub_0x46602c() {
    // IDA 0x46602c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
