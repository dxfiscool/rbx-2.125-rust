//! platform — generated_plat_bj — 100 stubs EA-sorted asc second fallback gap filler | Source ida/export.json | range 0x248bc4..0x250588 | rbx_core::SharedPtr not boost | excludes above namespaces
//! Source: ida/export.json (85545 funcs) global gap filler next 100 EA-sorted asc not yet stubbed in platform (second fallback)
//! Distinct stub_ 30901/85545 -> 31001/85545 | uncovered 54644 -> 54544 (platform)
//! Batch: 100 stubs | range 0x248bc4..0x250588 | rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x248bc4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_deleter(std::type_info const&)")]
pub fn stub_248bc4() -> ! {
    todo!("0x248bc4 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_deleter(std::type_info const&)")
}

// 0x248bdc — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_untyped_deleter(void)")]
pub fn stub_248bdc() -> ! {
    todo!("0x248bdc boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_untyped_deleter(void)")
}

// 0x248be0 — __GLOBAL__I_a_50
// type: void __fastcall(int, int, int, int, void *, int)
#[doc(alias = "global constructor keyed to_a_50")]
pub fn stub_248be0() -> ! {
    todo!("0x248be0 `global constructor keyed to_a_50")
}

// 0x248e38 — __ZNK3RBX13TaskScheduler3Job16averageDutyCycleEv
// type: __int64 __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::averageDutyCycle(void)const")]
pub fn stub_248e38() -> ! {
    todo!("0x248e38 RBX::TaskScheduler::Job::averageDutyCycle(void)const")
}

// 0x248eb0 — __ZNK3RBX13TaskScheduler3Job21averageStepsPerSecondEv
// type: __int64 __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::averageStepsPerSecond(void)const")]
pub fn stub_248eb0() -> ! {
    todo!("0x248eb0 RBX::TaskScheduler::Job::averageStepsPerSecond(void)const")
}

// 0x248f10 — __ZNK3RBX13TaskScheduler3Job15averageStepTimeEv
// type: __int64 __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::averageStepTime(void)const")]
pub fn stub_248f10() -> ! {
    todo!("0x248f10 RBX::TaskScheduler::Job::averageStepTime(void)const")
}

// 0x248f20 — __ZNK3RBX13TaskScheduler3Job12averageErrorEv
// type: __int64 __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::averageError(void)const")]
pub fn stub_248f20() -> ! {
    todo!("0x248f20 RBX::TaskScheduler::Job::averageError(void)const")
}

// 0x248f2c — __ZN3RBX13TaskScheduler3Job17removeCoordinatorEN5boost10shared_ptrINS_5Tasks11CoordinatorEEE
// type: void __fastcall(int, _DWORD *)
// was: RBX::TaskScheduler::Job::removeCoordinator(boost::shared_ptr<RBX::Tasks::Coordinator>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::TaskScheduler::Job::removeCoordinator(rbx_core::SharedPtr<RBX::Tasks::Coordinator>)")]
pub fn stub_248f2c() -> ! {
    todo!("0x248f2c RBX::TaskScheduler::Job::removeCoordinator(rbx_core::SharedPtr<RBX::Tasks::Coordinator>)")
}

// 0x2490ac — __ZN3RBX13TaskScheduler3Job14addCoordinatorEN5boost10shared_ptrINS_5Tasks11CoordinatorEEE
// type: void __fastcall(int, _DWORD *)
// was: RBX::TaskScheduler::Job::addCoordinator(boost::shared_ptr<RBX::Tasks::Coordinator>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::TaskScheduler::Job::addCoordinator(rbx_core::SharedPtr<RBX::Tasks::Coordinator>)")]
pub fn stub_2490ac() -> ! {
    todo!("0x2490ac RBX::TaskScheduler::Job::addCoordinator(rbx_core::SharedPtr<RBX::Tasks::Coordinator>)")
}

// 0x249184 — __ZN3RBX13TaskScheduler3Job10isDisabledEv
// type: bool __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::isDisabled(void)")]
pub fn stub_249184() -> ! {
    todo!("0x249184 RBX::TaskScheduler::Job::isDisabled(void)")
}

