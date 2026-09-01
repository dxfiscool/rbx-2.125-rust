//! network generated_net_23 — auto-generated, do not edit manually
//! Filter: RakNet|Network|Replicator -> 5109 total, 0 remaining (complete) — global gap filler EA-sorted asc
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +150 stubs | range 0x246e98..0x250fc8 | 25299->25449 network distinct (rbx_core::SharedPtr not boost) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


// 0x246e98 — __ZN3RBX13TaskScheduler11scheduleJobERNS0_3JobE
// type: int __fastcall(RBX::TaskScheduler *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::TaskScheduler::scheduleJob(RBX::TaskScheduler::Job &)")]
pub fn stub_246e98() -> ! {
    todo!("0x246e98 __ZN3RBX13TaskScheduler11scheduleJobERNS0_3JobE")
}

// 0x246f90 — __ZN3RBX13TaskScheduler3addEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::add(boost::shared_ptr<RBX::TaskScheduler::Job>)")]
pub fn stub_246f90() -> ! {
    todo!("0x246f90 __ZN3RBX13TaskScheduler3addEN5boost10shared_ptrINS0_3JobEEE")
}

// 0x24710c — __ZN3RBX13TaskScheduler20incrementThreadCountEv
// type: int __fastcall(int32_t *this, volatile int *)
#[doc(alias = "RBX::TaskScheduler::incrementThreadCount(void)")]
pub fn stub_24710c() -> ! {
    todo!("0x24710c __ZN3RBX13TaskScheduler20incrementThreadCountEv")
}

// 0x24711c — __ZN3RBX13TaskScheduler20decrementThreadCountEv
// type: int __fastcall(int32_t *this, volatile int *)
#[doc(alias = "RBX::TaskScheduler::decrementThreadCount(void)")]
pub fn stub_24711c() -> ! {
    todo!("0x24711c __ZN3RBX13TaskScheduler20decrementThreadCountEv")
}

// 0x247130 — __ZNK3RBX13TaskScheduler20getShortestSleepTimeEv
// type: int __fastcall(RBX::TaskScheduler *this, int)
#[doc(alias = "RBX::TaskScheduler::getShortestSleepTime(void)const")]
pub fn stub_247130() -> ! {
    todo!("0x247130 __ZNK3RBX13TaskScheduler20getShortestSleepTimeEv")
}

// 0x247154 — __ZN3RBX13TaskScheduler16wakeSleepingJobsEv
// type: int __fastcall(RBX::TaskScheduler *this)
#[doc(alias = "RBX::TaskScheduler::wakeSleepingJobs(void)")]
pub fn stub_247154() -> ! {
    todo!("0x247154 __ZN3RBX13TaskScheduler16wakeSleepingJobsEv")
}

// 0x247220 — __ZN3RBX13TaskScheduler12findJobToRunEN5boost10shared_ptrINS0_6ThreadEEE
// type: void __fastcall(RBX::TaskScheduler::Job **, int, int *, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::findJobToRun(boost::shared_ptr<RBX::TaskScheduler::Thread>)")]
pub fn stub_247220() -> ! {
    todo!("0x247220 __ZN3RBX13TaskScheduler12findJobToRunEN5boost10shared_ptrINS0_6ThreadEEE")
}

// 0x247bd8 — __ZN3rbx25thread_specific_referenceIN3RBX13TaskScheduler3JobEED1Ev
// type: 
#[doc(alias = "rbx::thread_specific_reference<RBX::TaskScheduler::Job>::~thread_specific_reference()")]
pub fn stub_247bd8() -> ! {
    todo!("0x247bd8 __ZN3rbx25thread_specific_referenceIN3RBX13TaskScheduler3JobEED1Ev")
}

// 0x247be8 — __ZNK3RBX13TaskScheduler3Job12getDebugNameEv
// type: void __fastcall(RBX::TaskScheduler::Job *this, int)
#[doc(alias = "RBX::TaskScheduler::Job::getDebugName(void)const")]
pub fn stub_247be8() -> ! {
    todo!("0x247be8 __ZNK3RBX13TaskScheduler3Job12getDebugNameEv")
}

// 0x247db0 — __ZN3RBX14RunningAverageIidE6sampleEi
// type: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::RunningAverage<int,double>::sample(int)")]
pub fn stub_247db0() -> ! {
    todo!("0x247db0 __ZN3RBX14RunningAverageIidE6sampleEi")
}

// 0x247e74 — __ZN3RBX16ExclusiveArbiter11arbiterNameEv
// type: int __fastcall(RBX::ExclusiveArbiter *this)
#[doc(alias = "RBX::ExclusiveArbiter::arbiterName(void)")]
pub fn stub_247e74() -> ! {
    todo!("0x247e74 __ZN3RBX16ExclusiveArbiter11arbiterNameEv")
}

// 0x247e90 — __ZN3RBX16ExclusiveArbiter11isThrottledEv
// type: int __fastcall(RBX::ExclusiveArbiter *this)
#[doc(alias = "RBX::ExclusiveArbiter::isThrottled(void)")]
pub fn stub_247e90() -> ! {
    todo!("0x247e90 __ZN3RBX16ExclusiveArbiter11isThrottledEv")
}

// 0x247e94 — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(boost::weak_ptr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_247e94() -> ! {
    todo!("0x247e94 __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE")
}

// 0x247fac — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>)")]
pub fn stub_247fac() -> ! {
    todo!("0x247fac __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_")
}

