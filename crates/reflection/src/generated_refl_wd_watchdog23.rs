//! reflection — generated_refl_wd_watchdog23 — 100 stubs EA-sorted asc global gap filler 0x386b60..0x38b490 distinct from watchdog22
//! Source: ida/export.json (85545 funcs) EA asc reflection missing next 100 after reserving 100 for watchdog22 (after 0x37d1f8)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Shard watchdog23 — global gap filler distinct from watchdog22

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x386b60 — __ZN5boost10shared_ptrINS_6threadEE5resetIS1_EEvPT_
#[doc(alias = "void boost::shared_ptr<boost::thread>::reset<boost::thread>(boost::thread *)")]
#[doc(alias = "__ZN5boost10shared_ptrINS_6threadEE5resetIS1_EEvPT_")]
pub fn stub_0x386b60() -> ! {
    todo!("0x386b60 void boost::shared_ptr<boost::thread>::reset<boost::thread>(boost::thread *)")
}

// 0x386b8c — __ZN5boost4bindIvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS1_INS2_5mutexEEES5_S7_EENS_3_bi6bind_tIT_PFSA_T0_T1_ENS8_9list_av_2IT2_T3_E4typeEEESE_SG_SH_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list_av_2<boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>>::type> boost::bind<void,boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>,boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>>(void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>)")]
#[doc(alias = "__ZN5boost4bindIvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS1_INS2_5mutexEEES5_S7_EENS_3_bi6bind_tIT_PFSA_T0_T1_ENS8_9list_av_2IT2_T3_E4typeEEESE_SG_SH_")]
pub fn stub_0x386b8c() -> ! {
    todo!("0x386b8c boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list_av_2<boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>>::type> boost::bind<void,boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>,boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>>(void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>)")
}

// 0x386d74 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_ENSB_5list1INS2_3argILi1EEEEEEEET0_T_SL_SK_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>>)")]
#[doc(alias = "__ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_ENSB_5list1INS2_3argILi1EEEEEEEET0_T_SL_SK_")]
pub fn stub_0x386d74() -> ! {
    todo!("0x386d74 boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>>)")
}

// 0x386db4 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_NS2_9date_time18subsecond_durationINS2_10posix_time13time_durationELx1000EEEENSB_5list2INS2_3argILi1EEENSB_5valueISH_EEEEEEET0_T_SS_SR_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>)")]
#[doc(alias = "__ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_NS2_9date_time18subsecond_durationINS2_10posix_time13time_durationELx1000EEEENSB_5list2INS2_3argILi1EEENSB_5valueISH_EEEEEEET0_T_SS_SR_")]
pub fn stub_0x386db4() -> ! {
    todo!("0x386db4 boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>)")
}

// 0x386df0 — __ZNK5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEclES4_
#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::mutex>>::operator()(boost::shared_ptr<RBX::mutex>)const")]
#[doc(alias = "__ZNK5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEclES4_")]
pub fn stub_0x386df0() -> ! {
    todo!("0x386df0 boost::function1<void,boost::shared_ptr<RBX::mutex>>::operator()(boost::shared_ptr<RBX::mutex>)const")
}

// 0x386f00 — __ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE4pushERKS8_
#[doc(alias = "rbx::safe_queue<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>::push(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")]
#[doc(alias = "__ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE4pushERKS8_")]
pub fn stub_0x386f00() -> ! {
    todo!("0x386f00 rbx::safe_queue<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>::push(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")
}

// 0x386fc4 — __ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE9push_heapERKS3_
#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::push_heap(RBX::PriorityThreadPool::PriorityTask const&)")]
#[doc(alias = "__ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE9push_heapERKS3_")]
pub fn stub_0x386fc4() -> ! {
    todo!("0x386fc4 rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::push_heap(RBX::PriorityThreadPool::PriorityTask const&)")
}

// 0x3870ec — __ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE19pop_heap_if_presentERS3_
#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::pop_heap_if_present(RBX::PriorityThreadPool::PriorityTask&)")]
#[doc(alias = "__ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE19pop_heap_if_presentERS3_")]
pub fn stub_0x3870ec() -> ! {
    todo!("0x3870ec rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::pop_heap_if_present(RBX::PriorityThreadPool::PriorityTask&)")
}