// 0x249270 — __ZN3RBX13TaskScheduler3JobC2EPKcN5boost10shared_ptrINS0_7ArbiterEEENS_4Time8IntervalE
// type: char *__fastcall(int, volatile int *, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, char, int, pthread_mutex_t *, int, int, int, int, int)
// was: RBX::TaskScheduler::Job::Job(char const*,boost::shared_ptr<RBX::TaskScheduler::Arbiter>,RBX::Time::Interval) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::TaskScheduler::Job::Job(char const*,rbx_core::SharedPtr<RBX::TaskScheduler::Arbiter>,RBX::Time::Interval)")]
pub fn stub_249270() -> ! {
    todo!("0x249270 RBX::TaskScheduler::Job::Job(char const*,rbx_core::SharedPtr<RBX::TaskScheduler::Arbiter>,RBX::Time::Interval)")
}

// 0x249920 — __ZN3RBX13TaskScheduler3JobD0Ev
// type: void __fastcall(RBX::TaskScheduler::Job *__hidden this)
#[doc(alias = "RBX::TaskScheduler::Job::~Job()")]
pub fn stub_249920() -> ! {
    todo!("0x249920 RBX::TaskScheduler::Job::~Job()")
}

// 0x2499c0 — __ZN3RBX13TaskScheduler3JobD1Ev
// type: void __fastcall(RBX::TaskScheduler::Job *__hidden this)
#[doc(alias = "RBX::TaskScheduler::Job::~Job()")]
pub fn stub_2499c0() -> ! {
    todo!("0x2499c0 RBX::TaskScheduler::Job::~Job()")
}

// 0x2499cc — __ZN3RBX13TaskScheduler3JobD2Ev
// type: void __fastcall(RBX::TaskScheduler::Job *this, int, int)
#[doc(alias = "RBX::TaskScheduler::Job::~Job()")]
pub fn stub_2499cc() -> ! {
    todo!("0x2499cc RBX::TaskScheduler::Job::~Job()")
}

// 0x24a1f8 — __ZN3RBX13TaskScheduler3Job20computeStandardErrorERKNS1_5StatsEd
// type: int __fastcall(int result, int, double *, unsigned int, unsigned int)
#[doc(alias = "RBX::TaskScheduler::Job::computeStandardError(RBX::TaskScheduler::Job::Stats const&,double)")]
pub fn stub_24a1f8() -> ! {
    todo!("0x24a1f8 RBX::TaskScheduler::Job::computeStandardError(RBX::TaskScheduler::Job::Stats const&,double)")
}

// 0x24a210 — __ZN3RBX13TaskScheduler3Job24computeStandardSleepTimeERKNS1_5StatsEd
// type: void __fastcall(RBX::TaskScheduler::Job *this, const RBX::TaskScheduler::Job::Stats *, double, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::TaskScheduler::Job::computeStandardSleepTime(RBX::TaskScheduler::Job::Stats const&,double)")]
pub fn stub_24a210() -> ! {
    todo!("0x24a210 RBX::TaskScheduler::Job::computeStandardSleepTime(RBX::TaskScheduler::Job::Stats const&,double)")
}

// 0x24a408 — __ZN3RBX13TaskScheduler3Job5StatsC1ERS1_NS_4TimeE
// type: RBX::TaskScheduler::Job::Stats *__fastcall(RBX::TaskScheduler::Job::Stats *this, RBX::TaskScheduler::Job *, Time)
#[doc(alias = "RBX::TaskScheduler::Job::Stats::Stats(RBX::TaskScheduler::Job&,RBX::Time)")]
pub fn stub_24a408() -> ! {
    todo!("0x24a408 RBX::TaskScheduler::Job::Stats::Stats(RBX::TaskScheduler::Job&,RBX::Time)")
}

// 0x24a440 — __ZN3RBX13TaskScheduler3Job12startWaitingEv
// type: int __fastcall(int this)
#[doc(alias = "RBX::TaskScheduler::Job::startWaiting(void)")]
pub fn stub_24a440() -> ! {
    todo!("0x24a440 RBX::TaskScheduler::Job::startWaiting(void)")
}

// 0x24a448 — __ZN3RBX13TaskScheduler3Job13startSleepingEv
// type: int __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::startSleeping(void)")]
pub fn stub_24a448() -> ! {
    todo!("0x24a448 RBX::TaskScheduler::Job::startSleeping(void)")
}