// 0x248020 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<boost::shared_ptr<RBX::TaskScheduler::Job>> *)")]
pub fn stub_248020() -> ! {
    todo!("0x248020 __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

// 0x248050 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, _DWORD *, unsigned int M_parent, int)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_insert_unique(boost::shared_ptr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_248050() -> ! {
    todo!("0x248050 __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_")
}

// 0x248104 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_create_node(boost::shared_ptr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_248104() -> ! {
    todo!("0x248104 __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_")
}

// 0x248224 — __ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CEvent>(RBX::CEvent *)")]
pub fn stub_248224() -> ! {
    todo!("0x248224 __ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_")
}

// 0x24831c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()")]
pub fn stub_24831c() -> ! {
    todo!("0x24831c __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED1Ev")
}

// 0x248320 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()")]
pub fn stub_248320() -> ! {
    todo!("0x248320 __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED0Ev")
}

// 0x24832c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::dispose(void)")]
pub fn stub_24832c() -> ! {
    todo!("0x24832c __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE7disposeEv")
}

// 0x24834c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::get_deleter(std::type_info const&)")]
pub fn stub_24834c() -> ! {
    todo!("0x24834c __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE11get_deleterERKSt9type_info")
}

// 0x248350 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::get_untyped_deleter(void)")]
pub fn stub_248350() -> ! {
    todo!("0x248350 __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE19get_untyped_deleterEv")
}

// 0x248358 — __ZN5boost6detail11thread_dataINS_9function0IvEEED1Ev
// type: int __fastcall(boost::detail::thread_data_base *)
#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::~thread_data()")]
pub fn stub_248358() -> ! {
    todo!("0x248358 __ZN5boost6detail11thread_dataINS_9function0IvEEED1Ev")
}

// 0x248448 — __ZN5boost18condition_variableC2Ev
// type: boost::condition_variable *__fastcall(boost::condition_variable *this)
#[doc(alias = "boost::condition_variable::condition_variable(void)")]
pub fn stub_248448() -> ! {
    todo!("0x248448 __ZN5boost18condition_variableC2Ev")
}

// 0x248620 — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_9function0IvEEEEEEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const")]
pub fn stub_248620() -> ! {
    todo!("0x248620 __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_9function0IvEEEEEEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0x248778 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::get_untyped_deleter(void)")]
pub fn stub_248778() -> ! {
    todo!("0x248778 __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE19get_untyped_deleterEv")
}

// 0x24877c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_24877c() -> ! {
    todo!("0x24877c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")
}

// 0x2487dc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_2487dc() -> ! {
    todo!("0x2487dc __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE")
}

// 0x2487f8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&)")]
pub fn stub_2487f8() -> ! {
    todo!("0x2487f8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_")
}

// 0x248938 — __ZN5boost9function0IvE5dummy7nonnullEv
// type: void()
#[doc(alias = "boost::function0<void>::dummy::nonnull(void)")]
pub fn stub_248938() -> ! {
    todo!("0x248938 __ZN5boost9function0IvE5dummy7nonnullEv")
}

// 0x248940 — __ZN3RBX5mutexC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *this)
#[doc(alias = "RBX::mutex::mutex(void)")]
pub fn stub_248940() -> ! {
    todo!("0x248940 __ZN3RBX5mutexC2Ev")
}

// 0x248a8c — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev
// type: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::~thread_specific_ptr()")]
pub fn stub_248a8c() -> ! {
    todo!("0x248a8c __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev")
}

// 0x248b80 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD1Ev
// type: void()
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()")]
pub fn stub_248b80() -> ! {
    todo!("0x248b80 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD1Ev")
}

// 0x248b84 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()")]
pub fn stub_248b84() -> ! {
    todo!("0x248b84 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD0Ev")
}

// 0x248b90 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataclEPv
// type: void __fastcall(int, void *)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::operator()(void *)")]
pub fn stub_248b90() -> ! {
    todo!("0x248b90 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataclEPv")
}

// 0x248ba0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()")]
pub fn stub_248ba0() -> ! {
    todo!("0x248ba0 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED1Ev")
}

// 0x248ba4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()")]
pub fn stub_248ba4() -> ! {
    todo!("0x248ba4 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED0Ev")
}

// 0x248bb0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::dispose(void)")]
pub fn stub_248bb0() -> ! {
    todo!("0x248bb0 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE7disposeEv")
}

// 0x248bc4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_deleter(std::type_info const&)")]
pub fn stub_248bc4() -> ! {
    todo!("0x248bc4 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE11get_deleterERKSt9type_info")
}

// 0x248bdc — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_untyped_deleter(void)")]
pub fn stub_248bdc() -> ! {
    todo!("0x248bdc __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE19get_untyped_deleterEv")
}

// 0x248be0 — __GLOBAL__I_a_50
// type: void __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__GLOBAL__I_a_50")]
pub fn stub_248be0() -> ! {
    todo!("0x248be0 __GLOBAL__I_a_50")
}

// 0x248e38 — __ZNK3RBX13TaskScheduler3Job16averageDutyCycleEv
// type: __int64 __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::averageDutyCycle(void)const")]
pub fn stub_248e38() -> ! {
    todo!("0x248e38 __ZNK3RBX13TaskScheduler3Job16averageDutyCycleEv")
}

// 0x248eb0 — __ZNK3RBX13TaskScheduler3Job21averageStepsPerSecondEv
// type: __int64 __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::averageStepsPerSecond(void)const")]
pub fn stub_248eb0() -> ! {
    todo!("0x248eb0 __ZNK3RBX13TaskScheduler3Job21averageStepsPerSecondEv")
}