// 0x387290 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEaSERKS6_
#[doc(alias = "boost::function<void ()(boost::shared_ptr<RBX::mutex>)>::operator=(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")]
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEaSERKS6_")]
pub fn stub_0x387290() -> ! {
    todo!("0x387290 boost::function<void ()(boost::shared_ptr<RBX::mutex>)>::operator=(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")
}

// 0x387354 — __ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD1Ev
#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()")]
#[doc(alias = "__ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD1Ev")]
pub fn stub_0x387354() -> ! {
    todo!("0x387354 RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()")
}

// 0x387448 — __ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD0Ev
#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()")]
#[doc(alias = "__ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD0Ev")]
pub fn stub_0x387448() -> ! {
    todo!("0x387448 RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()")
}

// 0x38754c — __ZN3RBX10ThreadPoolD1Ev
#[doc(alias = "RBX::ThreadPool::~ThreadPool()")]
#[doc(alias = "__ZN3RBX10ThreadPoolD1Ev")]
pub fn stub_0x38754c() -> ! {
    todo!("0x38754c RBX::ThreadPool::~ThreadPool()")
}

// 0x387550 — __ZN3RBX10ThreadPoolD0Ev
#[doc(alias = "RBX::ThreadPool::~ThreadPool()")]
#[doc(alias = "__ZN3RBX10ThreadPoolD0Ev")]
pub fn stub_0x387550() -> ! {
    todo!("0x387550 RBX::ThreadPool::~ThreadPool()")
}

// 0x3875f0 — __ZN3RBX18PriorityThreadPoolD1Ev
#[doc(alias = "RBX::PriorityThreadPool::~PriorityThreadPool()")]
#[doc(alias = "__ZN3RBX18PriorityThreadPoolD1Ev")]
pub fn stub_0x3875f0() -> ! {
    todo!("0x3875f0 RBX::PriorityThreadPool::~PriorityThreadPool()")
}

// 0x3875f4 — __ZN3RBX18PriorityThreadPoolD0Ev
#[doc(alias = "RBX::PriorityThreadPool::~PriorityThreadPool()")]
#[doc(alias = "__ZN3RBX18PriorityThreadPoolD0Ev")]
pub fn stub_0x3875f4() -> ! {
    todo!("0x3875f4 RBX::PriorityThreadPool::~PriorityThreadPool()")
}

// 0x387694 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE4swapERS5_
#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::mutex>>::swap(boost::function1<void,boost::shared_ptr<RBX::mutex>>&)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE4swapERS5_")]
pub fn stub_0x387694() -> ! {
    todo!("0x387694 boost::function1<void,boost::shared_ptr<RBX::mutex>>::swap(boost::function1<void,boost::shared_ptr<RBX::mutex>>&)")
}

// 0x387770 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE11move_assignERS5_
#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::mutex>>::move_assign(boost::function1<void,boost::shared_ptr<RBX::mutex>>&)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE11move_assignERS5_")]
pub fn stub_0x387770() -> ! {
    todo!("0x387770 boost::function1<void,boost::shared_ptr<RBX::mutex>>::move_assign(boost::function1<void,boost::shared_ptr<RBX::mutex>>&)")
}

// 0x387874 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
#[doc(alias = "__ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_")]
pub fn stub_0x387874() -> ! {
    todo!("0x387874 void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")
}

// 0x3879ec — __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
#[doc(alias = "__ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_")]
pub fn stub_0x3879ec() -> ! {
    todo!("0x3879ec void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")
}

// 0x387a60 — __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::push_back(RBX::PriorityThreadPool::PriorityTask const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE9push_backERKS2_")]
pub fn stub_0x387a60() -> ! {
    todo!("0x387a60 std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::push_back(RBX::PriorityThreadPool::PriorityTask const&)")
}

// 0x387aac — __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask*,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,RBX::PriorityThreadPool::PriorityTask const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x387aac() -> ! {
    todo!("0x387aac std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask*,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,RBX::PriorityThreadPool::PriorityTask const&)")
}

// 0x387e64 — __ZNSt12_Vector_baseIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE11_M_allocateEm")]
pub fn stub_0x387e64() -> ! {
    todo!("0x387e64 std::_Vector_base<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_allocate(unsigned long)")
}

