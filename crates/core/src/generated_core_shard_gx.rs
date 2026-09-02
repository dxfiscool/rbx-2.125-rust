//! core shard GX — 100 core stubs EA-sorted, 0x3e010..0x45fa4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after GW 0x3df1c).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after GW 0x3df1c (0x3e010..0x45fa4, 18021->18121 covered, 3797 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()")]
// 0x3e010 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED1Ev
pub fn stub_0x3e010() -> ! {
    todo!("0x3e010 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()")]
// 0x3e014 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED0Ev
pub fn stub_0x3e014() -> ! {
    todo!("0x3e014 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::dispose(void)")]
// 0x3e018 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE7disposeEv
pub fn stub_0x3e018() -> ! {
    todo!("0x3e018 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_deleter(std::type_info const&)")]
// 0x3e028 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE11get_deleterERKSt9type_info
pub fn stub_0x3e028() -> ! {
    todo!("0x3e028 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_untyped_deleter(void)")]
// 0x3e02c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE19get_untyped_deleterEv
pub fn stub_0x3e02c() -> ! {
    todo!("0x3e02c __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x3e030 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
pub fn stub_0x3e030() -> ! {
    todo!("0x3e030 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x3e090 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE
pub fn stub_0x3e090() -> ! {
    todo!("0x3e090 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>::operator()(void)")]
// 0x3e094 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
pub fn stub_0x3e094() -> ! {
    todo!("0x3e094 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")
}

#[doc(alias = "boost::singleton_pool<RBX::NormalBreakConnector,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x3e198 — __ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_0x3e198() -> ! {
    todo!("0x3e198 __ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "boost::singleton_pool<XmlElement,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x3e238 — __ZN5boost14singleton_poolI10XmlElementLj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_0x3e238() -> ! {
    todo!("0x3e238 __ZN5boost14singleton_poolI10XmlElementLj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "boost::function0<void>::assign_to_own(boost::function0<void> const&)")]
// 0x3e288 — __ZN5boost9function0IvE13assign_to_ownERKS1_
pub fn stub_0x3e288() -> ! {
    todo!("0x3e288 __ZN5boost9function0IvE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::exception_detail::bad_exception_::~bad_exception_()")]
// 0x3e2b8 — __ZN5boost16exception_detail14bad_exception_D1Ev
pub fn stub_0x3e2b8() -> ! {
    todo!("0x3e2b8 __ZN5boost16exception_detail14bad_exception_D1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone(void)const")]
// 0x3e2e8 — __ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv
pub fn stub_0x3e2e8() -> ! {
    todo!("0x3e2e8 __ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_> const&,boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone_tag)")]
// 0x3e3a8 — __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EEC1ERKS3_NS3_9clone_tagE
pub fn stub_0x3e3a8() -> ! {
    todo!("0x3e3a8 __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EEC1ERKS3_NS3_9clone_tagE")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::bad_exception_::~bad_exception_()")]
// 0x3e528 — __ZThn20_N5boost16exception_detail14bad_exception_D0Ev
// was: non-virtual thunk toboost::exception_detail::bad_exception_::~bad_exception_()
pub fn stub_0x3e528() -> ! {
    todo!("0x3e528 __ZThn20_N5boost16exception_detail14bad_exception_D0Ev")
}

#[doc(alias = "rbx_core::SharedPtr<boost::exception_detail::clone_base const>::shared_ptr<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_> *)")]
// 0x3e558 — __ZN5boost10shared_ptrIKNS_16exception_detail10clone_baseEEC2INS1_10clone_implINS1_14bad_exception_EEEEEPT_
// was: boost::shared_ptr<boost::exception_detail::clone_base const>::shared_ptr<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_> *)
pub fn stub_0x3e558() -> ! {
    todo!("0x3e558 __ZN5boost10shared_ptrIKNS_16exception_detail10clone_baseEEC2INS1_10clone_implINS1_14bad_exception_EEEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>::~sp_counted_impl_p()")]
// 0x3e640 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEED1Ev
pub fn stub_0x3e640() -> ! {
    todo!("0x3e640 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_impl(boost::exception_detail::bad_alloc_ const&)")]
// 0x3e648 — __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EEC1ERKS2_
pub fn stub_0x3e648() -> ! {
    todo!("0x3e648 __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EEC1ERKS2_")
}

#[doc(alias = "boost::exception_detail::bad_alloc_::~bad_alloc_()")]
// 0x3e7c8 — __ZN5boost16exception_detail10bad_alloc_D1Ev
pub fn stub_0x3e7c8() -> ! {
    todo!("0x3e7c8 __ZN5boost16exception_detail10bad_alloc_D1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone(void)const")]
// 0x3e7f8 — __ZNK5boost16exception_detail10clone_implINS0_10bad_alloc_EE5cloneEv
pub fn stub_0x3e7f8() -> ! {
    todo!("0x3e7f8 __ZNK5boost16exception_detail10clone_implINS0_10bad_alloc_EE5cloneEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::rethrow(void)const")]
// 0x3e8b8 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_10bad_alloc_EE7rethrowEv
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::rethrow(void)const
pub fn stub_0x3e8b8() -> ! {
    todo!("0x3e8b8 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_10bad_alloc_EE7rethrowEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()")]
// 0x3e8c8 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED0Ev
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()
pub fn stub_0x3e8c8() -> ! {
    todo!("0x3e8c8 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> const&,boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_tag)")]
// 0x3e900 — __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EEC1ERKS3_NS3_9clone_tagE
pub fn stub_0x3e900() -> ! {
    todo!("0x3e900 __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EEC1ERKS3_NS3_9clone_tagE")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::bad_alloc_::~bad_alloc_()")]
// 0x3ea80 — __ZThn20_N5boost16exception_detail10bad_alloc_D0Ev
// was: non-virtual thunk toboost::exception_detail::bad_alloc_::~bad_alloc_()
pub fn stub_0x3ea80() -> ! {
    todo!("0x3ea80 __ZThn20_N5boost16exception_detail10bad_alloc_D0Ev")
}

#[doc(alias = "rbx_core::SharedPtr<boost::exception_detail::clone_base const>::shared_ptr<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> *)")]
// 0x3eab0 — __ZN5boost10shared_ptrIKNS_16exception_detail10clone_baseEEC2INS1_10clone_implINS1_10bad_alloc_EEEEEPT_
// was: boost::shared_ptr<boost::exception_detail::clone_base const>::shared_ptr<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> *)
pub fn stub_0x3eab0() -> ! {
    todo!("0x3eab0 __ZN5boost10shared_ptrIKNS_16exception_detail10clone_baseEEC2INS1_10clone_implINS1_10bad_alloc_EEEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::dispose(void)")]
// 0x3eb98 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE7disposeEv
pub fn stub_0x3eb98() -> ! {
    todo!("0x3eb98 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::get_deleter(std::type_info const&)")]
// 0x3eba8 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE11get_deleterERKSt9type_info
pub fn stub_0x3eba8() -> ! {
    todo!("0x3eba8 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE11get_deleterERKSt9type_info")
}

#[doc(alias = "RBX::Tasks::Sequence::onPreStep(RBX::TaskScheduler::Job *)")]
// 0x3ebb0 — __ZN3RBX5Tasks8Sequence9onPreStepEPNS_13TaskScheduler3JobE
pub fn stub_0x3ebb0() -> ! {
    todo!("0x3ebb0 __ZN3RBX5Tasks8Sequence9onPreStepEPNS_13TaskScheduler3JobE")
}

#[doc(alias = "RBX::Tasks::ExclusiveSequence::onPostStep(RBX::TaskScheduler::Job *)")]
// 0x3ebb4 — __ZN3RBX5Tasks17ExclusiveSequence10onPostStepEPNS_13TaskScheduler3JobE
pub fn stub_0x3ebb4() -> ! {
    todo!("0x3ebb4 __ZN3RBX5Tasks17ExclusiveSequence10onPostStepEPNS_13TaskScheduler3JobE")
}

#[doc(alias = "void rbx_core::SharedPtr_weak_release<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
// 0x3ebb8 — __ZN5boost26intrusive_ptr_weak_releaseIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// was: void boost::intrusive_ptr_weak_release<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)
pub fn stub_0x3ebb8() -> ! {
    todo!("0x3ebb8 __ZN5boost26intrusive_ptr_weak_releaseIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE")
}

#[doc(alias = "RBX::TaskScheduler::Job::getDesiredConcurrencyCount(void)const")]
// 0x3f090 — __ZNK3RBX13TaskScheduler3Job26getDesiredConcurrencyCountEv
pub fn stub_0x3f090() -> ! {
    todo!("0x3f090 __ZNK3RBX13TaskScheduler3Job26getDesiredConcurrencyCountEv")
}

#[doc(alias = "RobloxView::RenderJob::getMetricValue(std::string const&)const")]
// 0x3f598 — __ZNK10RobloxView9RenderJob14getMetricValueERKSs
pub fn stub_0x3f598() -> ! {
    todo!("0x3f598 __ZNK10RobloxView9RenderJob14getMetricValueERKSs")
}

#[doc(alias = "RobloxView::RenderJob::getMetric(std::string const&)const")]
// 0x3f700 — __ZNK10RobloxView9RenderJob9getMetricERKSs
pub fn stub_0x3f700() -> ! {
    todo!("0x3f700 __ZNK10RobloxView9RenderJob9getMetricERKSs")
}

#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::getMetric(std::string const&)const")]
// 0x3fa94 — __ZThn480_NK10RobloxView9RenderJob9getMetricERKSs
// was: non-virtual thunk toRobloxView::RenderJob::getMetric(std::string const&)const
pub fn stub_0x3fa94() -> ! {
    todo!("0x3fa94 __ZThn480_NK10RobloxView9RenderJob9getMetricERKSs")
}

#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::getMetricValue(std::string const&)const")]
// 0x3faa4 — __ZThn480_NK10RobloxView9RenderJob14getMetricValueERKSs
// was: non-virtual thunk toRobloxView::RenderJob::getMetricValue(std::string const&)const
pub fn stub_0x3faa4() -> ! {
    todo!("0x3faa4 __ZThn480_NK10RobloxView9RenderJob14getMetricValueERKSs")
}

#[doc(alias = "boost::bad_weak_ptr::~bad_weak_ptr()")]
// 0x3fcf8 — __ZN5boost12bad_weak_ptrD0Ev
pub fn stub_0x3fcf8() -> ! {
    todo!("0x3fcf8 __ZN5boost12bad_weak_ptrD0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
// 0x3fd10 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev
pub fn stub_0x3fd10() -> ! {
    todo!("0x3fd10 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
// 0x3fd38 — __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev
pub fn stub_0x3fd38() -> ! {
    todo!("0x3fd38 __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
// 0x3fd60 — __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()
pub fn stub_0x3fd60() -> ! {
    todo!("0x3fd60 __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
// 0x3fd88 — __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()
pub fn stub_0x3fd88() -> ! {
    todo!("0x3fd88 __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::rethrow(void)const")]
// 0x3fdb8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv
pub fn stub_0x3fdb8() -> ! {
    todo!("0x3fdb8 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
// 0x3fee0 — __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()
pub fn stub_0x3fee0() -> ! {
    todo!("0x3fee0 __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::rethrow(void)const")]
// 0x3ff18 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::rethrow(void)const
pub fn stub_0x3ff18() -> ! {
    todo!("0x3ff18 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
// 0x3ff28 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()
pub fn stub_0x3ff28() -> ! {
    todo!("0x3ff28 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
// 0x3ff60 — __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev
pub fn stub_0x3ff60() -> ! {
    todo!("0x3ff60 __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
// 0x3ff90 — __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()
pub fn stub_0x3ff90() -> ! {
    todo!("0x3ff90 __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_tag)")]
// 0x3ffc0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEEC1ERKS5_NS5_9clone_tagE
pub fn stub_0x3ffc0() -> ! {
    todo!("0x3ffc0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEEC1ERKS5_NS5_9clone_tagE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x40160 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
pub fn stub_0x40160() -> ! {
    todo!("0x40160 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x401dc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEvE6invokeERNS1_15function_bufferE
pub fn stub_0x401dc() -> ! {
    todo!("0x401dc __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x401f0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
pub fn stub_0x401f0() -> ! {
    todo!("0x401f0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x40270 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEvE6invokeERNS1_15function_bufferE
pub fn stub_0x40270() -> ! {
    todo!("0x40270 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double> &,boost::_bi::list0 &,int)")]
// 0x4027c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0x4027c() -> ! {
    todo!("0x4027c __ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x402a8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
pub fn stub_0x402a8() -> ! {
    todo!("0x402a8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x40308 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
pub fn stub_0x40308() -> ! {
    todo!("0x40308 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "boost::function0<void>::clear(void)")]
// 0x406e0 — __ZN5boost9function0IvE5clearEv
pub fn stub_0x406e0() -> ! {
    todo!("0x406e0 __ZN5boost9function0IvE5clearEv")
}

#[doc(alias = "SimpleJSON::DefaultHandler(std::string const&,std::string const&)")]
// 0x43360 — __ZN10SimpleJSON14DefaultHandlerERKSsS1_
pub fn stub_0x43360() -> ! {
    todo!("0x43360 __ZN10SimpleJSON14DefaultHandlerERKSsS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,void (*)(char const*)>> *)")]
// 0x43364 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
pub fn stub_0x43364() -> ! {
    todo!("0x43364 __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")
}

#[doc(alias = "RBX::FunctionMarshaller::FunctionMarshaller(unsigned int)")]
// 0x4352c — __ZN3RBX18FunctionMarshallerC2Ej
pub fn stub_0x4352c() -> ! {
    todo!("0x4352c __ZN3RBX18FunctionMarshallerC2Ej")
}

#[doc(alias = "RBX::FunctionMarshaller::GetWindow(void)")]
// 0x43624 — __ZN3RBX18FunctionMarshaller9GetWindowEv
pub fn stub_0x43624() -> ! {
    todo!("0x43624 __ZN3RBX18FunctionMarshaller9GetWindowEv")
}

#[doc(alias = "RBX::FunctionMarshaller::ReleaseWindow(RBX::FunctionMarshaller*)")]
// 0x43804 — __ZN3RBX18FunctionMarshaller13ReleaseWindowEPS0_
pub fn stub_0x43804() -> ! {
    todo!("0x43804 __ZN3RBX18FunctionMarshaller13ReleaseWindowEPS0_")
}

#[doc(alias = "RBX::FunctionMarshaller::handleAppEvent(void *)")]
// 0x43930 — __ZN3RBX18FunctionMarshaller14handleAppEventEPv
pub fn stub_0x43930() -> ! {
    todo!("0x43930 __ZN3RBX18FunctionMarshaller14handleAppEventEPv")
}

#[doc(alias = "RBX::FunctionMarshaller::Execute(boost::function<void ()(void)>,RBX::CEvent *)")]
// 0x43a98 — __ZN3RBX18FunctionMarshaller7ExecuteEN5boost8functionIFvvEEEPNS_6CEventE
pub fn stub_0x43a98() -> ! {
    todo!("0x43a98 __ZN3RBX18FunctionMarshaller7ExecuteEN5boost8functionIFvvEEEPNS_6CEventE")
}

#[doc(alias = "RBX::FunctionMarshaller::Submit(boost::function<void ()(void)>)")]
// 0x43b98 — __ZN3RBX18FunctionMarshaller6SubmitEN5boost8functionIFvvEEE
pub fn stub_0x43b98() -> ! {
    todo!("0x43b98 __ZN3RBX18FunctionMarshaller6SubmitEN5boost8functionIFvvEEE")
}

#[doc(alias = "RBX::FunctionMarshaller::ProcessMessages(void)")]
// 0x43c70 — __ZN3RBX18FunctionMarshaller15ProcessMessagesEv
pub fn stub_0x43c70() -> ! {
    todo!("0x43c70 __ZN3RBX18FunctionMarshaller15ProcessMessagesEv")
}

#[doc(alias = "RBX::FunctionMarshaller::StaticData::~StaticData()")]
// 0x43c74 — __ZN3RBX18FunctionMarshaller10StaticDataD1Ev
pub fn stub_0x43c74() -> ! {
    todo!("0x43c74 __ZN3RBX18FunctionMarshaller10StaticDataD1Ev")
}

#[doc(alias = "RBX::FunctionMarshaller::StaticData::~StaticData()")]
// 0x43c78 — __ZN3RBX18FunctionMarshaller10StaticDataD2Ev
pub fn stub_0x43c78() -> ! {
    todo!("0x43c78 __ZN3RBX18FunctionMarshaller10StaticDataD2Ev")
}

#[doc(alias = "std::map<unsigned int,RBX::FunctionMarshaller *,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::operator[](unsigned int const&)")]
// 0x43d14 — __ZNSt3mapIjPN3RBX18FunctionMarshallerESt4lessIjESaISt4pairIKjS2_EEEixERS6_
pub fn stub_0x43d14() -> ! {
    todo!("0x43d14 __ZNSt3mapIjPN3RBX18FunctionMarshallerESt4lessIjESaISt4pairIKjS2_EEEixERS6_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(unsigned int const&)")]
// 0x43d6c — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseERS1_
pub fn stub_0x43d6c() -> ! {
    todo!("0x43d6c __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseERS1_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::equal_range(unsigned int const&)")]
// 0x43d94 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE11equal_rangeERS1_
pub fn stub_0x43d94() -> ! {
    todo!("0x43d94 __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE11equal_rangeERS1_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>)")]
// 0x43de0 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
pub fn stub_0x43de0() -> ! {
    todo!("0x43de0 __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,RBX::FunctionMarshaller *>> *)")]
// 0x43e40 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0x43e40() -> ! {
    todo!("0x43e40 __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
// 0x43e68 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_0x43e68() -> ! {
    todo!("0x43e68 __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
// 0x43f1c — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_0x43f1c() -> ! {
    todo!("0x43f1c __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
// 0x43f74 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_0x43f74() -> ! {
    todo!("0x43f74 __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "boost::unique_lock<boost::recursive_mutex>::lock(void)")]
// 0x43fdc — __ZN5boost11unique_lockINS_15recursive_mutexEE4lockEv
pub fn stub_0x43fdc() -> ! {
    todo!("0x43fdc __ZN5boost11unique_lockINS_15recursive_mutexEE4lockEv")
}

#[doc(alias = "RBX::FunctionMarshaller::safe_static_init_staticData(void)")]
// 0x441a8 — __ZN3RBX18FunctionMarshaller27safe_static_init_staticDataEv
pub fn stub_0x441a8() -> ! {
    todo!("0x441a8 __ZN3RBX18FunctionMarshaller27safe_static_init_staticDataEv")
}

#[doc(alias = "RBX::FunctionMarshaller::safe_static_do_get_staticData(void)")]
// 0x441ac — __ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv
pub fn stub_0x441ac() -> ! {
    todo!("0x441ac __ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv")
}

#[doc(alias = "boost::recursive_mutex::recursive_mutex(void)")]
// 0x442bc — __ZN5boost15recursive_mutexC2Ev
pub fn stub_0x442bc() -> ! {
    todo!("0x442bc __ZN5boost15recursive_mutexC2Ev")
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::~_Deque_base()")]
// 0x44564 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EED2Ev
pub fn stub_0x44564() -> ! {
    todo!("0x44564 __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EED2Ev")
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_initialize_map(unsigned long)")]
// 0x44590 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE17_M_initialize_mapEm
pub fn stub_0x44590() -> ! {
    todo!("0x44590 __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_allocate_map(unsigned long)")]
// 0x446e8 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_allocate_mapEm
pub fn stub_0x446e8() -> ! {
    todo!("0x446e8 __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_allocate_mapEm")
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_create_nodes(boost::function<void ()(void)> ***,boost::function<void ()(void)> ***)")]
// 0x44700 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_create_nodesEPPS4_S8_
pub fn stub_0x44700() -> ! {
    todo!("0x44700 __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_create_nodesEPPS4_S8_")
}

#[doc(alias = "std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::deque(std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>> const&)")]
// 0x447f4 — __ZNSt5dequeIPN5boost8functionIFvvEEESaIS4_EEC2ERKS6_
pub fn stub_0x447f4() -> ! {
    todo!("0x447f4 __ZNSt5dequeIPN5boost8functionIFvvEEESaIS4_EEC2ERKS6_")
}

#[doc(alias = "std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>>(std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>)")]
// 0x44888 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN5boost8functionIFvvEEERKS8_PS9_ES3_IS8_RS8_PS8_EEET0_T_SH_SG_
pub fn stub_0x44888() -> ! {
    todo!("0x44888 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN5boost8functionIFvvEEERKS8_PS9_ES3_IS8_RS8_PS8_EEET0_T_SH_SG_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::connect<boost::function<void ()(bool,void *,RBX::UIEvent)>>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&)")]
// 0x4546c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
pub fn stub_0x4546c() -> ! {
    todo!("0x4546c __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::insert(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
// 0x45554 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6insertEPNS6_4slotE
pub fn stub_0x45554() -> ! {
    todo!("0x45554 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6insertEPNS6_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)")]
// 0x45764 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)
pub fn stub_0x45764() -> ! {
    todo!("0x45764 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)")]
// 0x45808 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)
pub fn stub_0x45808() -> ! {
    todo!("0x45808 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_do_get_mutex(void)")]
// 0x458ac — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv
pub fn stub_0x458ac() -> ! {
    todo!("0x458ac __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&,rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*)")]
// 0x459a4 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
pub fn stub_0x459a4() -> ! {
    todo!("0x459a4 __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
// 0x45aa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED1Ev
pub fn stub_0x45aa0() -> ! {
    todo!("0x45aa0 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
// 0x45b74 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED0Ev
pub fn stub_0x45b74() -> ! {
    todo!("0x45b74 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::disconnect(void)")]
// 0x45c4c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot10disconnectEv
pub fn stub_0x45c4c() -> ! {
    todo!("0x45c4c __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::connected(void)const")]
// 0x45d5c — __ZNK3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot9connectedEv
pub fn stub_0x45d5c() -> ! {
    todo!("0x45d5c __ZNK3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
// 0x45d68 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
pub fn stub_0x45d68() -> ! {
    todo!("0x45d68 __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
// 0x45d98 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)
pub fn stub_0x45d98() -> ! {
    todo!("0x45d98 __ZThn4_N3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_")
}

#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::operator()(bool,void *,RBX::UIEvent)const")]
// 0x45dc8 — __ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_
pub fn stub_0x45dc8() -> ! {
    todo!("0x45dc8 __ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::remove(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
// 0x45eb0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6removeEPNS6_4slotE
pub fn stub_0x45eb0() -> ! {
    todo!("0x45eb0 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_init_mutex(void)")]
// 0x45fa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot22safe_static_init_mutexEv
pub fn stub_0x45fa0() -> ! {
    todo!("0x45fa0 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_do_get_mutex(void)")]
// 0x45fa4 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x45fa4() -> ! {
    todo!("0x45fa4 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv")
}