// 0x248f10 — __ZNK3RBX13TaskScheduler3Job15averageStepTimeEv
// type: __int64 __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::averageStepTime(void)const")]
pub fn stub_248f10() -> ! {
    todo!("0x248f10 __ZNK3RBX13TaskScheduler3Job15averageStepTimeEv")
}

// 0x248f20 — __ZNK3RBX13TaskScheduler3Job12averageErrorEv
// type: __int64 __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::averageError(void)const")]
pub fn stub_248f20() -> ! {
    todo!("0x248f20 __ZNK3RBX13TaskScheduler3Job12averageErrorEv")
}

// 0x248f2c — __ZN3RBX13TaskScheduler3Job17removeCoordinatorEN5boost10shared_ptrINS_5Tasks11CoordinatorEEE
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "RBX::TaskScheduler::Job::removeCoordinator(boost::shared_ptr<RBX::Tasks::Coordinator>)")]
pub fn stub_248f2c() -> ! {
    todo!("0x248f2c __ZN3RBX13TaskScheduler3Job17removeCoordinatorEN5boost10shared_ptrINS_5Tasks11CoordinatorEEE")
}

// 0x2490ac — __ZN3RBX13TaskScheduler3Job14addCoordinatorEN5boost10shared_ptrINS_5Tasks11CoordinatorEEE
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "RBX::TaskScheduler::Job::addCoordinator(boost::shared_ptr<RBX::Tasks::Coordinator>)")]
pub fn stub_2490ac() -> ! {
    todo!("0x2490ac __ZN3RBX13TaskScheduler3Job14addCoordinatorEN5boost10shared_ptrINS_5Tasks11CoordinatorEEE")
}

// 0x249184 — __ZN3RBX13TaskScheduler3Job10isDisabledEv
// type: bool __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::isDisabled(void)")]
pub fn stub_249184() -> ! {
    todo!("0x249184 __ZN3RBX13TaskScheduler3Job10isDisabledEv")
}

// 0x249270 — __ZN3RBX13TaskScheduler3JobC2EPKcN5boost10shared_ptrINS0_7ArbiterEEENS_4Time8IntervalE
// type: char *__fastcall(int, volatile int *, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, char, int, pthread_mutex_t *, int, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::Job::Job(char const*,boost::shared_ptr<RBX::TaskScheduler::Arbiter>,RBX::Time::Interval)")]
pub fn stub_249270() -> ! {
    todo!("0x249270 __ZN3RBX13TaskScheduler3JobC2EPKcN5boost10shared_ptrINS0_7ArbiterEEENS_4Time8IntervalE")
}

// 0x249920 — __ZN3RBX13TaskScheduler3JobD0Ev
// type: void __fastcall(RBX::TaskScheduler::Job *__hidden this)
#[doc(alias = "RBX::TaskScheduler::Job::~Job()")]
pub fn stub_249920() -> ! {
    todo!("0x249920 __ZN3RBX13TaskScheduler3JobD0Ev")
}

// 0x2499c0 — __ZN3RBX13TaskScheduler3JobD1Ev
// type: void __fastcall(RBX::TaskScheduler::Job *__hidden this)
#[doc(alias = "RBX::TaskScheduler::Job::~Job()")]
pub fn stub_2499c0() -> ! {
    todo!("0x2499c0 __ZN3RBX13TaskScheduler3JobD1Ev")
}

// 0x2499cc — __ZN3RBX13TaskScheduler3JobD2Ev
// type: void __fastcall(RBX::TaskScheduler::Job *this, int, int)
#[doc(alias = "RBX::TaskScheduler::Job::~Job()")]
pub fn stub_2499cc() -> ! {
    todo!("0x2499cc __ZN3RBX13TaskScheduler3JobD2Ev")
}

// 0x24a1f8 — __ZN3RBX13TaskScheduler3Job20computeStandardErrorERKNS1_5StatsEd
// type: int __fastcall(int result, int, double *, unsigned int, unsigned int)
#[doc(alias = "RBX::TaskScheduler::Job::computeStandardError(RBX::TaskScheduler::Job::Stats const&,double)")]
pub fn stub_24a1f8() -> ! {
    todo!("0x24a1f8 __ZN3RBX13TaskScheduler3Job20computeStandardErrorERKNS1_5StatsEd")
}

// 0x24a210 — __ZN3RBX13TaskScheduler3Job24computeStandardSleepTimeERKNS1_5StatsEd
// type: void __fastcall(RBX::TaskScheduler::Job *this, const RBX::TaskScheduler::Job::Stats *, double, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::TaskScheduler::Job::computeStandardSleepTime(RBX::TaskScheduler::Job::Stats const&,double)")]
pub fn stub_24a210() -> ! {
    todo!("0x24a210 __ZN3RBX13TaskScheduler3Job24computeStandardSleepTimeERKNS1_5StatsEd")
}

// 0x24a408 — __ZN3RBX13TaskScheduler3Job5StatsC1ERS1_NS_4TimeE
// type: RBX::TaskScheduler::Job::Stats *__fastcall(RBX::TaskScheduler::Job::Stats *this, RBX::TaskScheduler::Job *, Time)
#[doc(alias = "RBX::TaskScheduler::Job::Stats::Stats(RBX::TaskScheduler::Job&,RBX::Time)")]
pub fn stub_24a408() -> ! {
    todo!("0x24a408 __ZN3RBX13TaskScheduler3Job5StatsC1ERS1_NS_4TimeE")
}

// 0x24a440 — __ZN3RBX13TaskScheduler3Job12startWaitingEv
// type: int __fastcall(int this)
#[doc(alias = "RBX::TaskScheduler::Job::startWaiting(void)")]
pub fn stub_24a440() -> ! {
    todo!("0x24a440 __ZN3RBX13TaskScheduler3Job12startWaitingEv")
}

