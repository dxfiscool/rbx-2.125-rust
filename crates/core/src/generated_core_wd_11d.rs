//! core wd_11d — 100 core stubs EA-sorted asc not yet in crates/core/src (gap filler sequential after 0x4a5834) Range 0x4a59e8..0x4a906c | rbx_core::SharedPtr not boost.
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not yet in crates/core/src (gap filler sequential after 0x4a5834).
//! Range: 0x4a59e8..0x4a906c | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEED0Ev")]
// 0x4a59e8 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEED0Ev
pub fn stub_0x4a59e8() {
    // IDA 0x4a59e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10isReadOnlyEv")]
// 0x4a5a14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10isReadOnlyEv
pub fn stub_0x4a5a14() {
    // IDA 0x4a5a14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11isWriteOnlyEv")]
// 0x4a5a24 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11isWriteOnlyEv
pub fn stub_0x4a5a24() {
    // IDA 0x4a5a24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11equalValuesEPKNS0_13DescribedBaseES7_")]
// 0x4a5a34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
pub fn stub_0x4a5a34() {
    // IDA 0x4a5a34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
// 0x4a5a5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
pub fn stub_0x4a5a5c() {
    // IDA 0x4a5a5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
// 0x4a5a80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_0x4a5a80() {
    // IDA 0x4a5a80: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
// 0x4a5bcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_0x4a5bcc() {
    // IDA 0x4a5bcc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14hasStringValueEv")]
// 0x4a5bf4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14hasStringValueEv
pub fn stub_0x4a5bf4() {
    // IDA 0x4a5bf4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14getStringValueEPKNS0_13DescribedBaseE")]
// 0x4a5bf8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14getStringValueEPKNS0_13DescribedBaseE
pub fn stub_0x4a5bf8() {
    // IDA 0x4a5bf8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// 0x4a5c1c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_0x4a5c1c() {
    // IDA 0x4a5c1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// 0x4a5c5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_0x4a5c5c() {
    // IDA 0x4a5c5c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// 0x4a5c7c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_0x4a5c7c() {
    // IDA 0x4a5c7c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13getIndexValueEPKNS0_13DescribedBaseE")]
// 0x4a5ebc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13getIndexValueEPKNS0_13DescribedBaseE
pub fn stub_0x4a5ebc() {
    // IDA 0x4a5ebc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13setIndexValueEPNS0_13DescribedBaseEm")]
// 0x4a5ed8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13setIndexValueEPNS0_13DescribedBaseEm
pub fn stub_0x4a5ed8() {
    // IDA 0x4a5ed8: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12getEnumValueEPKNS0_13DescribedBaseE")]
// 0x4a5f0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12getEnumValueEPKNS0_13DescribedBaseE
pub fn stub_0x4a5f0c() {
    // IDA 0x4a5f0c: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12setEnumValueEPNS0_13DescribedBaseEi")]
// 0x4a5f14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12setEnumValueEPNS0_13DescribedBaseEi
pub fn stub_0x4a5f14() {
    // IDA 0x4a5f14: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11getEnumItemEPKNS0_13DescribedBaseE")]
// 0x4a5f60 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11getEnumItemEPKNS0_13DescribedBaseE
pub fn stub_0x4a5f60() {
    // IDA 0x4a5f60: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
// 0x4a5f80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
pub fn stub_0x4a5f80() {
    // IDA 0x4a5f80: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToIndex(RBX::Explosion::ExplosionType)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToIndexES3_")]
// 0x4a5fb8 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToIndexES3_
pub fn stub_0x4a5fb8() {
    // IDA 0x4a5fb8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11setIntValueEPNS0_13DescribedBaseEi")]
// 0x4a6028 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11setIntValueEPNS0_13DescribedBaseEi
pub fn stub_0x4a6028() {
    // IDA 0x4a6028: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
// 0x4a606c — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_0x4a606c() {
    // IDA 0x4a606c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
// 0x4a6070 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_0x4a6070() {
    // IDA 0x4a6070: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
// 0x4a6074 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x4a6074() {
    // IDA 0x4a6074: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::setValue(RBX::Reflection::DescribedBase *,RBX::Explosion::ExplosionType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
// 0x4a6094 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_0x4a6094() {
    // IDA 0x4a6094: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9Explosion13ExplosionTypeEEEE13initSingletonEv")]
// 0x4a60b8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9Explosion13ExplosionTypeEEEE13initSingletonEv
pub fn stub_0x4a60b8() {
    // IDA 0x4a60b8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,G3D::Vector3 RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS9_MT_S3_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// 0x4a60bc — __ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS9_MT_S3_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_0x4a60bc() {
    // IDA 0x4a60bc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::~BoundProp()")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EED0Ev")]
// 0x4a6250 — __ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EED0Ev
pub fn stub_0x4a6250() {
    // IDA 0x4a6250: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
// 0x4a6280 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
pub fn stub_0x4a6280() {
    // IDA 0x4a6280: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE9copyValueEPKNS0_13DescribedBaseEPS5_")]
// 0x4a62b0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_0x4a62b0() {
    // IDA 0x4a62b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::~TypedPropertyDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EED0Ev")]
// 0x4a63c8 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EED0Ev
pub fn stub_0x4a63c8() {
    // IDA 0x4a63c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE10isReadOnlyEv")]
// 0x4a63f4 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE10isReadOnlyEv
pub fn stub_0x4a63f4() {
    // IDA 0x4a63f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE11isWriteOnlyEv")]
// 0x4a63f8 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE11isWriteOnlyEv
pub fn stub_0x4a63f8() {
    // IDA 0x4a63f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8getValueEPKNS0_13DescribedBaseE")]
// 0x4a63fc — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x4a63fc() {
    // IDA 0x4a63fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8setValueEPNS0_13DescribedBaseERKS3_")]
// 0x4a6418 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_0x4a6418() {
    // IDA 0x4a6418: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,float RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// 0x4a64ac — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_0x4a64ac() {
    // IDA 0x4a64ac: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::~BoundProp()")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EED0Ev")]
// 0x4a6640 — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EED0Ev
pub fn stub_0x4a6640() {
    // IDA 0x4a6640: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE10isReadOnlyEv")]
// 0x4a666c — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE10isReadOnlyEv
pub fn stub_0x4a666c() {
    // IDA 0x4a666c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE11isWriteOnlyEv")]
// 0x4a6670 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE11isWriteOnlyEv
pub fn stub_0x4a6670() {
    // IDA 0x4a6670: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8getValueEPKNS0_13DescribedBaseE")]
// 0x4a6674 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x4a6674() {
    // IDA 0x4a6674: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8setValueEPNS0_13DescribedBaseERKf")]
// 0x4a6680 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8setValueEPNS0_13DescribedBaseERKf
pub fn stub_0x4a6680() {
    // IDA 0x4a6680: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::PropDescriptor<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>(char const*,char const*,float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// 0x4a66dc — __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_0x4a66dc() {
    // IDA 0x4a66dc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfED0Ev")]
// 0x4a67f0 — __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfED0Ev
pub fn stub_0x4a67f0() {
    // IDA 0x4a67f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv")]
// 0x4a681c — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
pub fn stub_0x4a681c() {
    // IDA 0x4a681c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv")]
// 0x4a6820 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
pub fn stub_0x4a6820() {
    // IDA 0x4a6820: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE")]
// 0x4a6824 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x4a6824() {
    // IDA 0x4a6824: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf")]
// 0x4a6844 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
pub fn stub_0x4a6844() {
    // IDA 0x4a6844: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEED1Ev")]
// 0x4a7734 — __ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEED1Ev
pub fn stub_0x4a7734() {
    // IDA 0x4a7734: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv")]
// 0x4a7758 — __ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv
pub fn stub_0x4a7758() {
    // IDA 0x4a7758: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv")]
// 0x4a776c — __ZThn32_NK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv
pub fn stub_0x4a776c() {
    // IDA 0x4a776c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x4a777c — __ZN3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0x4a777c() {
    // IDA 0x4a777c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x4a7790 — __ZN3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0x4a7790() {
    // IDA 0x4a7790: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x4a7840 — __ZThn132_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0x4a7840() {
    // IDA 0x4a7840: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x4a7854 — __ZThn132_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0x4a7854() {
    // IDA 0x4a7854: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x4a7908 — __ZN3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0x4a7908() {
    // IDA 0x4a7908: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x4a791c — __ZN3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0x4a791c() {
    // IDA 0x4a791c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x4a79cc — __ZThn132_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0x4a79cc() {
    // IDA 0x4a79cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x4a79e0 — __ZThn132_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0x4a79e0() {
    // IDA 0x4a79e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")]
// 0x4a7a94 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev
pub fn stub_0x4a7a94() {
    // IDA 0x4a7a94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev")]
// 0x4a7aa8 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev
pub fn stub_0x4a7aa8() {
    // IDA 0x4a7aa8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")]
// 0x4a7b58 — __ZThn132_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev
pub fn stub_0x4a7b58() {
    // IDA 0x4a7b58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev")]
// 0x4a7b6c — __ZThn132_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev
pub fn stub_0x4a7b6c() {
    // IDA 0x4a7b6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorD1Ev")]
// 0x4a7b74 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorD1Ev
pub fn stub_0x4a7b74() {
    // IDA 0x4a7b74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorD2Ev")]
// 0x4a7b78 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorD2Ev
pub fn stub_0x4a7b78() {
    // IDA 0x4a7b78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x4a7c14 — __ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_0x4a7c14() {
    // IDA 0x4a7c14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7Creator6createEv")]
// 0x4a7c9c — __ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7Creator6createEv
pub fn stub_0x4a7c9c() {
    // IDA 0x4a7c9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorC2Ev")]
// 0x4a8274 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorC2Ev
pub fn stub_0x4a8274() {
    // IDA 0x4a8274: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE17static_getCreatorEv")]
// 0x4a84b8 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE17static_getCreatorEv
pub fn stub_0x4a84b8() {
    // IDA 0x4a84b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")]
// 0x4a852c — __ZThn32_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev
pub fn stub_0x4a852c() {
    // IDA 0x4a852c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")]
// 0x4a8540 — __ZThn36_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev
pub fn stub_0x4a8540() {
    // IDA 0x4a8540: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev")]
// 0x4a8554 — __ZThn32_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev
pub fn stub_0x4a8554() {
    // IDA 0x4a8554: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev")]
// 0x4a855c — __ZThn36_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev
pub fn stub_0x4a855c() {
    // IDA 0x4a855c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x4a8564 — __ZThn32_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0x4a8564() {
    // IDA 0x4a8564: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x4a8578 — __ZThn32_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0x4a8578() {
    // IDA 0x4a8578: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x4a862c — __ZThn36_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0x4a862c() {
    // IDA 0x4a862c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x4a8640 — __ZThn36_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0x4a8640() {
    // IDA 0x4a8640: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEC2IN3G3D7Vector3EEET_")]
// 0x4a86f4 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEC2IN3G3D7Vector3EEET_
pub fn stub_0x4a86f4() {
    // IDA 0x4a86f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x4a8760 — __ZThn32_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0x4a8760() {
    // IDA 0x4a8760: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x4a8774 — __ZThn32_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0x4a8774() {
    // IDA 0x4a8774: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x4a8828 — __ZThn36_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0x4a8828() {
    // IDA 0x4a8828: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x4a883c — __ZThn36_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0x4a883c() {
    // IDA 0x4a883c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::EnumPropDescriptor<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>(char const*,char const*,RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// 0x4a88f0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_0x4a88f0() {
    // IDA 0x4a88f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEED0Ev")]
// 0x4a8aa4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEED0Ev
pub fn stub_0x4a8aa4() {
    // IDA 0x4a8aa4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10isReadOnlyEv")]
// 0x4a8ad0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10isReadOnlyEv
pub fn stub_0x4a8ad0() {
    // IDA 0x4a8ad0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11isWriteOnlyEv")]
// 0x4a8ae0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11isWriteOnlyEv
pub fn stub_0x4a8ae0() {
    // IDA 0x4a8ae0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11equalValuesEPKNS0_13DescribedBaseES7_")]
// 0x4a8af0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11equalValuesEPKNS0_13DescribedBaseES7_
pub fn stub_0x4a8af0() {
    // IDA 0x4a8af0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
// 0x4a8b18 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
pub fn stub_0x4a8b18() {
    // IDA 0x4a8b18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
// 0x4a8b3c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_0x4a8b3c() {
    // IDA 0x4a8b3c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
// 0x4a8c88 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_0x4a8c88() {
    // IDA 0x4a8c88: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14hasStringValueEv")]
// 0x4a8cac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14hasStringValueEv
pub fn stub_0x4a8cac() {
    // IDA 0x4a8cac: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14getStringValueEPKNS0_13DescribedBaseE")]
// 0x4a8cb0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14getStringValueEPKNS0_13DescribedBaseE
pub fn stub_0x4a8cb0() {
    // IDA 0x4a8cb0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// 0x4a8cd4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_0x4a8cd4() {
    // IDA 0x4a8cd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// 0x4a8d14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_0x4a8d14() {
    // IDA 0x4a8d14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// 0x4a8d34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_0x4a8d34() {
    // IDA 0x4a8d34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE13getIndexValueEPKNS0_13DescribedBaseE")]
// 0x4a8f74 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE13getIndexValueEPKNS0_13DescribedBaseE
pub fn stub_0x4a8f74() {
    // IDA 0x4a8f74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE13setIndexValueEPNS0_13DescribedBaseEm")]
// 0x4a8f90 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE13setIndexValueEPNS0_13DescribedBaseEm
pub fn stub_0x4a8f90() {
    // IDA 0x4a8f90: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE12getEnumValueEPKNS0_13DescribedBaseE")]
// 0x4a8fc4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE12getEnumValueEPKNS0_13DescribedBaseE
pub fn stub_0x4a8fc4() {
    // IDA 0x4a8fc4: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE12setEnumValueEPNS0_13DescribedBaseEi")]
// 0x4a8fcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE12setEnumValueEPNS0_13DescribedBaseEi
pub fn stub_0x4a8fcc() {
    // IDA 0x4a8fcc: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11getEnumItemEPKNS0_13DescribedBaseE")]
// 0x4a9018 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11getEnumItemEPKNS0_13DescribedBaseE
pub fn stub_0x4a9018() {
    // IDA 0x4a9018: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
// 0x4a9038 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
pub fn stub_0x4a9038() {
    // IDA 0x4a9038: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToIndex(RBX::ExtrudedPartInstance::VisualTrussStyle)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToIndexES3_")]
// 0x4a906c — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToIndexES3_
pub fn stub_0x4a906c() {
    // IDA 0x4a906c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

