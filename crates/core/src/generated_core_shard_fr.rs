//! core shard FR — 100 core stubs EA-sorted, 0xf391a4..0xf3a264 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after FQ 0xf39194).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf39194.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx_core::SharedPtr<RBX::TimerService>::operator=(rbx_core::SharedPtr<RBX::TimerService> const&)")]
// 0xf391a4 — j___ZN5boost10shared_ptrIN3RBX12TimerServiceEEaSERKS3_
// was: boost::shared_ptr<RBX::TimerService>::operator=(boost::shared_ptr<RBX::TimerService> const&)
pub fn stub_f391a4() -> ! {
    todo!("0xf391a4 j___ZN5boost10shared_ptrIN3RBX12TimerServiceEEaSERKS3_")
}


#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DebugSettings::ErrorReporting>(RBX::DebugSettings::ErrorReporting const&)")]
// 0xf397a4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13DebugSettings14ErrorReportingEEERS3_RKT_
pub fn stub_f397a4() -> ! {
    todo!("0xf397a4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13DebugSettings14ErrorReportingEEERS3_RKT_")
}


#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TaskScheduler::PriorityMethod>(RBX::TaskScheduler::PriorityMethod const&)")]
// 0xf397b4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler14PriorityMethodEEERS3_RKT_
pub fn stub_f397b4() -> ! {
    todo!("0xf397b4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler14PriorityMethodEEERS3_RKT_")
}


#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TaskScheduler::ThreadPoolConfig>(RBX::TaskScheduler::ThreadPoolConfig const&)")]
// 0xf397c4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler16ThreadPoolConfigEEERS3_RKT_
pub fn stub_f397c4() -> ! {
    todo!("0xf397c4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler16ThreadPoolConfigEEERS3_RKT_")
}


#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TaskScheduler::Job::SleepAdjustMethod>(RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
// 0xf397d4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler3Job17SleepAdjustMethodEEERS3_RKT_
pub fn stub_f397d4() -> ! {
    todo!("0xf397d4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler3Job17SleepAdjustMethodEEERS3_RKT_")
}


#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Time::SampleMethod>(RBX::Time::SampleMethod const&)")]
// 0xf397f4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4Time12SampleMethodEEERS3_RKT_
pub fn stub_f397f4() -> ! {
    todo!("0xf397f4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4Time12SampleMethodEEERS3_RKT_")
}


#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::EThrottle::EThrottleType>(RBX::EThrottle::EThrottleType const&)")]
// 0xf39804 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9EThrottle13EThrottleTypeEEERS3_RKT_
pub fn stub_f39804() -> ! {
    todo!("0xf39804 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9EThrottle13EThrottleTypeEEERS3_RKT_")
}