// 0x24a448 — __ZN3RBX13TaskScheduler3Job13startSleepingEv
// type: int __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::startSleeping(void)")]
pub fn stub_24a448() -> ! {
    todo!("0x24a448 __ZN3RBX13TaskScheduler3Job13startSleepingEv")
}

// 0x24a468 — __ZN3RBX13TaskScheduler3Job14updateWakeTimeEv
// type: int __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::updateWakeTime(void)")]
pub fn stub_24a468() -> ! {
    todo!("0x24a468 __ZN3RBX13TaskScheduler3Job14updateWakeTimeEv")
}

// 0x24a4c0 — __ZN3RBX13TaskScheduler3Job11updateErrorERKNS_4TimeE
// type: int __fastcall(RBX::TaskScheduler::Job *this, const RBX::Time *)
#[doc(alias = "RBX::TaskScheduler::Job::updateError(RBX::Time const&)")]
pub fn stub_24a4c0() -> ! {
    todo!("0x24a4c0 __ZN3RBX13TaskScheduler3Job11updateErrorERKNS_4TimeE")
}

// 0x24a598 — __ZN3RBX13TaskScheduler3Job25notifyCoordinatorsPreStepEv
// type: void __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::notifyCoordinatorsPreStep(void)")]
pub fn stub_24a598() -> ! {
    todo!("0x24a598 __ZN3RBX13TaskScheduler3Job25notifyCoordinatorsPreStepEv")
}

// 0x24a684 — __ZN3RBX13TaskScheduler3Job7preStepEv
// type: void __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::preStep(void)")]
pub fn stub_24a684() -> ! {
    todo!("0x24a684 __ZN3RBX13TaskScheduler3Job7preStepEv")
}

// 0x24a8b8 — __ZN3RBX13TaskScheduler3Job8postStepENS0_10StepResultE
// type: void __fastcall(int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::Job::postStep(RBX::TaskScheduler::StepResult)")]
pub fn stub_24a8b8() -> ! {
    todo!("0x24a8b8 __ZN3RBX13TaskScheduler3Job8postStepENS0_10StepResultE")
}

// 0x24ab18 — __ZN3RBX13TaskScheduler3Job26notifyCoordinatorsPostStepEv
// type: void __fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::notifyCoordinatorsPostStep(void)")]
pub fn stub_24ab18() -> ! {
    todo!("0x24ab18 __ZN3RBX13TaskScheduler3Job26notifyCoordinatorsPostStepEv")
}

// 0x24ac08 — __ZN3RBX13TaskScheduler3Job14updatePriorityEv
// type: double *__fastcall(RBX::TaskScheduler::Job *this)
#[doc(alias = "RBX::TaskScheduler::Job::updatePriority(void)")]
pub fn stub_24ac08() -> ! {
    todo!("0x24ac08 __ZN3RBX13TaskScheduler3Job14updatePriorityEv")
}

// 0x24ad00 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>::push_back(boost::shared_ptr<RBX::Tasks::Coordinator> const&)")]
pub fn stub_24ad00() -> ! {
    todo!("0x24ad00 __ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE9push_backERKS5_")
}

// 0x24ad90 — __ZN3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE6sampleENS1_8IntervalE
// type: int __fastcall(__int64 *, unsigned int, unsigned int)
#[doc(alias = "RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::sample(RBX::Time::Interval)")]
pub fn stub_24ad90() -> ! {
    todo!("0x24ad90 __ZN3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE6sampleENS1_8IntervalE")
}

// 0x24ae08 — __ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv
// type: int __fastcall(int)
#[doc(alias = "RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample(void)")]
pub fn stub_24ae08() -> ! {
    todo!("0x24ae08 __ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv")
}

// 0x24ae88 — __ZN5boost15circular_bufferIdSaIdEE8allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::allocate(unsigned long)")]
pub fn stub_24ae88() -> ! {
    todo!("0x24ae88 __ZN5boost15circular_bufferIdSaIdEE8allocateEm")
}

// 0x24afb0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev
// type: void __fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
pub fn stub_24afb0() -> ! {
    todo!("0x24afb0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev")
}

// 0x24b070 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv
// type: void __fastcall __noreturn(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const")]
pub fn stub_24b070() -> ! {
    todo!("0x24b070 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv")
}

// 0x24b120 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv
// type: void __fastcall __noreturn(_DWORD *)
#[doc(alias = "__ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv")]
pub fn stub_24b120() -> ! {
    todo!("0x24b120 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv")
}

// 0x24b130 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev")]
pub fn stub_24b130() -> ! {
    todo!("0x24b130 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev")
}

// 0x24b208 — __ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "__ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED0Ev")]
pub fn stub_24b208() -> ! {
    todo!("0x24b208 __ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED0Ev")
}

// 0x24b2c8 — __ZN3RBX13WindowAverageIddE6sampleINS_13FOnBeforeDropEEEvdRT_
// type: int __fastcall(__int64 *, unsigned int, unsigned int, int)
#[doc(alias = "void RBX::WindowAverage<double,double>::sample<RBX::FOnBeforeDrop>(double,RBX::FOnBeforeDrop &)")]
pub fn stub_24b2c8() -> ! {
    todo!("0x24b2c8 __ZN3RBX13WindowAverageIddE6sampleINS_13FOnBeforeDropEEEvdRT_")
}