// 0x24a468 — __ZN3RBX13TaskScheduler3Job14updateWakeTimeEv
// type: int __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::updateWakeTime(void)")]
pub fn stub_24a468() -> ! {
    todo!("0x24a468 RBX::TaskScheduler::Job::updateWakeTime(void)")
}

// 0x24a4c0 — __ZN3RBX13TaskScheduler3Job11updateErrorERKNS_4TimeE
// type: int __fastcall(RBX::TaskScheduler::Job *this, const RBX::Time *)
#[doc(alias = "RBX::TaskScheduler::Job::updateError(RBX::Time const&)")]
pub fn stub_24a4c0() -> ! {
    todo!("0x24a4c0 RBX::TaskScheduler::Job::updateError(RBX::Time const&)")
}

// 0x24a598 — __ZN3RBX13TaskScheduler3Job25notifyCoordinatorsPreStepEv
// type: void __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::notifyCoordinatorsPreStep(void)")]
pub fn stub_24a598() -> ! {
    todo!("0x24a598 RBX::TaskScheduler::Job::notifyCoordinatorsPreStep(void)")
}

// 0x24a684 — __ZN3RBX13TaskScheduler3Job7preStepEv
// type: void __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::preStep(void)")]
pub fn stub_24a684() -> ! {
    todo!("0x24a684 RBX::TaskScheduler::Job::preStep(void)")
}

// 0x24a8b8 — __ZN3RBX13TaskScheduler3Job8postStepENS0_10StepResultE
// type: void __fastcall(int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::Job::postStep(RBX::TaskScheduler::StepResult)")]
pub fn stub_24a8b8() -> ! {
    todo!("0x24a8b8 RBX::TaskScheduler::Job::postStep(RBX::TaskScheduler::StepResult)")
}

// 0x24ab18 — __ZN3RBX13TaskScheduler3Job26notifyCoordinatorsPostStepEv
// type: void __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::notifyCoordinatorsPostStep(void)")]
pub fn stub_24ab18() -> ! {
    todo!("0x24ab18 RBX::TaskScheduler::Job::notifyCoordinatorsPostStep(void)")
}

// 0x24ac08 — __ZN3RBX13TaskScheduler3Job14updatePriorityEv
// type: double *__fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::updatePriority(void)")]
pub fn stub_24ac08() -> ! {
    todo!("0x24ac08 RBX::TaskScheduler::Job::updatePriority(void)")
}

// 0x24ad00 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int, int)
// was: std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>::push_back(boost::shared_ptr<RBX::Tasks::Coordinator> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::push_back(rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&)")]
pub fn stub_24ad00() -> ! {
    todo!("0x24ad00 std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::push_back(rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&)")
}

// 0x24ad90 — __ZN3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE6sampleENS1_8IntervalE
// type: int __fastcall(__int64 *, unsigned int, unsigned int)
#[doc(alias = "RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::sample(RBX::Time::Interval)")]
pub fn stub_24ad90() -> ! {
    todo!("0x24ad90 RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::sample(RBX::Time::Interval)")
}

// 0x24ae08 — __ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv
// type: int __fastcall(int)
#[doc(alias = "RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample(void)")]
pub fn stub_24ae08() -> ! {
    todo!("0x24ae08 RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample(void)")
}

// 0x24ae88 — __ZN5boost15circular_bufferIdSaIdEE8allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::allocate(unsigned long)")]
pub fn stub_24ae88() -> ! {
    todo!("0x24ae88 boost::circular_buffer<double,std::allocator<double>>::allocate(unsigned long)")
}

// 0x24afb0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev
// type: void __fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
pub fn stub_24afb0() -> ! {
    todo!("0x24afb0 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")
}

// 0x24b070 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv
// type: void __fastcall __noreturn(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const")]
pub fn stub_24b070() -> ! {
    todo!("0x24b070 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const")
}

// 0x24b120 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv
// type: void __fastcall __noreturn(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const")]
pub fn stub_24b120() -> ! {
    todo!("0x24b120 `virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const")
}

// 0x24b130 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
pub fn stub_24b130() -> ! {
    todo!("0x24b130 `virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")
}

