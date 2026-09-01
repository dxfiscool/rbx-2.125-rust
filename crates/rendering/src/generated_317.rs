//! rendering shard 317 — 100 stubs 0x47c418..0x47f10c EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 34500->34600 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 34500 before -> 34600 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x47c418 (lowest remaining 0x47c418..0x47f10c, next lowest 0x47f13c)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x47c418 — __ZN3RBX21TaskSchedulerSettings19setThreadPoolConfigENS_13TaskScheduler16ThreadPoolConfigE
#[doc(alias = "RBX::TaskSchedulerSettings::setThreadPoolConfig(RBX::TaskScheduler::ThreadPoolConfig)")]
// was: __ZN3RBX21TaskSchedulerSettings19setThreadPoolConfigENS_13TaskScheduler16ThreadPoolConfigE
pub fn stub_47c418() -> ! {
    todo!("0x47c418 RBX::TaskSchedulerSettings::setThreadPoolConfig(RBX::TaskScheduler::ThreadPoolConfig)")
}

// 0x47c460 — __ZN3RBX21TaskSchedulerSettings14setThreadShareEdi
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this, double, int)
#[doc(alias = "RBX::TaskSchedulerSettings::setThreadShare(double,int)")]
// was: __ZN3RBX21TaskSchedulerSettings14setThreadShareEdi
pub fn stub_47c460() -> ! {
    todo!("0x47c460 RBX::TaskSchedulerSettings::setThreadShare(double,int)")
}

// 0x47c464 — __ZN3RBX21TaskSchedulerSettings17setPriorityMethodENS_13TaskScheduler14PriorityMethodE
#[doc(alias = "RBX::TaskSchedulerSettings::setPriorityMethod(RBX::TaskScheduler::PriorityMethod)")]
// was: __ZN3RBX21TaskSchedulerSettings17setPriorityMethodENS_13TaskScheduler14PriorityMethodE
pub fn stub_47c464() -> ! {
    todo!("0x47c464 RBX::TaskSchedulerSettings::setPriorityMethod(RBX::TaskScheduler::PriorityMethod)")
}

// 0x47c4a0 — __ZN3RBX21TaskSchedulerSettings20setSleepAdjustMethodENS_13TaskScheduler3Job17SleepAdjustMethodE
#[doc(alias = "RBX::TaskSchedulerSettings::setSleepAdjustMethod(RBX::TaskScheduler::Job::SleepAdjustMethod)")]
// was: __ZN3RBX21TaskSchedulerSettings20setSleepAdjustMethodENS_13TaskScheduler3Job17SleepAdjustMethodE
pub fn stub_47c4a0() -> ! {
    todo!("0x47c4a0 RBX::TaskSchedulerSettings::setSleepAdjustMethod(RBX::TaskScheduler::Job::SleepAdjustMethod)")
}

// 0x47c4dc — __ZN3RBX21TaskSchedulerSettings19setConcurrencyModelENS_16DataModelArbiter16ConcurrencyModelE
#[doc(alias = "RBX::TaskSchedulerSettings::setConcurrencyModel(RBX::DataModelArbiter::ConcurrencyModel)")]
// was: __ZN3RBX21TaskSchedulerSettings19setConcurrencyModelENS_16DataModelArbiter16ConcurrencyModelE
pub fn stub_47c4dc() -> ! {
    todo!("0x47c4dc RBX::TaskSchedulerSettings::setConcurrencyModel(RBX::DataModelArbiter::ConcurrencyModel)")
}

// 0x47c518 — __ZN3RBX21TaskSchedulerSettings21setIsArbiterThrottledEb
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this, bool)
#[doc(alias = "RBX::TaskSchedulerSettings::setIsArbiterThrottled(bool)")]
// was: __ZN3RBX21TaskSchedulerSettings21setIsArbiterThrottledEb
pub fn stub_47c518() -> ! {
    todo!("0x47c518 RBX::TaskSchedulerSettings::setIsArbiterThrottled(bool)")
}

// 0x47c53c — __ZN3RBX21TaskSchedulerSettings24setThrottledJobSleepTimeEd
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this, double)
#[doc(alias = "RBX::TaskSchedulerSettings::setThrottledJobSleepTime(double)")]
// was: __ZN3RBX21TaskSchedulerSettings24setThrottledJobSleepTimeEd
pub fn stub_47c53c() -> ! {
    todo!("0x47c53c RBX::TaskSchedulerSettings::setThrottledJobSleepTime(double)")
}

