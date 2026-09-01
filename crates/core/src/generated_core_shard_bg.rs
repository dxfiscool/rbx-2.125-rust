//! core shard BG — 100 core stubs EA-sorted, next uncovered after BF 0x47b9a4 (strict RBX|boost|std|rbx earliest gap, after BF 0x47e0e8..0x48cdac).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x47b9a4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "RBX::TaskSchedulerSettings::schedulerRate(void)const")]
// 0x47e0e8 — __ZNK3RBX21TaskSchedulerSettings13schedulerRateEv — RBX::TaskSchedulerSettings::schedulerRate(void)const
pub fn stub_47e0e8() -> ! {
    todo!("0x47e0e8 __ZNK3RBX21TaskSchedulerSettings13schedulerRateEv")
}

#[doc(alias = "RBX::TaskSchedulerSettings::schedulerDutyCyclePerThread(void)const")]
// 0x47e0fc — __ZNK3RBX21TaskSchedulerSettings27schedulerDutyCyclePerThreadEv — RBX::TaskSchedulerSettings::schedulerDutyCyclePerThread(void)const
pub fn stub_47e0fc() -> ! {
    todo!("0x47e0fc __ZNK3RBX21TaskSchedulerSettings27schedulerDutyCyclePerThreadEv")
}

#[doc(alias = "RBX::DebugSettings::getErrorReporting(void)const")]
// 0x47e158 — __ZNK3RBX13DebugSettings17getErrorReportingEv — RBX::DebugSettings::getErrorReporting(void)const
pub fn stub_47e158() -> ! {
    todo!("0x47e158 __ZNK3RBX13DebugSettings17getErrorReportingEv")
}

#[doc(alias = "RBX::DebugSettings::noOpt(void)")]
// 0x47e180 — __ZN3RBX13DebugSettings5noOptEv — RBX::DebugSettings::noOpt(void)
pub fn stub_47e180() -> ! {
    todo!("0x47e180 __ZN3RBX13DebugSettings5noOptEv")
}

#[doc(alias = "RBX::DebugSettings::setBlockingRemove(bool)")]
// 0x47e1a8 — __ZN3RBX13DebugSettings17setBlockingRemoveEb — RBX::DebugSettings::setBlockingRemove(bool)
pub fn stub_47e1a8() -> ! {
    todo!("0x47e1a8 __ZN3RBX13DebugSettings17setBlockingRemoveEb")
}

#[doc(alias = "RBX::TaskSchedulerSettings::getPriorityMethod(void)const")]
// 0x47e25c — __ZNK3RBX21TaskSchedulerSettings17getPriorityMethodEv — RBX::TaskSchedulerSettings::getPriorityMethod(void)const
pub fn stub_47e25c() -> ! {
    todo!("0x47e25c __ZNK3RBX21TaskSchedulerSettings17getPriorityMethodEv")
}

#[doc(alias = "RBX::TaskSchedulerSettings::getSleepAdjustMethod(void)const")]
// 0x47e290 — __ZNK3RBX21TaskSchedulerSettings20getSleepAdjustMethodEv — RBX::TaskSchedulerSettings::getSleepAdjustMethod(void)const
pub fn stub_47e290() -> ! {
    todo!("0x47e290 __ZNK3RBX21TaskSchedulerSettings20getSleepAdjustMethodEv")
}

#[doc(alias = "RBX::TaskSchedulerSettings::getConcurrencyModel(void)const")]
// 0x47e2c4 — __ZNK3RBX21TaskSchedulerSettings19getConcurrencyModelEv — RBX::TaskSchedulerSettings::getConcurrencyModel(void)const
pub fn stub_47e2c4() -> ! {
    todo!("0x47e2c4 __ZNK3RBX21TaskSchedulerSettings19getConcurrencyModelEv")
}

#[doc(alias = "RBX::TaskSchedulerSettings::getIsArbiterThrottled(void)const")]
// 0x47e2f8 — __ZNK3RBX21TaskSchedulerSettings21getIsArbiterThrottledEv — RBX::TaskSchedulerSettings::getIsArbiterThrottled(void)const
pub fn stub_47e2f8() -> ! {
    todo!("0x47e2f8 __ZNK3RBX21TaskSchedulerSettings21getIsArbiterThrottledEv")
}

