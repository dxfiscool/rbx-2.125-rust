// Auto-generated shard BT — next 100 RBX::Reflection stubs — EA-sorted ascending 0xf39544..0xf3ad14 (remaining 2913) — starts 0xf39544
// Source: ida/export.json filtered demangled contains RBX::Reflection (16171 total, 13258 prior -> 13358 total)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr (was boost::shared_ptr)
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;

// 0xf39544 — j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::addPair(RBX::TaskScheduler::ThreadPoolConfig,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE7addPairES3_PKc")]
pub fn stub_f39544(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf39544: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf39554 — j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE9addLegacyEiPKcS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::addLegacy(int,char const*,RBX::TaskScheduler::ThreadPoolConfig)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE9addLegacyEiPKcS3_")]
pub fn stub_f39554(desc: &mut crate::enum_desc::EnumDesc, legacy_index: usize, name: &str, value: i32) {
    // IDA 0xf39554: EnumDesc<T>::addLegacy -- grow legacy vector, map legacy name->value (decompiled 0x47cd20, model 0xa208). Delegates to the shared model.
    desc.add_legacy(legacy_index, name, value)
}

// 0xf39564 — j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEED2Ev")]
pub fn stub_f39564() {
    // IDA 0xf39564: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf39574 — j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE7addPairES4_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::addPair(RBX::TaskScheduler::Job::SleepAdjustMethod,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE7addPairES4_PKc")]
pub fn stub_f39574(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf39574: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf39584 — j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEED2Ev")]
pub fn stub_f39584() {
    // IDA 0xf39584: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf39594 — j___ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::addPair(RBX::Time::SampleMethod,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE7addPairES3_PKc")]
pub fn stub_f39594(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf39594: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf395a4 — j___ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEED2Ev")]
pub fn stub_f395a4() {
    // IDA 0xf395a4: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf395b4 — j___ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::addPair(RBX::EThrottle::EThrottleType,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE7addPairES3_PKc")]
pub fn stub_f395b4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf395b4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf395c4 — j___ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEED2Ev")]
pub fn stub_f395c4() {
    // IDA 0xf395c4: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf395d4 — j___ZN3RBX10Reflection9ArgHelper6getArgIdLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: double RBX::Reflection::ArgHelper::getArg<double,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
#[doc(alias = "double RBX::Reflection::ArgHelper::getArg<double,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper6getArgIdLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_f395d4() -> ! {
    todo!("0xf395d4 double RBX::Reflection::ArgHelper::getArg<double,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf395e4 — j___ZN3RBX10Reflection9ArgHelper6getArgIdLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: double RBX::Reflection::ArgHelper::getArg<double,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
#[doc(alias = "double RBX::Reflection::ArgHelper::getArg<double,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper6getArgIdLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_f395e4() -> ! {
    todo!("0xf395e4 double RBX::Reflection::ArgHelper::getArg<double,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf395f4 — j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13DebugSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::DebugSettings>(char const*,char const*,bool RBX::DebugSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13DebugSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f395f4() -> ! {
    todo!("0xf395f4 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::DebugSettings>(char const*,char const*,bool RBX::DebugSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf39644 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv")]
pub fn stub_f39644() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf39644: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x47aee0)
}

// 0xf39654 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE14doGetSingletonEv")]
pub fn stub_f39654() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf39654: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x47ab28)
}

// 0xf39664 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv")]
pub fn stub_f39664() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf39664: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x47a87c)
}

// 0xf39674 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE14doGetSingletonEv")]
pub fn stub_f39674() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf39674: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x47ad04)
}

// 0xf39684 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEEE14doGetSingletonEv")]
pub fn stub_f39684() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf39684: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x4727ec)
}

// 0xf39694 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Time::SampleMethod> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE14doGetSingletonEv")]
pub fn stub_f39694() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf39694: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x47b2f4)
}

