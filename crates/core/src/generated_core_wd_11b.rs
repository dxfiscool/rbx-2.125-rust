//! core wd_11b — 100 core stubs EA-sorted asc not yet in crates/core/src (gap filler sequential after 0x4892fc) Range 0x489348..0x491ddc | rbx_core::SharedPtr not boost.
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not yet in crates/core/src (gap filler sequential after 0x4892fc).
//! Range: 0x489348..0x491ddc | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x489348 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_0x489348() {
    // IDA 0x489348: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x489368 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_0x489368() {
    // IDA 0x489368: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48939c — __ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToIndex(RBX::DebugSettings::ErrorReporting)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_")]
pub fn stub_0x48939c() {
    // IDA 0x48939c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48940c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x48940c() {
    // IDA 0x48940c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48944c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
pub fn stub_0x48944c() {
    // IDA 0x48944c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x489450 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_0x489450() {
    // IDA 0x489450: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x489454 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x489454() {
    // IDA 0x489454: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x489474 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::setValue(RBX::Reflection::DescribedBase *,RBX::DebugSettings::ErrorReporting const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_0x489474() {
    // IDA 0x489474: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x489498 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE13initSingletonEv")]
pub fn stub_0x489498() {
    // IDA 0x489498: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48949c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv")]
pub fn stub_0x48949c() {
    // IDA 0x48949c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48958c — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13DebugSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::DebugSettings>(char const*,char const*,bool RBX::DebugSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13DebugSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x48958c() {
    // IDA 0x48958c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x489720 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE10isReadOnlyEv")]
pub fn stub_0x489720() {
    // IDA 0x489720: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x489724 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE11isWriteOnlyEv")]
pub fn stub_0x489724() {
    // IDA 0x489724: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x489728 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x489728() {
    // IDA 0x489728: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x489734 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0x489734() {
    // IDA 0x489734: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x489784 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EEC2EMS2_FvbdEPKcS8_bS8_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::BoundFuncDesc(void (RBX::TaskSchedulerSettings::*)(bool,double),char const*,char const*,bool,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EEC2EMS2_FvbdEPKcS8_bS8_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x489784() {
    // IDA 0x489784: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x4899b4 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_")]
pub fn stub_0x4899b4() {
    // IDA 0x4899b4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x489a00 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EED0Ev")]
pub fn stub_0x489a00() {
    // IDA 0x489a00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x489ae0 — __ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x489ae0() {
    // IDA 0x489ae0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x489cd8 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::PropDescriptor<double (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,double (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x489cd8() {
    // IDA 0x489cd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x489de4 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE10isReadOnlyEv")]
pub fn stub_0x489de4() {
    // IDA 0x489de4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x489de8 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv")]
pub fn stub_0x489de8() {
    // IDA 0x489de8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x489dec — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x489dec() {
    // IDA 0x489dec: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x489e0c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd")]
pub fn stub_0x489e0c() {
    // IDA 0x489e0c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x489f2c — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiEC2IMS2_KFjvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::PropDescriptor<unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiEC2IMS2_KFjvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x489f2c() {
    // IDA 0x489f2c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48a038 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiED0Ev")]
pub fn stub_0x48a038() {
    // IDA 0x48a038: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48a064 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE10isReadOnlyEv")]
pub fn stub_0x48a064() {
    // IDA 0x48a064: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48a068 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE11isWriteOnlyEv")]
pub fn stub_0x48a068() {
    // IDA 0x48a068: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48a06c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x48a06c() {
    // IDA 0x48a06c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48a08c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_0x48a08c() {
    // IDA 0x48a08c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48a538 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFlvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::PropDescriptor<long (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,long (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFlvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x48a538() {
    // IDA 0x48a538: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48a644 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<long (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE10isReadOnlyEv")]
pub fn stub_0x48a644() {
    // IDA 0x48a644: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48a648 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<long (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE11isWriteOnlyEv")]
pub fn stub_0x48a648() {
    // IDA 0x48a648: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48a64c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<long (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x48a64c() {
    // IDA 0x48a64c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

// 0x48a66c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<long (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_0x48a66c() {
    // IDA 0x48a66c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48a78c — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::PropDescriptor<double (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,double (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x48a78c() {
    // IDA 0x48a78c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48a898 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetImpl<double (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE10isReadOnlyEv")]
pub fn stub_0x48a898() {
    // IDA 0x48a898: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48a89c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetImpl<double (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv")]
pub fn stub_0x48a89c() {
    // IDA 0x48a89c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48a8a0 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetImpl<double (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x48a8a0() {
    // IDA 0x48a8a0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48a8c0 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetImpl<double (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd")]
pub fn stub_0x48a8c0() {
    // IDA 0x48a8c0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48a9e0 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::PropDescriptor<bool (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,bool (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x48a9e0() {
    // IDA 0x48a9e0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48aaec — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetImpl<bool (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE10isReadOnlyEv")]
pub fn stub_0x48aaec() {
    // IDA 0x48aaec: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48aaf0 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetImpl<bool (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv")]
pub fn stub_0x48aaf0() {
    // IDA 0x48aaf0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48aaf4 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetImpl<bool (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x48aaf4() {
    // IDA 0x48aaf4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48ab18 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetImpl<bool (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0x48ab18() {
    // IDA 0x48ab18: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48ac38 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::PropDescriptor<int (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,int (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x48ac38() {
    // IDA 0x48ac38: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48ad44 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<int (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE10isReadOnlyEv")]
pub fn stub_0x48ad44() {
    // IDA 0x48ad44: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48ad48 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<int (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE11isWriteOnlyEv")]
pub fn stub_0x48ad48() {
    // IDA 0x48ad48: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48ad4c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<int (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x48ad4c() {
    // IDA 0x48ad4c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48ad6c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<int (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_0x48ad6c() {
    // IDA 0x48ad6c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48ae8c — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfEC2IMS2_KFfvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::PropDescriptor<float (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,float (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfEC2IMS2_KFfvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x48ae8c() {
    // IDA 0x48ae8c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48af98 — __ZN3RBX10Reflection23TypedPropertyDescriptorIfEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<float>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorIfEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x48af98() {
    // IDA 0x48af98: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x48b0bc — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfED0Ev")]
pub fn stub_0x48b0bc() {
    // IDA 0x48b0bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48b0e8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIfE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIfE10isReadOnlyEv")]
pub fn stub_0x48b0e8() {
    // IDA 0x48b0e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48b0f8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIfE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIfE11isWriteOnlyEv")]
pub fn stub_0x48b0f8() {
    // IDA 0x48b0f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48b108 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIfE11equalValuesEPKNS0_13DescribedBaseES5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIfE11equalValuesEPKNS0_13DescribedBaseES5_")]
pub fn stub_0x48b108() {
    // IDA 0x48b108: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48b2f0 — __ZN3RBX10Reflection23TypedPropertyDescriptorIfED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::~TypedPropertyDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorIfED0Ev")]
pub fn stub_0x48b2f0() {
    // IDA 0x48b2f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48b31c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::GetImpl<float (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE10isReadOnlyEv")]
pub fn stub_0x48b31c() {
    // IDA 0x48b31c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48b320 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::GetImpl<float (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE11isWriteOnlyEv")]
pub fn stub_0x48b320() {
    // IDA 0x48b320: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48b324 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::GetImpl<float (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x48b324() {
    // IDA 0x48b324: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48b344 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::GetImpl<float (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE8setValueEPNS0_13DescribedBaseERKf")]
pub fn stub_0x48b344() {
    // IDA 0x48b344: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48b464 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsEC2IMS2_KFSsvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::PropDescriptor<std::string (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,std::string (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsEC2IMS2_KFSsvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x48b464() {
    // IDA 0x48b464: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x48b570 — __ZN3RBX10Reflection23TypedPropertyDescriptorISsEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<std::string>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<std::string>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorISsEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x48b570() {
    // IDA 0x48b570: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x48b694 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsED0Ev")]
pub fn stub_0x48b694() {
    // IDA 0x48b694: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48b6c0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorISsE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<std::string>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorISsE10isReadOnlyEv")]
pub fn stub_0x48b6c0() {
    // IDA 0x48b6c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48b6d0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorISsE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<std::string>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorISsE11isWriteOnlyEv")]
pub fn stub_0x48b6d0() {
    // IDA 0x48b6d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48b6e0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorISsE11equalValuesEPKNS0_13DescribedBaseES5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<std::string>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorISsE11equalValuesEPKNS0_13DescribedBaseES5_")]
pub fn stub_0x48b6e0() {
    // IDA 0x48b6e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48b850 — __ZNK3RBX10Reflection23TypedPropertyDescriptorISsE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<std::string>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorISsE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x48b850() {
    // IDA 0x48b850: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48ba40 — __ZN3RBX10Reflection23TypedPropertyDescriptorISsED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<std::string>::~TypedPropertyDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorISsED1Ev")]
pub fn stub_0x48ba40() {
    // IDA 0x48ba40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48ba68 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::GetImpl<std::string (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE10isReadOnlyEv")]
pub fn stub_0x48ba68() {
    // IDA 0x48ba68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48ba6c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::GetImpl<std::string (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE11isWriteOnlyEv")]
pub fn stub_0x48ba6c() {
    // IDA 0x48ba6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48ba70 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::GetImpl<std::string (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x48ba70() {
    // IDA 0x48ba70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48ba98 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::GetImpl<std::string (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE8setValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x48ba98() {
    // IDA 0x48ba98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x48fc28 — __ZN3RBX10Reflection4Type12getSingletonINS_9TextureIdEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextureId>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_9TextureIdEEERKS1_v")]
pub fn stub_0x48fc28() {
    // IDA 0x48fc28: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x48fc2c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x48fc2c() {
    // IDA 0x48fc2c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x48fe14 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x48fe14() {
    // IDA 0x48fe14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x48ffbc — __ZN3RBX10Reflection7Variant7convertINS_9TextureIdEEERT_v
#[doc(alias = "RBX::TextureId & RBX::Reflection::Variant::convert<RBX::TextureId>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant7convertINS_9TextureIdEEERT_v")]
pub fn stub_0x48ffbc() {
    // IDA 0x48ffbc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x4901a8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE11getDataSizeEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE11getDataSizeEPKNS0_13DescribedBaseE")]
pub fn stub_0x4901a8() {
    // IDA 0x4901a8: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x490204 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14hasStringValueEv")]
pub fn stub_0x490204() {
    // IDA 0x490204: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x490208 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x490208() {
    // IDA 0x490208: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

// 0x490324 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x490324() {
    // IDA 0x490324: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x490770 — __ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEED1Ev")]
pub fn stub_0x490770() {
    // IDA 0x490770: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x49079c — __ZN3RBX10Reflection14PropDescriptorINS_5DecalEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5DecalEfED1Ev")]
pub fn stub_0x49079c() {
    // IDA 0x49079c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4907d0 — __ZN3RBX10Reflection7Variant14genericConvertINS_9TextureIdEEERT_v
#[doc(alias = "RBX::TextureId & RBX::Reflection::Variant::genericConvert<RBX::TextureId>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_9TextureIdEEERT_v")]
pub fn stub_0x4907d0() {
    // IDA 0x4907d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x490a84 — __ZN3RBX10Reflection14PropDescriptorINS_7TextureEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextureEfED1Ev")]
pub fn stub_0x490a84() {
    // IDA 0x490a84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x490ab0 — __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x490ab0() {
    // IDA 0x490ab0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x490ab4 — __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x490ab4() {
    // IDA 0x490ab4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x490bd4 — __ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x490bd4() {
    // IDA 0x490bd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x490d04 — __ZThn32_NK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x490d04() {
    // IDA 0x490d04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x490f50 — __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x490f50() {
    // IDA 0x490f50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x491080 — __ZThn32_NK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x491080() {
    // IDA 0x491080: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4911b0 — __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE17static_getCreatorEv")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x4911b0() {
    // IDA 0x4911b0: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}

// 0x491224 — __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x491224() {
    // IDA 0x491224: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}

// 0x491390 — __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE17static_getCreatorEv")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x491390() {
    // IDA 0x491390: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}

// 0x491404 — __ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x491404() {
    // IDA 0x491404: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}

// 0x491570 — __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x491570() {
    // IDA 0x491570: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x49160c — __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator6createEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x49160c() {
    // IDA 0x49160c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x491afc — __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorC2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x491afc() {
    // IDA 0x491afc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x491d40 — __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x491d40() {
    // IDA 0x491d40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x491ddc — __ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator6createEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x491ddc() {
    // IDA 0x491ddc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