// 0x24b364 — __ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EEC2ENS1_8IntervalE
// type: int __fastcall(int, unsigned int, unsigned int)
#[doc(alias = "RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::WindowAverageTimeInterval(RBX::Time::Interval)")]
pub fn stub_24b364() -> ! {
    todo!("0x24b364 __ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EEC2ENS1_8IntervalE")
}

// 0x24b45c — __ZN3RBX14RunningAverageIddEC2Eddj
// type: int *__fastcall(int *, int, int, unsigned int, unsigned int, unsigned int, int, int, int, int)
#[doc(alias = "RBX::RunningAverage<double,double>::RunningAverage(double,double,unsigned int)")]
pub fn stub_24b45c() -> ! {
    todo!("0x24b45c __ZN3RBX14RunningAverageIddEC2Eddj")
}

// 0x24b5a4 — __ZN5boost6detail12shared_countC2INS_15circular_bufferIdSaIdEEEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::circular_buffer<double,std::allocator<double>>>(boost::circular_buffer<double,std::allocator<double>> *)")]
pub fn stub_24b5a4() -> ! {
    todo!("0x24b5a4 __ZN5boost6detail12shared_countC2INS_15circular_bufferIdSaIdEEEEEPT_")
}

// 0x24b6c8 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()")]
pub fn stub_24b6c8() -> ! {
    todo!("0x24b6c8 __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEED1Ev")
}

// 0x24b6cc — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()")]
pub fn stub_24b6cc() -> ! {
    todo!("0x24b6cc __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEED0Ev")
}

// 0x24b6d8 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::dispose(void)")]
pub fn stub_24b6d8() -> ! {
    todo!("0x24b6d8 __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE7disposeEv")
}

// 0x24b714 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_deleter(std::type_info const&)")]
pub fn stub_24b714() -> ! {
    todo!("0x24b714 __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE11get_deleterERKSt9type_info")
}

// 0x24b718 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_untyped_deleter(void)")]
pub fn stub_24b718() -> ! {
    todo!("0x24b718 __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE19get_untyped_deleterEv")
}

// 0x24b71c — __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESt6vectorIS7_SaIS7_EEEENS2_3_bi6bind_tIbNS2_4_mfi3mf1IbS6_PNS4_13TaskScheduler3JobEEENSD_5list2INS2_3argILi1EEENSD_5valueISJ_EEEEEEET_SS_SS_T0_St26random_access_iterator_tag
// type: _DWORD *__fastcall(_DWORD *, int, int (*)(void), int, int)
#[doc(alias = "__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>> std::__find_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>,std::random_access_iterator_tag)")]
pub fn stub_24b71c() -> ! {
    todo!("0x24b71c __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESt6vectorIS7_SaIS7_EEEENS2_3_bi6bind_tIbNS2_4_mfi3mf1IbS6_PNS4_13TaskScheduler3JobEEENSD_5list2INS2_3argILi1EEENSD_5valueISJ_EEEEEEET_SS_SS_T0_St26random_access_iterator_tag")
}

// 0x24b860 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: void __fastcall(int, char *, int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator>*,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::shared_ptr<RBX::Tasks::Coordinator> const&)")]
pub fn stub_24b860() -> ! {
    todo!("0x24b860 __ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")
}

// 0x24bdf8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::Tasks::Coordinator> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *>(boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *)")]
pub fn stub_24bdf8() -> ! {
    todo!("0x24bdf8 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_")
}

// 0x24beb0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_
// type: _DWORD *__fastcall(int, int, _DWORD *)
#[doc(alias = "boost::shared_ptr<RBX::Tasks::Coordinator> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *>(boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *)")]
pub fn stub_24beb0() -> ! {
    todo!("0x24beb0 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_")
}

// 0x24bf64 — __GLOBAL__I_a_51
// type: 
#[doc(alias = "__GLOBAL__I_a_51")]
pub fn stub_24bf64() -> ! {
    todo!("0x24bf64 __GLOBAL__I_a_51")
}

// 0x24c02c — __ZN3RBX13TaskScheduler13endAllThreadsEv
// type: int __fastcall(int this)
#[doc(alias = "RBX::TaskScheduler::endAllThreads(void)")]
pub fn stub_24c02c() -> ! {
    todo!("0x24c02c __ZN3RBX13TaskScheduler13endAllThreadsEv")
}

// 0x24c048 — __ZN3RBX13TaskScheduler14setThreadCountENS0_16ThreadPoolConfigE
// type: void __fastcall(int, int, int, int, boost::detail::sp_counted_base *, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::setThreadCount(RBX::TaskScheduler::ThreadPoolConfig)")]
pub fn stub_24c048() -> ! {
    todo!("0x24c048 __ZN3RBX13TaskScheduler14setThreadCountENS0_16ThreadPoolConfigE")
}

// 0x24c1ec — __ZN3RBX13TaskScheduler6Thread6runJobEv
// type: int __fastcall(RBX::TaskScheduler::Thread *this)
#[doc(alias = "RBX::TaskScheduler::Thread::runJob(void)")]
pub fn stub_24c1ec() -> ! {
    todo!("0x24c1ec __ZN3RBX13TaskScheduler6Thread6runJobEv")
}

// 0x24c43c — __ZNK3RBX13TaskScheduler25conflictsWithScheduledJobEPNS0_3JobE
// type: int __fastcall(RBX::TaskScheduler *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::TaskScheduler::conflictsWithScheduledJob(RBX::TaskScheduler::Job *)const")]
pub fn stub_24c43c() -> ! {
    todo!("0x24c43c __ZNK3RBX13TaskScheduler25conflictsWithScheduledJobEPNS0_3JobE")
}