// 0x24b208 — __ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
pub fn stub_24b208() -> ! {
    todo!("0x24b208 `non-virtual thunk toboost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")
}

// 0x24b2c8 — __ZN3RBX13WindowAverageIddE6sampleINS_13FOnBeforeDropEEEvdRT_
// type: int __fastcall(__int64 *, unsigned int, unsigned int, int)
#[doc(alias = "void RBX::WindowAverage<double,double>::sample<RBX::FOnBeforeDrop>(double,RBX::FOnBeforeDrop &)")]
pub fn stub_24b2c8() -> ! {
    todo!("0x24b2c8 void RBX::WindowAverage<double,double>::sample<RBX::FOnBeforeDrop>(double,RBX::FOnBeforeDrop &)")
}

// 0x24b364 — __ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EEC2ENS1_8IntervalE
// type: int __fastcall(int, unsigned int, unsigned int)
#[doc(alias = "RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::WindowAverageTimeInterval(RBX::Time::Interval)")]
pub fn stub_24b364() -> ! {
    todo!("0x24b364 RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::WindowAverageTimeInterval(RBX::Time::Interval)")
}

// 0x24b45c — __ZN3RBX14RunningAverageIddEC2Eddj
// type: int *__fastcall(int *, int, int, unsigned int, unsigned int, unsigned int, int, int, int, int)
#[doc(alias = "RBX::RunningAverage<double,double>::RunningAverage(double,double,unsigned int)")]
pub fn stub_24b45c() -> ! {
    todo!("0x24b45c RBX::RunningAverage<double,double>::RunningAverage(double,double,unsigned int)")
}

// 0x24b5a4 — __ZN5boost6detail12shared_countC2INS_15circular_bufferIdSaIdEEEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::circular_buffer<double,std::allocator<double>>>(boost::circular_buffer<double,std::allocator<double>> *)")]
pub fn stub_24b5a4() -> ! {
    todo!("0x24b5a4 boost::detail::shared_count::shared_count<boost::circular_buffer<double,std::allocator<double>>>(boost::circular_buffer<double,std::allocator<double>> *)")
}

// 0x24b6c8 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()")]
pub fn stub_24b6c8() -> ! {
    todo!("0x24b6c8 boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()")
}

// 0x24b6cc — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()")]
pub fn stub_24b6cc() -> ! {
    todo!("0x24b6cc boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()")
}

// 0x24b6d8 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::dispose(void)")]
pub fn stub_24b6d8() -> ! {
    todo!("0x24b6d8 boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::dispose(void)")
}

// 0x24b714 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_deleter(std::type_info const&)")]
pub fn stub_24b714() -> ! {
    todo!("0x24b714 boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_deleter(std::type_info const&)")
}

// 0x24b718 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_untyped_deleter(void)")]
pub fn stub_24b718() -> ! {
    todo!("0x24b718 boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_untyped_deleter(void)")
}

// 0x24b71c — __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESt6vectorIS7_SaIS7_EEEENS2_3_bi6bind_tIbNS2_4_mfi3mf1IbS6_PNS4_13TaskScheduler3JobEEENSD_5list2INS2_3argILi1EEENSD_5valueISJ_EEEEEEET_SS_SS_T0_St26random_access_iterator_tag
// type: _DWORD *__fastcall(_DWORD *, int, int (*)(void), int, int)
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>> std::__find_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>,std::random_access_iterator_tag) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>> std::__find_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>,std::random_access_iterator_tag)")]
pub fn stub_24b71c() -> ! {
    todo!("0x24b71c __gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>> std::__find_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>,std::random_access_iterator_tag)")
}

// 0x24b860 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: void __fastcall(int, char *, int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
// was: std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator>*,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::shared_ptr<RBX::Tasks::Coordinator> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>*,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&)")]
pub fn stub_24b860() -> ! {
    todo!("0x24b860 std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>*,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&)")
}

// 0x24bdf8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_
// type: int __fastcall(int, int, int)
// was: boost::shared_ptr<RBX::Tasks::Coordinator> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *>(boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *)")]
pub fn stub_24bdf8() -> ! {
    todo!("0x24bdf8 rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *)")
}

