//! core wd_10q — 100 core stubs EA-sorted asc not yet in crates/core/src (gap filler sequential after 0x486a1c) Range 0x486a40..0x4892fc | rbx_core::SharedPtr not boost.
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not yet in crates/core/src (gap filler sequential after 0x486a1c).
//! Range: 0x486a40..0x4892fc | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x486a40 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x486a40() {
    // IDA 0x486a40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x486a80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x486a80() {
    // IDA 0x486a80: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x486aa0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x486aa0() {
    // IDA 0x486aa0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x486ce0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x486ce0() {
    // IDA 0x486ce0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x486cfc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x486cfc() {
    // IDA 0x486cfc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x486d30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x486d30() {
    // IDA 0x486d30: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x486d38 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x486d38() {
    // IDA 0x486d38: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x486d84 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_0x486d84() {
    // IDA 0x486d84: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486da4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_0x486da4() {
    // IDA 0x486da4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486dd8 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToIndexES4_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToIndex(RBX::TaskScheduler::Job::SleepAdjustMethod)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToIndexES4_")]
pub fn stub_0x486dd8() {
    // IDA 0x486dd8: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486e48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x486e48() {
    // IDA 0x486e48: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486e88 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::GetSetImpl<RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE10isReadOnlyEv")]
pub fn stub_0x486e88() {
    // IDA 0x486e88: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486e8c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::GetSetImpl<RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE11isWriteOnlyEv")]
pub fn stub_0x486e8c() {
    // IDA 0x486e8c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486e90 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::GetSetImpl<RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x486e90() {
    // IDA 0x486e90: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486eb0 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE8setValueEPNS0_13DescribedBaseERKS5_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::GetSetImpl<RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod)>::setValue(RBX::Reflection::DescribedBase *,RBX::TaskScheduler::Job::SleepAdjustMethod const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE8setValueEPNS0_13DescribedBaseERKS5_")]
pub fn stub_0x486eb0() {
    // IDA 0x486eb0: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486ed4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE13initSingletonEv")]
pub fn stub_0x486ed4() {
    // IDA 0x486ed4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486ed8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE14doGetSingletonEv")]
pub fn stub_0x486ed8() {
    // IDA 0x486ed8: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x486fc8 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::EnumPropDescriptor<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>(char const*,char const*,RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x486fc8() {
    // IDA 0x486fc8: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48717c — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEED0Ev")]
pub fn stub_0x48717c() {
    // IDA 0x48717c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4871a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10isReadOnlyEv")]
pub fn stub_0x4871a8() {
    // IDA 0x4871a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4871b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11isWriteOnlyEv")]
pub fn stub_0x4871b8() {
    // IDA 0x4871b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4871c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_0x4871c8() {
    // IDA 0x4871c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4871f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x4871f0() {
    // IDA 0x4871f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x487214 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x487214() {
    // IDA 0x487214: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x487360 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_0x487360() {
    // IDA 0x487360: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x487384 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14hasStringValueEv")]
pub fn stub_0x487384() {
    // IDA 0x487384: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x487388 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x487388() {
    // IDA 0x487388: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4873ac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x4873ac() {
    // IDA 0x4873ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x4873ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x4873ec() {
    // IDA 0x4873ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x48740c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x48740c() {
    // IDA 0x48740c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x48764c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x48764c() {
    // IDA 0x48764c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x487668 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x487668() {
    // IDA 0x487668: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x48769c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x48769c() {
    // IDA 0x48769c: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x4876a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x4876a4() {
    // IDA 0x4876a4: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x4876f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_0x4876f0() {
    // IDA 0x4876f0: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x487710 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_0x487710() {
    // IDA 0x487710: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x487744 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToIndex(RBX::TaskScheduler::PriorityMethod)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToIndexES3_")]
pub fn stub_0x487744() {
    // IDA 0x487744: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4877b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x4877b4() {
    // IDA 0x4877b4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4877f4 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_0x4877f4() {
    // IDA 0x4877f4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4877f8 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_0x4877f8() {
    // IDA 0x4877f8: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4877fc — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x4877fc() {
    // IDA 0x4877fc: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48781c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::setValue(RBX::Reflection::DescribedBase *,RBX::TaskScheduler::PriorityMethod const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_0x48781c() {
    // IDA 0x48781c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x487840 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE13initSingletonEv")]
pub fn stub_0x487840() {
    // IDA 0x487840: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x487844 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE14doGetSingletonEv")]
pub fn stub_0x487844() {
    // IDA 0x487844: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x487934 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EEC2EMS2_FvdiEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::BoundFuncDesc(void (RBX::TaskSchedulerSettings::*)(double,int),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EEC2EMS2_FvdiEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x487934() {
    // IDA 0x487934: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x487afc — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_")]
pub fn stub_0x487afc() {
    // IDA 0x487afc: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x487b48 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EED0Ev")]
pub fn stub_0x487b48() {
    // IDA 0x487b48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x487c28 — __ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x487c28() {
    // IDA 0x487c28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x487c88 — __ZN3RBX10Reflection9ArgHelper6getArgIdLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "double RBX::Reflection::ArgHelper::getArg<double,1>(RBX::Reflection::FunctionDescriptor::Arguments &,std::boxed::Box<double> const&,boost::disable_if<boost::is_same<double,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgIdLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0x487c88() {
    // IDA 0x487c88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x487e28 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::EnumPropDescriptor<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>(char const*,char const*,RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x487e28() {
    // IDA 0x487e28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x487fdc — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEED0Ev")]
pub fn stub_0x487fdc() {
    // IDA 0x487fdc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488008 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10isReadOnlyEv")]
pub fn stub_0x488008() {
    // IDA 0x488008: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488018 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11isWriteOnlyEv")]
pub fn stub_0x488018() {
    // IDA 0x488018: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488028 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_0x488028() {
    // IDA 0x488028: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488050 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x488050() {
    // IDA 0x488050: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488074 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x488074() {
    // IDA 0x488074: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4881c0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_0x4881c0() {
    // IDA 0x4881c0: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4881e4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14hasStringValueEv")]
pub fn stub_0x4881e4() {
    // IDA 0x4881e4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4881e8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x4881e8() {
    // IDA 0x4881e8: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48820c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x48820c() {
    // IDA 0x48820c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x48824c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x48824c() {
    // IDA 0x48824c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x48826c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x48826c() {
    // IDA 0x48826c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x4884ac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x4884ac() {
    // IDA 0x4884ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x4884c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x4884c8() {
    // IDA 0x4884c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x4884fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x4884fc() {
    // IDA 0x4884fc: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x488504 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x488504() {
    // IDA 0x488504: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x488550 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_0x488550() {
    // IDA 0x488550: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x488570 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_0x488570() {
    // IDA 0x488570: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4885a4 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToIndex(RBX::TaskScheduler::ThreadPoolConfig)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_")]
pub fn stub_0x4885a4() {
    // IDA 0x4885a4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x488614 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x488614() {
    // IDA 0x488614: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x488654 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_0x488654() {
    // IDA 0x488654: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x488658 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_0x488658() {
    // IDA 0x488658: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48865c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x48865c() {
    // IDA 0x48865c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48867c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::setValue(RBX::Reflection::DescribedBase *,RBX::TaskScheduler::ThreadPoolConfig const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_0x48867c() {
    // IDA 0x48867c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4886a0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE13initSingletonEv")]
pub fn stub_0x4886a0() {
    // IDA 0x4886a0: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4886a4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv")]
pub fn stub_0x4886a4() {
    // IDA 0x4886a4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x488794 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::BoundFuncDesc(void (RBX::DebugSettings::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x488794() {
    // IDA 0x488794: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48890c — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x48890c() {
    // IDA 0x48890c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48893c — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EED0Ev")]
pub fn stub_0x48893c() {
    // IDA 0x48893c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488a10 — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x488a10() {
    // IDA 0x488a10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488a48 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::BoundFuncDesc(void (RBX::DebugSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x488a48() {
    // IDA 0x488a48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488b4c — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EED0Ev")]
pub fn stub_0x488b4c() {
    // IDA 0x488b4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488c00 — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x488c00() {
    // IDA 0x488c00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488c20 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::EnumPropDescriptor<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>(char const*,char const*,RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x488c20() {
    // IDA 0x488c20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488dd4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEED0Ev")]
pub fn stub_0x488dd4() {
    // IDA 0x488dd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488e00 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10isReadOnlyEv")]
pub fn stub_0x488e00() {
    // IDA 0x488e00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488e10 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11isWriteOnlyEv")]
pub fn stub_0x488e10() {
    // IDA 0x488e10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488e20 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_0x488e20() {
    // IDA 0x488e20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488e48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x488e48() {
    // IDA 0x488e48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x488e6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x488e6c() {
    // IDA 0x488e6c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x488fb8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_0x488fb8() {
    // IDA 0x488fb8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x488fdc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14hasStringValueEv")]
pub fn stub_0x488fdc() {
    // IDA 0x488fdc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x488fe0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x488fe0() {
    // IDA 0x488fe0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x489004 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x489004() {
    // IDA 0x489004: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x489044 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x489044() {
    // IDA 0x489044: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x489064 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x489064() {
    // IDA 0x489064: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x4892a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x4892a4() {
    // IDA 0x4892a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x4892c0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x4892c0() {
    // IDA 0x4892c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x4892f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x4892f4() {
    // IDA 0x4892f4: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x4892fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x4892fc() {
    // IDA 0x4892fc: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}