// 0x47c564 — __ZNK3RBX13DebugSettings21getIsProfilingEnabledEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getIsProfilingEnabled(void)const")]
// was: __ZNK3RBX13DebugSettings21getIsProfilingEnabledEv
pub fn stub_47c564() -> ! {
    todo!("0x47c564 RBX::DebugSettings::getIsProfilingEnabled(void)const")
}

// 0x47c570 — __ZN3RBX13DebugSettings21setIsProfilingEnabledEb
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this, RBX::Profiling *)
#[doc(alias = "RBX::DebugSettings::setIsProfilingEnabled(bool)")]
// was: __ZN3RBX13DebugSettings21setIsProfilingEnabledEb
pub fn stub_47c570() -> ! {
    todo!("0x47c570 RBX::DebugSettings::setIsProfilingEnabled(bool)")
}

// 0x47c578 — __ZNK3RBX13DebugSettings18getProfilingWindowEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getProfilingWindow(void)const")]
// was: __ZNK3RBX13DebugSettings18getProfilingWindowEv
pub fn stub_47c578() -> ! {
    todo!("0x47c578 RBX::DebugSettings::getProfilingWindow(void)const")
}

// 0x47c590 — __ZN3RBX13DebugSettings18setProfilingWindowEd
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this, double)
#[doc(alias = "RBX::DebugSettings::setProfilingWindow(double)")]
// was: __ZN3RBX13DebugSettings18setProfilingWindowEd
pub fn stub_47c590() -> ! {
    todo!("0x47c590 RBX::DebugSettings::setProfilingWindow(double)")
}

// 0x47c5a8 — __ZNK3RBX13DebugSettings14getLuaRamLimitEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getLuaRamLimit(void)const")]
// was: __ZNK3RBX13DebugSettings14getLuaRamLimitEv
pub fn stub_47c5a8() -> ! {
    todo!("0x47c5a8 RBX::DebugSettings::getLuaRamLimit(void)const")
}

// 0x47c5b8 — __ZN3RBX13DebugSettings14setLuaRamLimitEi
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this, int)
#[doc(alias = "RBX::DebugSettings::setLuaRamLimit(int)")]
// was: __ZN3RBX13DebugSettings14setLuaRamLimitEi
pub fn stub_47c5b8() -> ! {
    todo!("0x47c5b8 RBX::DebugSettings::setLuaRamLimit(int)")
}

// 0x47c5c8 — __ZNK3RBX13DebugSettings21getInstanceCountLimitEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getInstanceCountLimit(void)const")]
// was: __ZNK3RBX13DebugSettings21getInstanceCountLimitEv
pub fn stub_47c5c8() -> ! {
    todo!("0x47c5c8 RBX::DebugSettings::getInstanceCountLimit(void)const")
}

// 0x47c5d8 — __ZN3RBX13DebugSettings21setInstanceCountLimitEi
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this, int)
#[doc(alias = "RBX::DebugSettings::setInstanceCountLimit(int)")]
// was: __ZN3RBX13DebugSettings21setInstanceCountLimitEi
pub fn stub_47c5d8() -> ! {
    todo!("0x47c5d8 RBX::DebugSettings::setInstanceCountLimit(int)")
}

// 0x47c5e8 — __ZNK3RBX13DebugSettings28getEnforceInstanceCountLimitEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getEnforceInstanceCountLimit(void)const")]
// was: __ZNK3RBX13DebugSettings28getEnforceInstanceCountLimitEv
pub fn stub_47c5e8() -> ! {
    todo!("0x47c5e8 RBX::DebugSettings::getEnforceInstanceCountLimit(void)const")
}

// 0x47c5f8 — __ZN3RBX13DebugSettings28setEnforceInstanceCountLimitEb
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this, bool)
#[doc(alias = "RBX::DebugSettings::setEnforceInstanceCountLimit(bool)")]
// was: __ZN3RBX13DebugSettings28setEnforceInstanceCountLimitEb
pub fn stub_47c5f8() -> ! {
    todo!("0x47c5f8 RBX::DebugSettings::setEnforceInstanceCountLimit(bool)")
}