// 0x24beb0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_
// type: _DWORD *__fastcall(int, int, _DWORD *)
// was: boost::shared_ptr<RBX::Tasks::Coordinator> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *>(boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *)")]
pub fn stub_24beb0() -> ! {
    todo!("0x24beb0 rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *)")
}

// 0x24bf64 — __GLOBAL__I_a_51
#[doc(alias = "global constructor keyed to_a_51")]
pub fn stub_24bf64() -> ! {
    todo!("0x24bf64 `global constructor keyed to_a_51")
}

// 0x24c02c — __ZN3RBX13TaskScheduler13endAllThreadsEv
// type: int __fastcall(int this)
#[doc(alias = "RBX::TaskScheduler::endAllThreads(void)")]
pub fn stub_24c02c() -> ! {
    todo!("0x24c02c RBX::TaskScheduler::endAllThreads(void)")
}

// 0x24c048 — __ZN3RBX13TaskScheduler14setThreadCountENS0_16ThreadPoolConfigE
// type: void __fastcall(int, int, int, int, boost::detail::sp_counted_base *, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::setThreadCount(RBX::TaskScheduler::ThreadPoolConfig)")]
pub fn stub_24c048() -> ! {
    todo!("0x24c048 RBX::TaskScheduler::setThreadCount(RBX::TaskScheduler::ThreadPoolConfig)")
}

// 0x24c1ec — __ZN3RBX13TaskScheduler6Thread6runJobEv
// type: int __fastcall(RBX::TaskScheduler::Thread *this)
#[doc(alias = "RBX::TaskScheduler::Thread::runJob(void)")]
pub fn stub_24c1ec() -> ! {
    todo!("0x24c1ec RBX::TaskScheduler::Thread::runJob(void)")
}

// 0x24c43c — __ZNK3RBX13TaskScheduler25conflictsWithScheduledJobEPNS0_3JobE
// type: int __fastcall(RBX::TaskScheduler *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::TaskScheduler::conflictsWithScheduledJob(RBX::TaskScheduler::Job *)const")]
pub fn stub_24c43c() -> ! {
    todo!("0x24c43c RBX::TaskScheduler::conflictsWithScheduledJob(RBX::TaskScheduler::Job *)const")
}

// 0x24c5bc — __ZN3RBX13TaskScheduler13enableThreadsERSt6vectorIN5boost10shared_ptrINS0_6ThreadEEESaIS5_EE
// type: int __fastcall(int, __int64 *, int)
// was: RBX::TaskScheduler::enableThreads(std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>> &) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::TaskScheduler::enableThreads(std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>> &)")]
pub fn stub_24c5bc() -> ! {
    todo!("0x24c5bc RBX::TaskScheduler::enableThreads(std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>> &)")
}

// 0x24c660 — __ZN3RBX13TaskScheduler6Thread4loopEv
// type: void __fastcall(int32_t **this, volatile int *)
#[doc(alias = "RBX::TaskScheduler::Thread::loop(void)")]
pub fn stub_24c660() -> ! {
    todo!("0x24c660 RBX::TaskScheduler::Thread::loop(void)")
}

// 0x24cd18 — __ZN3RBX13TaskScheduler11getJobsInfoERSt6vectorIN5boost10shared_ptrIKNS0_3JobEEESaIS6_EE
// type: void __fastcall(int, int)
// was: RBX::TaskScheduler::getJobsInfo(std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>> &) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::TaskScheduler::getJobsInfo(std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>> &)")]
pub fn stub_24cd18() -> ! {
    todo!("0x24cd18 RBX::TaskScheduler::getJobsInfo(std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>> &)")
}

// 0x24ce78 — __ZN3RBX13TaskScheduler26setJobsExtendedStatsWindowEd
// type: void __fastcall(RBX::TaskScheduler *this, double)
#[doc(alias = "RBX::TaskScheduler::setJobsExtendedStatsWindow(double)")]
pub fn stub_24ce78() -> ! {
    todo!("0x24ce78 RBX::TaskScheduler::setJobsExtendedStatsWindow(double)")
}

// 0x24d05c — __ZL25setJobExtendedStatsWindowN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEEd
// type: int *__fastcall(int *result, unsigned int, unsigned int)
// was: setJobExtendedStatsWindow(boost::shared_ptr<RBX::TaskScheduler::Job>,double) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "setJobExtendedStatsWindow(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double)")]
pub fn stub_24d05c() -> ! {
    todo!("0x24d05c setJobExtendedStatsWindow(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double)")
}