#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<float>(float const&)")]
// 0xf39814 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIfEERS3_RKT_
pub fn stub_f39814() -> ! {
    todo!("0xf39814 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIfEERS3_RKT_")
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::singleton(void)")]
// 0xf39824 — j___ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE9singletonEv
pub fn stub_f39824() -> ! {
    todo!("0xf39824 j___ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE9singletonEv")
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::PriorityMethod>::singleton(void)")]
// 0xf39834 — j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE9singletonEv
pub fn stub_f39834() -> ! {
    todo!("0xf39834 j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE9singletonEv")
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::ThreadPoolConfig>::singleton(void)")]
// 0xf39844 — j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE9singletonEv
pub fn stub_f39844() -> ! {
    todo!("0xf39844 j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE9singletonEv")
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::Job::SleepAdjustMethod>::singleton(void)")]
// 0xf39854 — j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE9singletonEv
pub fn stub_f39854() -> ! {
    todo!("0xf39854 j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE9singletonEv")
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::Time::SampleMethod>::singleton(void)")]
// 0xf39874 — j___ZN3rbx14implementation12typed_holderIN3RBX4Time12SampleMethodEE9singletonEv
pub fn stub_f39874() -> ! {
    todo!("0xf39874 j___ZN3rbx14implementation12typed_holderIN3RBX4Time12SampleMethodEE9singletonEv")
}


#[doc(alias = "rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::singleton(void)")]
// 0xf39884 — j___ZN3rbx14implementation12typed_holderIN3RBX9EThrottle13EThrottleTypeEE9singletonEv
pub fn stub_f39884() -> ! {
    todo!("0xf39884 j___ZN3rbx14implementation12typed_holderIN3RBX9EThrottle13EThrottleTypeEE9singletonEv")
}


#[doc(alias = "rbx::implementation::typed_holder<float>::singleton(void)")]
// 0xf39894 — j___ZN3rbx14implementation12typed_holderIfE9singletonEv
pub fn stub_f39894() -> ! {
    todo!("0xf39894 j___ZN3rbx14implementation12typed_holderIfE9singletonEv")
}


#[doc(alias = "RBX::DebugSettings::ErrorReporting const& rbx::any_cast<RBX::DebugSettings::ErrorReporting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf398a4 — j___ZN3rbx8any_castIRKN3RBX13DebugSettings14ErrorReportingENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f398a4() -> ! {
    todo!("0xf398a4 j___ZN3rbx8any_castIRKN3RBX13DebugSettings14ErrorReportingENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}


#[doc(alias = "RBX::TaskScheduler::PriorityMethod const& rbx::any_cast<RBX::TaskScheduler::PriorityMethod const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf398b4 — j___ZN3rbx8any_castIRKN3RBX13TaskScheduler14PriorityMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f398b4() -> ! {
    todo!("0xf398b4 j___ZN3rbx8any_castIRKN3RBX13TaskScheduler14PriorityMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}


#[doc(alias = "RBX::TaskScheduler::ThreadPoolConfig const& rbx::any_cast<RBX::TaskScheduler::ThreadPoolConfig const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf398c4 — j___ZN3rbx8any_castIRKN3RBX13TaskScheduler16ThreadPoolConfigENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f398c4() -> ! {
    todo!("0xf398c4 j___ZN3rbx8any_castIRKN3RBX13TaskScheduler16ThreadPoolConfigENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}


#[doc(alias = "RBX::TaskScheduler::Job::SleepAdjustMethod const& rbx::any_cast<RBX::TaskScheduler::Job::SleepAdjustMethod const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf398d4 — j___ZN3rbx8any_castIRKN3RBX13TaskScheduler3Job17SleepAdjustMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f398d4() -> ! {
    todo!("0xf398d4 j___ZN3rbx8any_castIRKN3RBX13TaskScheduler3Job17SleepAdjustMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}


#[doc(alias = "RBX::Time::SampleMethod const& rbx::any_cast<RBX::Time::SampleMethod const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf398e4 — j___ZN3rbx8any_castIRKN3RBX4Time12SampleMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f398e4() -> ! {
    todo!("0xf398e4 j___ZN3rbx8any_castIRKN3RBX4Time12SampleMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}


#[doc(alias = "RBX::EThrottle::EThrottleType const& rbx::any_cast<RBX::EThrottle::EThrottleType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf398f4 — j___ZN3rbx8any_castIRKN3RBX9EThrottle13EThrottleTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f398f4() -> ! {
    todo!("0xf398f4 j___ZN3rbx8any_castIRKN3RBX9EThrottle13EThrottleTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}


#[doc(alias = "float const& rbx::any_cast<float const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf39904 — j___ZN3rbx8any_castIRKfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f39904() -> ! {
    todo!("0xf39904 j___ZN3rbx8any_castIRKfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}


#[doc(alias = "rbx_core::SharedPtr<DummyArbiter>::shared_ptr<DummyArbiter>(DummyArbiter *)")]
// 0xf39914 — j___ZN5boost10shared_ptrI12DummyArbiterEC2IS1_EEPT_
// was: boost::shared_ptr<DummyArbiter>::shared_ptr<DummyArbiter>(DummyArbiter *)
pub fn stub_f39914() -> ! {
    todo!("0xf39914 j___ZN5boost10shared_ptrI12DummyArbiterEC2IS1_EEPT_")
}


#[doc(alias = "rbx_core::SharedPtr<DummyJob>::shared_ptr<DummyJob>(DummyJob *)")]
// 0xf39924 — j___ZN5boost10shared_ptrI8DummyJobEC2IS1_EEPT_
// was: boost::shared_ptr<DummyJob>::shared_ptr<DummyJob>(DummyJob *)
pub fn stub_f39924() -> ! {
    todo!("0xf39924 j___ZN5boost10shared_ptrI8DummyJobEC2IS1_EEPT_")
}


#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf39954 — j___ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f39954() -> ! {
    todo!("0xf39954 j___ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}


#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::destroy_content(boost::integral_constant<bool,true> const&)")]
// 0xf39964 — j___ZN5boost15circular_bufferIdSaIdEE15destroy_contentERKNS_17integral_constantIbLb1EEE
pub fn stub_f39964() -> ! {
    todo!("0xf39964 j___ZN5boost15circular_bufferIdSaIdEE15destroy_contentERKNS_17integral_constantIbLb1EEE")
}


#[doc(alias = "boost::detail::shared_count::shared_count<DummyArbiter>(DummyArbiter *)")]
// 0xf39974 — j___ZN5boost6detail12shared_countC2I12DummyArbiterEEPT_
pub fn stub_f39974() -> ! {
    todo!("0xf39974 j___ZN5boost6detail12shared_countC2I12DummyArbiterEEPT_")
}


#[doc(alias = "boost::detail::shared_count::shared_count<DummyJob>(DummyJob *)")]
// 0xf39984 — j___ZN5boost6detail12shared_countC2I8DummyJobEEPT_
pub fn stub_f39984() -> ! {
    todo!("0xf39984 j___ZN5boost6detail12shared_countC2I8DummyJobEEPT_")
}


#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::rate(void)const")]
// 0xf39bd4 — j___ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE4rateEv
pub fn stub_f39bd4() -> ! {
    todo!("0xf39bd4 j___ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE4rateEv")
}


#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<DummyJob,DummyJob>(rbx_core::SharedPtr<DummyJob> const*,DummyJob *)const")]
// 0xf39be4 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerI8DummyJobS6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<DummyJob,DummyJob>(boost::shared_ptr<DummyJob> const*,DummyJob *)const
pub fn stub_f39be4() -> ! {
    todo!("0xf39be4 j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerI8DummyJobS6_EEvPKNS_10shared_ptrIT_EEPT0_")
}


#[doc(alias = "std::_Vector_base<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_allocate(unsigned long)")]
// 0xf39bf4 — j___ZNSt12_Vector_baseIN3RBX13DebugSettings14ErrorReportingESaIS2_EE11_M_allocateEm
pub fn stub_f39bf4() -> ! {
    todo!("0xf39bf4 j___ZNSt12_Vector_baseIN3RBX13DebugSettings14ErrorReportingESaIS2_EE11_M_allocateEm")
}


#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_allocate(unsigned long)")]
// 0xf39c04 — j___ZNSt12_Vector_baseIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE11_M_allocateEm
pub fn stub_f39c04() -> ! {
    todo!("0xf39c04 j___ZNSt12_Vector_baseIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE11_M_allocateEm")
}


#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_allocate(unsigned long)")]
// 0xf39c14 — j___ZNSt12_Vector_baseIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE11_M_allocateEm
pub fn stub_f39c14() -> ! {
    todo!("0xf39c14 j___ZNSt12_Vector_baseIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE11_M_allocateEm")
}


#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_allocate(unsigned long)")]
// 0xf39c24 — j___ZNSt12_Vector_baseIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE11_M_allocateEm
pub fn stub_f39c24() -> ! {
    todo!("0xf39c24 j___ZNSt12_Vector_baseIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE11_M_allocateEm")
}


#[doc(alias = "std::_Vector_base<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_allocate(unsigned long)")]
// 0xf39c34 — j___ZNSt12_Vector_baseIN3RBX4Time12SampleMethodESaIS2_EE11_M_allocateEm
pub fn stub_f39c34() -> ! {
    todo!("0xf39c34 j___ZNSt12_Vector_baseIN3RBX4Time12SampleMethodESaIS2_EE11_M_allocateEm")
}


#[doc(alias = "std::_Vector_base<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_allocate(unsigned long)")]
// 0xf39c44 — j___ZNSt12_Vector_baseIN3RBX9EThrottle13EThrottleTypeESaIS2_EE11_M_allocateEm
pub fn stub_f39c44() -> ! {
    todo!("0xf39c44 j___ZNSt12_Vector_baseIN3RBX9EThrottle13EThrottleTypeESaIS2_EE11_M_allocateEm")
}


#[doc(alias = "RBX::DebugSettings::ErrorReporting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *>(RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *)")]
// 0xf39c54 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DebugSettings14ErrorReportingES6_EET0_T_S8_S7_
pub fn stub_f39c54() -> ! {
    todo!("0xf39c54 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DebugSettings14ErrorReportingES6_EET0_T_S8_S7_")
}


#[doc(alias = "RBX::TaskScheduler::PriorityMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *>(RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *)")]
// 0xf39c64 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler14PriorityMethodES6_EET0_T_S8_S7_
pub fn stub_f39c64() -> ! {
    todo!("0xf39c64 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler14PriorityMethodES6_EET0_T_S8_S7_")
}


#[doc(alias = "RBX::TaskScheduler::ThreadPoolConfig * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *>(RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *)")]
// 0xf39c74 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler16ThreadPoolConfigES6_EET0_T_S8_S7_
pub fn stub_f39c74() -> ! {
    todo!("0xf39c74 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler16ThreadPoolConfigES6_EET0_T_S8_S7_")
}


#[doc(alias = "RBX::TaskScheduler::Job::SleepAdjustMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *>(RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *)")]
// 0xf39c84 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler3Job17SleepAdjustMethodES7_EET0_T_S9_S8_
pub fn stub_f39c84() -> ! {
    todo!("0xf39c84 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler3Job17SleepAdjustMethodES7_EET0_T_S9_S8_")
}


#[doc(alias = "RBX::Time::SampleMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Time::SampleMethod *,RBX::Time::SampleMethod *>(RBX::Time::SampleMethod *,RBX::Time::SampleMethod *,RBX::Time::SampleMethod *)")]
// 0xf39c94 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4Time12SampleMethodES6_EET0_T_S8_S7_
pub fn stub_f39c94() -> ! {
    todo!("0xf39c94 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4Time12SampleMethodES6_EET0_T_S8_S7_")
}


#[doc(alias = "RBX::EThrottle::EThrottleType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *>(RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *)")]
// 0xf39ca4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9EThrottle13EThrottleTypeES6_EET0_T_S8_S7_
pub fn stub_f39ca4() -> ! {
    todo!("0xf39ca4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9EThrottle13EThrottleTypeES6_EET0_T_S8_S7_")
}


#[doc(alias = "std::map<RBX::Name const*,RBX::DebugSettings::ErrorReporting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::operator[](RBX::Name const* const&)")]
// 0xf39cb4 — j___ZNSt3mapIPKN3RBX4NameENS0_13DebugSettings14ErrorReportingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f39cb4() -> ! {
    todo!("0xf39cb4 j___ZNSt3mapIPKN3RBX4NameENS0_13DebugSettings14ErrorReportingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}


#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::PriorityMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::operator[](RBX::Name const* const&)")]
// 0xf39cc4 — j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler14PriorityMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f39cc4() -> ! {
    todo!("0xf39cc4 j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler14PriorityMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}


#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::ThreadPoolConfig,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::operator[](RBX::Name const* const&)")]
// 0xf39cd4 — j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler16ThreadPoolConfigESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f39cd4() -> ! {
    todo!("0xf39cd4 j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler16ThreadPoolConfigESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}


#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::Job::SleepAdjustMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::operator[](RBX::Name const* const&)")]
// 0xf39ce4 — j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler3Job17SleepAdjustMethodESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_
pub fn stub_f39ce4() -> ! {
    todo!("0xf39ce4 j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler3Job17SleepAdjustMethodESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_")
}


#[doc(alias = "std::map<RBX::Name const*,RBX::Time::SampleMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::operator[](RBX::Name const* const&)")]
// 0xf39cf4 — j___ZNSt3mapIPKN3RBX4NameENS0_4Time12SampleMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f39cf4() -> ! {
    todo!("0xf39cf4 j___ZNSt3mapIPKN3RBX4NameENS0_4Time12SampleMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}


#[doc(alias = "std::map<RBX::Name const*,RBX::EThrottle::EThrottleType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::operator[](RBX::Name const* const&)")]
// 0xf39d04 — j___ZNSt3mapIPKN3RBX4NameENS0_9EThrottle13EThrottleTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f39d04() -> ! {
    todo!("0xf39d04 j___ZNSt3mapIPKN3RBX4NameENS0_9EThrottle13EThrottleTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}


#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,RBX::DebugSettings::ErrorReporting const&)")]
// 0xf39d14 — j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f39d14() -> ! {
    todo!("0xf39d14 j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}


#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,unsigned long,RBX::DebugSettings::ErrorReporting const&)")]
// 0xf39d24 — j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f39d24() -> ! {
    todo!("0xf39d24 j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}


#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::resize(unsigned long,RBX::DebugSettings::ErrorReporting)")]
// 0xf39d34 — j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE6resizeEmS2_
pub fn stub_f39d34() -> ! {
    todo!("0xf39d34 j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE6resizeEmS2_")
}


#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::push_back(RBX::DebugSettings::ErrorReporting const&)")]
// 0xf39d44 — j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE9push_backERKS2_
pub fn stub_f39d44() -> ! {
    todo!("0xf39d44 j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE9push_backERKS2_")
}


#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,RBX::TaskScheduler::PriorityMethod const&)")]
// 0xf39d54 — j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f39d54() -> ! {
    todo!("0xf39d54 j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}


#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,unsigned long,RBX::TaskScheduler::PriorityMethod const&)")]
// 0xf39d64 — j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f39d64() -> ! {
    todo!("0xf39d64 j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}


#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::resize(unsigned long,RBX::TaskScheduler::PriorityMethod)")]
// 0xf39d74 — j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE6resizeEmS2_
pub fn stub_f39d74() -> ! {
    todo!("0xf39d74 j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE6resizeEmS2_")
}


#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::push_back(RBX::TaskScheduler::PriorityMethod const&)")]
// 0xf39d84 — j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE9push_backERKS2_
pub fn stub_f39d84() -> ! {
    todo!("0xf39d84 j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE9push_backERKS2_")
}


#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,RBX::TaskScheduler::ThreadPoolConfig const&)")]
// 0xf39d94 — j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f39d94() -> ! {
    todo!("0xf39d94 j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}


#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,unsigned long,RBX::TaskScheduler::ThreadPoolConfig const&)")]
// 0xf39da4 — j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f39da4() -> ! {
    todo!("0xf39da4 j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}


#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::resize(unsigned long,RBX::TaskScheduler::ThreadPoolConfig)")]
// 0xf39db4 — j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE6resizeEmS2_
pub fn stub_f39db4() -> ! {
    todo!("0xf39db4 j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE6resizeEmS2_")
}


#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::push_back(RBX::TaskScheduler::ThreadPoolConfig const&)")]
// 0xf39dc4 — j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE9push_backERKS2_
pub fn stub_f39dc4() -> ! {
    todo!("0xf39dc4 j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE9push_backERKS2_")
}


#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
// 0xf39dd4 — j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
pub fn stub_f39dd4() -> ! {
    todo!("0xf39dd4 j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}


#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
// 0xf39de4 — j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
pub fn stub_f39de4() -> ! {
    todo!("0xf39de4 j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")
}


#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::resize(unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod)")]
// 0xf39df4 — j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE6resizeEmS3_
pub fn stub_f39df4() -> ! {
    todo!("0xf39df4 j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE6resizeEmS3_")
}


#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::push_back(RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
// 0xf39e04 — j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE9push_backERKS3_
pub fn stub_f39e04() -> ! {
    todo!("0xf39e04 j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE9push_backERKS3_")
}


#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,RBX::Time::SampleMethod const&)")]
// 0xf39e14 — j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f39e14() -> ! {
    todo!("0xf39e14 j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}


#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,unsigned long,RBX::Time::SampleMethod const&)")]
// 0xf39e24 — j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f39e24() -> ! {
    todo!("0xf39e24 j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}


#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::resize(unsigned long,RBX::Time::SampleMethod)")]
// 0xf39e34 — j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE6resizeEmS2_
pub fn stub_f39e34() -> ! {
    todo!("0xf39e34 j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE6resizeEmS2_")
}


#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::push_back(RBX::Time::SampleMethod const&)")]
// 0xf39e44 — j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE9push_backERKS2_
pub fn stub_f39e44() -> ! {
    todo!("0xf39e44 j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE9push_backERKS2_")
}


#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,RBX::EThrottle::EThrottleType const&)")]
// 0xf39e54 — j___ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f39e54() -> ! {
    todo!("0xf39e54 j___ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}


#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,unsigned long,RBX::EThrottle::EThrottleType const&)")]
// 0xf39e64 — j___ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f39e64() -> ! {
    todo!("0xf39e64 j___ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}


#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::resize(unsigned long,RBX::EThrottle::EThrottleType)")]
// 0xf39e74 — j___ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE6resizeEmS2_
pub fn stub_f39e74() -> ! {
    todo!("0xf39e74 j___ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE6resizeEmS2_")
}


#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::push_back(RBX::EThrottle::EThrottleType const&)")]
// 0xf39e84 — j___ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE9push_backERKS2_
pub fn stub_f39e84() -> ! {
    todo!("0xf39e84 j___ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE9push_backERKS2_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
// 0xf39ea4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f39ea4() -> ! {
    todo!("0xf39ea4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
// 0xf39eb4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f39eb4() -> ! {
    todo!("0xf39eb4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>> *)")]
// 0xf39ec4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f39ec4() -> ! {
    todo!("0xf39ec4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
// 0xf39ed4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f39ed4() -> ! {
    todo!("0xf39ed4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
// 0xf39ee4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f39ee4() -> ! {
    todo!("0xf39ee4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
// 0xf39ef4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f39ef4() -> ! {
    todo!("0xf39ef4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>> *)")]
// 0xf39f04 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f39f04() -> ! {
    todo!("0xf39f04 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
// 0xf39f14 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f39f14() -> ! {
    todo!("0xf39f14 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
// 0xf39f24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f39f24() -> ! {
    todo!("0xf39f24 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
// 0xf39f34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f39f34() -> ! {
    todo!("0xf39f34 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>> *)")]
// 0xf39f44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f39f44() -> ! {
    todo!("0xf39f44 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
// 0xf39f54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f39f54() -> ! {
    todo!("0xf39f54 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler16ThreadPoolConfigEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
// 0xf39f64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_
pub fn stub_f39f64() -> ! {
    todo!("0xf39f64 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
// 0xf39f74 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
pub fn stub_f39f74() -> ! {
    todo!("0xf39f74 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>> *)")]
// 0xf39f84 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
pub fn stub_f39f84() -> ! {
    todo!("0xf39f84 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
// 0xf39f94 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
pub fn stub_f39f94() -> ! {
    todo!("0xf39f94 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")]
// 0xf39fa4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f39fa4() -> ! {
    todo!("0xf39fa4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")]
// 0xf39fb4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f39fb4() -> ! {
    todo!("0xf39fb4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>> *)")]
// 0xf39fc4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f39fc4() -> ! {
    todo!("0xf39fc4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")]
// 0xf39fd4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f39fd4() -> ! {
    todo!("0xf39fd4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
// 0xf39fe4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f39fe4() -> ! {
    todo!("0xf39fe4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
// 0xf39ff4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f39ff4() -> ! {
    todo!("0xf39ff4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>> *)")]
// 0xf3a004 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3a004() -> ! {
    todo!("0xf3a004 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
// 0xf3a014 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f3a014() -> ! {
    todo!("0xf3a014 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}


#[doc(alias = "RBX::TextureId * rbx::any_cast<RBX::TextureId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf3a114 — j___ZN3rbx8any_castIN3RBX9TextureIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f3a114() -> ! {
    todo!("0xf3a114 j___ZN3rbx8any_castIN3RBX9TextureIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}


#[doc(alias = "RBX::TextureId & rbx::any_cast<RBX::TextureId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf3a124 — j___ZN3rbx8any_castIRN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f3a124() -> ! {
    todo!("0xf3a124 j___ZN3rbx8any_castIRN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}


#[doc(alias = "RBX::DialogChoice::~DialogChoice()")]
// 0xf3a1c4 — j___ZN3RBX12DialogChoiceD1Ev
pub fn stub_f3a1c4() -> ! {
    todo!("0xf3a1c4 j___ZN3RBX12DialogChoiceD1Ev")
}


#[doc(alias = "RBX::DialogRoot::~DialogRoot()")]
// 0xf3a264 — j___ZN3RBX10DialogRootD2Ev
pub fn stub_f3a264() -> ! {
    todo!("0xf3a264 j___ZN3RBX10DialogRootD2Ev")
}