// 0x47c608 — __ZN3RBX13DebugSettingsC1Ev
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::DebugSettings(void)")]
// was: __ZN3RBX13DebugSettingsC1Ev
pub fn stub_47c608() -> ! {
    todo!("0x47c608 RBX::DebugSettings::DebugSettings(void)")
}

// 0x47c60c — __ZN3RBX13DebugSettingsC2Ev
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::DebugSettings(void)")]
// was: __ZN3RBX13DebugSettingsC2Ev
pub fn stub_47c60c() -> ! {
    todo!("0x47c60c RBX::DebugSettings::DebugSettings(void)")
}

// 0x47c7e4 — __ZN12DummyArbiter12areExclusiveEPN3RBX13TaskScheduler3JobES3_
// type: _DWORD __fastcall(DummyArbiter *__hidden this, RBX::TaskScheduler::Job *, RBX::TaskScheduler::Job *)
#[doc(alias = "DummyArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
// was: __ZN12DummyArbiter12areExclusiveEPN3RBX13TaskScheduler3JobES3_
pub fn stub_47c7e4() -> ! {
    todo!("0x47c7e4 DummyArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")
}

// 0x47c800 — __ZN3RBX21TaskSchedulerSettingsC2Ev
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::TaskSchedulerSettings(void)")]
// was: __ZN3RBX21TaskSchedulerSettingsC2Ev
pub fn stub_47c800() -> ! {
    todo!("0x47c800 RBX::TaskSchedulerSettings::TaskSchedulerSettings(void)")
}

// 0x47c9c0 — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::addPair(RBX::TaskScheduler::ThreadPoolConfig,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE7addPairES3_PKc
pub fn stub_47c9c0() -> ! {
    todo!("0x47c9c0 RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::addPair(RBX::TaskScheduler::ThreadPoolConfig,char const*)")
}

// 0x47cd20 — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE9addLegacyEiPKcS3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::addLegacy(int,char const*,RBX::TaskScheduler::ThreadPoolConfig)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE9addLegacyEiPKcS3_
pub fn stub_47cd20() -> ! {
    todo!("0x47cd20 RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::addLegacy(int,char const*,RBX::TaskScheduler::ThreadPoolConfig)")
}

// 0x47cd74 — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::addPair(RBX::TaskScheduler::PriorityMethod,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE7addPairES3_PKc
pub fn stub_47cd74() -> ! {
    todo!("0x47cd74 RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::addPair(RBX::TaskScheduler::PriorityMethod,char const*)")
}

// 0x47d0d4 — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE7addPairES4_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::addPair(RBX::TaskScheduler::Job::SleepAdjustMethod,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE7addPairES4_PKc
pub fn stub_47d0d4() -> ! {
    todo!("0x47d0d4 RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::addPair(RBX::TaskScheduler::Job::SleepAdjustMethod,char const*)")
}

// 0x47d434 — __ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::addPair(RBX::DebugSettings::ErrorReporting,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE7addPairES3_PKc
pub fn stub_47d434() -> ! {
    todo!("0x47d434 RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::addPair(RBX::DebugSettings::ErrorReporting,char const*)")
}

// 0x47d794 — __ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::addPair(RBX::EThrottle::EThrottleType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE7addPairES3_PKc
pub fn stub_47d794() -> ! {
    todo!("0x47d794 RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::addPair(RBX::EThrottle::EThrottleType,char const*)")
}

// 0x47daf4 — __ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::addPair(RBX::Time::SampleMethod,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE7addPairES3_PKc
pub fn stub_47daf4() -> ! {
    todo!("0x47daf4 RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::addPair(RBX::Time::SampleMethod,char const*)")
}

// 0x47de54 — __ZNK3RBX13DebugSettings16getRobloxVersionEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getRobloxVersion(void)const")]
// was: __ZNK3RBX13DebugSettings16getRobloxVersionEv
pub fn stub_47de54() -> ! {
    todo!("0x47de54 RBX::DebugSettings::getRobloxVersion(void)const")
}

// 0x47dec8 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsED1Ev
pub fn stub_47dec8() -> ! {
    todo!("0x47dec8 RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::~PropDescriptor()")
}