// 0x24d0ec — __ZN3RBX13TaskScheduler13getJobsByNameERKSsRSt6vectorIN5boost10shared_ptrIKNS0_3JobEEESaIS8_EE
// type: void __fastcall(int, const void **, int)
// was: RBX::TaskScheduler::getJobsByName(std::string const&,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>> &) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::TaskScheduler::getJobsByName(std::string const&,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>> &)")]
pub fn stub_24d0ec() -> ! {
    todo!("0x24d0ec RBX::TaskScheduler::getJobsByName(std::string const&,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>> &)")
}

// 0x24d284 — __ZN3RBX13TaskScheduler6Thread4joinEv
// type: pthread_t __fastcall(RBX::TaskScheduler::Thread *this)
#[doc(alias = "RBX::TaskScheduler::Thread::join(void)")]
pub fn stub_24d284() -> ! {
    todo!("0x24d284 RBX::TaskScheduler::Thread::join(void)")
}

// 0x24d2dc — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int, int)
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Thread> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&)")]
pub fn stub_24d2dc() -> ! {
    todo!("0x24d2dc std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&)")
}

// 0x24d368 — __ZN3RBX13TaskScheduler6Thread6createEPS0_
// type: void __fastcall(RBX::TaskScheduler::Thread *this, RBX::TaskScheduler *)
#[doc(alias = "RBX::TaskScheduler::Thread::create(RBX::TaskScheduler*)")]
pub fn stub_24d368() -> ! {
    todo!("0x24d368 RBX::TaskScheduler::Thread::create(RBX::TaskScheduler*)")
}

// 0x24d770 — __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE9push_backERKS6_
// type: int __fastcall(int, int)
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Job const> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&)")]
pub fn stub_24d770() -> ! {
    todo!("0x24d770 std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&)")
}

// 0x24d7fc — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int, int)
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Job> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_24d7fc() -> ! {
    todo!("0x24d7fc std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")
}

// 0x24d88c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIdEEEclIPFvNS_10shared_ptrIN3RBX13TaskScheduler3JobEEEdENS0_5list1IRSC_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(_DWORD *, void (__fastcall **)(int *, _DWORD, _DWORD), int **)
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<double>>::operator()<void (*)(boost::shared_ptr<RBX::TaskScheduler::Job>,double),boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::TaskScheduler::Job>,double) &,boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job>&> &,int) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<double>>::operator()<void (*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double),boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job>&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job>&> &,int)")]
pub fn stub_24d88c() -> ! {
    todo!("0x24d88c void boost::_bi::list2<boost::arg<1>,boost::_bi::value<double>>::operator()<void (*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double),boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job>&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job>&> &,int)")
}

// 0x24d9a4 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: void __fastcall(int, char *, int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>>,boost::shared_ptr<RBX::TaskScheduler::Job> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_24d9a4() -> ! {
    todo!("0x24d9a4 std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")
}

// 0x24df3c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES9_EET0_T_SB_SA_
// type: int __fastcall(int, int, int)
// was: boost::shared_ptr<RBX::TaskScheduler::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *>(boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *)")]
pub fn stub_24df3c() -> ! {
    todo!("0x24df3c rbx_core::SharedPtr<RBX::TaskScheduler::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *)")
}

// 0x24dff8 — __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_
// type: void __fastcall(int, char *, int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::shared_ptr<RBX::TaskScheduler::Job const> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&)")]
pub fn stub_24dff8() -> ! {
    todo!("0x24dff8 std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&)")
}

// 0x24e590 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESA_EET0_T_SC_SB_
// type: int __fastcall(int, int, int)
// was: boost::shared_ptr<RBX::TaskScheduler::Job const> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *>(boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job const> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *)")]
pub fn stub_24e590() -> ! {
    todo!("0x24e590 rbx_core::SharedPtr<RBX::TaskScheduler::Job const> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *)")
}

// 0x24e648 — __ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE6sampleEv
// type: int __fastcall(int, int)
#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::sample(void)")]
pub fn stub_24e648() -> ! {
    todo!("0x24e648 RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::sample(void)")
}