// 0x387e88 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18PriorityThreadPool12PriorityTaskES6_EET0_T_S8_S7_
#[doc(alias = "RBX::PriorityThreadPool::PriorityTask * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *>(RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18PriorityThreadPool12PriorityTaskES6_EET0_T_S8_S7_")]
pub fn stub_0x387e88() -> ! {
    todo!("0x387e88 RBX::PriorityThreadPool::PriorityTask * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *>(RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *)")
}

// 0x387ee8 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9push_backERKS7_
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::push_back(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9push_backERKS7_")]
pub fn stub_0x387ee8() -> ! {
    todo!("0x387ee8 std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::push_back(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")
}

// 0x387f18 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE16_M_push_back_auxERKS7_
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_push_back_aux(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE16_M_push_back_auxERKS7_")]
pub fn stub_0x387f18() -> ! {
    todo!("0x387f18 std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_push_back_aux(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")
}

// 0x388050 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE22_M_reserve_map_at_backEm
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_reserve_map_at_back(unsigned long)")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE22_M_reserve_map_at_backEm")]
pub fn stub_0x388050() -> ! {
    todo!("0x388050 std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_reserve_map_at_back(unsigned long)")
}

// 0x38806c — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_reallocate_map(unsigned long,bool)")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_reallocate_mapEmb")]
pub fn stub_0x38806c() -> ! {
    todo!("0x38806c std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_reallocate_map(unsigned long,bool)")
}

// 0x388144 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_allocate_mapEm
#[doc(alias = "std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_allocate_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_allocate_mapEm")]
pub fn stub_0x388144() -> ! {
    todo!("0x388144 std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_allocate_map(unsigned long)")
}

// 0x38815c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_9date_time18subsecond_durationINS_10posix_time13time_durationELx1000EEEEEEclIPFvNS_10shared_ptrINS_6threadEEES9_ENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>::operator()<void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list1<boost::shared_ptr<boost::thread>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>) &,boost::_bi::list1<boost::shared_ptr<boost::thread>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_9date_time18subsecond_durationINS_10posix_time13time_durationELx1000EEEEEEclIPFvNS_10shared_ptrINS_6threadEEES9_ENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x38815c() -> ! {
    todo!("0x38815c void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>::operator()<void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list1<boost::shared_ptr<boost::thread>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>) &,boost::_bi::list1<boost::shared_ptr<boost::thread>&> &,int)")
}

// 0x388238 — __ZN5boost3_bi5list1INS_3argILi1EEEEclIPFvNS_10shared_ptrINS_6threadEEEENS1_IRS8_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list1<boost::arg<1>>::operator()<void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::shared_ptr<boost::thread>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<boost::thread>) &,boost::_bi::list1<boost::shared_ptr<boost::thread>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list1INS_3argILi1EEEEclIPFvNS_10shared_ptrINS_6threadEEEENS1_IRS8_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x388238() -> ! {
    todo!("0x388238 void boost::_bi::list1<boost::arg<1>>::operator()<void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::shared_ptr<boost::thread>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<boost::thread>) &,boost::_bi::list1<boost::shared_ptr<boost::thread>&> &,int)")
}

// 0x388304 — __ZN5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEC2ERKNS_9gregorian4dateERKNS2_13time_durationE
#[doc(alias = "boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::counted_time_rep(boost::gregorian::date const&,boost::posix_time::time_duration const&)")]
#[doc(alias = "__ZN5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEC2ERKNS_9gregorian4dateERKNS2_13time_durationE")]
pub fn stub_0x388304() -> ! {
    todo!("0x388304 boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::counted_time_rep(boost::gregorian::date const&,boost::posix_time::time_duration const&)")
}

// 0x38845c — __ZN5boost9date_time22time_resolution_traitsINS0_37time_resolution_traits_adapted64_implELNS0_16time_resolutionsE5ELx1000000ELt6EiE13to_tick_countEiiix
#[doc(alias = "boost::date_time::time_resolution_traits<boost::date_time::time_resolution_traits_adapted64_impl,(boost::date_time::time_resolutions)5,1000000ll,(unsigned short)6,int>::to_tick_count(int,int,int,long long)")]
#[doc(alias = "__ZN5boost9date_time22time_resolution_traitsINS0_37time_resolution_traits_adapted64_implELNS0_16time_resolutionsE5ELx1000000ELt6EiE13to_tick_countEiiix")]
pub fn stub_0x38845c() -> ! {
    todo!("0x38845c boost::date_time::time_resolution_traits<boost::date_time::time_resolution_traits_adapted64_impl,(boost::date_time::time_resolutions)5,1000000ll,(unsigned short)6,int>::to_tick_count(int,int,int,long long)")
}