// 0x47deec — __ZNK3RBX13DebugSettings20getRobloxProductNameEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getRobloxProductName(void)const")]
// was: __ZNK3RBX13DebugSettings20getRobloxProductNameEv
pub fn stub_47deec() -> ! {
    todo!("0x47deec RBX::DebugSettings::getRobloxProductName(void)const")
}

// 0x47df60 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfED1Ev
pub fn stub_47df60() -> ! {
    todo!("0x47df60 RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::~PropDescriptor()")
}

// 0x47df84 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiED1Ev
pub fn stub_47df84() -> ! {
    todo!("0x47df84 RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::~PropDescriptor()")
}

// 0x47dfa8 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbED1Ev
pub fn stub_47dfa8() -> ! {
    todo!("0x47dfa8 RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::~PropDescriptor()")
}

// 0x47dfcc — __ZNK3RBX13DebugSettings16nameDatabaseSizeEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::nameDatabaseSize(void)const")]
// was: __ZNK3RBX13DebugSettings16nameDatabaseSizeEv
pub fn stub_47dfcc() -> ! {
    todo!("0x47dfcc RBX::DebugSettings::nameDatabaseSize(void)const")
}

// 0x47dfd0 — __ZNK3RBX13DebugSettings17nameDatabaseBytesEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::nameDatabaseBytes(void)const")]
// was: __ZNK3RBX13DebugSettings17nameDatabaseBytesEv
pub fn stub_47dfd0() -> ! {
    todo!("0x47dfd0 RBX::DebugSettings::nameDatabaseBytes(void)const")
}

// 0x47dfd4 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdED1Ev
pub fn stub_47dfd4() -> ! {
    todo!("0x47dfd4 RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::~PropDescriptor()")
}

// 0x47dff8 — __ZNK3RBX13DebugSettings13instanceCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::instanceCount(void)const")]
// was: __ZNK3RBX13DebugSettings13instanceCountEv
pub fn stub_47dff8() -> ! {
    todo!("0x47dff8 RBX::DebugSettings::instanceCount(void)const")
}

// 0x47e008 — __ZNK3RBX13DebugSettings8jobCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::jobCount(void)const")]
// was: __ZNK3RBX13DebugSettings8jobCountEv
pub fn stub_47e008() -> ! {
    todo!("0x47e008 RBX::DebugSettings::jobCount(void)const")
}

// 0x47e018 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED1Ev
pub fn stub_47e018() -> ! {
    todo!("0x47e018 RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()")
}

// 0x47e03c — __ZNK3RBX21TaskSchedulerSettings14threadPoolSizeEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::threadPoolSize(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings14threadPoolSizeEv
pub fn stub_47e03c() -> ! {
    todo!("0x47e03c RBX::TaskSchedulerSettings::threadPoolSize(void)const")
}

// 0x47e050 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiED1Ev
pub fn stub_47e050() -> ! {
    todo!("0x47e050 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::~PropDescriptor()")
}

// 0x47e074 — __ZNK3RBX21TaskSchedulerSettings14threadAffinityEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::threadAffinity(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings14threadAffinityEv
pub fn stub_47e074() -> ! {
    todo!("0x47e074 RBX::TaskSchedulerSettings::threadAffinity(void)const")
}

// 0x47e088 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdED1Ev
pub fn stub_47e088() -> ! {
    todo!("0x47e088 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::~PropDescriptor()")
}

// 0x47e0ac — __ZNK3RBX21TaskSchedulerSettings15numSleepingJobsEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::numSleepingJobs(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings15numSleepingJobsEv
pub fn stub_47e0ac() -> ! {
    todo!("0x47e0ac RBX::TaskSchedulerSettings::numSleepingJobs(void)const")
}

// 0x47e0c0 — __ZNK3RBX21TaskSchedulerSettings14numWaitingJobsEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::numWaitingJobs(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings14numWaitingJobsEv
pub fn stub_47e0c0() -> ! {
    todo!("0x47e0c0 RBX::TaskSchedulerSettings::numWaitingJobs(void)const")
}

// 0x47e0d4 — __ZNK3RBX21TaskSchedulerSettings14numRunningJobsEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::numRunningJobs(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings14numRunningJobsEv
pub fn stub_47e0d4() -> ! {
    todo!("0x47e0d4 RBX::TaskSchedulerSettings::numRunningJobs(void)const")
}