// 0x24e6a8 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_
// type: _DWORD *__fastcall(int, int, _DWORD *)
// was: boost::shared_ptr<RBX::TaskScheduler::Thread> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *>(boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *)")]
pub fn stub_24e6a8() -> ! {
    todo!("0x24e6a8 rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *)")
}

// 0x24e75c — __ZN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEC2IS3_EERKNS_8weak_ptrIT_EE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
// was: boost::shared_ptr<RBX::TaskScheduler::Thread>::shared_ptr<RBX::TaskScheduler::Thread>(boost::weak_ptr<RBX::TaskScheduler::Thread> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Thread>::shared_ptr<RBX::TaskScheduler::Thread>(rbx_core::WeakPtr<RBX::TaskScheduler::Thread> const&)")]
pub fn stub_24e75c() -> ! {
    todo!("0x24e75c rbx_core::SharedPtr<RBX::TaskScheduler::Thread>::shared_ptr<RBX::TaskScheduler::Thread>(rbx_core::WeakPtr<RBX::TaskScheduler::Thread> const&)")
}

// 0x24e870 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE5resetEPS4_
// type: void __fastcall(int *, const void *)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::reset(RBX::TaskScheduler::Job **)")]
pub fn stub_24e870() -> ! {
    todo!("0x24e870 boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::reset(RBX::TaskScheduler::Job **)")
}

// 0x24e98c — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: void __fastcall(int, char *, int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Thread>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::shared_ptr<RBX::TaskScheduler::Thread> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&)")]
pub fn stub_24e98c() -> ! {
    todo!("0x24e98c std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&)")
}

// 0x24ef24 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_
// type: int __fastcall(int, int, int)
// was: boost::shared_ptr<RBX::TaskScheduler::Thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *>(boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *)")]
pub fn stub_24ef24() -> ! {
    todo!("0x24ef24 rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *)")
}

// 0x24efe0 — __ZN5boost15circular_bufferIdSaIdEE12set_capacityEm
// type: int __fastcall(char **, unsigned int)
#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::set_capacity(unsigned long)")]
pub fn stub_24efe0() -> ! {
    todo!("0x24efe0 boost::circular_buffer<double,std::allocator<double>>::set_capacity(unsigned long)")
}

// 0x24f0d8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
// type: std::logic_error *__fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
pub fn stub_24f0d8() -> ! {
    todo!("0x24f0d8 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")
}

// 0x24f190 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorED1Ev
// type: int __fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
pub fn stub_24f190() -> ! {
    todo!("0x24f190 boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")
}

// 0x24f248 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&)")]
pub fn stub_24f248() -> ! {
    todo!("0x24f248 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&)")
}

// 0x24f388 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_NS5_9clone_tagE
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_tag)")]
pub fn stub_24f388() -> ! {
    todo!("0x24f388 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_tag)")
}

// 0x24f520 — __ZN5boost4bindIvN3RBX13TaskScheduler6ThreadENS_10shared_ptrIS3_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf0IS8_T0_EENS6_9list_av_1IT1_E4typeEEEMSB_FS8_vESE_
// type: void __fastcall(int, struct _Unwind_Exception *, boost::detail::sp_counted_base *, int)
// was: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list_av_1<boost::shared_ptr<RBX::TaskScheduler::Thread>>::type> boost::bind<void,RBX::TaskScheduler::Thread,boost::shared_ptr<RBX::TaskScheduler::Thread>>(void (RBX::TaskScheduler::Thread::*)(void),boost::shared_ptr<RBX::TaskScheduler::Thread>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>::type> boost::bind<void,RBX::TaskScheduler::Thread,rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>(void (RBX::TaskScheduler::Thread::*)(void),rbx_core::SharedPtr<RBX::TaskScheduler::Thread>)")]
pub fn stub_24f520() -> ! {
    todo!("0x24f520 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>::type> boost::bind<void,RBX::TaskScheduler::Thread,rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>(void (RBX::TaskScheduler::Thread::*)(void),rbx_core::SharedPtr<RBX::TaskScheduler::Thread>)")
}