// 0xf39794 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKNS1_10Reflection5TupleEEEEERS3_RKT_
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<boost::shared_ptr<RBX::Reflection::Tuple const>>(boost::shared_ptr<RBX::Reflection::Tuple const> const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKNS1_10Reflection5TupleEEEEERS3_RKT_")]
pub fn stub_f39794() -> ! {
    todo!("0xf39794 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")
}

// 0xf39934 — j___ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEaSERKS5_
// was: boost::shared_ptr<RBX::Reflection::Tuple const>::operator=(boost::shared_ptr<RBX::Reflection::Tuple const> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const>::operator=(rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEaSERKS5_")]
pub fn stub_f39934() -> ! {
    todo!("0xf39934 rbx_core::SharedPtr<RBX::Reflection::Tuple const>::operator=(rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")
}

// 0xf399b4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f399b4() -> ! {
    todo!("0xf399b4 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf399c4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f399c4() -> ! {
    todo!("0xf399c4 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf399d4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f399d4() -> ! {
    todo!("0xf399d4 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf399e4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f399e4() -> ! {
    todo!("0xf399e4 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf399f4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f399f4() -> ! {
    todo!("0xf399f4 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf39a04 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f39a04() -> ! {
    todo!("0xf39a04 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf39a14 — j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToItem(RBX::DebugSettings::ErrorReporting const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE13convertToItemERKS3_")]
pub fn stub_f39a14(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf39a14: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf39a24 — j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToIndex(RBX::DebugSettings::ErrorReporting)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_")]
pub fn stub_f39a24(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf39a24: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf39a34 — j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToValue(RBX::Name const&,RBX::DebugSettings::ErrorReporting&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_f39a34(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf39a34: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf39a44 — j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToString(RBX::DebugSettings::ErrorReporting const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE15convertToStringERKS3_")]
pub fn stub_f39a44(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf39a44: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf39a54 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToItem(RBX::TaskScheduler::PriorityMethod const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE13convertToItemERKS3_")]
pub fn stub_f39a54(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf39a54: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf39a64 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToIndex(RBX::TaskScheduler::PriorityMethod)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToIndexES3_")]
pub fn stub_f39a64(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf39a64: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf39a74 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToValue(RBX::Name const&,RBX::TaskScheduler::PriorityMethod&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_f39a74(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf39a74: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf39a84 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToString(RBX::TaskScheduler::PriorityMethod const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE15convertToStringERKS3_")]
pub fn stub_f39a84(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf39a84: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf39a94 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToItem(RBX::TaskScheduler::ThreadPoolConfig const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE13convertToItemERKS3_")]
pub fn stub_f39a94(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf39a94: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf39aa4 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToIndex(RBX::TaskScheduler::ThreadPoolConfig)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_")]
pub fn stub_f39aa4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf39aa4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf39ab4 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToValue(RBX::Name const&,RBX::TaskScheduler::ThreadPoolConfig&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_f39ab4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf39ab4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf39ac4 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToString(RBX::TaskScheduler::ThreadPoolConfig const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE15convertToStringERKS3_")]
pub fn stub_f39ac4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf39ac4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf39ad4 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE13convertToItemERKS4_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToItem(RBX::TaskScheduler::Job::SleepAdjustMethod const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE13convertToItemERKS4_")]
pub fn stub_f39ad4(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf39ad4: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf39ae4 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToIndexES4_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToIndex(RBX::TaskScheduler::Job::SleepAdjustMethod)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToIndexES4_")]
pub fn stub_f39ae4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf39ae4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf39af4 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToValueERKNS_4NameERS4_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToValue(RBX::Name const&,RBX::TaskScheduler::Job::SleepAdjustMethod&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToValueERKNS_4NameERS4_")]
pub fn stub_f39af4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf39af4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf39b04 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE15convertToStringERKS4_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToString(RBX::TaskScheduler::Job::SleepAdjustMethod const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE15convertToStringERKS4_")]
pub fn stub_f39b04(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf39b04: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf39b14 — j___ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToIndex(RBX::DataModelArbiter::ConcurrencyModel)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToIndexES3_")]
pub fn stub_f39b14(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf39b14: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf39b24 — j___ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToValue(RBX::Name const&,RBX::DataModelArbiter::ConcurrencyModel&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_f39b24(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf39b24: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf39b34 — j___ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToString(RBX::DataModelArbiter::ConcurrencyModel const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE15convertToStringERKS3_")]
pub fn stub_f39b34(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf39b34: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf39b44 — j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToItem(RBX::Time::SampleMethod const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE13convertToItemERKS3_")]
pub fn stub_f39b44(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf39b44: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf39b54 — j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToIndex(RBX::Time::SampleMethod)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToIndexES3_")]
pub fn stub_f39b54(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf39b54: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf39b64 — j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToValue(RBX::Name const&,RBX::Time::SampleMethod&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_f39b64(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf39b64: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf39b74 — j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToString(RBX::Time::SampleMethod const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE15convertToStringERKS3_")]
pub fn stub_f39b74(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf39b74: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf39b84 — j___ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToItem(RBX::EThrottle::EThrottleType const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE13convertToItemERKS3_")]
pub fn stub_f39b84(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf39b84: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf39b94 — j___ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToValue(RBX::Name const&,RBX::EThrottle::EThrottleType&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_f39b94(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf39b94: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf39ba4 — j___ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToString(RBX::EThrottle::EThrottleType const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE15convertToStringERKS3_")]
pub fn stub_f39ba4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf39ba4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3a024 — j___ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEEC2IMS2_KFRKS3_vEMS2_FvS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::PropDescriptor<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEEC2IMS2_KFRKS3_vEMS2_FvS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f3a024() -> ! {
    todo!("0xf3a024 RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::PropDescriptor<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf3a034 — j___ZN3RBX10Reflection14PropDescriptorINS_5DecalEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,float>::PropDescriptor<float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float)>(char const*,char const*,float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_5DecalEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f3a034() -> ! {
    todo!("0xf3a034 RBX::Reflection::PropDescriptor<RBX::Decal,float>::PropDescriptor<float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float)>(char const*,char const*,float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf3a044 — j___ZN3RBX10Reflection14PropDescriptorINS_7TextureEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::PropDescriptor<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>(char const*,char const*,float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_7TextureEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f3a044() -> ! {
    todo!("0xf3a044 RBX::Reflection::PropDescriptor<RBX::Texture,float>::PropDescriptor<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>(char const*,char const*,float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf3a054 — j___ZN3RBX10Reflection7Variant14genericConvertINS_9TextureIdEEERT_v
#[doc(alias = "RBX::TextureId & RBX::Reflection::Variant::genericConvert<RBX::TextureId>(void)")]
#[doc(alias = "j___ZN3RBX10Reflection7Variant14genericConvertINS_9TextureIdEEERT_v")]
pub fn stub_f3a054() -> ! {
    todo!("0xf3a054 RBX::TextureId & RBX::Reflection::Variant::genericConvert<RBX::TextureId>(void)")
}

// 0xf3a194 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5DecalES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Decal,RBX::Decal>(boost::shared_ptr<RBX::Decal> const*,RBX::Decal *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Decal,RBX::Decal>(rbx_core::SharedPtr<RBX::Decal> const*,RBX::Decal *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5DecalES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f3a194() {
    // IDA 0xf3a194: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3a1a4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextureES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Texture,RBX::Texture>(boost::shared_ptr<RBX::Texture> const*,RBX::Texture *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Texture,RBX::Texture>(rbx_core::SharedPtr<RBX::Texture> const*,RBX::Texture *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextureES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f3a1a4() {
    // IDA 0xf3a1a4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3a1b4 — j___ZN3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::PropDescriptor<std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string)>(char const*,char const*,std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f3a1b4() -> ! {
    todo!("0xf3a1b4 RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::PropDescriptor<std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string)>(char const*,char const*,std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf3a254 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12DialogChoiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DialogChoice,RBX::DialogChoice>(boost::shared_ptr<RBX::DialogChoice> const*,RBX::DialogChoice *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DialogChoice,RBX::DialogChoice>(rbx_core::SharedPtr<RBX::DialogChoice> const*,RBX::DialogChoice *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12DialogChoiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f3a254() {
    // IDA 0xf3a254: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3a274 — j___ZN3RBX10Reflection11Call2HelperINS_10DialogRootEMS2_FvN5boost10shared_ptrINS_8InstanceEEES6_ES6_S6_vE4callEPS2_S8_RNS0_7VariantERKS6_SE_
// was: RBX::Reflection::Call2Helper<RBX::DialogRoot,void (RBX::DialogRoot::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,void>::call(RBX::DialogRoot*,void (RBX::DialogRoot::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DialogRoot,void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::DialogRoot*,void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "j___ZN3RBX10Reflection11Call2HelperINS_10DialogRootEMS2_FvN5boost10shared_ptrINS_8InstanceEEES6_ES6_S6_vE4callEPS2_S8_RNS0_7VariantERKS6_SE_")]
pub fn stub_f3a274() -> ! {
    todo!("0xf3a274 RBX::Reflection::Call2Helper<RBX::DialogRoot,void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::DialogRoot*,void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf3a284 — j___ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
// was: RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_")]
pub fn stub_f3a284() -> ! {
    todo!("0xf3a284 RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0xf3a294 — j___ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EEC2EMS2_FvS6_S6_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::DialogRoot::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EEC2EMS2_FvS6_S6_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_f3a294() -> ! {
    todo!("0xf3a294 RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf3a2a4 — j___ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::PropDescriptor<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>(char const*,char const*,std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f3a2a4() -> ! {
    todo!("0xf3a2a4 RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::PropDescriptor<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>(char const*,char const*,std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf3a2b4 — j___ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::PropDescriptor<bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool)>(char const*,char const*,bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f3a2b4() -> ! {
    todo!("0xf3a2b4 RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::PropDescriptor<bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool)>(char const*,char const*,bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf3a2c4 — j___ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::PropDescriptor<float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float)>(char const*,char const*,float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f3a2c4() -> ! {
    todo!("0xf3a2c4 RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::PropDescriptor<float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float)>(char const*,char const*,float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf3a2d4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::EnumPropDescriptor<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>(char const*,char const*,RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f3a2d4() -> ! {
    todo!("0xf3a2d4 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::EnumPropDescriptor<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>(char const*,char const*,RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf3a2e4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::EnumPropDescriptor<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>(char const*,char const*,RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f3a2e4() -> ! {
    todo!("0xf3a2e4 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::EnumPropDescriptor<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>(char const*,char const*,RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf3a2f4 — j___ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceES6_S6_
// was: RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "j___ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceES6_S6_")]
pub fn stub_f3a2f4() -> ! {
    todo!("0xf3a2f4 RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xf3a304 — j___ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_S6_S6_
// was: RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::DialogRoot*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::DialogRoot*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "j___ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_S6_S6_")]
pub fn stub_f3a304() -> ! {
    todo!("0xf3a304 RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::DialogRoot*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xf3a314 — j___ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::addPair(RBX::DialogRoot::DialogTone,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE7addPairES3_PKc")]
pub fn stub_f3a314(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3a314: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3a324 — j___ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::addPair(RBX::DialogRoot::DialogPurpose,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE7addPairES3_PKc")]
pub fn stub_f3a324(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3a324: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3a334 — j___ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_f3a334() -> ! {
    todo!("0xf3a334 RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf3a3e4 — j___ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_S6_
// was: RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::fireEvent(RBX::DialogRoot*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)const
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::fireEvent(RBX::DialogRoot*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")]
#[doc(alias = "j___ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_S6_")]
pub fn stub_f3a3e4() -> ! {
    todo!("0xf3a3e4 RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::fireEvent(RBX::DialogRoot*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0xf3a3f4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f3a3f4() -> ! {
    todo!("0xf3a3f4 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf3a404 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f3a404() -> ! {
    todo!("0xf3a404 RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf3a414 — j___ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToIndex(RBX::DialogRoot::DialogTone)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToIndexES3_")]
pub fn stub_f3a414(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf3a414: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf3a424 — j___ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToIndex(RBX::DialogRoot::DialogPurpose)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToIndexES3_")]
pub fn stub_f3a424(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf3a424: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf3a444 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10DialogRootES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DialogRoot,RBX::DialogRoot>(boost::shared_ptr<RBX::DialogRoot> const*,RBX::DialogRoot *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DialogRoot,RBX::DialogRoot>(rbx_core::SharedPtr<RBX::DialogRoot> const*,RBX::DialogRoot *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10DialogRootES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f3a444() {
    // IDA 0xf3a444: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3a594 — j___ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::addPair(RBX::PrismInstance::NumSidesEnum,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE7addPairES3_PKc")]
pub fn stub_f3a594(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3a594: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3a5a4 — j___ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::addPair(RBX::PyramidInstance::NumSidesEnum,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE7addPairES3_PKc")]
pub fn stub_f3a5a4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3a5a4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3a5b4 — j___ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::addPair(RBX::BasicPartInstance::LegacyPartType,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE7addPairES3_PKc")]
pub fn stub_f3a5b4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3a5b4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3a5c4 — j___ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::addPair(RBX::ExtrudedPartInstance::VisualTrussStyle,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE7addPairES3_PKc")]
pub fn stub_f3a5c4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3a5c4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3a5d4 — j___ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::addPair(RBX::Handles::VisualStyle,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE7addPairES3_PKc")]
pub fn stub_f3a5d4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3a5d4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3a5e4 — j___ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::addPair(RBX::GuiObject::SizeConstraint,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE7addPairES3_PKc")]
pub fn stub_f3a5e4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3a5e4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3a9b4 — j___ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::PropDescriptor<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>(char const*,char const*,float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f3a9b4() -> ! {
    todo!("0xf3a9b4 RBX::Reflection::PropDescriptor<RBX::Explosion,float>::PropDescriptor<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>(char const*,char const*,float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf3a9c4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::EnumPropDescriptor<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>(char const*,char const*,RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f3a9c4() -> ! {
    todo!("0xf3a9c4 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::EnumPropDescriptor<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>(char const*,char const*,RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf3a9d4 — j___ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::addPair(RBX::Explosion::ExplosionType,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE7addPairES3_PKc")]
pub fn stub_f3a9d4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3a9d4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3a9e4 — j___ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED2Ev")]
pub fn stub_f3a9e4() {
    // IDA 0xf3a9e4: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf3a9f4 — j___ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS9_MT_S3_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,G3D::Vector3 RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS9_MT_S3_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f3a9f4() -> ! {
    todo!("0xf3a9f4 RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,G3D::Vector3 RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf3aa04 — j___ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,float RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f3aa04() -> ! {
    todo!("0xf3aa04 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,float RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf3aa14 — j___ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::EventDesc<RBX::Explosion,void ()(boost::shared_ptr<RBX::Instance>,float),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)> RBX::Explosion::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)> RBX::Explosion::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_f3aa14() -> ! {
    todo!("0xf3aa14 RBX::Reflection::EventDesc<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf3abe4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKfEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS9_fEEvRT_RT0_
// was: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<boost::shared_ptr<RBX::Instance>,float>(boost::shared_ptr<RBX::Instance> &,float &)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,float>(rbx_core::SharedPtr<RBX::Instance> &,float &)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKfEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS9_fEEvRT_RT0_")]
pub fn stub_f3abe4() -> ! {
    todo!("0xf3abe4 void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,float>(rbx_core::SharedPtr<RBX::Instance> &,float &)")
}

// 0xf3ac04 — j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKfNS4_IS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,float const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKfNS4_IS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_")]
pub fn stub_f3ac04() -> ! {
    todo!("0xf3ac04 boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")
}

// 0xf3ac34 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_f3ac34() -> ! {
    todo!("0xf3ac34 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf3ac94 — j___ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
// was: void boost::function2<void,boost::shared_ptr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
#[doc(alias = "void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
#[doc(alias = "j___ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_")]
pub fn stub_f3ac94() -> ! {
    todo!("0xf3ac94 void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0xf3acb4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f3acb4() -> ! {
    todo!("0xf3acb4 RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf3acc4 — j___ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToItem(RBX::Explosion::ExplosionType const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE13convertToItemERKS3_")]
pub fn stub_f3acc4(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3acc4: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3acd4 — j___ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToIndex(RBX::Explosion::ExplosionType)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToIndexES3_")]
pub fn stub_f3acd4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf3acd4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf3ad14 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TimerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TimerService,RBX::TimerService>(boost::shared_ptr<RBX::TimerService> const*,RBX::TimerService *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TimerService,RBX::TimerService>(rbx_core::SharedPtr<RBX::TimerService> const*,RBX::TimerService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TimerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f3ad14() {
    // IDA 0xf3ad14: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}