#[doc(alias = "RBX::TaskSchedulerSettings::getThrottledJobSleepTime(void)const")]
// 0x47e32c — __ZNK3RBX21TaskSchedulerSettings24getThrottledJobSleepTimeEv — RBX::TaskSchedulerSettings::getThrottledJobSleepTime(void)const
pub fn stub_47e32c() -> ! {
    todo!("0x47e32c __ZNK3RBX21TaskSchedulerSettings24getThrottledJobSleepTimeEv")
}

#[doc(alias = "RBX::DebugSettings::getTickCountPreciseOverride(void)const")]
// 0x47e344 — __ZNK3RBX13DebugSettings27getTickCountPreciseOverrideEv — RBX::DebugSettings::getTickCountPreciseOverride(void)const
pub fn stub_47e344() -> ! {
    todo!("0x47e344 __ZNK3RBX13DebugSettings27getTickCountPreciseOverrideEv")
}

#[doc(alias = "RBX::DebugSettings::setTickCountPreciseOverride(RBX::Time::SampleMethod)")]
// 0x47e354 — __ZN3RBX13DebugSettings27setTickCountPreciseOverrideENS_4Time12SampleMethodE — RBX::DebugSettings::setTickCountPreciseOverride(RBX::Time::SampleMethod)
pub fn stub_47e354() -> ! {
    todo!("0x47e354 __ZN3RBX13DebugSettings27setTickCountPreciseOverrideENS_4Time12SampleMethodE")
}

#[doc(alias = "RBX::TaskScheduler::Arbiter::preStep(RBX::TaskScheduler::Job *)")]
// 0x47e8e8 — __ZN3RBX13TaskScheduler7Arbiter7preStepEPNS0_3JobE — RBX::TaskScheduler::Arbiter::preStep(RBX::TaskScheduler::Job *)
pub fn stub_47e8e8() -> ! {
    todo!("0x47e8e8 __ZN3RBX13TaskScheduler7Arbiter7preStepEPNS0_3JobE")
}

#[doc(alias = "RBX::TaskScheduler::Arbiter::postStep(RBX::TaskScheduler::Job *)")]
// 0x47e900 — __ZN3RBX13TaskScheduler7Arbiter8postStepEPNS0_3JobE — RBX::TaskScheduler::Arbiter::postStep(RBX::TaskScheduler::Job *)
pub fn stub_47e900() -> ! {
    todo!("0x47e900 __ZN3RBX13TaskScheduler7Arbiter8postStepEPNS0_3JobE")
}

#[doc(alias = "RBX::DebugSettings::~DebugSettings()")]
// 0x47f7ac — __ZN3RBX13DebugSettingsD1Ev — RBX::DebugSettings::~DebugSettings()
pub fn stub_47f7ac() -> ! {
    todo!("0x47f7ac __ZN3RBX13DebugSettingsD1Ev")
}

#[doc(alias = "RBX::DebugSettings::~DebugSettings()")]
// 0x47f7ec — __ZN3RBX13DebugSettingsD0Ev — RBX::DebugSettings::~DebugSettings()
pub fn stub_47f7ec() -> ! {
    todo!("0x47f7ec __ZN3RBX13DebugSettingsD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DebugSettings::~DebugSettings()")]
// 0x47f91c — __ZThn32_N3RBX13DebugSettingsD1Ev — non-virtual thunk toRBX::DebugSettings::~DebugSettings()
pub fn stub_47f91c() -> ! {
    todo!("0x47f91c __ZThn32_N3RBX13DebugSettingsD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DebugSettings::~DebugSettings()")]
// 0x47f960 — __ZThn32_N3RBX13DebugSettingsD0Ev — non-virtual thunk toRBX::DebugSettings::~DebugSettings()
pub fn stub_47f960() -> ! {
    todo!("0x47f960 __ZThn32_N3RBX13DebugSettingsD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DebugSettings::~DebugSettings()")]