// 0x47e0e8 — __ZNK3RBX21TaskSchedulerSettings13schedulerRateEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::schedulerRate(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings13schedulerRateEv
pub fn stub_47e0e8() -> ! {
    todo!("0x47e0e8 RBX::TaskSchedulerSettings::schedulerRate(void)const")
}

// 0x47e0fc — __ZNK3RBX21TaskSchedulerSettings27schedulerDutyCyclePerThreadEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::schedulerDutyCyclePerThread(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings27schedulerDutyCyclePerThreadEv
pub fn stub_47e0fc() -> ! {
    todo!("0x47e0fc RBX::TaskSchedulerSettings::schedulerDutyCyclePerThread(void)const")
}

// 0x47e10c — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EED1Ev
pub fn stub_47e10c() -> ! {
    todo!("0x47e10c RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::~BoundFuncDesc()")
}

// 0x47e158 — __ZNK3RBX13DebugSettings17getErrorReportingEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getErrorReporting(void)const")]
// was: __ZNK3RBX13DebugSettings17getErrorReportingEv
pub fn stub_47e158() -> ! {
    todo!("0x47e158 RBX::DebugSettings::getErrorReporting(void)const")
}

// 0x47e15c — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEED1Ev
pub fn stub_47e15c() -> ! {
    todo!("0x47e15c RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::~EnumPropDescriptor()")
}

// 0x47e180 — __ZN3RBX13DebugSettings5noOptEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::noOpt(void)")]
// was: __ZN3RBX13DebugSettings5noOptEv
pub fn stub_47e180() -> ! {
    todo!("0x47e180 RBX::DebugSettings::noOpt(void)")
}

// 0x47e184 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EED1Ev
pub fn stub_47e184() -> ! {
    todo!("0x47e184 RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::~BoundFuncDesc()")
}

// 0x47e1a8 — __ZN3RBX13DebugSettings17setBlockingRemoveEb
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this, bool)
#[doc(alias = "RBX::DebugSettings::setBlockingRemove(bool)")]
// was: __ZN3RBX13DebugSettings17setBlockingRemoveEb
pub fn stub_47e1a8() -> ! {
    todo!("0x47e1a8 RBX::DebugSettings::setBlockingRemove(bool)")
}

// 0x47e1b0 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EED1Ev
pub fn stub_47e1b0() -> ! {
    todo!("0x47e1b0 RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::~BoundFuncDesc()")
}

// 0x47e1f0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEED1Ev
pub fn stub_47e1f0() -> ! {
    todo!("0x47e1f0 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::~EnumPropDescriptor()")
}

// 0x47e214 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EED1Ev
pub fn stub_47e214() -> ! {
    todo!("0x47e214 RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::~BoundFuncDesc()")
}

// 0x47e25c — __ZNK3RBX21TaskSchedulerSettings17getPriorityMethodEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::getPriorityMethod(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings17getPriorityMethodEv
pub fn stub_47e25c() -> ! {
    todo!("0x47e25c RBX::TaskSchedulerSettings::getPriorityMethod(void)const")
}

// 0x47e26c — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEED1Ev
pub fn stub_47e26c() -> ! {
    todo!("0x47e26c RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::~EnumPropDescriptor()")
}

// 0x47e290 — __ZNK3RBX21TaskSchedulerSettings20getSleepAdjustMethodEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::getSleepAdjustMethod(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings20getSleepAdjustMethodEv
pub fn stub_47e290() -> ! {
    todo!("0x47e290 RBX::TaskSchedulerSettings::getSleepAdjustMethod(void)const")
}

// 0x47e2a0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEED1Ev
pub fn stub_47e2a0() -> ! {
    todo!("0x47e2a0 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::~EnumPropDescriptor()")
}

// 0x47e2c4 — __ZNK3RBX21TaskSchedulerSettings19getConcurrencyModelEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::getConcurrencyModel(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings19getConcurrencyModelEv
pub fn stub_47e2c4() -> ! {
    todo!("0x47e2c4 RBX::TaskSchedulerSettings::getConcurrencyModel(void)const")
}

// 0x47e2d4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEED1Ev
pub fn stub_47e2d4() -> ! {
    todo!("0x47e2d4 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::~EnumPropDescriptor()")
}