// 0x24c5bc — __ZN3RBX13TaskScheduler13enableThreadsERSt6vectorIN5boost10shared_ptrINS0_6ThreadEEESaIS5_EE
// type: int __fastcall(int, __int64 *, int)
#[doc(alias = "RBX::TaskScheduler::enableThreads(std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>> &)")]
pub fn stub_24c5bc() -> ! {
    todo!("0x24c5bc __ZN3RBX13TaskScheduler13enableThreadsERSt6vectorIN5boost10shared_ptrINS0_6ThreadEEESaIS5_EE")
}

// 0x24c660 — __ZN3RBX13TaskScheduler6Thread4loopEv
// type: void __fastcall(int32_t **this, volatile int *)
#[doc(alias = "RBX::TaskScheduler::Thread::loop(void)")]
pub fn stub_24c660() -> ! {
    todo!("0x24c660 __ZN3RBX13TaskScheduler6Thread4loopEv")
}

// 0x24cd18 — __ZN3RBX13TaskScheduler11getJobsInfoERSt6vectorIN5boost10shared_ptrIKNS0_3JobEEESaIS6_EE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::TaskScheduler::getJobsInfo(std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>> &)")]
pub fn stub_24cd18() -> ! {
    todo!("0x24cd18 __ZN3RBX13TaskScheduler11getJobsInfoERSt6vectorIN5boost10shared_ptrIKNS0_3JobEEESaIS6_EE")
}

// 0x24ce78 — __ZN3RBX13TaskScheduler26setJobsExtendedStatsWindowEd
// type: void __fastcall(RBX::TaskScheduler *this, double)
#[doc(alias = "RBX::TaskScheduler::setJobsExtendedStatsWindow(double)")]
pub fn stub_24ce78() -> ! {
    todo!("0x24ce78 __ZN3RBX13TaskScheduler26setJobsExtendedStatsWindowEd")
}

// 0x24d05c — __ZL25setJobExtendedStatsWindowN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEEd
// type: int *__fastcall(int *result, unsigned int, unsigned int)
#[doc(alias = "setJobExtendedStatsWindow(boost::shared_ptr<RBX::TaskScheduler::Job>,double)")]
pub fn stub_24d05c() -> ! {
    todo!("0x24d05c __ZL25setJobExtendedStatsWindowN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEEd")
}

// 0x24d0ec — __ZN3RBX13TaskScheduler13getJobsByNameERKSsRSt6vectorIN5boost10shared_ptrIKNS0_3JobEEESaIS8_EE
// type: void __fastcall(int, const void **, int)
#[doc(alias = "RBX::TaskScheduler::getJobsByName(std::string const&,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>> &)")]
pub fn stub_24d0ec() -> ! {
    todo!("0x24d0ec __ZN3RBX13TaskScheduler13getJobsByNameERKSsRSt6vectorIN5boost10shared_ptrIKNS0_3JobEEESaIS8_EE")
}

// 0x24d284 — __ZN3RBX13TaskScheduler6Thread4joinEv
// type: pthread_t __fastcall(RBX::TaskScheduler::Thread *this)
#[doc(alias = "RBX::TaskScheduler::Thread::join(void)")]
pub fn stub_24d284() -> ! {
    todo!("0x24d284 __ZN3RBX13TaskScheduler6Thread4joinEv")
}

// 0x24d2dc — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Thread> const&)")]
pub fn stub_24d2dc() -> ! {
    todo!("0x24d2dc __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE9push_backERKS5_")
}

// 0x24d368 — __ZN3RBX13TaskScheduler6Thread6createEPS0_
// type: void __fastcall(RBX::TaskScheduler::Thread *this, RBX::TaskScheduler *)
#[doc(alias = "RBX::TaskScheduler::Thread::create(RBX::TaskScheduler*)")]
pub fn stub_24d368() -> ! {
    todo!("0x24d368 __ZN3RBX13TaskScheduler6Thread6createEPS0_")
}

// 0x24d770 — __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE9push_backERKS6_
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Job const> const&)")]
pub fn stub_24d770() -> ! {
    todo!("0x24d770 __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE9push_backERKS6_")
}

// 0x24d7fc — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_24d7fc() -> ! {
    todo!("0x24d7fc __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE9push_backERKS5_")
}

// 0x24d88c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIdEEEclIPFvNS_10shared_ptrIN3RBX13TaskScheduler3JobEEEdENS0_5list1IRSC_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(_DWORD *, void (__fastcall **)(int *, _DWORD, _DWORD), int **)
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<double>>::operator()<void (*)(boost::shared_ptr<RBX::TaskScheduler::Job>,double),boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::TaskScheduler::Job>,double) &,boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job>&> &,int)")]
pub fn stub_24d88c() -> ! {
    todo!("0x24d88c __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIdEEEclIPFvNS_10shared_ptrIN3RBX13TaskScheduler3JobEEEdENS0_5list1IRSC_EEEEvNS0_4typeIvEERT_RT0_i")
}

// 0x24d9a4 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: void __fastcall(int, char *, int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>>,boost::shared_ptr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_24d9a4() -> ! {
    todo!("0x24d9a4 __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")
}

// 0x24df3c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES9_EET0_T_SB_SA_
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *>(boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *)")]
pub fn stub_24df3c() -> ! {
    todo!("0x24df3c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES9_EET0_T_SB_SA_")
}

// 0x24dff8 — __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_
// type: void __fastcall(int, char *, int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::shared_ptr<RBX::TaskScheduler::Job const> const&)")]
pub fn stub_24dff8() -> ! {
    todo!("0x24dff8 __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_")
}