// 0x47fa58 — __ZThn36_N3RBX13DebugSettingsD1Ev — non-virtual thunk toRBX::DebugSettings::~DebugSettings()
pub fn stub_47fa58() -> ! {
    todo!("0x47fa58 __ZThn36_N3RBX13DebugSettingsD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DebugSettings::~DebugSettings()")]
// 0x47fa9c — __ZThn36_N3RBX13DebugSettingsD0Ev — non-virtual thunk toRBX::DebugSettings::~DebugSettings()
pub fn stub_47fa9c() -> ! {
    todo!("0x47fa9c __ZThn36_N3RBX13DebugSettingsD0Ev")
}

#[doc(alias = "RBX::TaskSchedulerSettings::~TaskSchedulerSettings()")]
// 0x47fb88 — __ZN3RBX21TaskSchedulerSettingsD1Ev — RBX::TaskSchedulerSettings::~TaskSchedulerSettings()
pub fn stub_47fb88() -> ! {
    todo!("0x47fb88 __ZN3RBX21TaskSchedulerSettingsD1Ev")
}

#[doc(alias = "RBX::TaskSchedulerSettings::~TaskSchedulerSettings()")]
// 0x47fbc8 — __ZN3RBX21TaskSchedulerSettingsD0Ev — RBX::TaskSchedulerSettings::~TaskSchedulerSettings()
pub fn stub_47fbc8() -> ! {
    todo!("0x47fbc8 __ZN3RBX21TaskSchedulerSettingsD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::TaskSchedulerSettings::~TaskSchedulerSettings()")]
// 0x47fcb8 — __ZThn32_N3RBX21TaskSchedulerSettingsD1Ev — non-virtual thunk toRBX::TaskSchedulerSettings::~TaskSchedulerSettings()
pub fn stub_47fcb8() -> ! {
    todo!("0x47fcb8 __ZThn32_N3RBX21TaskSchedulerSettingsD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::TaskSchedulerSettings::~TaskSchedulerSettings()")]
// 0x47fcfc — __ZThn32_N3RBX21TaskSchedulerSettingsD0Ev — non-virtual thunk toRBX::TaskSchedulerSettings::~TaskSchedulerSettings()
pub fn stub_47fcfc() -> ! {
    todo!("0x47fcfc __ZThn32_N3RBX21TaskSchedulerSettingsD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::TaskSchedulerSettings::~TaskSchedulerSettings()")]
// 0x47fdec — __ZThn36_N3RBX21TaskSchedulerSettingsD1Ev — non-virtual thunk toRBX::TaskSchedulerSettings::~TaskSchedulerSettings()
pub fn stub_47fdec() -> ! {
    todo!("0x47fdec __ZThn36_N3RBX21TaskSchedulerSettingsD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::TaskSchedulerSettings::~TaskSchedulerSettings()")]