// 0x47e2f8 — __ZNK3RBX21TaskSchedulerSettings21getIsArbiterThrottledEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::getIsArbiterThrottled(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings21getIsArbiterThrottledEv
pub fn stub_47e2f8() -> ! {
    todo!("0x47e2f8 RBX::TaskSchedulerSettings::getIsArbiterThrottled(void)const")
}

// 0x47e308 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbED1Ev
pub fn stub_47e308() -> ! {
    todo!("0x47e308 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::~PropDescriptor()")
}

// 0x47e32c — __ZNK3RBX21TaskSchedulerSettings24getThrottledJobSleepTimeEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::getThrottledJobSleepTime(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings24getThrottledJobSleepTimeEv
pub fn stub_47e32c() -> ! {
    todo!("0x47e32c RBX::TaskSchedulerSettings::getThrottledJobSleepTime(void)const")
}

// 0x47e344 — __ZNK3RBX13DebugSettings27getTickCountPreciseOverrideEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getTickCountPreciseOverride(void)const")]
// was: __ZNK3RBX13DebugSettings27getTickCountPreciseOverrideEv
pub fn stub_47e344() -> ! {
    todo!("0x47e344 RBX::DebugSettings::getTickCountPreciseOverride(void)const")
}

// 0x47e354 — __ZN3RBX13DebugSettings27setTickCountPreciseOverrideENS_4Time12SampleMethodE
#[doc(alias = "RBX::DebugSettings::setTickCountPreciseOverride(RBX::Time::SampleMethod)")]
// was: __ZN3RBX13DebugSettings27setTickCountPreciseOverrideENS_4Time12SampleMethodE
pub fn stub_47e354() -> ! {
    todo!("0x47e354 RBX::DebugSettings::setTickCountPreciseOverride(RBX::Time::SampleMethod)")
}

// 0x47e364 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEED1Ev
pub fn stub_47e364() -> ! {
    todo!("0x47e364 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::~EnumPropDescriptor()")
}

// 0x47e388 — __ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEEC2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEEC2Ev")]
// was: __ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEEC2Ev
pub fn stub_47e388() -> ! {
    todo!("0x47e388 __ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEEC2Ev")
}

// 0x47e640 — __ZN5boost10shared_ptrI12DummyArbiterED1Ev
#[doc(alias = "rbx_core::SharedPtr<DummyArbiter>::~shared_ptr()")]
// was: __ZN5boost10shared_ptrI12DummyArbiterED1Ev
pub fn stub_47e640() -> ! {
    todo!("0x47e640 boost::shared_ptr<DummyArbiter>::~shared_ptr()")
}

// 0x47e654 — __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEEC2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEEC2Ev")]
// was: __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEEC2Ev
pub fn stub_47e654() -> ! {
    todo!("0x47e654 __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEEC2Ev")
}

// 0x47e8c8 — __ZN12DummyArbiter11arbiterNameEv
// type: _DWORD __fastcall(DummyArbiter *__hidden this)
#[doc(alias = "DummyArbiter::arbiterName(void)")]
// was: __ZN12DummyArbiter11arbiterNameEv
pub fn stub_47e8c8() -> ! {
    todo!("0x47e8c8 DummyArbiter::arbiterName(void)")
}

// 0x47e8e4 — __ZN12DummyArbiter11isThrottledEv
// type: _DWORD __fastcall(DummyArbiter *__hidden this)
#[doc(alias = "DummyArbiter::isThrottled(void)")]
// was: __ZN12DummyArbiter11isThrottledEv
pub fn stub_47e8e4() -> ! {
    todo!("0x47e8e4 DummyArbiter::isThrottled(void)")
}

// 0x47e8e8 — __ZN3RBX13TaskScheduler7Arbiter7preStepEPNS0_3JobE
// type: _DWORD __fastcall(RBX::TaskScheduler::Arbiter *__hidden this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::TaskScheduler::Arbiter::preStep(RBX::TaskScheduler::Job *)")]
// was: __ZN3RBX13TaskScheduler7Arbiter7preStepEPNS0_3JobE
pub fn stub_47e8e8() -> ! {
    todo!("0x47e8e8 RBX::TaskScheduler::Arbiter::preStep(RBX::TaskScheduler::Job *)")
}

