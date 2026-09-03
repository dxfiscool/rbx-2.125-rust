//! core wd_10p — 100 core stubs EA-sorted asc not yet in crates/core/src (gap filler sequential after 0x4848b4) Range 0x4848e0..0x486a1c | rbx_core::SharedPtr not boost.
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not yet in crates/core/src (gap filler sequential after 0x4848b4).
//! Range: 0x4848e0..0x486a1c | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4848e0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10isReadOnlyEv")]
pub fn stub_0x4848e0() {
    // IDA 0x4848e0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x4848f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11isWriteOnlyEv")]
pub fn stub_0x4848f0() {
    // IDA 0x4848f0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484900 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_0x484900() {
    // IDA 0x484900: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484928 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x484928() {
    // IDA 0x484928: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48494c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x48494c() {
    // IDA 0x48494c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484a98 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_0x484a98() {
    // IDA 0x484a98: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484ac0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14hasStringValueEv")]
pub fn stub_0x484ac0() {
    // IDA 0x484ac0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484ac4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x484ac4() {
    // IDA 0x484ac4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484ae8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x484ae8() {
    // IDA 0x484ae8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x484b28 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x484b28() {
    // IDA 0x484b28: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x484b48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x484b48() {
    // IDA 0x484b48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x484d88 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x484d88() {
    // IDA 0x484d88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x484da4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x484da4() {
    // IDA 0x484da4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x484dd8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x484dd8() {
    // IDA 0x484dd8: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x484de0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x484de0() {
    // IDA 0x484de0: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x484e2c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_0x484e2c() {
    // IDA 0x484e2c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484e4c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_0x484e4c() {
    // IDA 0x484e4c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484e80 — __ZN3RBX10Reflection14EnumDescriptor10equalValueEPKNS1_4ItemEi
#[doc(alias = "RBX::Reflection::EnumDescriptor::equalValue(RBX::Reflection::EnumDescriptor::Item const*,int)")]
#[doc(alias = "__ZN3RBX10Reflection14EnumDescriptor10equalValueEPKNS1_4ItemEi")]
pub fn stub_0x484e80() {
    // IDA 0x484e80: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484e8c — __ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToIndex(RBX::Time::SampleMethod)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToIndexES3_")]
pub fn stub_0x484e8c() {
    // IDA 0x484e8c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484efc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x484efc() {
    // IDA 0x484efc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484f3c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::GetSetImpl<RBX::Time::SampleMethod (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::Time::SampleMethod)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_0x484f3c() {
    // IDA 0x484f3c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484f40 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::GetSetImpl<RBX::Time::SampleMethod (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::Time::SampleMethod)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_0x484f40() {
    // IDA 0x484f40: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484f44 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::GetSetImpl<RBX::Time::SampleMethod (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::Time::SampleMethod)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x484f44() {
    // IDA 0x484f44: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484f64 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::GetSetImpl<RBX::Time::SampleMethod (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::Time::SampleMethod)>::setValue(RBX::Reflection::DescribedBase *,RBX::Time::SampleMethod const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_0x484f64() {
    // IDA 0x484f64: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484f88 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Time::SampleMethod> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE13initSingletonEv")]
pub fn stub_0x484f88() {
    // IDA 0x484f88: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x484f8c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Time::SampleMethod> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE14doGetSingletonEv")]
pub fn stub_0x484f8c() {
    // IDA 0x484f8c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48507c — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::PropDescriptor<double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double)>(char const*,char const*,double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x48507c() {
    // IDA 0x48507c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x485190 — __ZN3RBX10Reflection23TypedPropertyDescriptorIdEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<double>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorIdEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x485190() {
    // IDA 0x485190: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x4852b4 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdED0Ev")]
pub fn stub_0x4852b4() {
    // IDA 0x4852b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4852e0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIdE10isReadOnlyEv")]
pub fn stub_0x4852e0() {
    // IDA 0x4852e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4852f0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIdE11isWriteOnlyEv")]
pub fn stub_0x4852f0() {
    // IDA 0x4852f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485300 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE11equalValuesEPKNS0_13DescribedBaseES5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIdE11equalValuesEPKNS0_13DescribedBaseES5_")]
pub fn stub_0x485300() {
    // IDA 0x485300: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485340 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIdE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x485340() {
    // IDA 0x485340: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485378 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE9copyValueEPKNS0_13DescribedBaseEPS3_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIdE9copyValueEPKNS0_13DescribedBaseEPS3_")]
pub fn stub_0x485378() {
    // IDA 0x485378: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x4853b0 — __ZN3RBX10Reflection23TypedPropertyDescriptorIdED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::~TypedPropertyDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorIdED0Ev")]
pub fn stub_0x4853b0() {
    // IDA 0x4853b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4853dc — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetSetImpl<double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE10isReadOnlyEv")]
pub fn stub_0x4853dc() {
    // IDA 0x4853dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4853e0 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetSetImpl<double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE11isWriteOnlyEv")]
pub fn stub_0x4853e0() {
    // IDA 0x4853e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4853e4 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetSetImpl<double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x4853e4() {
    // IDA 0x4853e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485404 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8setValueEPNS0_13DescribedBaseERKd
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetSetImpl<double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double)>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8setValueEPNS0_13DescribedBaseERKd")]
pub fn stub_0x485404() {
    // IDA 0x485404: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48542c — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::PropDescriptor<bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool)>(char const*,char const*,bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x48542c() {
    // IDA 0x48542c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x485544 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbED0Ev")]
pub fn stub_0x485544() {
    // IDA 0x485544: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485574 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetSetImpl<bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
pub fn stub_0x485574() {
    // IDA 0x485574: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485578 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetSetImpl<bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_0x485578() {
    // IDA 0x485578: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48557c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetSetImpl<bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x48557c() {
    // IDA 0x48557c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4855a0 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetSetImpl<bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0x4855a0() {
    // IDA 0x4855a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4855c4 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::PropDescriptor<double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double)>(char const*,char const*,double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x4855c4() {
    // IDA 0x4855c4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4856d8 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdED0Ev")]
pub fn stub_0x4856d8() {
    // IDA 0x4856d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485704 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetSetImpl<double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE10isReadOnlyEv")]
pub fn stub_0x485704() {
    // IDA 0x485704: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485708 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetSetImpl<double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE11isWriteOnlyEv")]
pub fn stub_0x485708() {
    // IDA 0x485708: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48570c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetSetImpl<double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x48570c() {
    // IDA 0x48570c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48572c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8setValueEPNS0_13DescribedBaseERKd
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetSetImpl<double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double)>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8setValueEPNS0_13DescribedBaseERKd")]
pub fn stub_0x48572c() {
    // IDA 0x48572c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485754 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::PropDescriptor<bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool)>(char const*,char const*,bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x485754() {
    // IDA 0x485754: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x485868 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbED0Ev")]
pub fn stub_0x485868() {
    // IDA 0x485868: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485894 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::GetSetImpl<bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
pub fn stub_0x485894() {
    // IDA 0x485894: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485898 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::GetSetImpl<bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_0x485898() {
    // IDA 0x485898: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48589c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::GetSetImpl<bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x48589c() {
    // IDA 0x48589c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4858c0 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::GetSetImpl<bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0x4858c0() {
    // IDA 0x4858c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4858e4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::EnumPropDescriptor<RBX::DataModelArbiter::ConcurrencyModel (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::DataModelArbiter::ConcurrencyModel)>(char const*,char const*,RBX::DataModelArbiter::ConcurrencyModel (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::DataModelArbiter::ConcurrencyModel),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x4858e4() {
    // IDA 0x4858e4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x485a98 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEED0Ev")]
pub fn stub_0x485a98() {
    // IDA 0x485a98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485ac4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10isReadOnlyEv")]
pub fn stub_0x485ac4() {
    // IDA 0x485ac4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485ad4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11isWriteOnlyEv")]
pub fn stub_0x485ad4() {
    // IDA 0x485ad4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485ae4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_0x485ae4() {
    // IDA 0x485ae4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485b0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x485b0c() {
    // IDA 0x485b0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x485b30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x485b30() {
    // IDA 0x485b30: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x485c7c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_0x485c7c() {
    // IDA 0x485c7c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x485ca0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14hasStringValueEv")]
pub fn stub_0x485ca0() {
    // IDA 0x485ca0: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x485ca4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x485ca4() {
    // IDA 0x485ca4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x485cc8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x485cc8() {
    // IDA 0x485cc8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x485d08 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x485d08() {
    // IDA 0x485d08: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x485d28 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x485d28() {
    // IDA 0x485d28: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x485f68 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x485f68() {
    // IDA 0x485f68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x485f84 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x485f84() {
    // IDA 0x485f84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x485fb8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x485fb8() {
    // IDA 0x485fb8: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x485fc0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x485fc0() {
    // IDA 0x485fc0: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x48600c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_0x48600c() {
    // IDA 0x48600c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48602c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_0x48602c() {
    // IDA 0x48602c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486060 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToValue(RBX::Name const&,RBX::DataModelArbiter::ConcurrencyModel&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x486060() {
    // IDA 0x486060: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4860e0 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToIndex(RBX::DataModelArbiter::ConcurrencyModel)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToIndexES3_")]
pub fn stub_0x4860e0() {
    // IDA 0x4860e0: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486150 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x486150() {
    // IDA 0x486150: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486190 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToString(RBX::DataModelArbiter::ConcurrencyModel const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE15convertToStringERKS3_")]
pub fn stub_0x486190() {
    // IDA 0x486190: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486330 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::GetSetImpl<RBX::DataModelArbiter::ConcurrencyModel (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::DataModelArbiter::ConcurrencyModel)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_0x486330() {
    // IDA 0x486330: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486334 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::GetSetImpl<RBX::DataModelArbiter::ConcurrencyModel (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::DataModelArbiter::ConcurrencyModel)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_0x486334() {
    // IDA 0x486334: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486338 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::GetSetImpl<RBX::DataModelArbiter::ConcurrencyModel (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::DataModelArbiter::ConcurrencyModel)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x486338() {
    // IDA 0x486338: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486358 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::GetSetImpl<RBX::DataModelArbiter::ConcurrencyModel (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::DataModelArbiter::ConcurrencyModel)>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModelArbiter::ConcurrencyModel const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_0x486358() {
    // IDA 0x486358: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48637c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEEE13initSingletonEv")]
pub fn stub_0x48637c() {
    // IDA 0x48637c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486380 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEEE14doGetSingletonEv")]
pub fn stub_0x486380() {
    // IDA 0x486380: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486470 — __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEED0Ev")]
pub fn stub_0x486470() {
    // IDA 0x486470: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x486510 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE6lookupEPKc")]
pub fn stub_0x486510() {
    // IDA 0x486510: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x486540 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE6lookupERKNS0_7VariantE")]
pub fn stub_0x486540() {
    // IDA 0x486540: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x486560 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x486560() {
    // IDA 0x486560: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48665c — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEEC2IMS2_KFS5_vEMS2_FvS5_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::EnumPropDescriptor<RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod)>(char const*,char const*,RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEEC2IMS2_KFS5_vEMS2_FvS5_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x48665c() {
    // IDA 0x48665c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x486810 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEED0Ev")]
pub fn stub_0x486810() {
    // IDA 0x486810: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48683c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10isReadOnlyEv")]
pub fn stub_0x48683c() {
    // IDA 0x48683c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48684c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11isWriteOnlyEv")]
pub fn stub_0x48684c() {
    // IDA 0x48684c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48685c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11equalValuesEPKNS0_13DescribedBaseES9_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11equalValuesEPKNS0_13DescribedBaseES9_")]
pub fn stub_0x48685c() {
    // IDA 0x48685c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x486884 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x486884() {
    // IDA 0x486884: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4868a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x4868a8() {
    // IDA 0x4868a8: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4869f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE9copyValueEPKNS0_13DescribedBaseEPS7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE9copyValueEPKNS0_13DescribedBaseEPS7_")]
pub fn stub_0x4869f4() {
    // IDA 0x4869f4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486a18 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14hasStringValueEv")]
pub fn stub_0x486a18() {
    // IDA 0x486a18: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486a1c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x486a1c() {
    // IDA 0x486a1c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}