// 0x24f6c0 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13TaskScheduler6ThreadEEEEEEC2ES8_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>)")]
pub fn stub_24f6c0() -> ! {
    todo!("0x24f6c0 boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>)")
}

// 0x24f808 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub fn stub_24f808() -> ! {
    todo!("0x24f808 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")
}

// 0x24f93c — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>)")]
pub fn stub_24f93c() -> ! {
    todo!("0x24f93c void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>)")
}

// 0x24fa7c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_24fa7c() -> ! {
    todo!("0x24fa7c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x24faa0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int *)
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,void>::invoke(boost::detail::function::function_buffer &) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_24faa0() -> ! {
    todo!("0x24faa0 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x24fac0 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &)const (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_24fac0() -> ! {
    todo!("0x24fac0 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &)const")
}

// 0x24fbf4 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_24fbf4() -> ! {
    todo!("0x24fbf4 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x24fdac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_24fdac() -> ! {
    todo!("0x24fdac boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x24ff48 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::~sp_counted_impl_p()")]
pub fn stub_24ff48() -> ! {
    todo!("0x24ff48 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::~sp_counted_impl_p()")
}

// 0x24ff58 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler6ThreadEE22_internal_accept_ownerIS3_S3_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Thread>::_internal_accept_owner<RBX::TaskScheduler::Thread,RBX::TaskScheduler::Thread>(boost::shared_ptr<RBX::TaskScheduler::Thread> const*,RBX::TaskScheduler::Thread *)const (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Thread>::_internal_accept_owner<RBX::TaskScheduler::Thread,RBX::TaskScheduler::Thread>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const*,RBX::TaskScheduler::Thread *)const")]
pub fn stub_24ff58() -> ! {
    todo!("0x24ff58 void boost::enable_shared_from_this<RBX::TaskScheduler::Thread>::_internal_accept_owner<RBX::TaskScheduler::Thread,RBX::TaskScheduler::Thread>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const*,RBX::TaskScheduler::Thread *)const")
}

// 0x2500b0 — __ZN5boost6detail12shared_countC2IN3RBX13TaskScheduler6ThreadEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskScheduler::Thread>(RBX::TaskScheduler::Thread *)")]
pub fn stub_2500b0() -> ! {
    todo!("0x2500b0 boost::detail::shared_count::shared_count<RBX::TaskScheduler::Thread>(RBX::TaskScheduler::Thread *)")
}

// 0x2501bc — __ZN3RBX13TaskScheduler6ThreadD2Ev
// type: void __fastcall(boost::thread **this)
#[doc(alias = "RBX::TaskScheduler::Thread::~Thread()")]
pub fn stub_2501bc() -> ! {
    todo!("0x2501bc RBX::TaskScheduler::Thread::~Thread()")
}

// 0x2503f4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()")]
pub fn stub_2503f4() -> ! {
    todo!("0x2503f4 boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()")
}

// 0x2503f8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()")]
pub fn stub_2503f8() -> ! {
    todo!("0x2503f8 boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()")
}

// 0x250404 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::dispose(void)")]
pub fn stub_250404() -> ! {
    todo!("0x250404 boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::dispose(void)")
}

// 0x2504a8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_deleter(std::type_info const&)")]
pub fn stub_2504a8() -> ! {
    todo!("0x2504a8 boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_deleter(std::type_info const&)")
}

// 0x2504ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_untyped_deleter(void)")]
pub fn stub_2504ac() -> ! {
    todo!("0x2504ac boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_untyped_deleter(void)")
}

// 0x2504b0 — __ZN5boost6thread10timed_joinERKNS_10posix_time5ptimeE
// type: int __fastcall(boost::thread *, int)
#[doc(alias = "boost::thread::timed_join(boost::posix_time::ptime const&)")]
pub fn stub_2504b0() -> ! {
    todo!("0x2504b0 boost::thread::timed_join(boost::posix_time::ptime const&)")
}

// 0x250588 — __ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE17add_time_durationERKS5_NS3_13time_durationE
// type: int __fastcall(int result, int *, __int64 *)
#[doc(alias = "boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::add_time_duration(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::posix_time::time_duration)")]
pub fn stub_250588() -> ! {
    todo!("0x250588 boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::add_time_duration(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::posix_time::time_duration)")
}