// 0x38850c — __ZN5boost10shared_ptrINS_6threadEEC2IS1_EEPT_
#[doc(alias = "boost::shared_ptr<boost::thread>::shared_ptr<boost::thread>(boost::thread *)")]
#[doc(alias = "__ZN5boost10shared_ptrINS_6threadEEC2IS1_EEPT_")]
pub fn stub_0x38850c() -> ! {
    todo!("0x38850c boost::shared_ptr<boost::thread>::shared_ptr<boost::thread>(boost::thread *)")
}

// 0x3885e0 — __ZN5boost6detail12shared_countC2INS_6threadEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<boost::thread>(boost::thread *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2INS_6threadEEEPT_")]
pub fn stub_0x3885e0() -> ! {
    todo!("0x3885e0 boost::detail::shared_count::shared_count<boost::thread>(boost::thread *)")
}

// 0x3886ec — __ZN5boost6detail17sp_counted_impl_pINS_6threadEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_6threadEED1Ev")]
pub fn stub_0x3886ec() -> ! {
    todo!("0x3886ec boost::detail::sp_counted_impl_p<boost::thread>::~sp_counted_impl_p()")
}

// 0x3886f0 — __ZN5boost6detail17sp_counted_impl_pINS_6threadEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_6threadEED0Ev")]
pub fn stub_0x3886f0() -> ! {
    todo!("0x3886f0 boost::detail::sp_counted_impl_p<boost::thread>::~sp_counted_impl_p()")
}

// 0x3886f4 — __ZN5boost6detail17sp_counted_impl_pINS_6threadEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_6threadEE7disposeEv")]
pub fn stub_0x3886f4() -> ! {
    todo!("0x3886f4 boost::detail::sp_counted_impl_p<boost::thread>::dispose(void)")
}

// 0x388798 — __ZN5boost6detail17sp_counted_impl_pINS_6threadEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_6threadEE11get_deleterERKSt9type_info")]
pub fn stub_0x388798() -> ! {
    todo!("0x388798 boost::detail::sp_counted_impl_p<boost::thread>::get_deleter(std::type_info const&)")
}

// 0x38879c — __ZN5boost6detail17sp_counted_impl_pINS_6threadEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_6threadEE19get_untyped_deleterEv")]
pub fn stub_0x38879c() -> ! {
    todo!("0x38879c boost::detail::sp_counted_impl_p<boost::thread>::get_untyped_deleter(void)")
}

// 0x3887a0 — __ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSJ_NS_6detail13thread_move_tISJ_EEEE5valueEPNS0_5dummyEE4typeE
#[doc(alias = "__ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSJ_NS_6detail13thread_move_tISJ_EEEE5valueEPNS0_5dummyEE4typeE")]
pub fn stub_0x3887a0() -> ! {
    todo!("0x3887a0 __ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSJ_NS_6detail13thread_move_tISJ_EEEE5valueEPNS0_5dummyEE4typeE")
}

// 0x388934 — __ZN5boost6detail13heap_new_implINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEERSJ_EEPT_T0_
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>&>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>&)")]
#[doc(alias = "__ZN5boost6detail13heap_new_implINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEERSJ_EEPT_T0_")]
pub fn stub_0x388934() -> ! {
    todo!("0x388934 boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>&>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>&)")
}

// 0x388ab8 — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEC2ESI_
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>)")]
#[doc(alias = "__ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEC2ESI_")]
pub fn stub_0x388ab8() -> ! {
    todo!("0x388ab8 boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>)")
}

// 0x388bec — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEED1Ev
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::~thread_data()")]
#[doc(alias = "__ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEED1Ev")]
pub fn stub_0x388bec() -> ! {
    todo!("0x388bec boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::~thread_data()")
}

// 0x388cec — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEED0Ev
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::~thread_data()")]
#[doc(alias = "__ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEED0Ev")]
pub fn stub_0x388cec() -> ! {
    todo!("0x388cec boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::~thread_data()")
}

// 0x388dfc — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEE3runEv
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::run(void)")]
#[doc(alias = "__ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEE3runEv")]
pub fn stub_0x388dfc() -> ! {
    todo!("0x388dfc boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::run(void)")
}

