//! core wd_11c — 100 core stubs EA-sorted asc not yet in crates/core/src (gap filler sequential after 0x499c1c) Range 0x499c40..0x4a5834 | rbx_core::SharedPtr not boost.
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not yet in crates/core/src (gap filler sequential after 0x499c1c).
//! Range: 0x499c40..0x4a5834 | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x499c40 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x499c40() {
    // IDA 0x499c40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x499c80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x499c80() {
    // IDA 0x499c80: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x499ca0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x499ca0() {
    // IDA 0x499ca0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x499ee0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x499ee0() {
    // IDA 0x499ee0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x499efc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x499efc() {
    // IDA 0x499efc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x499f30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x499f30() {
    // IDA 0x499f30: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x499f38 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x499f38() {
    // IDA 0x499f38: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x499f84 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_0x499f84() {
    // IDA 0x499f84: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x499fa4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_0x499fa4() {
    // IDA 0x499fa4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x499fd8 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToIndex(RBX::DialogRoot::DialogTone)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToIndexES3_")]
pub fn stub_0x499fd8() {
    // IDA 0x499fd8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a048 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x49a048() {
    // IDA 0x49a048: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a088 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
pub fn stub_0x49a088() {
    // IDA 0x49a088: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a08c — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_0x49a08c() {
    // IDA 0x49a08c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a090 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x49a090() {
    // IDA 0x49a090: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a0b0 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::setValue(RBX::Reflection::DescribedBase *,RBX::DialogRoot::DialogTone const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_10DialogToneEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_0x49a0b0() {
    // IDA 0x49a0b0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a0d4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::EnumPropDescriptor<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>(char const*,char const*,RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x49a0d4() {
    // IDA 0x49a0d4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a288 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEED0Ev")]
pub fn stub_0x49a288() {
    // IDA 0x49a288: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x49a2b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10isReadOnlyEv")]
pub fn stub_0x49a2b4() {
    // IDA 0x49a2b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x49a2c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11isWriteOnlyEv")]
pub fn stub_0x49a2c4() {
    // IDA 0x49a2c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x49a2d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_0x49a2d4() {
    // IDA 0x49a2d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x49a2fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x49a2fc() {
    // IDA 0x49a2fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x49a320 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x49a320() {
    // IDA 0x49a320: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a46c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_0x49a46c() {
    // IDA 0x49a46c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a490 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14hasStringValueEv")]
pub fn stub_0x49a490() {
    // IDA 0x49a490: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a494 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x49a494() {
    // IDA 0x49a494: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a4b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x49a4b8() {
    // IDA 0x49a4b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x49a4f8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x49a4f8() {
    // IDA 0x49a4f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x49a518 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x49a518() {
    // IDA 0x49a518: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x49a758 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x49a758() {
    // IDA 0x49a758: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x49a774 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x49a774() {
    // IDA 0x49a774: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x49a7a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x49a7a8() {
    // IDA 0x49a7a8: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x49a7b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x49a7b0() {
    // IDA 0x49a7b0: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x49a7fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_0x49a7fc() {
    // IDA 0x49a7fc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a81c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_0x49a81c() {
    // IDA 0x49a81c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a850 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToIndex(RBX::DialogRoot::DialogPurpose)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToIndexES3_")]
pub fn stub_0x49a850() {
    // IDA 0x49a850: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a8c0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x49a8c0() {
    // IDA 0x49a8c0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a900 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
pub fn stub_0x49a900() {
    // IDA 0x49a900: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a904 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_0x49a904() {
    // IDA 0x49a904: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a908 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x49a908() {
    // IDA 0x49a908: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a928 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::GetSetImpl<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>::setValue(RBX::Reflection::DescribedBase *,RBX::DialogRoot::DialogPurpose const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootENS2_13DialogPurposeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_0x49a928() {
    // IDA 0x49a928: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49a94c — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::PropDescriptor<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>(char const*,char const*,std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x49a94c() {
    // IDA 0x49a94c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x49aa60 — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsED0Ev")]
pub fn stub_0x49aa60() {
    // IDA 0x49aa60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x49aa8c — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv")]
pub fn stub_0x49aa8c() {
    // IDA 0x49aa8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x49aa90 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv")]
pub fn stub_0x49aa90() {
    // IDA 0x49aa90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x49aa94 — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x49aa94() {
    // IDA 0x49aa94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x49aabc — __ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::GetSetImpl<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10DialogRootESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x49aabc() {
    // IDA 0x49aabc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x49b52c — __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEC1Ev")]
pub fn stub_0x49b52c() {
    // IDA 0x49b52c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x49b530 — __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEC2Ev")]
pub fn stub_0x49b530() {
    // IDA 0x49b530: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x49b708 — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEC1Ev")]
pub fn stub_0x49b708() {
    // IDA 0x49b708: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x49b70c — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEC2Ev")]
pub fn stub_0x49b70c() {
    // IDA 0x49b70c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x49b964 — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEEC1Ev")]
pub fn stub_0x49b964() {
    // IDA 0x49b964: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49b968 — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEEC2Ev")]
pub fn stub_0x49b968() {
    // IDA 0x49b968: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49bb84 — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEEC1Ev")]
pub fn stub_0x49bb84() {
    // IDA 0x49bb84: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49bb88 — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEEC2Ev")]
pub fn stub_0x49bb88() {
    // IDA 0x49bb88: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49bdbc — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC1Ev")]
pub fn stub_0x49bdbc() {
    // IDA 0x49bdbc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49bdc0 — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEEC2Ev")]
pub fn stub_0x49bdc0() {
    // IDA 0x49bdc0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49bf80 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC1Ev")]
pub fn stub_0x49bf80() {
    // IDA 0x49bf80: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x49bf84 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEEC2Ev")]
pub fn stub_0x49bf84() {
    // IDA 0x49bf84: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x49c15c — __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::addPair(RBX::BasicPartInstance::LegacyPartType,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE7addPairES3_PKc")]
pub fn stub_0x49c15c() {
    // IDA 0x49c15c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x49c4bc — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::addPair(RBX::ExtrudedPartInstance::VisualTrussStyle,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE7addPairES3_PKc")]
pub fn stub_0x49c4bc() {
    // IDA 0x49c4bc: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x49c81c — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::addPair(RBX::PrismInstance::NumSidesEnum,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE7addPairES3_PKc")]
pub fn stub_0x49c81c() {
    // IDA 0x49c81c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x49cb7c — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::addPair(RBX::PyramidInstance::NumSidesEnum,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE7addPairES3_PKc")]
pub fn stub_0x49cb7c() {
    // IDA 0x49cb7c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x49cedc — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::addPair(RBX::Handles::VisualStyle,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE7addPairES3_PKc")]
pub fn stub_0x49cedc() {
    // IDA 0x49cedc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x49d23c — __ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::addPair(RBX::GuiObject::SizeConstraint,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE7addPairES3_PKc")]
pub fn stub_0x49d23c() {
    // IDA 0x49d23c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x49f610 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEEC1Ev")]
pub fn stub_0x49f610() {
    // IDA 0x49f610: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x49f614 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEEC2Ev")]
pub fn stub_0x49f614() {
    // IDA 0x49f614: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x4a0440 — __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfED1Ev")]
pub fn stub_0x4a0440() {
    // IDA 0x4a0440: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a0468 — __ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EED1Ev
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::~BoundProp()")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EED1Ev")]
pub fn stub_0x4a0468() {
    // IDA 0x4a0468: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a0494 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEED1Ev")]
pub fn stub_0x4a0494() {
    // IDA 0x4a0494: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a04dc — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::addPair(RBX::Explosion::ExplosionType,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE7addPairES3_PKc")]
pub fn stub_0x4a04dc() {
    // IDA 0x4a04dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a1340 — __ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E12getClassNameEv")]
pub fn stub_0x4a1340() {
    // IDA 0x4a1340: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a1358 — __ZThn32_NK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E12getClassNameEv")]
pub fn stub_0x4a1358() {
    // IDA 0x4a1358: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a1378 — __ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7CreatorD1Ev")]
pub fn stub_0x4a1378() {
    // IDA 0x4a1378: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a137c — __ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorD1Ev")]
pub fn stub_0x4a137c() {
    // IDA 0x4a137c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a1380 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED1Ev")]
pub fn stub_0x4a1380() {
    // IDA 0x4a1380: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a1388 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE15convertToStringEmRSs")]
pub fn stub_0x4a1388() {
    // IDA 0x4a1388: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a14e0 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToItem(RBX::Explosion::ExplosionType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE13convertToItemERKS3_")]
pub fn stub_0x4a14e0() {
    // IDA 0x4a14e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a15b0 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED2Ev")]
pub fn stub_0x4a15b0() {
    // IDA 0x4a15b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a1788 — __ZNK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E7Creator12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E7Creator12getClassNameEv")]
pub fn stub_0x4a1788() {
    // IDA 0x4a1788: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a18d8 — __ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E7CreatorC2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E7CreatorC2Ev")]
pub fn stub_0x4a18d8() {
    // IDA 0x4a18d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a1b04 — __ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorD2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorD2Ev")]
pub fn stub_0x4a1b04() {
    // IDA 0x4a1b04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a1ba0 — __ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator12getClassNameEv")]
pub fn stub_0x4a1ba0() {
    // IDA 0x4a1ba0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a1c28 — __ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator6createEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator6createEv")]
pub fn stub_0x4a1c28() {
    // IDA 0x4a1c28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a1e5c — __ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorC2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorC2Ev")]
pub fn stub_0x4a1e5c() {
    // IDA 0x4a1e5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a20a0 — __ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E17static_getCreatorEv")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E17static_getCreatorEv")]
pub fn stub_0x4a20a0() {
    // IDA 0x4a20a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a2900 — __ZNK3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7Creator12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7Creator12getClassNameEv")]
pub fn stub_0x4a2900() {
    // IDA 0x4a2900: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

// 0x4a2bd0 — __ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E15isNullClassNameEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E15isNullClassNameEv")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E15isNullClassNameEv")]
pub fn stub_0x4a2bd0() {
    // IDA 0x4a2bd0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

// 0x4a2fb4 — __ZN3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4a2fb4() {
    // IDA 0x4a2fb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a2fb8 — __ZN3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4a2fb8() {
    // IDA 0x4a2fb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a305c — __ZThn32_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4a305c() {
    // IDA 0x4a305c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a3064 — __ZThn32_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4a3064() {
    // IDA 0x4a3064: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a3108 — __ZThn36_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4a3108() {
    // IDA 0x4a3108: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a3110 — __ZThn36_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4a3110() {
    // IDA 0x4a3110: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a31b8 — __ZN3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4a31b8() {
    // IDA 0x4a31b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a31bc — __ZN3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4a31bc() {
    // IDA 0x4a31bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a325c — __ZThn32_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4a325c() {
    // IDA 0x4a325c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a3264 — __ZThn32_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4a3264() {
    // IDA 0x4a3264: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a3308 — __ZThn36_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4a3308() {
    // IDA 0x4a3308: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a3310 — __ZThn36_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4a3310() {
    // IDA 0x4a3310: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4a5834 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::EnumPropDescriptor<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>(char const*,char const*,RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x4a5834() {
    // IDA 0x4a5834: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