// 0x47e900 — __ZN3RBX13TaskScheduler7Arbiter8postStepEPNS0_3JobE
// type: _DWORD __fastcall(RBX::TaskScheduler::Arbiter *__hidden this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::TaskScheduler::Arbiter::postStep(RBX::TaskScheduler::Job *)")]
// was: __ZN3RBX13TaskScheduler7Arbiter8postStepEPNS0_3JobE
pub fn stub_47e900() -> ! {
    todo!("0x47e900 RBX::TaskScheduler::Arbiter::postStep(RBX::TaskScheduler::Job *)")
}

// 0x47e918 — __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD1Ev
pub fn stub_47e918() -> ! {
    todo!("0x47e918 __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD1Ev")
}

// 0x47e920 — __ZN3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7CreatorD1Ev
pub fn stub_47e920() -> ! {
    todo!("0x47e920 __ZN3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7CreatorD1Ev")
}

// 0x47e924 — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEED1Ev
pub fn stub_47e924() -> ! {
    todo!("0x47e924 RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::~EnumDesc()")
}

// 0x47e928 — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEED0Ev
pub fn stub_47e928() -> ! {
    todo!("0x47e928 RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::~EnumDesc()")
}

// 0x47e9c8 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE6lookupEPKc
pub fn stub_47e9c8() -> ! {
    todo!("0x47e9c8 RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::lookup(char const*)const")
}

// 0x47e9f8 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE6lookupERKNS0_7VariantE
pub fn stub_47e9f8() -> ! {
    todo!("0x47e9f8 RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x47ea18 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToValueEmRNS0_7VariantE
pub fn stub_47ea18() -> ! {
    todo!("0x47ea18 RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x47ea4c — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE15convertToStringEmRSs
pub fn stub_47ea4c() -> ! {
    todo!("0x47ea4c RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToString(unsigned long,std::string &)const")
}

// 0x47eb90 — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEED1Ev
pub fn stub_47eb90() -> ! {
    todo!("0x47eb90 RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::~EnumDesc()")
}

// 0x47eb94 — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEED0Ev
pub fn stub_47eb94() -> ! {
    todo!("0x47eb94 RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::~EnumDesc()")
}

// 0x47ec34 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE6lookupEPKc
pub fn stub_47ec34() -> ! {
    todo!("0x47ec34 RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::lookup(char const*)const")
}

// 0x47ec64 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE6lookupERKNS0_7VariantE
pub fn stub_47ec64() -> ! {
    todo!("0x47ec64 RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x47ec84 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToValueEmRNS0_7VariantE
pub fn stub_47ec84() -> ! {
    todo!("0x47ec84 RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x47ecb8 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE15convertToStringEmRSs
pub fn stub_47ecb8() -> ! {
    todo!("0x47ecb8 RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToString(unsigned long,std::string &)const")
}

// 0x47edfc — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEED1Ev
pub fn stub_47edfc() -> ! {
    todo!("0x47edfc RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::~EnumDesc()")
}

// 0x47ee00 — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEED0Ev
pub fn stub_47ee00() -> ! {
    todo!("0x47ee00 RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::~EnumDesc()")
}

// 0x47eea0 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE6lookupEPKc
pub fn stub_47eea0() -> ! {
    todo!("0x47eea0 RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::lookup(char const*)const")
}

// 0x47eed0 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE6lookupERKNS0_7VariantE
pub fn stub_47eed0() -> ! {
    todo!("0x47eed0 RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x47eef0 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToValueEmRNS0_7VariantE
pub fn stub_47eef0() -> ! {
    todo!("0x47eef0 RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x47ef24 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE15convertToStringEmRSs
pub fn stub_47ef24() -> ! {
    todo!("0x47ef24 RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToString(unsigned long,std::string &)const")
}

// 0x47f068 — __ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEED1Ev
pub fn stub_47f068() -> ! {
    todo!("0x47f068 RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::~EnumDesc()")
}

// 0x47f06c — __ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEED0Ev
pub fn stub_47f06c() -> ! {
    todo!("0x47f06c RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::~EnumDesc()")
}

// 0x47f10c — __ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE6lookupEPKc
pub fn stub_47f10c() -> ! {
    todo!("0x47f10c RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::lookup(char const*)const")
}