// 0x24e590 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESA_EET0_T_SC_SB_
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Job const> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *>(boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *)")]
pub fn stub_24e590() -> ! {
    todo!("0x24e590 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESA_EET0_T_SC_SB_")
}

// 0x24e648 — __ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE6sampleEv
// type: int __fastcall(int, int)
#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::sample(void)")]
pub fn stub_24e648() -> ! {
    todo!("0x24e648 __ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE6sampleEv")
}

// 0x24e6a8 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_
// type: _DWORD *__fastcall(int, int, _DWORD *)
#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Thread> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *>(boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *)")]
pub fn stub_24e6a8() -> ! {
    todo!("0x24e6a8 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_")
}

// 0x24e75c — __ZN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEC2IS3_EERKNS_8weak_ptrIT_EE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Thread>::shared_ptr<RBX::TaskScheduler::Thread>(boost::weak_ptr<RBX::TaskScheduler::Thread> const&)")]
pub fn stub_24e75c() -> ! {
    todo!("0x24e75c __ZN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEC2IS3_EERKNS_8weak_ptrIT_EE")
}

// 0x24e870 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE5resetEPS4_
// type: void __fastcall(int *, const void *)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::reset(RBX::TaskScheduler::Job **)")]
pub fn stub_24e870() -> ! {
    todo!("0x24e870 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE5resetEPS4_")
}

// 0x24e98c — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: void __fastcall(int, char *, int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Thread>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::shared_ptr<RBX::TaskScheduler::Thread> const&)")]
pub fn stub_24e98c() -> ! {
    todo!("0x24e98c __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")
}

// 0x24ef24 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *>(boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *)")]
pub fn stub_24ef24() -> ! {
    todo!("0x24ef24 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_")
}

// 0x24efe0 — __ZN5boost15circular_bufferIdSaIdEE12set_capacityEm
// type: int __fastcall(char **, unsigned int)
#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::set_capacity(unsigned long)")]
pub fn stub_24efe0() -> ! {
    todo!("0x24efe0 __ZN5boost15circular_bufferIdSaIdEE12set_capacityEm")
}

// 0x24f0d8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
// type: std::logic_error *__fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
pub fn stub_24f0d8() -> ! {
    todo!("0x24f0d8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev")
}

// 0x24f190 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorED1Ev
// type: int __fastcall(std::logic_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
pub fn stub_24f190() -> ! {
    todo!("0x24f190 __ZN5boost16exception_detail19error_info_injectorISt12length_errorED1Ev")
}

// 0x24f248 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&)")]
pub fn stub_24f248() -> ! {
    todo!("0x24f248 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_")
}

// 0x24f388 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_NS5_9clone_tagE
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_tag)")]
pub fn stub_24f388() -> ! {
    todo!("0x24f388 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_NS5_9clone_tagE")
}

// 0x24f520 — __ZN5boost4bindIvN3RBX13TaskScheduler6ThreadENS_10shared_ptrIS3_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf0IS8_T0_EENS6_9list_av_1IT1_E4typeEEEMSB_FS8_vESE_
// type: void __fastcall(int, struct _Unwind_Exception *, boost::detail::sp_counted_base *, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list_av_1<boost::shared_ptr<RBX::TaskScheduler::Thread>>::type> boost::bind<void,RBX::TaskScheduler::Thread,boost::shared_ptr<RBX::TaskScheduler::Thread>>(void (RBX::TaskScheduler::Thread::*)(void),boost::shared_ptr<RBX::TaskScheduler::Thread>)")]
pub fn stub_24f520() -> ! {
    todo!("0x24f520 __ZN5boost4bindIvN3RBX13TaskScheduler6ThreadENS_10shared_ptrIS3_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf0IS8_T0_EENS6_9list_av_1IT1_E4typeEEEMSB_FS8_vESE_")
}

// 0x24f6c0 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13TaskScheduler6ThreadEEEEEEC2ES8_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>)")]
pub fn stub_24f6c0() -> ! {
    todo!("0x24f6c0 __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13TaskScheduler6ThreadEEEEEEC2ES8_")
}

// 0x24f808 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub fn stub_24f808() -> ! {
    todo!("0x24f808 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")
}

// 0x24f93c — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>)")]
pub fn stub_24f93c() -> ! {
    todo!("0x24f93c __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEEvT_")
}

// 0x24fa7c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// type: 
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_24fa7c() -> ! {
    todo!("0x24fa7c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE")
}

// 0x24faa0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int *)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_24faa0() -> ! {
    todo!("0x24faa0 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEvE6invokeERNS1_15function_bufferE")
}

// 0x24fac0 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_24fac0() -> ! {
    todo!("0x24fac0 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferE")
}