// 0x47fe30 — __ZThn36_N3RBX21TaskSchedulerSettingsD0Ev — non-virtual thunk toRBX::TaskSchedulerSettings::~TaskSchedulerSettings()
pub fn stub_47fe30() -> ! {
    todo!("0x47fe30 __ZThn36_N3RBX21TaskSchedulerSettingsD0Ev")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Time::SampleMethod>(RBX::Time::SampleMethod const&)")]
// 0x48030c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4Time12SampleMethodEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Time::SampleMethod>(RBX::Time::SampleMethod const&)
pub fn stub_48030c() -> ! {
    todo!("0x48030c __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4Time12SampleMethodEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Time::SampleMethod>::singleton(void)")]
// 0x48035c — __ZN3rbx14implementation12typed_holderIN3RBX4Time12SampleMethodEE9singletonEv — rbx::implementation::typed_holder<RBX::Time::SampleMethod>::singleton(void)
pub fn stub_48035c() -> ! {
    todo!("0x48035c __ZN3rbx14implementation12typed_holderIN3RBX4Time12SampleMethodEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Time::SampleMethod>::construct_func(char const*,char *)")]
// 0x4803c8 — __ZN3rbx14implementation12typed_holderIN3RBX4Time12SampleMethodEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Time::SampleMethod>::construct_func(char const*,char *)
pub fn stub_4803c8() -> ! {
    todo!("0x4803c8 __ZN3rbx14implementation12typed_holderIN3RBX4Time12SampleMethodEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Time::SampleMethod>::destruct_func(char *)")]
// 0x4803d4 — __ZN3rbx14implementation12typed_holderIN3RBX4Time12SampleMethodEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Time::SampleMethod>::destruct_func(char *)
pub fn stub_4803d4() -> ! {
    todo!("0x4803d4 __ZN3rbx14implementation12typed_holderIN3RBX4Time12SampleMethodEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Time::SampleMethod const& rbx::any_cast<RBX::Time::SampleMethod const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4804a4 — __ZN3rbx8any_castIRKN3RBX4Time12SampleMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Time::SampleMethod const& rbx::any_cast<RBX::Time::SampleMethod const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4804a4() -> ! {
    todo!("0x4804a4 __ZN3rbx8any_castIRKN3RBX4Time12SampleMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::EThrottle::EThrottleType>(RBX::EThrottle::EThrottleType const&)")]
// 0x480988 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9EThrottle13EThrottleTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::EThrottle::EThrottleType>(RBX::EThrottle::EThrottleType const&)
pub fn stub_480988() -> ! {
    todo!("0x480988 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9EThrottle13EThrottleTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::singleton(void)")]
// 0x4809d8 — __ZN3rbx14implementation12typed_holderIN3RBX9EThrottle13EThrottleTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::singleton(void)
pub fn stub_4809d8() -> ! {
    todo!("0x4809d8 __ZN3rbx14implementation12typed_holderIN3RBX9EThrottle13EThrottleTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::construct_func(char const*,char *)")]
// 0x480a44 — __ZN3rbx14implementation12typed_holderIN3RBX9EThrottle13EThrottleTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::construct_func(char const*,char *)
pub fn stub_480a44() -> ! {
    todo!("0x480a44 __ZN3rbx14implementation12typed_holderIN3RBX9EThrottle13EThrottleTypeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::destruct_func(char *)")]
// 0x480a50 — __ZN3rbx14implementation12typed_holderIN3RBX9EThrottle13EThrottleTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::destruct_func(char *)
pub fn stub_480a50() -> ! {
    todo!("0x480a50 __ZN3rbx14implementation12typed_holderIN3RBX9EThrottle13EThrottleTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::EThrottle::EThrottleType const& rbx::any_cast<RBX::EThrottle::EThrottleType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x480b20 — __ZN3rbx8any_castIRKN3RBX9EThrottle13EThrottleTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::EThrottle::EThrottleType const& rbx::any_cast<RBX::EThrottle::EThrottleType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_480b20() -> ! {
    todo!("0x480b20 __ZN3rbx8any_castIRKN3RBX9EThrottle13EThrottleTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DebugSettings::ErrorReporting>(RBX::DebugSettings::ErrorReporting const&)")]
// 0x481000 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13DebugSettings14ErrorReportingEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DebugSettings::ErrorReporting>(RBX::DebugSettings::ErrorReporting const&)
pub fn stub_481000() -> ! {
    todo!("0x481000 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13DebugSettings14ErrorReportingEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::singleton(void)")]
// 0x481050 — __ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE9singletonEv — rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::singleton(void)
pub fn stub_481050() -> ! {
    todo!("0x481050 __ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::construct_func(char const*,char *)")]
// 0x4810bc — __ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::construct_func(char const*,char *)
pub fn stub_4810bc() -> ! {
    todo!("0x4810bc __ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::destruct_func(char *)")]
// 0x4810c8 — __ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::destruct_func(char *)
pub fn stub_4810c8() -> ! {
    todo!("0x4810c8 __ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE13destruct_funcEPc")
}

#[doc(alias = "RBX::DebugSettings::ErrorReporting const& rbx::any_cast<RBX::DebugSettings::ErrorReporting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x481198 — __ZN3rbx8any_castIRKN3RBX13DebugSettings14ErrorReportingENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::DebugSettings::ErrorReporting const& rbx::any_cast<RBX::DebugSettings::ErrorReporting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_481198() -> ! {
    todo!("0x481198 __ZN3rbx8any_castIRKN3RBX13DebugSettings14ErrorReportingENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TaskScheduler::Job::SleepAdjustMethod>(RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
// 0x481678 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler3Job17SleepAdjustMethodEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TaskScheduler::Job::SleepAdjustMethod>(RBX::TaskScheduler::Job::SleepAdjustMethod const&)
pub fn stub_481678() -> ! {
    todo!("0x481678 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler3Job17SleepAdjustMethodEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::Job::SleepAdjustMethod>::singleton(void)")]
// 0x4816c8 — __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE9singletonEv — rbx::implementation::typed_holder<RBX::TaskScheduler::Job::SleepAdjustMethod>::singleton(void)
pub fn stub_4816c8() -> ! {
    todo!("0x4816c8 __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::Job::SleepAdjustMethod>::construct_func(char const*,char *)")]
// 0x481734 — __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::TaskScheduler::Job::SleepAdjustMethod>::construct_func(char const*,char *)
pub fn stub_481734() -> ! {
    todo!("0x481734 __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::Job::SleepAdjustMethod>::destruct_func(char *)")]
// 0x481740 — __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::TaskScheduler::Job::SleepAdjustMethod>::destruct_func(char *)
pub fn stub_481740() -> ! {
    todo!("0x481740 __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE13destruct_funcEPc")
}

#[doc(alias = "RBX::TaskScheduler::Job::SleepAdjustMethod const& rbx::any_cast<RBX::TaskScheduler::Job::SleepAdjustMethod const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x481810 — __ZN3rbx8any_castIRKN3RBX13TaskScheduler3Job17SleepAdjustMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::TaskScheduler::Job::SleepAdjustMethod const& rbx::any_cast<RBX::TaskScheduler::Job::SleepAdjustMethod const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_481810() -> ! {
    todo!("0x481810 __ZN3rbx8any_castIRKN3RBX13TaskScheduler3Job17SleepAdjustMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TaskScheduler::PriorityMethod>(RBX::TaskScheduler::PriorityMethod const&)")]
// 0x481cf0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler14PriorityMethodEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TaskScheduler::PriorityMethod>(RBX::TaskScheduler::PriorityMethod const&)
pub fn stub_481cf0() -> ! {
    todo!("0x481cf0 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler14PriorityMethodEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::PriorityMethod>::singleton(void)")]
// 0x481d40 — __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE9singletonEv — rbx::implementation::typed_holder<RBX::TaskScheduler::PriorityMethod>::singleton(void)
pub fn stub_481d40() -> ! {
    todo!("0x481d40 __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::PriorityMethod>::construct_func(char const*,char *)")]
// 0x481dac — __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::TaskScheduler::PriorityMethod>::construct_func(char const*,char *)
pub fn stub_481dac() -> ! {
    todo!("0x481dac __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::PriorityMethod>::destruct_func(char *)")]
// 0x481db8 — __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::TaskScheduler::PriorityMethod>::destruct_func(char *)
pub fn stub_481db8() -> ! {
    todo!("0x481db8 __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE13destruct_funcEPc")
}

#[doc(alias = "RBX::TaskScheduler::PriorityMethod const& rbx::any_cast<RBX::TaskScheduler::PriorityMethod const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x481e88 — __ZN3rbx8any_castIRKN3RBX13TaskScheduler14PriorityMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::TaskScheduler::PriorityMethod const& rbx::any_cast<RBX::TaskScheduler::PriorityMethod const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_481e88() -> ! {
    todo!("0x481e88 __ZN3rbx8any_castIRKN3RBX13TaskScheduler14PriorityMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TaskScheduler::ThreadPoolConfig>(RBX::TaskScheduler::ThreadPoolConfig const&)")]
// 0x482368 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler16ThreadPoolConfigEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TaskScheduler::ThreadPoolConfig>(RBX::TaskScheduler::ThreadPoolConfig const&)
pub fn stub_482368() -> ! {
    todo!("0x482368 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler16ThreadPoolConfigEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::ThreadPoolConfig>::singleton(void)")]
// 0x4823b8 — __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE9singletonEv — rbx::implementation::typed_holder<RBX::TaskScheduler::ThreadPoolConfig>::singleton(void)
pub fn stub_4823b8() -> ! {
    todo!("0x4823b8 __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::ThreadPoolConfig>::construct_func(char const*,char *)")]
// 0x482424 — __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::TaskScheduler::ThreadPoolConfig>::construct_func(char const*,char *)
pub fn stub_482424() -> ! {
    todo!("0x482424 __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::ThreadPoolConfig>::destruct_func(char *)")]
// 0x482430 — __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::TaskScheduler::ThreadPoolConfig>::destruct_func(char *)
pub fn stub_482430() -> ! {
    todo!("0x482430 __ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE13destruct_funcEPc")
}

#[doc(alias = "RBX::TaskScheduler::ThreadPoolConfig const& rbx::any_cast<RBX::TaskScheduler::ThreadPoolConfig const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x482500 — __ZN3rbx8any_castIRKN3RBX13TaskScheduler16ThreadPoolConfigENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::TaskScheduler::ThreadPoolConfig const& rbx::any_cast<RBX::TaskScheduler::ThreadPoolConfig const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_482500() -> ! {
    todo!("0x482500 __ZN3rbx8any_castIRKN3RBX13TaskScheduler16ThreadPoolConfigENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::BlockMesh::~BlockMesh()")]
// 0x482dd0 — __ZN3RBX9BlockMeshD1Ev — RBX::BlockMesh::~BlockMesh()
pub fn stub_482dd0() -> ! {
    todo!("0x482dd0 __ZN3RBX9BlockMeshD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
// 0x482de8 — __ZThn36_N3RBX9BlockMeshD0Ev — non-virtual thunk toRBX::BlockMesh::~BlockMesh()
pub fn stub_482de8() -> ! {
    todo!("0x482de8 __ZThn36_N3RBX9BlockMeshD0Ev")
}

#[doc(alias = "float const& rbx::any_cast<float const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x48b140 — __ZN3rbx8any_castIRKfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE — float const& rbx::any_cast<float const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_48b140() -> ! {
    todo!("0x48b140 __ZN3rbx8any_castIRKfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<float>(float const&)")]
// 0x48b228 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIfEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<float>(float const&)
pub fn stub_48b228() -> ! {
    todo!("0x48b228 __ZN3rbx13placement_anyIN3RBX7Region3EEaSIfEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<float>::singleton(void)")]
// 0x48b278 — __ZN3rbx14implementation12typed_holderIfE9singletonEv — rbx::implementation::typed_holder<float>::singleton(void)
pub fn stub_48b278() -> ! {
    todo!("0x48b278 __ZN3rbx14implementation12typed_holderIfE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<float>::destruct_func(char *)")]
// 0x48b2e8 — __ZN3rbx14implementation12typed_holderIfE13destruct_funcEPc — rbx::implementation::typed_holder<float>::destruct_func(char *)
pub fn stub_48b2e8() -> ! {
    todo!("0x48b2e8 __ZN3rbx14implementation12typed_holderIfE13destruct_funcEPc")
}

#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::resize(unsigned long,RBX::Time::SampleMethod)")]
// 0x48bbbc — __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE6resizeEmS2_ — std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::resize(unsigned long,RBX::Time::SampleMethod)
pub fn stub_48bbbc() -> ! {
    todo!("0x48bbbc __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::push_back(RBX::Time::SampleMethod const&)")]
// 0x48bbf4 — __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE9push_backERKS2_ — std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::push_back(RBX::Time::SampleMethod const&)
pub fn stub_48bbf4() -> ! {
    todo!("0x48bbf4 __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Time::SampleMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::operator[](RBX::Name const* const&)")]
// 0x48bc20 — __ZNSt3mapIPKN3RBX4NameENS0_4Time12SampleMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::Time::SampleMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::operator[](RBX::Name const* const&)
pub fn stub_48bc20() -> ! {
    todo!("0x48bc20 __ZNSt3mapIPKN3RBX4NameENS0_4Time12SampleMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")]
// 0x48bc78 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)
pub fn stub_48bc78() -> ! {
    todo!("0x48bc78 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")]
// 0x48bd2c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)
pub fn stub_48bd2c() -> ! {
    todo!("0x48bd2c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")]
// 0x48bd84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)
pub fn stub_48bd84() -> ! {
    todo!("0x48bd84 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,RBX::Time::SampleMethod const&)")]
// 0x48bdf0 — __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,RBX::Time::SampleMethod const&)
pub fn stub_48bdf0() -> ! {
    todo!("0x48bdf0 __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_allocate(unsigned long)")]
// 0x48bed4 — __ZNSt12_Vector_baseIN3RBX4Time12SampleMethodESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_allocate(unsigned long)
pub fn stub_48bed4() -> ! {
    todo!("0x48bed4 __ZNSt12_Vector_baseIN3RBX4Time12SampleMethodESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Time::SampleMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Time::SampleMethod *,RBX::Time::SampleMethod *>(RBX::Time::SampleMethod *,RBX::Time::SampleMethod *,RBX::Time::SampleMethod *)")]
// 0x48beec — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4Time12SampleMethodES6_EET0_T_S8_S7_ — RBX::Time::SampleMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Time::SampleMethod *,RBX::Time::SampleMethod *>(RBX::Time::SampleMethod *,RBX::Time::SampleMethod *,RBX::Time::SampleMethod *)
pub fn stub_48beec() -> ! {
    todo!("0x48beec __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4Time12SampleMethodES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,unsigned long,RBX::Time::SampleMethod const&)")]
// 0x48bf2c — __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,unsigned long,RBX::Time::SampleMethod const&)
pub fn stub_48bf2c() -> ! {
    todo!("0x48bf2c __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::resize(unsigned long,RBX::EThrottle::EThrottleType)")]
// 0x48c0c0 — __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE6resizeEmS2_ — std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::resize(unsigned long,RBX::EThrottle::EThrottleType)
pub fn stub_48c0c0() -> ! {
    todo!("0x48c0c0 __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::push_back(RBX::EThrottle::EThrottleType const&)")]
// 0x48c0f4 — __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE9push_backERKS2_ — std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::push_back(RBX::EThrottle::EThrottleType const&)
pub fn stub_48c0f4() -> ! {
    todo!("0x48c0f4 __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::EThrottle::EThrottleType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::operator[](RBX::Name const* const&)")]
// 0x48c11c — __ZNSt3mapIPKN3RBX4NameENS0_9EThrottle13EThrottleTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::EThrottle::EThrottleType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::operator[](RBX::Name const* const&)
pub fn stub_48c11c() -> ! {
    todo!("0x48c11c __ZNSt3mapIPKN3RBX4NameENS0_9EThrottle13EThrottleTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
// 0x48c174 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)
pub fn stub_48c174() -> ! {
    todo!("0x48c174 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
// 0x48c228 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)
pub fn stub_48c228() -> ! {
    todo!("0x48c228 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
// 0x48c280 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)
pub fn stub_48c280() -> ! {
    todo!("0x48c280 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,RBX::EThrottle::EThrottleType const&)")]
// 0x48c2e8 — __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,RBX::EThrottle::EThrottleType const&)
pub fn stub_48c2e8() -> ! {
    todo!("0x48c2e8 __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_allocate(unsigned long)")]
// 0x48c3cc — __ZNSt12_Vector_baseIN3RBX9EThrottle13EThrottleTypeESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_allocate(unsigned long)
pub fn stub_48c3cc() -> ! {
    todo!("0x48c3cc __ZNSt12_Vector_baseIN3RBX9EThrottle13EThrottleTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::EThrottle::EThrottleType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *>(RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *)")]
// 0x48c3e4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9EThrottle13EThrottleTypeES6_EET0_T_S8_S7_ — RBX::EThrottle::EThrottleType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *>(RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *)
pub fn stub_48c3e4() -> ! {
    todo!("0x48c3e4 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9EThrottle13EThrottleTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,unsigned long,RBX::EThrottle::EThrottleType const&)")]
// 0x48c420 — __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,unsigned long,RBX::EThrottle::EThrottleType const&)
pub fn stub_48c420() -> ! {
    todo!("0x48c420 __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::resize(unsigned long,RBX::DebugSettings::ErrorReporting)")]
// 0x48c5b0 — __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE6resizeEmS2_ — std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::resize(unsigned long,RBX::DebugSettings::ErrorReporting)
pub fn stub_48c5b0() -> ! {
    todo!("0x48c5b0 __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::push_back(RBX::DebugSettings::ErrorReporting const&)")]
// 0x48c5e4 — __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE9push_backERKS2_ — std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::push_back(RBX::DebugSettings::ErrorReporting const&)
pub fn stub_48c5e4() -> ! {
    todo!("0x48c5e4 __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DebugSettings::ErrorReporting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::operator[](RBX::Name const* const&)")]
// 0x48c60c — __ZNSt3mapIPKN3RBX4NameENS0_13DebugSettings14ErrorReportingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::DebugSettings::ErrorReporting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::operator[](RBX::Name const* const&)
pub fn stub_48c60c() -> ! {
    todo!("0x48c60c __ZNSt3mapIPKN3RBX4NameENS0_13DebugSettings14ErrorReportingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
// 0x48c664 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)
pub fn stub_48c664() -> ! {
    todo!("0x48c664 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
// 0x48c718 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)
pub fn stub_48c718() -> ! {
    todo!("0x48c718 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
// 0x48c770 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)
pub fn stub_48c770() -> ! {
    todo!("0x48c770 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,RBX::DebugSettings::ErrorReporting const&)")]
// 0x48c7d8 — __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,RBX::DebugSettings::ErrorReporting const&)
pub fn stub_48c7d8() -> ! {
    todo!("0x48c7d8 __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_allocate(unsigned long)")]
// 0x48c8bc — __ZNSt12_Vector_baseIN3RBX13DebugSettings14ErrorReportingESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_allocate(unsigned long)
pub fn stub_48c8bc() -> ! {
    todo!("0x48c8bc __ZNSt12_Vector_baseIN3RBX13DebugSettings14ErrorReportingESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::DebugSettings::ErrorReporting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *>(RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *)")]
// 0x48c8d4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DebugSettings14ErrorReportingES6_EET0_T_S8_S7_ — RBX::DebugSettings::ErrorReporting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *>(RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *)
pub fn stub_48c8d4() -> ! {
    todo!("0x48c8d4 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DebugSettings14ErrorReportingES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,unsigned long,RBX::DebugSettings::ErrorReporting const&)")]
// 0x48c910 — __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,unsigned long,RBX::DebugSettings::ErrorReporting const&)
pub fn stub_48c910() -> ! {
    todo!("0x48c910 __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::resize(unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod)")]
// 0x48caa0 — __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE6resizeEmS3_ — std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::resize(unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod)
pub fn stub_48caa0() -> ! {
    todo!("0x48caa0 __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE6resizeEmS3_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::push_back(RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
// 0x48cad4 — __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE9push_backERKS3_ — std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::push_back(RBX::TaskScheduler::Job::SleepAdjustMethod const&)
pub fn stub_48cad4() -> ! {
    todo!("0x48cad4 __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE9push_backERKS3_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::Job::SleepAdjustMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::operator[](RBX::Name const* const&)")]
// 0x48cafc — __ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler3Job17SleepAdjustMethodESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_ — std::map<RBX::Name const*,RBX::TaskScheduler::Job::SleepAdjustMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::operator[](RBX::Name const* const&)
pub fn stub_48cafc() -> ! {
    todo!("0x48cafc __ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler3Job17SleepAdjustMethodESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
// 0x48cb54 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)
pub fn stub_48cb54() -> ! {
    todo!("0x48cb54 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
// 0x48cc08 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)
pub fn stub_48cc08() -> ! {
    todo!("0x48cc08 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
// 0x48cc60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)
pub fn stub_48cc60() -> ! {
    todo!("0x48cc60 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
// 0x48ccc8 — __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_ — std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,RBX::TaskScheduler::Job::SleepAdjustMethod const&)
pub fn stub_48ccc8() -> ! {
    todo!("0x48ccc8 __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_allocate(unsigned long)")]
// 0x48cdac — __ZNSt12_Vector_baseIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE11_M_allocateEm — std::_Vector_base<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_allocate(unsigned long)
pub fn stub_48cdac() -> ! {
    todo!("0x48cdac __ZNSt12_Vector_baseIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE11_M_allocateEm")
}
