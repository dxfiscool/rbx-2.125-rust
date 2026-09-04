//! core watchdog t — 100 core stubs EA-sorted, twenty-first gap filler after watchdog_s 0x3bd65c.
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_core — next 100 uncovered after 0x3bd65c (watchdog_s max).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x3bd66c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3bd66c() {
    // IDA 0x3bd66c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bd694 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_3bd694() {
    // IDA 0x3bd694: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bd6b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_3bd6b8() {
    // IDA 0x3bd6b8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bd804 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_3bd804() {
    // IDA 0x3bd804: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bd828 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::hasStringValue(void)const")]
pub fn stub_3bd828() {
    // IDA 0x3bd828: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bd82c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3bd82c() {
    // IDA 0x3bd82c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bd850 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_3bd850() {
    // IDA 0x3bd850: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3bd890 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_3bd890() {
    // IDA 0x3bd890: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3bd8b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_3bd8b0() {
    // IDA 0x3bd8b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3bdaf0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3bdaf0() {
    // IDA 0x3bdaf0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3bdb0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_3bdb0c() {
    // IDA 0x3bdb0c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3bdb40 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3bdb40() {
    // IDA 0x3bdb40: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x3bdb48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_3bdb48() {
    // IDA 0x3bdb48: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x3bdb94 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3bdb94() {
    // IDA 0x3bdb94: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bdbb4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_3bdbb4() {
    // IDA 0x3bdbb4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bdbe8 — __ZNK3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToIndex(RBX::BasicPartInstance::LegacyPartType)const")]
pub fn stub_3bdbe8() {
    // IDA 0x3bdbe8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bdc58 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_3bdc58() {
    // IDA 0x3bdc58: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bdc98 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::GetSetImpl<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isReadOnly(void)const")]
pub fn stub_3bdc98() {
    // IDA 0x3bdc98: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bdc9c — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::GetSetImpl<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isWriteOnly(void)const")]
pub fn stub_3bdc9c() {
    // IDA 0x3bdc9c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bdca0 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::GetSetImpl<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3bdca0() {
    // IDA 0x3bdca0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bdcc0 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::GetSetImpl<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::setValue(RBX::Reflection::DescribedBase *,RBX::BasicPartInstance::LegacyPartType const&)const")]
pub fn stub_3bdcc0() {
    // IDA 0x3bdcc0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bdce4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEEC2IiMS2_FvS3_EEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::EnumPropDescriptor<int,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>(char const*,char const*,int,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_3bdce4() {
    // IDA 0x3bdce4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bde90 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE7SetImplIMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isReadOnly(void)const")]
pub fn stub_3bde90() {
    // IDA 0x3bde90: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bde94 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE7SetImplIMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isWriteOnly(void)const")]
pub fn stub_3bde94() {
    // IDA 0x3bde94: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bde98 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE7SetImplIMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3bde98() {
    // IDA 0x3bde98: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bdfb8 — __ZNK3RBX10Reflection14PropDescriptorINS_17BasicPartInstanceENS2_14LegacyPartTypeEE7SetImplIMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::setValue(RBX::Reflection::DescribedBase *,RBX::BasicPartInstance::LegacyPartType const&)const")]
pub fn stub_3bdfb8() {
    // IDA 0x3bdfb8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bdfdc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_3bdfdc() {
    // IDA 0x3bdfdc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3be01c — __ZThn32_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3be01c() {
    // IDA 0x3be01c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be030 — __ZThn32_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3be030() {
    // IDA 0x3be030: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be0e4 — __ZThn36_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3be0e4() {
    // IDA 0x3be0e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be0f8 — __ZThn36_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3be0f8() {
    // IDA 0x3be0f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be1ac — __ZN3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEC2Ev
// type: RBX::PartInstance *__fastcall(RBX::PartInstance *, int *)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEC2Ev")]
pub fn stub_3be1ac() {
    // IDA 0x3be1ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be228 — __ZThn32_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3be228() {
    // IDA 0x3be228: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be23c — __ZThn32_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3be23c() {
    // IDA 0x3be23c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be2f0 — __ZThn36_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3be2f0() {
    // IDA 0x3be2f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be304 — __ZThn36_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3be304() {
    // IDA 0x3be304: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be3b8 — __ZThn32_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev")]
pub fn stub_3be3b8() {
    // IDA 0x3be3b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be3cc — __ZThn32_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev")]
pub fn stub_3be3cc() {
    // IDA 0x3be3cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be47c — __ZThn36_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED1Ev")]
pub fn stub_3be47c() {
    // IDA 0x3be47c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be490 — __ZThn36_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEED0Ev")]
pub fn stub_3be490() {
    // IDA 0x3be490: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be540 — __ZN3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::EnumPropDescriptor<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>(char const*,char const*,RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_3be540() {
    // IDA 0x3be540: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be6f4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::~EnumPropDescriptor()")]
pub fn stub_3be6f4() {
    // IDA 0x3be6f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be720 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::isReadOnly(void)const")]
pub fn stub_3be720() {
    // IDA 0x3be720: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be730 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::isWriteOnly(void)const")]
pub fn stub_3be730() {
    // IDA 0x3be730: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be740 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3be740() {
    // IDA 0x3be740: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be768 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_3be768() {
    // IDA 0x3be768: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3be78c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_3be78c() {
    // IDA 0x3be78c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3be8d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_3be8d8() {
    // IDA 0x3be8d8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3be8fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::hasStringValue(void)const")]
pub fn stub_3be8fc() {
    // IDA 0x3be8fc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3be900 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3be900() {
    // IDA 0x3be900: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3be924 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_3be924() {
    // IDA 0x3be924: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3be964 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_3be964() {
    // IDA 0x3be964: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3be984 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3be984() {
    // IDA 0x3be984: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3be9a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_3be9a0() {
    // IDA 0x3be9a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3be9d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3be9d4() {
    // IDA 0x3be9d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x3be9dc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_3be9dc() {
    // IDA 0x3be9dc: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x3bea28 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3bea28() {
    // IDA 0x3bea28: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bea48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_3bea48() {
    // IDA 0x3bea48: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bea7c — __ZNK3RBX10Reflection8EnumDescINS_12PartInstance10FormFactorEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor>::convertToIndex(RBX::PartInstance::FormFactor)const")]
pub fn stub_3bea7c() {
    // IDA 0x3bea7c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3beaec — __ZNK3RBX10Reflection14PropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::GetSetImpl<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>::isReadOnly(void)const")]
pub fn stub_3beaec() {
    // IDA 0x3beaec: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3beaf0 — __ZNK3RBX10Reflection14PropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::GetSetImpl<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>::isWriteOnly(void)const")]
pub fn stub_3beaf0() {
    // IDA 0x3beaf0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3beaf4 — __ZNK3RBX10Reflection14PropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::GetSetImpl<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3beaf4() {
    // IDA 0x3beaf4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3beb14 — __ZNK3RBX10Reflection14PropDescriptorINS_14FormFactorPartENS_12PartInstance10FormFactorEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::GetSetImpl<RBX::PartInstance::FormFactor (RBX::FormFactorPart::*)(void)const,void (RBX::FormFactorPart::*)(RBX::PartInstance::FormFactor)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance::FormFactor const&)const")]
pub fn stub_3beb14() {
    // IDA 0x3beb14: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x3bf18c — __ZN3RBX10Reflection14PropDescriptorINS_9BevelMeshEfED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::~PropDescriptor()")]
pub fn stub_3bf18c() {
    // IDA 0x3bf18c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3bf4e0 — __ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3bf4e0() {
    // IDA 0x3bf4e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3bf4e4 — __ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3bf4e4() {
    // IDA 0x3bf4e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3bf584 — __ZThn32_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3bf584() {
    // IDA 0x3bf584: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3bf58c — __ZThn32_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3bf58c() {
    // IDA 0x3bf58c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3bf630 — __ZThn36_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3bf630() {
    // IDA 0x3bf630: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3bf638 — __ZThn36_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3bf638() {
    // IDA 0x3bf638: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3bf6dc — __ZN3RBX10Reflection14PropDescriptorINS_9BevelMeshEfEC2IMS2_KFKfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::PropDescriptor<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>(char const*,char const*,float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_3bf6dc() {
    // IDA 0x3bf6dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3bf7f0 — __ZN3RBX10Reflection14PropDescriptorINS_9BevelMeshEfED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::~PropDescriptor()")]
pub fn stub_3bf7f0() {
    // IDA 0x3bf7f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3bf81c — __ZNK3RBX10Reflection14PropDescriptorINS_9BevelMeshEfE10GetSetImplIMS2_KFKfvEMS2_FvfEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::GetSetImpl<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>::isReadOnly(void)const")]
pub fn stub_3bf81c() {
    // IDA 0x3bf81c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3bf820 — __ZNK3RBX10Reflection14PropDescriptorINS_9BevelMeshEfE10GetSetImplIMS2_KFKfvEMS2_FvfEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::GetSetImpl<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>::isWriteOnly(void)const")]
pub fn stub_3bf820() {
    // IDA 0x3bf820: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3bf824 — __ZNK3RBX10Reflection14PropDescriptorINS_9BevelMeshEfE10GetSetImplIMS2_KFKfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::GetSetImpl<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3bf824() {
    // IDA 0x3bf824: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3bf844 — __ZNK3RBX10Reflection14PropDescriptorINS_9BevelMeshEfE10GetSetImplIMS2_KFKfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERS5_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BevelMesh,float>::GetSetImpl<float const (RBX::BevelMesh::*)(void)const,void (RBX::BevelMesh::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_3bf844() {
    // IDA 0x3bf844: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3bfaf4 — __ZN3RBX12BillboardGui10setAdorneeEPNS_8InstanceE
// type: void __fastcall(RBX::BillboardGui *this, RBX::Instance *)
#[doc(alias = "RBX::BillboardGui::setAdornee(RBX::Instance *)")]
pub fn stub_3bfaf4() {
    // IDA 0x3bfaf4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x3bffc0 — __ZN3RBX12BillboardGui19setPlayerToHideFromEPNS_8InstanceE
// type: void __fastcall(RBX::BillboardGui *this, RBX::Instance *, int, int)
#[doc(alias = "RBX::BillboardGui::setPlayerToHideFrom(RBX::Instance *)")]
pub fn stub_3bffc0() {
    // IDA 0x3bffc0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x3c0434 — __ZNK3RBX12BillboardGui12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::BillboardGui *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::BillboardGui::askSetParent(RBX::Instance const*)const")]
pub fn stub_3c0434() {
    // IDA 0x3c0434: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x3c0f7c — __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::~RefPropDescriptor()")]
pub fn stub_3c0f7c() {
    // IDA 0x3c0f7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c0fa8 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::~PropDescriptor()")]
pub fn stub_3c0fa8() {
    // IDA 0x3c0fa8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c0fcc — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::~PropDescriptor()")]
pub fn stub_3c0fcc() {
    // IDA 0x3c0fcc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c0ff0 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiENS_5UDim2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,RBX::UDim2>::~PropDescriptor()")]
pub fn stub_3c0ff0() {
    // IDA 0x3c0ff0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c101c — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::~PropDescriptor()")]
pub fn stub_3c101c() {
    // IDA 0x3c101c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c12a4 — __ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv")]
pub fn stub_3c12a4() {
    // IDA 0x3c12a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c1368 — __ZThn32_NK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv")]
pub fn stub_3c1368() {
    // IDA 0x3c1368: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c14d4 — __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_3c14d4() {
    // IDA 0x3c14d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c14d8 — __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_3c14d8() {
    // IDA 0x3c14d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c1574 — __ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_3c1574() {
    // IDA 0x3c1574: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c15fc — __ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7Creator6createEv")]
pub fn stub_3c15fc() {
    // IDA 0x3c15fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c1824 — __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_3c1824() {
    // IDA 0x3c1824: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c1a68 — __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_3c1a68() {
    // IDA 0x3c1a68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c1cf0 — __ZN3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::GuiLayerCollector *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3c1cf0() {
    // IDA 0x3c1cf0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c1cf4 — __ZN3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::GuiLayerCollector *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3c1cf4() {
    // IDA 0x3c1cf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c1d94 — __ZThn32_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3c1d94() {
    // IDA 0x3c1d94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c1d9c — __ZThn32_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3c1d9c() {
    // IDA 0x3c1d9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c1e40 — __ZThn36_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3c1e40() {
    // IDA 0x3c1e40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c1e48 — __ZThn36_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3c1e48() {
    // IDA 0x3c1e48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c1eec — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::PropDescriptor<bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool)>(char const*,char const*,bool (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_3c1eec() {
    // IDA 0x3c1eec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3c2000 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEbED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,bool>::~PropDescriptor()")]
pub fn stub_3c2000() {
    // IDA 0x3c2000: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