// 0x24fbf4 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_24fbf4() -> ! {
    todo!("0x24fbf4 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

// 0x24fdac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_24fdac() -> ! {
    todo!("0x24fdac __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

// 0x24ff48 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::~sp_counted_impl_p()")]
pub fn stub_24ff48() -> ! {
    todo!("0x24ff48 __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEED0Ev")
}

// 0x24ff58 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler6ThreadEE22_internal_accept_ownerIS3_S3_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Thread>::_internal_accept_owner<RBX::TaskScheduler::Thread,RBX::TaskScheduler::Thread>(boost::shared_ptr<RBX::TaskScheduler::Thread> const*,RBX::TaskScheduler::Thread *)const")]
pub fn stub_24ff58() -> ! {
    todo!("0x24ff58 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler6ThreadEE22_internal_accept_ownerIS3_S3_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0x2500b0 — __ZN5boost6detail12shared_countC2IN3RBX13TaskScheduler6ThreadEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskScheduler::Thread>(RBX::TaskScheduler::Thread *)")]
pub fn stub_2500b0() -> ! {
    todo!("0x2500b0 __ZN5boost6detail12shared_countC2IN3RBX13TaskScheduler6ThreadEEEPT_")
}

// 0x2501bc — __ZN3RBX13TaskScheduler6ThreadD2Ev
// type: void __fastcall(boost::thread **this)
#[doc(alias = "RBX::TaskScheduler::Thread::~Thread()")]
pub fn stub_2501bc() -> ! {
    todo!("0x2501bc __ZN3RBX13TaskScheduler6ThreadD2Ev")
}

// 0x2503f4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()")]
pub fn stub_2503f4() -> ! {
    todo!("0x2503f4 __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEED1Ev")
}

// 0x2503f8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()")]
pub fn stub_2503f8() -> ! {
    todo!("0x2503f8 __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEED0Ev")
}

// 0x250404 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::dispose(void)")]
pub fn stub_250404() -> ! {
    todo!("0x250404 __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE7disposeEv")
}

// 0x2504a8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_deleter(std::type_info const&)")]
pub fn stub_2504a8() -> ! {
    todo!("0x2504a8 __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE11get_deleterERKSt9type_info")
}

// 0x2504ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_untyped_deleter(void)")]
pub fn stub_2504ac() -> ! {
    todo!("0x2504ac __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE19get_untyped_deleterEv")
}

// 0x2504b0 — __ZN5boost6thread10timed_joinERKNS_10posix_time5ptimeE
// type: int __fastcall(boost::thread *, int)
#[doc(alias = "boost::thread::timed_join(boost::posix_time::ptime const&)")]
pub fn stub_2504b0() -> ! {
    todo!("0x2504b0 __ZN5boost6thread10timed_joinERKNS_10posix_time5ptimeE")
}

// 0x250588 — __ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE17add_time_durationERKS5_NS3_13time_durationE
// type: int __fastcall(int result, int *, __int64 *)
#[doc(alias = "boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::add_time_duration(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::posix_time::time_duration)")]
pub fn stub_250588() -> ! {
    todo!("0x250588 __ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE17add_time_durationERKS5_NS3_13time_durationE")
}

// 0x2506f8 — __ZN5boost15throw_exceptionISt13runtime_errorEEvRKT_
// type: void __fastcall __noreturn(int)
#[doc(alias = "void boost::throw_exception<std::runtime_error>(std::runtime_error const&)")]
pub fn stub_2506f8() -> ! {
    todo!("0x2506f8 __ZN5boost15throw_exceptionISt13runtime_errorEEvRKT_")
}

// 0x250848 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev
// type: std::runtime_error *__fastcall(std::runtime_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()")]
pub fn stub_250848() -> ! {
    todo!("0x250848 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev")
}

// 0x250900 — __ZN5boost16exception_detail19error_info_injectorISt13runtime_errorED1Ev
// type: int __fastcall(std::runtime_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()")]
pub fn stub_250900() -> ! {
    todo!("0x250900 __ZN5boost16exception_detail19error_info_injectorISt13runtime_errorED1Ev")
}

// 0x2509b8 — __ZThn8_N5boost16exception_detail19error_info_injectorISt13runtime_errorED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "__ZThn8_N5boost16exception_detail19error_info_injectorISt13runtime_errorED1Ev")]
pub fn stub_2509b8() -> ! {
    todo!("0x2509b8 __ZThn8_N5boost16exception_detail19error_info_injectorISt13runtime_errorED1Ev")
}

// 0x250a70 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "__ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev")]
pub fn stub_250a70() -> ! {
    todo!("0x250a70 __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev")
}

// 0x250b28 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev")]
pub fn stub_250b28() -> ! {
    todo!("0x250b28 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev")
}

// 0x250bf8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE5cloneEv
// type: char *__fastcall(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone(void)const")]
pub fn stub_250bf8() -> ! {
    todo!("0x250bf8 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE5cloneEv")
}

// 0x250cb8 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE5cloneEv
// type: char *__fastcall(_DWORD *)
#[doc(alias = "__ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE5cloneEv")]
pub fn stub_250cb8() -> ! {
    todo!("0x250cb8 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE5cloneEv")
}

// 0x250d80 — __ZN5boost16exception_detail19error_info_injectorISt13runtime_errorED0Ev
// type: void __fastcall(std::runtime_error *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()")]
pub fn stub_250d80() -> ! {
    todo!("0x250d80 __ZN5boost16exception_detail19error_info_injectorISt13runtime_errorED0Ev")
}

// 0x250e40 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEEC1ERKS5_NS5_9clone_tagE
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_tag)")]
pub fn stub_250e40() -> ! {
    todo!("0x250e40 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEEC1ERKS5_NS5_9clone_tagE")
}

// 0x250fc8 — __ZN5boost2CV23simple_exception_policyItLt1ELt31ENS_9gregorian16bad_day_of_monthEE8on_errorEttNS0_14violation_enumE
// type: void __noreturn()
#[doc(alias = "boost::CV::simple_exception_policy<unsigned short,(unsigned short)1,(unsigned short)31,boost::gregorian::bad_day_of_month>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)")]
pub fn stub_250fc8() -> ! {
    todo!("0x250fc8 __ZN5boost2CV23simple_exception_policyItLt1ELt31ENS_9gregorian16bad_day_of_monthEE8on_errorEttNS0_14violation_enumE")
}
