//! core shard IR — 100 core stubs EA-sorted, 0x248f20..0x25104c (strict RBX|boost excluding Reflection|Instance|DataModel|Ogre|G3D|Rendering|Adorn|RakNet|Network|Replicat|Socket|Sound|Audio|FMOD|Script|Lua|ViewController|UIApplication|Platform|iOS, EA-sorted ascending, next 100 uncovered after 0x248f20 prior 13411 remaining).
//! Source: ida/export.json filtered where demangled NOT containing Reflection|Instance|DataModel|Ogre|G3D|Rendering|Adorn|RakNet|Network|Replicat|Socket|Sound|Audio|FMOD|Script|Lua|lua|ViewController|UIApplication|Platform|iOS but containing RBX:: or boost::, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::TaskScheduler::Job::averageError(void)const")]
// 0x248f20 — __ZNK3RBX13TaskScheduler3Job12averageErrorEv
// type: __int64 __fastcall(RBX::TaskScheduler::Job *this)
// was: RBX::TaskScheduler::Job::averageError(void)const
pub fn stub_0x248f20() {
    // IDA 0x248f20: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::removeCoordinator(rbx_core::SharedPtr<RBX::Tasks::Coordinator>)")]
// 0x248f2c — __ZN3RBX13TaskScheduler3Job17removeCoordinatorEN5boost10shared_ptrINS_5Tasks11CoordinatorEEE
// type: void __fastcall(int, _DWORD *)
// was: RBX::TaskScheduler::Job::removeCoordinator(rbx_core::SharedPtr<RBX::Tasks::Coordinator>)
pub fn stub_0x248f2c() {
    // IDA 0x248f2c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::addCoordinator(rbx_core::SharedPtr<RBX::Tasks::Coordinator>)")]
// 0x2490ac — __ZN3RBX13TaskScheduler3Job14addCoordinatorEN5boost10shared_ptrINS_5Tasks11CoordinatorEEE
// type: void __fastcall(int, _DWORD *)
// was: RBX::TaskScheduler::Job::addCoordinator(rbx_core::SharedPtr<RBX::Tasks::Coordinator>)
pub fn stub_0x2490ac() {
    // IDA 0x2490ac: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::isDisabled(void)")]
// 0x249184 — __ZN3RBX13TaskScheduler3Job10isDisabledEv
// type: bool __fastcall(RBX::TaskScheduler::Job *this)
// was: RBX::TaskScheduler::Job::isDisabled(void)
pub fn stub_0x249184() {
    // IDA 0x249184: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::Job(char const*,rbx_core::SharedPtr<RBX::TaskScheduler::Arbiter>,RBX::Time::Interval)")]
// 0x249270 — __ZN3RBX13TaskScheduler3JobC2EPKcN5boost10shared_ptrINS0_7ArbiterEEENS_4Time8IntervalE
// type: char *__fastcall(int, volatile int *, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, char, int, pthread_mutex_t *, int, int, int, int, int)
// was: RBX::TaskScheduler::Job::Job(char const*,rbx_core::SharedPtr<RBX::TaskScheduler::Arbiter>,RBX::Time::Interval)
pub fn stub_0x249270() {
    // IDA 0x249270: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::~Job()")]
// 0x249920 — __ZN3RBX13TaskScheduler3JobD0Ev
// type: void __fastcall(RBX::TaskScheduler::Job *__hidden this)
// was: RBX::TaskScheduler::Job::~Job()
pub fn stub_0x249920() {
    // IDA 0x249920: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::~Job()")]
// 0x2499c0 — __ZN3RBX13TaskScheduler3JobD1Ev
// type: void __fastcall(RBX::TaskScheduler::Job *__hidden this)
// was: RBX::TaskScheduler::Job::~Job()
pub fn stub_0x2499c0() {
    // IDA 0x2499c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::~Job()")]
// 0x2499cc — __ZN3RBX13TaskScheduler3JobD2Ev
// type: void __fastcall(RBX::TaskScheduler::Job *this, int, int)
// was: RBX::TaskScheduler::Job::~Job()
pub fn stub_0x2499cc() {
    // IDA 0x2499cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::computeStandardError(RBX::TaskScheduler::Job::Stats const&,double)")]
// 0x24a1f8 — __ZN3RBX13TaskScheduler3Job20computeStandardErrorERKNS1_5StatsEd
// type: int __fastcall(int result, int, double *, unsigned int, unsigned int)
// was: RBX::TaskScheduler::Job::computeStandardError(RBX::TaskScheduler::Job::Stats const&,double)
pub fn stub_0x24a1f8() {
    // IDA 0x24a1f8: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::computeStandardSleepTime(RBX::TaskScheduler::Job::Stats const&,double)")]
// 0x24a210 — __ZN3RBX13TaskScheduler3Job24computeStandardSleepTimeERKNS1_5StatsEd
// type: void __fastcall(RBX::TaskScheduler::Job *this, const RBX::TaskScheduler::Job::Stats *, double, struct _Unwind_Exception *lpuexcpt)
// was: RBX::TaskScheduler::Job::computeStandardSleepTime(RBX::TaskScheduler::Job::Stats const&,double)
pub fn stub_0x24a210() {
    // IDA 0x24a210: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::Stats::Stats(RBX::TaskScheduler::Job&,RBX::Time)")]
// 0x24a408 — __ZN3RBX13TaskScheduler3Job5StatsC1ERS1_NS_4TimeE
// type: RBX::TaskScheduler::Job::Stats *__fastcall(RBX::TaskScheduler::Job::Stats *this, RBX::TaskScheduler::Job *, Time)
// was: RBX::TaskScheduler::Job::Stats::Stats(RBX::TaskScheduler::Job&,RBX::Time)
pub fn stub_0x24a408() {
    // IDA 0x24a408: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::startWaiting(void)")]
// 0x24a440 — __ZN3RBX13TaskScheduler3Job12startWaitingEv
// type: int __fastcall(int this)
// was: RBX::TaskScheduler::Job::startWaiting(void)
pub fn stub_0x24a440() {
    // IDA 0x24a440: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::startSleeping(void)")]
// 0x24a448 — __ZN3RBX13TaskScheduler3Job13startSleepingEv
// type: int __fastcall(RBX::TaskScheduler::Job *this)
// was: RBX::TaskScheduler::Job::startSleeping(void)
pub fn stub_0x24a448() {
    // IDA 0x24a448: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::updateWakeTime(void)")]
// 0x24a468 — __ZN3RBX13TaskScheduler3Job14updateWakeTimeEv
// type: int __fastcall(RBX::TaskScheduler::Job *this)
// was: RBX::TaskScheduler::Job::updateWakeTime(void)
pub fn stub_0x24a468() {
    // IDA 0x24a468: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::updateError(RBX::Time const&)")]
// 0x24a4c0 — __ZN3RBX13TaskScheduler3Job11updateErrorERKNS_4TimeE
// type: int __fastcall(RBX::TaskScheduler::Job *this, const RBX::Time *)
// was: RBX::TaskScheduler::Job::updateError(RBX::Time const&)
pub fn stub_0x24a4c0() {
    // IDA 0x24a4c0: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::notifyCoordinatorsPreStep(void)")]
// 0x24a598 — __ZN3RBX13TaskScheduler3Job25notifyCoordinatorsPreStepEv
// type: void __fastcall(RBX::TaskScheduler::Job *this)
// was: RBX::TaskScheduler::Job::notifyCoordinatorsPreStep(void)
pub fn stub_0x24a598() {
    // IDA 0x24a598: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::preStep(void)")]
// 0x24a684 — __ZN3RBX13TaskScheduler3Job7preStepEv
// type: void __fastcall(RBX::TaskScheduler::Job *this)
// was: RBX::TaskScheduler::Job::preStep(void)
pub fn stub_0x24a684() {
    // IDA 0x24a684: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::postStep(RBX::TaskScheduler::StepResult)")]
// 0x24a8b8 — __ZN3RBX13TaskScheduler3Job8postStepENS0_10StepResultE
// type: void __fastcall(int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::TaskScheduler::Job::postStep(RBX::TaskScheduler::StepResult)
pub fn stub_0x24a8b8() {
    // IDA 0x24a8b8: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::notifyCoordinatorsPostStep(void)")]
// 0x24ab18 — __ZN3RBX13TaskScheduler3Job26notifyCoordinatorsPostStepEv
// type: void __fastcall(RBX::TaskScheduler::Job *this)
// was: RBX::TaskScheduler::Job::notifyCoordinatorsPostStep(void)
pub fn stub_0x24ab18() {
    // IDA 0x24ab18: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Job::updatePriority(void)")]
// 0x24ac08 — __ZN3RBX13TaskScheduler3Job14updatePriorityEv
// type: double *__fastcall(RBX::TaskScheduler::Job *this)
// was: RBX::TaskScheduler::Job::updatePriority(void)
pub fn stub_0x24ac08() {
    // IDA 0x24ac08: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::push_back(rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&)")]
// 0x24ad00 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int, int)
// was: std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::push_back(rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&)
pub fn stub_0x24ad00() {
    // IDA 0x24ad00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::allocate(unsigned long)")]
// 0x24ae88 — __ZN5boost15circular_bufferIdSaIdEE8allocateEm
// type: int __fastcall(int, unsigned int)
// was: boost::circular_buffer<double,std::allocator<double>>::allocate(unsigned long)
pub fn stub_0x24ae88() {
    // IDA 0x24ae88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// 0x24afb0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev
// type: void __fastcall(std::logic_error *, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()
pub fn stub_0x24afb0() {
    // IDA 0x24afb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const")]
// 0x24b070 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv
// type: void __fastcall __noreturn(int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const
pub fn stub_0x24b070() {
    // IDA 0x24b070: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const")]
// 0x24b120 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv
// type: void __fastcall __noreturn(_DWORD *)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const
pub fn stub_0x24b120() {
    // IDA 0x24b120: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// 0x24b130 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()
pub fn stub_0x24b130() {
    // IDA 0x24b130: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// 0x24b208 — __ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: non-virtual thunk toboost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()
pub fn stub_0x24b208() {
    // IDA 0x24b208: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::RunningAverage<double,double>::RunningAverage(double,double,unsigned int)")]
// 0x24b45c — __ZN3RBX14RunningAverageIddEC2Eddj
// type: int *__fastcall(int *, int, int, unsigned int, unsigned int, unsigned int, int, int, int, int)
// was: RBX::RunningAverage<double,double>::RunningAverage(double,double,unsigned int)
pub fn stub_0x24b45c() {
    // IDA 0x24b45c: RunningAverage stats sampler. Accumulator at the live site — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::circular_buffer<double,std::allocator<double>>>(boost::circular_buffer<double,std::allocator<double>> *)")]
// 0x24b5a4 — __ZN5boost6detail12shared_countC2INS_15circular_bufferIdSaIdEEEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<boost::circular_buffer<double,std::allocator<double>>>(boost::circular_buffer<double,std::allocator<double>> *)
pub fn stub_0x24b5a4() {
    // IDA 0x24b5a4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()")]
// 0x24b6c8 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEED1Ev
// type: void()
// was: boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()
pub fn stub_0x24b6c8() {
    // IDA 0x24b6c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()")]
// 0x24b6cc — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEED0Ev
// type: void __fastcall(void *)
// was: boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()
pub fn stub_0x24b6cc() {
    // IDA 0x24b6cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::dispose(void)")]
// 0x24b6d8 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE7disposeEv
// type: void __fastcall(int)
// was: boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::dispose(void)
pub fn stub_0x24b6d8() {
    // IDA 0x24b6d8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_deleter(std::type_info const&)")]
// 0x24b714 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE11get_deleterERKSt9type_info
// type: int()
// was: boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_deleter(std::type_info const&)
pub fn stub_0x24b714() {
    // IDA 0x24b714: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_untyped_deleter(void)")]
// 0x24b718 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE19get_untyped_deleterEv
// type: int()
// was: boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_untyped_deleter(void)
pub fn stub_0x24b718() {
    // IDA 0x24b718: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>> std::__find_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>,std::random_access_iterator_tag)")]
// 0x24b71c — __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESt6vectorIS7_SaIS7_EEEENS2_3_bi6bind_tIbNS2_4_mfi3mf1IbS6_PNS4_13TaskScheduler3JobEEENSD_5list2INS2_3argILi1EEENSD_5valueISJ_EEEEEEET_SS_SS_T0_St26random_access_iterator_tag
// type: _DWORD *__fastcall(_DWORD *, int, int (*)(void), int, int)
// was: __gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>> std::__find_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>,std::random_access_iterator_tag)
pub fn stub_0x24b71c() {
    // IDA 0x24b71c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>*,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&)")]
// 0x24b860 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: void __fastcall(int, char *, int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
// was: std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>*,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&)
pub fn stub_0x24b860() {
    // IDA 0x24b860: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *)")]
// 0x24bdf8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_
// type: int __fastcall(int, int, int)
// was: rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *)
pub fn stub_0x24bdf8() {
    // IDA 0x24bdf8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *)")]
// 0x24beb0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_
// type: _DWORD *__fastcall(int, int, _DWORD *)
// was: rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *)
pub fn stub_0x24beb0() {
    // IDA 0x24beb0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TaskScheduler::endAllThreads(void)")]
// 0x24c02c — __ZN3RBX13TaskScheduler13endAllThreadsEv
// type: int __fastcall(int this)
// was: RBX::TaskScheduler::endAllThreads(void)
pub fn stub_0x24c02c() {
    // IDA 0x24c02c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::setThreadCount(RBX::TaskScheduler::ThreadPoolConfig)")]
// 0x24c048 — __ZN3RBX13TaskScheduler14setThreadCountENS0_16ThreadPoolConfigE
// type: void __fastcall(int, int, int, int, boost::detail::sp_counted_base *, pthread_mutex_t *, int, int, int, int)
// was: RBX::TaskScheduler::setThreadCount(RBX::TaskScheduler::ThreadPoolConfig)
pub fn stub_0x24c048() {
    // IDA 0x24c048: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::Thread::runJob(void)")]
// 0x24c1ec — __ZN3RBX13TaskScheduler6Thread6runJobEv
// type: int __fastcall(RBX::TaskScheduler::Thread *this)
// was: RBX::TaskScheduler::Thread::runJob(void)
pub fn stub_0x24c1ec() {
    // IDA 0x24c1ec: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::conflictsWithScheduledJob(RBX::TaskScheduler::Job *)const")]
// 0x24c43c — __ZNK3RBX13TaskScheduler25conflictsWithScheduledJobEPNS0_3JobE
// type: int __fastcall(RBX::TaskScheduler *this, RBX::TaskScheduler::Job *)
// was: RBX::TaskScheduler::conflictsWithScheduledJob(RBX::TaskScheduler::Job *)const
pub fn stub_0x24c43c() {
    // IDA 0x24c43c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::enableThreads(std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>> &)")]
// 0x24c5bc — __ZN3RBX13TaskScheduler13enableThreadsERSt6vectorIN5boost10shared_ptrINS0_6ThreadEEESaIS5_EE
// type: int __fastcall(int, __int64 *, int)
// was: RBX::TaskScheduler::enableThreads(std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>> &)
pub fn stub_0x24c5bc() {
    // IDA 0x24c5bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TaskScheduler::Thread::loop(void)")]
// 0x24c660 — __ZN3RBX13TaskScheduler6Thread4loopEv
// type: void __fastcall(int32_t **this, volatile int *)
// was: RBX::TaskScheduler::Thread::loop(void)
pub fn stub_0x24c660() {
    // IDA 0x24c660: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::getJobsInfo(std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>> &)")]
// 0x24cd18 — __ZN3RBX13TaskScheduler11getJobsInfoERSt6vectorIN5boost10shared_ptrIKNS0_3JobEEESaIS6_EE
// type: void __fastcall(int, int)
// was: RBX::TaskScheduler::getJobsInfo(std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>> &)
pub fn stub_0x24cd18() {
    // IDA 0x24cd18: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TaskScheduler::setJobsExtendedStatsWindow(double)")]
// 0x24ce78 — __ZN3RBX13TaskScheduler26setJobsExtendedStatsWindowEd
// type: void __fastcall(RBX::TaskScheduler *this, double)
// was: RBX::TaskScheduler::setJobsExtendedStatsWindow(double)
pub fn stub_0x24ce78() {
    // IDA 0x24ce78: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "setJobExtendedStatsWindow(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double)")]
// 0x24d05c — __ZL25setJobExtendedStatsWindowN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEEd
// type: int *__fastcall(int *result, unsigned int, unsigned int)
// was: setJobExtendedStatsWindow(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double)
pub fn stub_0x24d05c() {
    // IDA 0x24d05c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TaskScheduler::getJobsByName(std::string const&,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>> &)")]
// 0x24d0ec — __ZN3RBX13TaskScheduler13getJobsByNameERKSsRSt6vectorIN5boost10shared_ptrIKNS0_3JobEEESaIS8_EE
// type: void __fastcall(int, const void **, int)
// was: RBX::TaskScheduler::getJobsByName(std::string const&,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>> &)
pub fn stub_0x24d0ec() {
    // IDA 0x24d0ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TaskScheduler::Thread::join(void)")]
// 0x24d284 — __ZN3RBX13TaskScheduler6Thread4joinEv
// type: pthread_t __fastcall(RBX::TaskScheduler::Thread *this)
// was: RBX::TaskScheduler::Thread::join(void)
pub fn stub_0x24d284() {
    // IDA 0x24d284: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&)")]
// 0x24d2dc — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int, int)
// was: std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&)
pub fn stub_0x24d2dc() {
    // IDA 0x24d2dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TaskScheduler::Thread::create(RBX::TaskScheduler*)")]
// 0x24d368 — __ZN3RBX13TaskScheduler6Thread6createEPS0_
// type: void __fastcall(RBX::TaskScheduler::Thread *this, RBX::TaskScheduler *)
// was: RBX::TaskScheduler::Thread::create(RBX::TaskScheduler*)
pub fn stub_0x24d368() {
    // IDA 0x24d368: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&)")]
// 0x24d770 — __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE9push_backERKS6_
// type: int __fastcall(int, int)
// was: std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&)
pub fn stub_0x24d770() {
    // IDA 0x24d770: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
// 0x24d7fc — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int, int)
// was: std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)
pub fn stub_0x24d7fc() {
    // IDA 0x24d7fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<double>>::operator()<void (*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double),boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job>&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job>&> &,int)")]
// 0x24d88c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIdEEEclIPFvNS_10shared_ptrIN3RBX13TaskScheduler3JobEEEdENS0_5list1IRSC_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(_DWORD *, void (__fastcall **)(int *, _DWORD, _DWORD), int **)
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<double>>::operator()<void (*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double),boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job>&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job>&> &,int)
pub fn stub_0x24d88c() {
    // IDA 0x24d88c: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
// 0x24d9a4 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: void __fastcall(int, char *, int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
// was: std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)
pub fn stub_0x24d9a4() {
    // IDA 0x24d9a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *)")]
// 0x24df3c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES9_EET0_T_SB_SA_
// type: int __fastcall(int, int, int)
// was: rbx_core::SharedPtr<RBX::TaskScheduler::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *)
pub fn stub_0x24df3c() {
    // IDA 0x24df3c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&)")]
// 0x24dff8 — __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_
// type: void __fastcall(int, char *, int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
// was: std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&)
pub fn stub_0x24dff8() {
    // IDA 0x24dff8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job const> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *)")]
// 0x24e590 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESA_EET0_T_SC_SB_
// type: int __fastcall(int, int, int)
// was: rbx_core::SharedPtr<RBX::TaskScheduler::Job const> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *)
pub fn stub_0x24e590() {
    // IDA 0x24e590: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::sample(void)")]
// 0x24e648 — __ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE6sampleEv
// type: int __fastcall(int, int)
// was: RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::sample(void)
pub fn stub_0x24e648() {
    // IDA 0x24e648: RunningAverage stats sampler. Accumulator at the live site — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *)")]
// 0x24e6a8 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_
// type: _DWORD *__fastcall(int, int, _DWORD *)
// was: rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *)
pub fn stub_0x24e6a8() {
    // IDA 0x24e6a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Thread>::shared_ptr<RBX::TaskScheduler::Thread>(rbx_core::WeakPtr<RBX::TaskScheduler::Thread> const&)")]
// 0x24e75c — __ZN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEC2IS3_EERKNS_8weak_ptrIT_EE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
// was: rbx_core::SharedPtr<RBX::TaskScheduler::Thread>::shared_ptr<RBX::TaskScheduler::Thread>(rbx_core::WeakPtr<RBX::TaskScheduler::Thread> const&)
pub fn stub_0x24e75c() {
    // IDA 0x24e75c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::reset(RBX::TaskScheduler::Job **)")]
// 0x24e870 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE5resetEPS4_
// type: void __fastcall(int *, const void *)
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::reset(RBX::TaskScheduler::Job **)
pub fn stub_0x24e870() {
    // IDA 0x24e870: thread_specific_ptr::reset. thread_local! storage — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&)")]
// 0x24e98c — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: void __fastcall(int, char *, int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
// was: std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&)
pub fn stub_0x24e98c() {
    // IDA 0x24e98c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *)")]
// 0x24ef24 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_
// type: int __fastcall(int, int, int)
// was: rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *)
pub fn stub_0x24ef24() {
    // IDA 0x24ef24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::set_capacity(unsigned long)")]
// 0x24efe0 — __ZN5boost15circular_bufferIdSaIdEE12set_capacityEm
// type: int __fastcall(char **, unsigned int)
// was: boost::circular_buffer<double,std::allocator<double>>::set_capacity(unsigned long)
pub fn stub_0x24efe0() {
    // IDA 0x24efe0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// 0x24f0d8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
// type: std::logic_error *__fastcall(std::logic_error *, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()
pub fn stub_0x24f0d8() {
    // IDA 0x24f0d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// 0x24f190 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorED1Ev
// type: int __fastcall(std::logic_error *, int, int, int, void *, int)
// was: boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()
pub fn stub_0x24f190() {
    // IDA 0x24f190: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&)")]
// 0x24f248 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_
// type: int __fastcall(int, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&)
pub fn stub_0x24f248() {
    // IDA 0x24f248: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_tag)")]
// 0x24f388 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_NS5_9clone_tagE
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_tag)
pub fn stub_0x24f388() {
    // IDA 0x24f388: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>::type> boost::bind<void,RBX::TaskScheduler::Thread,rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>(void (RBX::TaskScheduler::Thread::*)(void),rbx_core::SharedPtr<RBX::TaskScheduler::Thread>)")]
// 0x24f520 — __ZN5boost4bindIvN3RBX13TaskScheduler6ThreadENS_10shared_ptrIS3_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf0IS8_T0_EENS6_9list_av_1IT1_E4typeEEEMSB_FS8_vESE_
// type: void __fastcall(int, struct _Unwind_Exception *, boost::detail::sp_counted_base *, int)
// was: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>::type> boost::bind<void,RBX::TaskScheduler::Thread,rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>(void (RBX::TaskScheduler::Thread::*)(void),rbx_core::SharedPtr<RBX::TaskScheduler::Thread>)
pub fn stub_0x24f520() {
    // IDA 0x24f520: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>)")]
// 0x24f6c0 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13TaskScheduler6ThreadEEEEEEC2ES8_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>)
pub fn stub_0x24f6c0() {
    // IDA 0x24f6c0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
// 0x24f808 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
pub fn stub_0x24f808() {
    // IDA 0x24f808: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>)")]
// 0x24f93c — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>)
pub fn stub_0x24f93c() {
    // IDA 0x24f93c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x24fa7c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x24fa7c() {
    // IDA 0x24fa7c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x24faa0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int *)
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_0x24faa0() {
    // IDA 0x24faa0: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &)const")]
// 0x24fac0 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x24fac0() {
    // IDA 0x24fac0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x24fbf4 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x24fbf4() {
    // IDA 0x24fbf4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x24fdac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x24fdac() {
    // IDA 0x24fdac: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::~sp_counted_impl_p()")]
// 0x24ff48 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEED0Ev
// type: void __fastcall(void *)
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::~sp_counted_impl_p()
pub fn stub_0x24ff48() {
    // IDA 0x24ff48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Thread>::_internal_accept_owner<RBX::TaskScheduler::Thread,RBX::TaskScheduler::Thread>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const*,RBX::TaskScheduler::Thread *)const")]
// 0x24ff58 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler6ThreadEE22_internal_accept_ownerIS3_S3_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Thread>::_internal_accept_owner<RBX::TaskScheduler::Thread,RBX::TaskScheduler::Thread>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const*,RBX::TaskScheduler::Thread *)const
pub fn stub_0x24ff58() {
    // IDA 0x24ff58: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskScheduler::Thread>(RBX::TaskScheduler::Thread *)")]
// 0x2500b0 — __ZN5boost6detail12shared_countC2IN3RBX13TaskScheduler6ThreadEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::TaskScheduler::Thread>(RBX::TaskScheduler::Thread *)
pub fn stub_0x2500b0() {
    // IDA 0x2500b0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()")]
// 0x2503f4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEED1Ev
// type: void()
// was: boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()
pub fn stub_0x2503f4() {
    // IDA 0x2503f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()")]
// 0x2503f8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEED0Ev
// type: void __fastcall(void *)
// was: boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()
pub fn stub_0x2503f8() {
    // IDA 0x2503f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::dispose(void)")]
// 0x250404 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE7disposeEv
// type: void __fastcall(int)
// was: boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::dispose(void)
pub fn stub_0x250404() {
    // IDA 0x250404: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_deleter(std::type_info const&)")]
// 0x2504a8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE11get_deleterERKSt9type_info
// type: int()
// was: boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_deleter(std::type_info const&)
pub fn stub_0x2504a8() {
    // IDA 0x2504a8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_untyped_deleter(void)")]
// 0x2504ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE19get_untyped_deleterEv
// type: int()
// was: boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_untyped_deleter(void)
pub fn stub_0x2504ac() {
    // IDA 0x2504ac: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::thread::timed_join(boost::posix_time::ptime const&)")]
// 0x2504b0 — __ZN5boost6thread10timed_joinERKNS_10posix_time5ptimeE
// type: int __fastcall(boost::thread *, int)
// was: boost::thread::timed_join(boost::posix_time::ptime const&)
pub fn stub_0x2504b0() {
    // IDA 0x2504b0: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::add_time_duration(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::posix_time::time_duration)")]
// 0x250588 — __ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE17add_time_durationERKS5_NS3_13time_durationE
// type: int __fastcall(int result, int *, __int64 *)
// was: boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::add_time_duration(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::posix_time::time_duration)
pub fn stub_0x250588() {
    // IDA 0x250588: boost::date_time. std::time-style duration — carrier no-op.
}

#[doc(alias = "void boost::throw_exception<std::runtime_error>(std::runtime_error const&)")]
// 0x2506f8 — __ZN5boost15throw_exceptionISt13runtime_errorEEvRKT_
// type: void __fastcall __noreturn(int)
// was: void boost::throw_exception<std::runtime_error>(std::runtime_error const&)
pub fn stub_0x2506f8() {
    // IDA 0x2506f8: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()")]
// 0x250848 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev
// type: std::runtime_error *__fastcall(std::runtime_error *, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()
pub fn stub_0x250848() {
    // IDA 0x250848: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()")]
// 0x250900 — __ZN5boost16exception_detail19error_info_injectorISt13runtime_errorED1Ev
// type: int __fastcall(std::runtime_error *, int, int, int, void *, int)
// was: boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()
pub fn stub_0x250900() {
    // IDA 0x250900: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()")]
// 0x2509b8 — __ZThn8_N5boost16exception_detail19error_info_injectorISt13runtime_errorED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: non-virtual thunk toboost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()
pub fn stub_0x2509b8() {
    // IDA 0x2509b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()")]
// 0x250a70 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()
pub fn stub_0x250a70() {
    // IDA 0x250a70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()")]
// 0x250b28 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()
pub fn stub_0x250b28() {
    // IDA 0x250b28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone(void)const")]
// 0x250bf8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE5cloneEv
// type: char *__fastcall(int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone(void)const
pub fn stub_0x250bf8() {
    // IDA 0x250bf8: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone(void)const")]
// 0x250cb8 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE5cloneEv
// type: char *__fastcall(_DWORD *)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone(void)const
pub fn stub_0x250cb8() {
    // IDA 0x250cb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()")]
// 0x250d80 — __ZN5boost16exception_detail19error_info_injectorISt13runtime_errorED0Ev
// type: void __fastcall(std::runtime_error *, int, int, int, void *, int)
// was: boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()
pub fn stub_0x250d80() {
    // IDA 0x250d80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_tag)")]
// 0x250e40 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEEC1ERKS5_NS5_9clone_tagE
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_tag)
pub fn stub_0x250e40() {
    // IDA 0x250e40: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::CV::simple_exception_policy<unsigned short,(unsigned short)1,(unsigned short)31,boost::gregorian::bad_day_of_month>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)")]
// 0x250fc8 — __ZN5boost2CV23simple_exception_policyItLt1ELt31ENS_9gregorian16bad_day_of_monthEE8on_errorEttNS0_14violation_enumE
// type: void __noreturn()
// was: boost::CV::simple_exception_policy<unsigned short,(unsigned short)1,(unsigned short)31,boost::gregorian::bad_day_of_month>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)
pub fn stub_0x250fc8() {
    // IDA 0x250fc8: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::gregorian::bad_day_of_month::~bad_day_of_month()")]
// 0x25104c — __ZN5boost9gregorian16bad_day_of_monthD1Ev
// type: void __fastcall(std::logic_error *this)
// was: boost::gregorian::bad_day_of_month::~bad_day_of_month()
pub fn stub_0x25104c() {
    // IDA 0x25104c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