// 0x388e18 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEclIPFvS7_SA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::operator()<void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>) &,boost::_bi::list0 &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEclIPFvS7_SA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x388e18() -> ! {
    todo!("0x388e18 void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::operator()<void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>) &,boost::_bi::list0 &,int)")
}

// 0x388f28 — __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEC2INS1_11thread_dataINS_3_bi6bind_tIvPFvNS0_IN3RBX14BaseThreadPool8PoolDataEEENS0_INS8_5mutexEEEENS6_5list2INS6_5valueISB_EENSH_ISD_EEEEEEEEEEPT_
#[doc(alias = "boost::shared_ptr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)")]
#[doc(alias = "__ZN5boost10shared_ptrINS_6detail16thread_data_baseEEC2INS1_11thread_dataINS_3_bi6bind_tIvPFvNS0_IN3RBX14BaseThreadPool8PoolDataEEENS0_INS8_5mutexEEEENS6_5list2INS6_5valueISB_EENSH_ISD_EEEEEEEEEEPT_")]
pub fn stub_0x388f28() -> ! {
    todo!("0x388f28 boost::shared_ptr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)")
}

// 0x389010 — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS8_INS9_5mutexEEEENS6_5list2INS6_5valueISC_EENSI_ISE_EEEEEEEEEEvPKNS8_IT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS8_INS9_5mutexEEEENS6_5list2INS6_5valueISC_EENSI_ISE_EEEEEEEEEEvPKNS8_IT_EEPT0_")]
pub fn stub_0x389010() {
    // IDA 0x389010: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x389134 — __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS6_INS7_5mutexEEEENS4_5list2INS4_5valueISA_EENSG_ISC_EEEEEEEEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS6_INS7_5mutexEEEENS4_5list2INS4_5valueISA_EENSG_ISC_EEEEEEEEEEPT_")]
pub fn stub_0x389134() -> ! {
    todo!("0x389134 boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)")
}

// 0x38922c — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEED1Ev")]
pub fn stub_0x38922c() -> ! {
    todo!("0x38922c boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::~sp_counted_impl_p()")
}

// 0x389230 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEED0Ev")]
pub fn stub_0x389230() -> ! {
    todo!("0x389230 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::~sp_counted_impl_p()")
}

// 0x389234 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE7disposeEv")]
pub fn stub_0x389234() -> ! {
    todo!("0x389234 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::dispose(void)")
}

// 0x389244 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE11get_deleterERKSt9type_info")]
pub fn stub_0x389244() -> ! {
    todo!("0x389244 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::get_deleter(std::type_info const&)")
}

// 0x389248 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE19get_untyped_deleterEv")]
pub fn stub_0x389248() -> ! {
    todo!("0x389248 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::get_untyped_deleter(void)")
}

// 0x38924c — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_")]
pub fn stub_0x38924c() -> ! {
    todo!("0x38924c boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>)")
}

// 0x389364 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_")]
pub fn stub_0x389364() -> ! {
    todo!("0x389364 boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>)")
}

// 0x389480 — __ZN5boost10shared_ptrIN3RBX5mutexEEC2IS2_EEPT_
#[doc(alias = "boost::shared_ptr<RBX::mutex>::shared_ptr<RBX::mutex>(RBX::mutex *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5mutexEEC2IS2_EEPT_")]
pub fn stub_0x389480() -> ! {
    todo!("0x389480 boost::shared_ptr<RBX::mutex>::shared_ptr<RBX::mutex>(RBX::mutex *)")
}

// 0x389554 — __ZN5boost6detail12shared_countC2IN3RBX5mutexEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::mutex>(RBX::mutex *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX5mutexEEEPT_")]
pub fn stub_0x389554() -> ! {
    todo!("0x389554 boost::detail::shared_count::shared_count<RBX::mutex>(RBX::mutex *)")
}

// 0x389660 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEED1Ev")]
pub fn stub_0x389660() -> ! {
    todo!("0x389660 boost::detail::sp_counted_impl_p<RBX::mutex>::~sp_counted_impl_p()")
}

// 0x389664 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEED0Ev")]
pub fn stub_0x389664() -> ! {
    todo!("0x389664 boost::detail::sp_counted_impl_p<RBX::mutex>::~sp_counted_impl_p()")
}

// 0x389668 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE7disposeEv")]
pub fn stub_0x389668() -> ! {
    todo!("0x389668 boost::detail::sp_counted_impl_p<RBX::mutex>::dispose(void)")
}

// 0x38970c — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE11get_deleterERKSt9type_info")]
pub fn stub_0x38970c() -> ! {
    todo!("0x38970c boost::detail::sp_counted_impl_p<RBX::mutex>::get_deleter(std::type_info const&)")
}

// 0x389710 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE19get_untyped_deleterEv")]
pub fn stub_0x389710() -> ! {
    todo!("0x389710 boost::detail::sp_counted_impl_p<RBX::mutex>::get_untyped_deleter(void)")
}

// 0x389714 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE15_M_erase_at_endEPS4_
#[doc(alias = "std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_erase_at_end(boost::shared_ptr<RBX::mutex>*)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE15_M_erase_at_endEPS4_")]
pub fn stub_0x389714() -> ! {
    todo!("0x389714 std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_erase_at_end(boost::shared_ptr<RBX::mutex>*)")
}

// 0x389744 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
#[doc(alias = "std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::mutex>*,std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>>,unsigned long,boost::shared_ptr<RBX::mutex> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_")]
pub fn stub_0x389744() -> ! {
    todo!("0x389744 std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::mutex>*,std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>>,unsigned long,boost::shared_ptr<RBX::mutex> const&)")
}

// 0x389d44 — __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE11_M_allocateEm")]
pub fn stub_0x389d44() -> ! {
    todo!("0x389d44 std::_Vector_base<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_allocate(unsigned long)")
}

// 0x389d5c — __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX5mutexEEEmS4_EvT_T0_RKT1_St12__false_type
#[doc(alias = "void std::__uninitialized_fill_n_aux<boost::shared_ptr<RBX::mutex> *,unsigned long,boost::shared_ptr<RBX::mutex>>(boost::shared_ptr<RBX::mutex> *,unsigned long,boost::shared_ptr<RBX::mutex> const&,std::__false_type)")]
#[doc(alias = "__ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX5mutexEEEmS4_EvT_T0_RKT1_St12__false_type")]
pub fn stub_0x389d5c() -> ! {
    todo!("0x389d5c void std::__uninitialized_fill_n_aux<boost::shared_ptr<RBX::mutex> *,unsigned long,boost::shared_ptr<RBX::mutex>>(boost::shared_ptr<RBX::mutex> *,unsigned long,boost::shared_ptr<RBX::mutex> const&,std::__false_type)")
}

// 0x389e84 — __ZN5boost10shared_ptrIN3RBX5mutexEEaSERKS3_
#[doc(alias = "boost::shared_ptr<RBX::mutex>::operator=(boost::shared_ptr<RBX::mutex> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5mutexEEaSERKS3_")]
pub fn stub_0x389e84() -> ! {
    todo!("0x389e84 boost::shared_ptr<RBX::mutex>::operator=(boost::shared_ptr<RBX::mutex> const&)")
}

// 0x389ebc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5mutexEEES8_EET0_T_SA_S9_
#[doc(alias = "boost::shared_ptr<RBX::mutex> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *>(boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5mutexEEES8_EET0_T_SA_S9_")]
pub fn stub_0x389ebc() -> ! {
    todo!("0x389ebc boost::shared_ptr<RBX::mutex> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *>(boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *)")
}

// 0x389f0c — __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE15_M_erase_at_endEPS3_
#[doc(alias = "std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_erase_at_end(boost::shared_ptr<boost::thread>*)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE15_M_erase_at_endEPS3_")]
pub fn stub_0x389f0c() -> ! {
    todo!("0x389f0c std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_erase_at_end(boost::shared_ptr<boost::thread>*)")
}

// 0x389f3c — __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
#[doc(alias = "std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread>*,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,unsigned long,boost::shared_ptr<boost::thread> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")]
pub fn stub_0x389f3c() -> ! {
    todo!("0x389f3c std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread>*,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,unsigned long,boost::shared_ptr<boost::thread> const&)")
}

// 0x38a53c — __ZNSt12_Vector_baseIN5boost10shared_ptrINS0_6threadEEESaIS3_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN5boost10shared_ptrINS0_6threadEEESaIS3_EE11_M_allocateEm")]
pub fn stub_0x38a53c() -> ! {
    todo!("0x38a53c std::_Vector_base<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_allocate(unsigned long)")
}

// 0x38a554 — __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrINS0_6threadEEEmS3_EvT_T0_RKT1_St12__false_type
#[doc(alias = "void std::__uninitialized_fill_n_aux<boost::shared_ptr<boost::thread> *,unsigned long,boost::shared_ptr<boost::thread>>(boost::shared_ptr<boost::thread> *,unsigned long,boost::shared_ptr<boost::thread> const&,std::__false_type)")]
#[doc(alias = "__ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrINS0_6threadEEEmS3_EvT_T0_RKT1_St12__false_type")]
pub fn stub_0x38a554() -> ! {
    todo!("0x38a554 void std::__uninitialized_fill_n_aux<boost::shared_ptr<boost::thread> *,unsigned long,boost::shared_ptr<boost::thread>>(boost::shared_ptr<boost::thread> *,unsigned long,boost::shared_ptr<boost::thread> const&,std::__false_type)")
}

// 0x38a67c — __ZN5boost10shared_ptrINS_6threadEEaSERKS2_
#[doc(alias = "boost::shared_ptr<boost::thread>::operator=(boost::shared_ptr<boost::thread> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrINS_6threadEEaSERKS2_")]
pub fn stub_0x38a67c() -> ! {
    todo!("0x38a67c boost::shared_ptr<boost::thread>::operator=(boost::shared_ptr<boost::thread> const&)")
}

// 0x38a6b4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrINS3_6threadEEES7_EET0_T_S9_S8_
#[doc(alias = "boost::shared_ptr<boost::thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *>(boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrINS3_6threadEEES7_EET0_T_S9_S8_")]
pub fn stub_0x38a6b4() -> ! {
    todo!("0x38a6b4 boost::shared_ptr<boost::thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *>(boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *)")
}

// 0x38a704 — __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EED2Ev
#[doc(alias = "std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EED2Ev")]
pub fn stub_0x38a704() -> ! {
    todo!("0x38a704 std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::~vector()")
}

// 0x38a7d0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EED2Ev
#[doc(alias = "std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EED2Ev")]
pub fn stub_0x38a7d0() -> ! {
    todo!("0x38a7d0 std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::~vector()")
}

// 0x38a89c — __ZN5boost10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEC2IS3_EEPT_
#[doc(alias = "boost::shared_ptr<RBX::BaseThreadPool::PoolData>::shared_ptr<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEC2IS3_EEPT_")]
pub fn stub_0x38a89c() -> ! {
    todo!("0x38a89c boost::shared_ptr<RBX::BaseThreadPool::PoolData>::shared_ptr<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")
}

// 0x38a970 — __ZN5boost6detail12shared_countC2IN3RBX14BaseThreadPool8PoolDataEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX14BaseThreadPool8PoolDataEEEPT_")]
pub fn stub_0x38a970() -> ! {
    todo!("0x38a970 boost::detail::shared_count::shared_count<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")
}

// 0x38aa68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEED1Ev")]
pub fn stub_0x38aa68() -> ! {
    todo!("0x38aa68 boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::~sp_counted_impl_p()")
}

// 0x38aa6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEED0Ev")]
pub fn stub_0x38aa6c() -> ! {
    todo!("0x38aa6c boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::~sp_counted_impl_p()")
}

// 0x38aa70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE7disposeEv")]
pub fn stub_0x38aa70() -> ! {
    todo!("0x38aa70 boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::dispose(void)")
}

// 0x38aa80 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE11get_deleterERKSt9type_info")]
pub fn stub_0x38aa80() -> ! {
    todo!("0x38aa80 boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_deleter(std::type_info const&)")
}

// 0x38aa84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE19get_untyped_deleterEv")]
pub fn stub_0x38aa84() -> ! {
    todo!("0x38aa84 boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_untyped_deleter(void)")
}

// 0x38aa88 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE13assign_to_ownERKS5_
#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::mutex>> const&)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE13assign_to_ownERKS5_")]
pub fn stub_0x38aa88() -> ! {
    todo!("0x38aa88 boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::mutex>> const&)")
}

// 0x38aab8 — __ZN3RBX14BaseThreadPool8PoolDataD2Ev
#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
#[doc(alias = "__ZN3RBX14BaseThreadPool8PoolDataD2Ev")]
pub fn stub_0x38aab8() -> ! {
    todo!("0x38aab8 RBX::BaseThreadPool::PoolData::~PoolData()")
}

// 0x38ab90 — __ZN3RBX14BaseThreadPool8PoolDataD1Ev
#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
#[doc(alias = "__ZN3RBX14BaseThreadPool8PoolDataD1Ev")]
pub fn stub_0x38ab90() -> ! {
    todo!("0x38ab90 RBX::BaseThreadPool::PoolData::~PoolData()")
}

// 0x38ab94 — __ZN3RBX14BaseThreadPool8PoolDataD0Ev
#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
#[doc(alias = "__ZN3RBX14BaseThreadPool8PoolDataD0Ev")]
pub fn stub_0x38ab94() -> ! {
    todo!("0x38ab94 RBX::BaseThreadPool::PoolData::~PoolData()")
}

// 0x38ac34 — __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EED2Ev
#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EED2Ev")]
pub fn stub_0x38ac34() -> ! {
    todo!("0x38ac34 std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::~vector()")
}

// 0x38ad00 — __ZN3RBX14BaseThreadPool8PoolDataC2Ev
#[doc(alias = "RBX::BaseThreadPool::PoolData::PoolData(void)")]
#[doc(alias = "__ZN3RBX14BaseThreadPool8PoolDataC2Ev")]
pub fn stub_0x38ad00() -> ! {
    todo!("0x38ad00 RBX::BaseThreadPool::PoolData::PoolData(void)")
}

// 0x38add0 — __ZN3RBX10ThreadPool14ThreadPoolDataD1Ev
#[doc(alias = "RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")]
#[doc(alias = "__ZN3RBX10ThreadPool14ThreadPoolDataD1Ev")]
pub fn stub_0x38add0() -> ! {
    todo!("0x38add0 RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")
}

// 0x38aec4 — __ZN3RBX10ThreadPool14ThreadPoolDataD0Ev
#[doc(alias = "RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")]
#[doc(alias = "__ZN3RBX10ThreadPool14ThreadPoolDataD0Ev")]
pub fn stub_0x38aec4() -> ! {
    todo!("0x38aec4 RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")
}

// 0x38afc8 — __ZN3RBX10ThreadPool14ThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE
#[doc(alias = "RBX::ThreadPool::ThreadPoolData::getNextTask(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> &)")]
#[doc(alias = "__ZN3RBX10ThreadPool14ThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE")]
pub fn stub_0x38afc8() -> ! {
    todo!("0x38afc8 RBX::ThreadPool::ThreadPoolData::getNextTask(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> &)")
}

// 0x38afd4 — __ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE14pop_if_presentERS8_
#[doc(alias = "rbx::safe_queue<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>::pop_if_present(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&)")]
#[doc(alias = "__ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE14pop_if_presentERS8_")]
pub fn stub_0x38afd4() -> ! {
    todo!("0x38afd4 rbx::safe_queue<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>::pop_if_present(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&)")
}

// 0x38b0b4 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9pop_frontEv
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::pop_front(void)")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9pop_frontEv")]
pub fn stub_0x38b0b4() -> ! {
    todo!("0x38b0b4 std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::pop_front(void)")
}

// 0x38b0ec — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::~deque()")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev")]
pub fn stub_0x38b0ec() -> ! {
    todo!("0x38b0ec std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::~deque()")
}

// 0x38b1d4 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
#[doc(alias = "std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev")]
pub fn stub_0x38b1d4() -> ! {
    todo!("0x38b1d4 std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::~_Deque_base()")
}

// 0x38b200 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>*>,std::_Deque_iterator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>*>)")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_")]
pub fn stub_0x38b200() -> ! {
    todo!("0x38b200 std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>*>,std::_Deque_iterator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>*>)")
}

// 0x38b338 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_initialize_mapEm
#[doc(alias = "std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_initialize_mapEm")]
pub fn stub_0x38b338() -> ! {
    todo!("0x38b338 std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_initialize_map(unsigned long)")
}

// 0x38b490 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_create_nodesEPPS7_SB_
#[doc(alias = "std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_create_nodes(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>**,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>**)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_create_nodesEPPS7_SB_")]
pub fn stub_0x38b490() -> ! {
    todo!("0x38b490 std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_create_nodes(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>**,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>**)")
}
