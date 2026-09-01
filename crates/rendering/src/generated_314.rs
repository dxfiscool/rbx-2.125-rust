//! rendering shard 314 — 100 stubs 0x46dff8..0x473f98 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 34200->34300 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 34200 before -> 34300 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x46dff8 (lowest remaining 0x46dff8..0x473f98, next lowest 0x473fd0)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x46dff8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEENS3_5list1INS3_5valueISB_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// type: void
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEENS3_5list1INS3_5valueISB_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
pub fn stub_46dff8() -> ! {
    todo!("0x46dff8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x46e014 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEENS3_5list1INS3_5valueISB_EEEEEEvPS7_E6invokeERNS1_15function_bufferESJ_
// type: void
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEENS3_5list1INS3_5valueISB_EEEEEEvPS7_E6invokeERNS1_15function_bufferESJ_
pub fn stub_46e014() -> ! {
    todo!("0x46e014 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")
}

// 0x46e030 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS4_10LegacyLock14Implementation6EventsEEEENS8_5list1INS8_5valueISE_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS4_10LegacyLock14Implementation6EventsEEEENS8_5list1INS8_5valueISE_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_46e030() -> ! {
    todo!("0x46e030 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,boost::detail::function::function_buffer &)const")
}

// 0x46e110 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS4_10LegacyLock14Implementation6EventsEEEENS8_5list1INS8_5valueISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS4_10LegacyLock14Implementation6EventsEEEENS8_5list1INS8_5valueISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_46e110() -> ! {
    todo!("0x46e110 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x46e208 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEEEEclIPFvS9_ENS1_IRPS5_EEEEvNS0_4typeIvEERT_RT0_i
// type: void
#[doc(alias = "void boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>) &,boost::_bi::list1<RBX::DataModel*&> &,int)")]
// was: __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEEEEclIPFvS9_ENS1_IRPS5_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_46e208() -> ! {
    todo!("0x46e208 void boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::operator()<void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>) &,boost::_bi::list1<RBX::DataModel*&> &,int)")
}

// 0x46e2d4 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEENS3_5list1INS3_5valueISB_EEEEEEE12manage_smallERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// type: int(void)
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEENS3_5list1INS3_5valueISB_EEEEEEE12manage_smallERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
pub fn stub_46e2d4() -> ! {
    todo!("0x46e2d4 boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x46e358 — __ZN3RBX9DataModel10LegacyLock14Implementation6EventsC2Ev
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation::Events *__hidden this)
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::Events::Events(void)")]
// was: __ZN3RBX9DataModel10LegacyLock14Implementation6EventsC2Ev
pub fn stub_46e358() -> ! {
    todo!("0x46e358 RBX::DataModel::LegacyLock::Implementation::Events::Events(void)")
}

// 0x46e404 — __ZN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEC2IS5_EEPT_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>(RBX::DataModel::LegacyLock::Implementation::Events *)")]
// was: __ZN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEC2IS5_EEPT_
pub fn stub_46e404() -> ! {
    todo!("0x46e404 boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>(RBX::DataModel::LegacyLock::Implementation::Events *)")
}

// 0x46e4d8 — __ZN5boost6detail12shared_countC2IN3RBX9DataModel10LegacyLock14Implementation6EventsEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DataModel::LegacyLock::Implementation::Events>(RBX::DataModel::LegacyLock::Implementation::Events *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX9DataModel10LegacyLock14Implementation6EventsEEEPT_
pub fn stub_46e4d8() -> ! {
    todo!("0x46e4d8 boost::detail::shared_count::shared_count<RBX::DataModel::LegacyLock::Implementation::Events>(RBX::DataModel::LegacyLock::Implementation::Events *)")
}

// 0x46e5d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEED1Ev
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEED1Ev
pub fn stub_46e5d8() -> ! {
    todo!("0x46e5d8 boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::~sp_counted_impl_p()")
}

// 0x46e5dc — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEED0Ev
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEED0Ev
pub fn stub_46e5dc() -> ! {
    todo!("0x46e5dc boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::~sp_counted_impl_p()")
}

// 0x46e5e0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEE7disposeEv
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEE7disposeEv
pub fn stub_46e5e0() -> ! {
    todo!("0x46e5e0 boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::dispose(void)")
}

// 0x46e604 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEE11get_deleterERKSt9type_info
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEE11get_deleterERKSt9type_info
pub fn stub_46e604() -> ! {
    todo!("0x46e604 boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::get_deleter(std::type_info const&)")
}

// 0x46e608 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEE19get_untyped_deleterEv
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEE19get_untyped_deleterEv
pub fn stub_46e608() -> ! {
    todo!("0x46e608 boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::get_untyped_deleter(void)")
}

// 0x46e60c — __ZN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEaSERKS6_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>::operator=(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEaSERKS6_
pub fn stub_46e60c() -> ! {
    todo!("0x46e60c boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>::operator=(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")
}

// 0x46e644 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE9pop_frontEv
// type: int(void)
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::pop_front(void)")]
// was: __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE9pop_frontEv
pub fn stub_46e644() -> ! {
    todo!("0x46e644 std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::pop_front(void)")
}

// 0x46e670 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE16_M_pop_front_auxEv
// type: int(void)
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_pop_front_aux(void)")]
// was: __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE16_M_pop_front_auxEv
pub fn stub_46e670() -> ! {
    todo!("0x46e670 std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_pop_front_aux(void)")
}

// 0x46e69c — __ZN3RBX9DataModel10LegacyLock14Implementation27safe_static_init_eventsPoolEv
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation *__hidden this)
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_init_eventsPool(void)")]
// was: __ZN3RBX9DataModel10LegacyLock14Implementation27safe_static_init_eventsPoolEv
pub fn stub_46e69c() -> ! {
    todo!("0x46e69c RBX::DataModel::LegacyLock::Implementation::safe_static_init_eventsPool(void)")
}

// 0x46e6a0 — __ZN3RBX9DataModel10LegacyLock14Implementation29safe_static_do_get_eventsPoolEv
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation *__hidden this)
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_eventsPool(void)")]
// was: __ZN3RBX9DataModel10LegacyLock14Implementation29safe_static_do_get_eventsPoolEv
pub fn stub_46e6a0() -> ! {
    todo!("0x46e6a0 RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_eventsPool(void)")
}

// 0x46e808 — __ZN3rbx10safe_queueIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEED1Ev
// type: void
#[doc(alias = "rbx::safe_queue<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>::~safe_queue()")]
// was: __ZN3rbx10safe_queueIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEED1Ev
pub fn stub_46e808() -> ! {
    todo!("0x46e808 rbx::safe_queue<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>::~safe_queue()")
}

// 0x46e8cc — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EED2Ev
// type: void
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~deque()")]
// was: __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EED2Ev
pub fn stub_46e8cc() -> ! {
    todo!("0x46e8cc std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~deque()")
}

// 0x46e9b4 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EED2Ev
// type: int(void)
#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~_Deque_base()")]
// was: __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EED2Ev
pub fn stub_46e9b4() -> ! {
    todo!("0x46e9b4 std::_Deque_base<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~_Deque_base()")
}

// 0x46e9e0 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_destroy_data_aux(std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>)")]
// was: __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_
pub fn stub_46e9e0() -> ! {
    todo!("0x46e9e0 std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*>)")
}

// 0x46eb20 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_initialize_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE17_M_initialize_mapEm
pub fn stub_46eb20() -> ! {
    todo!("0x46eb20 std::_Deque_base<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_initialize_map(unsigned long)")
}

// 0x46ec78 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE15_M_create_nodesEPPS7_SB_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_create_nodes(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>**,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>**)")]
// was: __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE15_M_create_nodesEPPS7_SB_
pub fn stub_46ec78() -> ! {
    todo!("0x46ec78 std::_Deque_base<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_create_nodes(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>**,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>**)")
}

// 0x46ed6c — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EEC2ERKS9_
// type: void
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::deque(std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>> const&)")]
// was: __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EEC2ERKS9_
pub fn stub_46ed6c() -> ! {
    todo!("0x46ed6c std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::deque(std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>> const&)")
}

// 0x46ee90 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEERKS8_PS9_ES0_IS8_RS8_PS8_EET0_T_SH_SG_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>>(std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::__false_type)")]
// was: __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEERKS8_PS9_ES0_IS8_RS8_PS8_EET0_T_SH_SG_St12__false_type
pub fn stub_46ee90() -> ! {
    todo!("0x46ee90 std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*> std::__uninitialized_copy_aux<std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*>>(std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::__false_type)")
}

// 0x46f030 — __ZN3RBX9DataModel10LegacyLock14Implementation27safe_static_init_currentJobEv
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation *__hidden this)
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_init_currentJob(void)")]
// was: __ZN3RBX9DataModel10LegacyLock14Implementation27safe_static_init_currentJobEv
pub fn stub_46f030() -> ! {
    todo!("0x46f030 RBX::DataModel::LegacyLock::Implementation::safe_static_init_currentJob(void)")
}

// 0x46f034 — __ZN3RBX9DataModel10LegacyLock14Implementation29safe_static_do_get_currentJobEv
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation *__hidden this)
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_currentJob(void)")]
// was: __ZN3RBX9DataModel10LegacyLock14Implementation29safe_static_do_get_currentJobEv
pub fn stub_46f034() -> ! {
    todo!("0x46f034 RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_currentJob(void)")
}

// 0x46f148 — __ZN3rbx25thread_specific_referenceIN3RBX9DataModel10GenericJobEED1Ev
// type: void
#[doc(alias = "rbx::thread_specific_reference<RBX::DataModel::GenericJob>::~thread_specific_reference()")]
// was: __ZN3rbx25thread_specific_referenceIN3RBX9DataModel10GenericJobEED1Ev
pub fn stub_46f148() -> ! {
    todo!("0x46f148 rbx::thread_specific_reference<RBX::DataModel::GenericJob>::~thread_specific_reference()")
}

// 0x46f158 — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEED2Ev
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::~thread_specific_ptr()")]
// was: __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEED2Ev
pub fn stub_46f158() -> ! {
    todo!("0x46f158 boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::~thread_specific_ptr()")
}

// 0x46f24c — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataD1Ev
// type: void
#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data()")]
// was: __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataD1Ev
pub fn stub_46f24c() -> ! {
    todo!("0x46f24c boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data()")
}

// 0x46f250 — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataD0Ev
// type: void
#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data()")]
// was: __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataD0Ev
pub fn stub_46f250() -> ! {
    todo!("0x46f250 boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data()")
}

// 0x46f254 — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataclEPv
// type: void
#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::operator()(void *)")]
// was: __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataclEPv
pub fn stub_46f254() -> ! {
    todo!("0x46f254 boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::operator()(void *)")
}

// 0x46f260 — __ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS9_EEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>(boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>)")]
// was: __ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS9_EEEET_T0_
pub fn stub_46f260() -> ! {
    todo!("0x46f260 boost::detail::shared_count::shared_count<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>(boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>)")
}

// 0x46f358 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEED1Ev
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEED1Ev
pub fn stub_46f358() -> ! {
    todo!("0x46f358 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd()")
}

// 0x46f35c — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEED0Ev
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEED0Ev
pub fn stub_46f35c() -> ! {
    todo!("0x46f35c boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd()")
}

// 0x46f360 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE7disposeEv
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE7disposeEv
pub fn stub_46f360() -> ! {
    todo!("0x46f360 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::dispose(void)")
}

// 0x46f370 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE11get_deleterERKSt9type_info
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE11get_deleterERKSt9type_info
pub fn stub_46f370() -> ! {
    todo!("0x46f370 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_deleter(std::type_info const&)")
}

// 0x46f388 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE19get_untyped_deleterEv
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE19get_untyped_deleterEv
pub fn stub_46f388() -> ! {
    todo!("0x46f388 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_untyped_deleter(void)")
}

// 0x46f38c — __ZN3rbx7signals6signalIFvRKSsEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(std::string const&)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvRKSsEE13disconnectAllEv
pub fn stub_46f38c() -> ! {
    todo!("0x46f38c rbx::signals::signal<void ()(std::string const&)>::disconnectAll(void)")
}

// 0x46f504 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE13disconnectAllEv
pub fn stub_46f504() -> ! {
    todo!("0x46f504 rbx::signals::signal<void ()(RBX::UIEvent const&)>::disconnectAll(void)")
}

// 0x46f67c — __ZNSt8_Rb_treeISsSt4pairIKSsiESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,std::allocator<std::pair<std::string const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,int>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsiESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_46f67c() -> ! {
    todo!("0x46f67c std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,std::allocator<std::pair<std::string const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,int>> *)")
}

// 0x46f6b0 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm
// type: int(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::reserve_for_insert(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm
pub fn stub_46f6b0() -> ! {
    todo!("0x46f6b0 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::reserve_for_insert(unsigned long)")
}

// 0x46f704 — __ZNK3RBX13GuiImageMixin18getImageRectOffsetEv
// type: _DWORD __fastcall(RBX::GuiImageMixin *__hidden this)
#[doc(alias = "RBX::GuiImageMixin::getImageRectOffset(void)const")]
// was: __ZNK3RBX13GuiImageMixin18getImageRectOffsetEv
pub fn stub_46f704() -> ! {
    todo!("0x46f704 RBX::GuiImageMixin::getImageRectOffset(void)const")
}

// 0x46f734 — __ZNK3RBX13GuiImageMixin16getImageRectSizeEv
// type: _DWORD __fastcall(RBX::GuiImageMixin *__hidden this)
#[doc(alias = "RBX::GuiImageMixin::getImageRectSize(void)const")]
// was: __ZNK3RBX13GuiImageMixin16getImageRectSizeEv
pub fn stub_46f734() -> ! {
    todo!("0x46f734 RBX::GuiImageMixin::getImageRectSize(void)const")
}

// 0x46fd8c — __ZN3RBX9DataModel10MouseStatsC2Ev
// type: _DWORD __fastcall(RBX::DataModel::MouseStats *__hidden this)
#[doc(alias = "RBX::DataModel::MouseStats::MouseStats(void)")]
// was: __ZN3RBX9DataModel10MouseStatsC2Ev
pub fn stub_46fd8c() -> ! {
    todo!("0x46fd8c RBX::DataModel::MouseStats::MouseStats(void)")
}

// 0x46feac — __ZN3RBX7IMetricD1Ev
// type: void __fastcall(RBX::IMetric *__hidden this)
#[doc(alias = "RBX::IMetric::~IMetric()")]
// was: __ZN3RBX7IMetricD1Ev
pub fn stub_46feac() -> ! {
    todo!("0x46feac RBX::IMetric::~IMetric()")
}

// 0x46feb0 — __ZN3RBX7IMetricD0Ev
// type: void __fastcall(RBX::IMetric *__hidden this)
#[doc(alias = "RBX::IMetric::~IMetric()")]
// was: __ZN3RBX7IMetricD0Ev
pub fn stub_46feb0() -> ! {
    todo!("0x46feb0 RBX::IMetric::~IMetric()")
}

// 0x46feb8 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE7releaseEv
// type: void
#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::release(void)")]
// was: __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE7releaseEv
pub fn stub_46feb8() -> ! {
    todo!("0x46feb8 boost::thread_specific_ptr<RBX::Security::Context>::release(void)")
}

// 0x46ff84 — __ZN3RBX9DataModel10GenericJobC2EN5boost10shared_ptrIS0_EEPKcNS_12DataModelJob8TaskTypeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int, int, int, RBX::TaskScheduler::Job *, int, int, int, int, int)
#[doc(alias = "RBX::DataModel::GenericJob::GenericJob(rbx_core::SharedPtr<RBX::DataModel>,char const*,RBX::DataModelJob::TaskType)")]
// was: __ZN3RBX9DataModel10GenericJobC2EN5boost10shared_ptrIS0_EEPKcNS_12DataModelJob8TaskTypeE
pub fn stub_46ff84() -> ! {
    todo!("0x46ff84 RBX::DataModel::GenericJob::GenericJob(boost::shared_ptr<RBX::DataModel>,char const*,RBX::DataModelJob::TaskType)")
}

// 0x47013c — __ZN3RBX9DataModel10GenericJobD1Ev
// type: void __fastcall(RBX::DataModel::GenericJob *__hidden this)
#[doc(alias = "RBX::DataModel::GenericJob::~GenericJob()")]
// was: __ZN3RBX9DataModel10GenericJobD1Ev
pub fn stub_47013c() -> ! {
    todo!("0x47013c RBX::DataModel::GenericJob::~GenericJob()")
}

// 0x47025c — __ZN3RBX9DataModel10GenericJobD0Ev
// type: void __fastcall(RBX::DataModel::GenericJob *__hidden this)
#[doc(alias = "RBX::DataModel::GenericJob::~GenericJob()")]
// was: __ZN3RBX9DataModel10GenericJobD0Ev
pub fn stub_47025c() -> ! {
    todo!("0x47025c RBX::DataModel::GenericJob::~GenericJob()")
}

// 0x470390 — __ZN3RBX9DataModel10GenericJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void
#[doc(alias = "RBX::DataModel::GenericJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX9DataModel10GenericJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
pub fn stub_470390() -> ! {
    todo!("0x470390 RBX::DataModel::GenericJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x470400 — __ZN3RBX9DataModel10GenericJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::DataModel::GenericJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::DataModel::GenericJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX9DataModel10GenericJob5errorERKNS_13TaskScheduler3Job5StatsE
pub fn stub_470400() -> ! {
    todo!("0x470400 RBX::DataModel::GenericJob::error(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x470484 — __ZN3RBX9DataModel10GenericJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::DataModel::GenericJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::DataModel::GenericJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX9DataModel10GenericJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
pub fn stub_470484() -> ! {
    todo!("0x470484 RBX::DataModel::GenericJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x470670 — __ZN3RBX9DataModel10GenericJob12processTasksEv
// type: _DWORD __fastcall(RBX::DataModel::GenericJob *__hidden this)
#[doc(alias = "RBX::DataModel::GenericJob::processTasks(void)")]
// was: __ZN3RBX9DataModel10GenericJob12processTasksEv
pub fn stub_470670() -> ! {
    todo!("0x470670 RBX::DataModel::GenericJob::processTasks(void)")
}

// 0x470818 — __ZN3RBX9DataModel10GenericJob4stepERN5boost8functionIFvPS0_EEE
// type: void
#[doc(alias = "RBX::DataModel::GenericJob::step(boost::function<void ()(RBX::DataModel*)> &)")]
// was: __ZN3RBX9DataModel10GenericJob4stepERN5boost8functionIFvPS0_EEE
pub fn stub_470818() -> ! {
    todo!("0x470818 RBX::DataModel::GenericJob::step(boost::function<void ()(RBX::DataModel*)> &)")
}

// 0x4708e0 — __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EED2Ev
// type: void
#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~deque()")]
// was: __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EED2Ev
pub fn stub_4708e0() -> ! {
    todo!("0x4708e0 std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~deque()")
}

// 0x4709c8 — __ZNSt11_Deque_baseIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EED2Ev
// type: int(void)
#[doc(alias = "std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~_Deque_base()")]
// was: __ZNSt11_Deque_baseIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EED2Ev
pub fn stub_4709c8() -> ! {
    todo!("0x4709c8 std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~_Deque_base()")
}

// 0x4709f8 — __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EEC2ERKSC_
// type: void
#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::deque(std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>> const&)")]
// was: __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EEC2ERKSC_
pub fn stub_4709f8() -> ! {
    todo!("0x4709f8 std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::deque(std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>> const&)")
}

// 0x470b30 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Instance10SaveFilterEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Instance10SaveFilterEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_470b30() -> ! {
    todo!("0x470b30 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>> *)")
}

// 0x470b58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::GearType>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_470b58() -> ! {
    todo!("0x470b58 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::GearType>> *)")
}

// 0x470b80 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_470b80() -> ! {
    todo!("0x470b80 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>> *)")
}

// 0x470ba8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::Genre>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_470ba8() -> ! {
    todo!("0x470ba8 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::Genre>> *)")
}

// 0x470bd0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_470bd0() -> ! {
    todo!("0x470bd0 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>> *)")
}

// 0x470bf8 — __GLOBAL__I_a_178
// type: void
#[doc(alias = "global constructor keyed to_a_178")]
// was: __GLOBAL__I_a_178
pub fn stub_470bf8() -> ! {
    todo!("0x470bf8 global constructor keyed to_a_178")
}

// 0x4727ec — __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEC1Ev
// type: void
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEC1Ev
pub fn stub_4727ec() -> ! {
    todo!("0x4727ec RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::EnumDesc(void)")
}

// 0x4727f0 — __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEC2Ev
// type: void
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEC2Ev
pub fn stub_4727f0() -> ! {
    todo!("0x4727f0 RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::EnumDesc(void)")
}

// 0x4729dc — __ZN3RBX12DataModelJobC2EPKcNS0_8TaskTypeEbN5boost10shared_ptrINS_16DataModelArbiterEEENS_4Time8IntervalE
// type: int __fastcall(char, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, RBX::TaskScheduler::Job *, int, int, int, int)
#[doc(alias = "RBX::DataModelJob::DataModelJob(char const*,RBX::DataModelJob::TaskType,bool,rbx_core::SharedPtr<RBX::DataModelArbiter>,RBX::Time::Interval)")]
// was: __ZN3RBX12DataModelJobC2EPKcNS0_8TaskTypeEbN5boost10shared_ptrINS_16DataModelArbiterEEENS_4Time8IntervalE
pub fn stub_4729dc() -> ! {
    todo!("0x4729dc RBX::DataModelJob::DataModelJob(char const*,RBX::DataModelJob::TaskType,bool,boost::shared_ptr<RBX::DataModelArbiter>,RBX::Time::Interval)")
}

// 0x472b4c — __ZN3RBX12DataModelJob4stepERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::DataModelJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::DataModelJob::step(RBX::TaskScheduler::Job::Stats const&)")]
// was: __ZN3RBX12DataModelJob4stepERKNS_13TaskScheduler3Job5StatsE
pub fn stub_472b4c() -> ! {
    todo!("0x472b4c RBX::DataModelJob::step(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x472cd4 — __ZN3RBX12DataModelJob17getPriorityFactorEv
// type: _DWORD __fastcall(RBX::DataModelJob *__hidden this)
#[doc(alias = "RBX::DataModelJob::getPriorityFactor(void)")]
// was: __ZN3RBX12DataModelJob17getPriorityFactorEv
pub fn stub_472cd4() -> ! {
    todo!("0x472cd4 RBX::DataModelJob::getPriorityFactor(void)")
}

// 0x472e00 — __ZN3RBX16DataModelArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_
// type: _DWORD __fastcall(RBX::DataModelArbiter *__hidden this, RBX::TaskScheduler::Job *, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::DataModelArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
// was: __ZN3RBX16DataModelArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_
pub fn stub_472e00() -> ! {
    todo!("0x472e00 RBX::DataModelArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")
}

// 0x472e2c — __ZN3RBX16DataModelArbiterC2Ev
// type: _DWORD __fastcall(RBX::DataModelArbiter *__hidden this)
#[doc(alias = "RBX::DataModelArbiter::DataModelArbiter(void)")]
// was: __ZN3RBX16DataModelArbiterC2Ev
pub fn stub_472e2c() -> ! {
    todo!("0x472e2c RBX::DataModelArbiter::DataModelArbiter(void)")
}

// 0x473124 — __ZN3RBX16DataModelArbiterD0Ev
// type: void __fastcall(RBX::DataModelArbiter *__hidden this)
#[doc(alias = "RBX::DataModelArbiter::~DataModelArbiter()")]
// was: __ZN3RBX16DataModelArbiterD0Ev
pub fn stub_473124() -> ! {
    todo!("0x473124 RBX::DataModelArbiter::~DataModelArbiter()")
}

// 0x4731c4 — __ZN3RBX16DataModelArbiterD1Ev
// type: void __fastcall(RBX::DataModelArbiter *__hidden this)
#[doc(alias = "RBX::DataModelArbiter::~DataModelArbiter()")]
// was: __ZN3RBX16DataModelArbiterD1Ev
pub fn stub_4731c4() -> ! {
    todo!("0x4731c4 RBX::DataModelArbiter::~DataModelArbiter()")
}

// 0x4731c8 — __ZN3RBX16DataModelArbiterD2Ev
// type: void __fastcall(RBX::DataModelArbiter *__hidden this)
#[doc(alias = "RBX::DataModelArbiter::~DataModelArbiter()")]
// was: __ZN3RBX16DataModelArbiterD2Ev
pub fn stub_4731c8() -> ! {
    todo!("0x4731c8 RBX::DataModelArbiter::~DataModelArbiter()")
}

// 0x473318 — __ZN3RBX16DataModelArbiter7preStepEPNS_13TaskScheduler3JobE
// type: _DWORD __fastcall(RBX::DataModelArbiter *__hidden this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::DataModelArbiter::preStep(RBX::TaskScheduler::Job *)")]
// was: __ZN3RBX16DataModelArbiter7preStepEPNS_13TaskScheduler3JobE
pub fn stub_473318() -> ! {
    todo!("0x473318 RBX::DataModelArbiter::preStep(RBX::TaskScheduler::Job *)")
}

// 0x473350 — __ZN3RBX16DataModelArbiter8postStepEPNS_13TaskScheduler3JobE
// type: _DWORD __fastcall(RBX::DataModelArbiter *__hidden this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::DataModelArbiter::postStep(RBX::TaskScheduler::Job *)")]
// was: __ZN3RBX16DataModelArbiter8postStepEPNS_13TaskScheduler3JobE
pub fn stub_473350() -> ! {
    todo!("0x473350 RBX::DataModelArbiter::postStep(RBX::TaskScheduler::Job *)")
}

// 0x473388 — __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::addPair(RBX::DataModelArbiter::ConcurrencyModel,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE7addPairES3_PKc
pub fn stub_473388() -> ! {
    todo!("0x473388 RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::addPair(RBX::DataModelArbiter::ConcurrencyModel,char const*)")
}

// 0x4736e8 — __ZN3RBX12DataModelJobD1Ev
// type: void __fastcall(RBX::DataModelJob *__hidden this)
#[doc(alias = "RBX::DataModelJob::~DataModelJob()")]
// was: __ZN3RBX12DataModelJobD1Ev
pub fn stub_4736e8() -> ! {
    todo!("0x4736e8 RBX::DataModelJob::~DataModelJob()")
}

// 0x4736ec — __ZN3RBX12DataModelJobD0Ev
// type: void __fastcall(RBX::DataModelJob *__hidden this)
#[doc(alias = "RBX::DataModelJob::~DataModelJob()")]
// was: __ZN3RBX12DataModelJobD0Ev
pub fn stub_4736ec() -> ! {
    todo!("0x4736ec RBX::DataModelJob::~DataModelJob()")
}

// 0x473790 — __ZN3RBX23SimpleThrottlingArbiter11isThrottledEv
// type: _DWORD __fastcall(RBX::SimpleThrottlingArbiter *__hidden this)
#[doc(alias = "RBX::SimpleThrottlingArbiter::isThrottled(void)")]
// was: __ZN3RBX23SimpleThrottlingArbiter11isThrottledEv
pub fn stub_473790() -> ! {
    todo!("0x473790 RBX::SimpleThrottlingArbiter::isThrottled(void)")
}

// 0x473858 — __ZN3RBX13TaskScheduler7Arbiter24getSyncronizationArbiterEv
// type: _DWORD __fastcall(RBX::TaskScheduler::Arbiter *__hidden this)
#[doc(alias = "RBX::TaskScheduler::Arbiter::getSyncronizationArbiter(void)")]
// was: __ZN3RBX13TaskScheduler7Arbiter24getSyncronizationArbiterEv
pub fn stub_473858() -> ! {
    todo!("0x473858 RBX::TaskScheduler::Arbiter::getSyncronizationArbiter(void)")
}

// 0x473860 — __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEED1Ev
// type: void
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEED1Ev
pub fn stub_473860() -> ! {
    todo!("0x473860 RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::~EnumDesc()")
}

// 0x473868 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE15convertToStringEmRSs
// type: void
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE15convertToStringEmRSs
pub fn stub_473868() -> ! {
    todo!("0x473868 RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToString(unsigned long,std::string &)const")
}

// 0x4739b0 — __ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE14construct_funcEPKcPc
// type: void
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModelArbiter::ConcurrencyModel>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE14construct_funcEPKcPc
pub fn stub_4739b0() -> ! {
    todo!("0x4739b0 rbx::implementation::typed_holder<RBX::DataModelArbiter::ConcurrencyModel>::construct_func(char const*,char *)")
}

// 0x4739c0 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE13convertToItemERKS3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToItem(RBX::DataModelArbiter::ConcurrencyModel const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE13convertToItemERKS3_
pub fn stub_4739c0() -> ! {
    todo!("0x4739c0 RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToItem(RBX::DataModelArbiter::ConcurrencyModel const&)const")
}

// 0x473a8c — __ZN3rbx8any_castIRKN3RBX16DataModelArbiter16ConcurrencyModelENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "RBX::DataModelArbiter::ConcurrencyModel const& rbx::any_cast<RBX::DataModelArbiter::ConcurrencyModel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX16DataModelArbiter16ConcurrencyModelENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_473a8c() -> ! {
    todo!("0x473a8c RBX::DataModelArbiter::ConcurrencyModel const& rbx::any_cast<RBX::DataModelArbiter::ConcurrencyModel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x473b80 — __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEED2Ev
// type: void
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEED2Ev
pub fn stub_473b80() -> ! {
    todo!("0x473b80 RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::~EnumDesc()")
}

// 0x473d54 — __ZN5boost10shared_ptrIN3RBX6Limits7CounterEEC2IS3_EEPT_
// type: void
#[doc(alias = "rbx_core::SharedPtr<RBX::Limits::Counter>::shared_ptr<RBX::Limits::Counter>(RBX::Limits::Counter *)")]
// was: __ZN5boost10shared_ptrIN3RBX6Limits7CounterEEC2IS3_EEPT_
pub fn stub_473d54() -> ! {
    todo!("0x473d54 boost::shared_ptr<RBX::Limits::Counter>::shared_ptr<RBX::Limits::Counter>(RBX::Limits::Counter *)")
}

// 0x473e2c — __ZN5boost6detail12shared_countC2IN3RBX6Limits7CounterEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Limits::Counter>(RBX::Limits::Counter *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX6Limits7CounterEEEPT_
pub fn stub_473e2c() -> ! {
    todo!("0x473e2c boost::detail::shared_count::shared_count<RBX::Limits::Counter>(RBX::Limits::Counter *)")
}

// 0x473f18 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED1Ev
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED1Ev
pub fn stub_473f18() -> ! {
    todo!("0x473f18 boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()")
}

// 0x473f1c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED0Ev
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED0Ev
pub fn stub_473f1c() -> ! {
    todo!("0x473f1c boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()")
}

// 0x473f20 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE7disposeEv
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE7disposeEv
pub fn stub_473f20() -> ! {
    todo!("0x473f20 boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::dispose(void)")
}

// 0x473f30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE11get_deleterERKSt9type_info
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE11get_deleterERKSt9type_info
pub fn stub_473f30() -> ! {
    todo!("0x473f30 boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_deleter(std::type_info const&)")
}

// 0x473f34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE19get_untyped_deleterEv
// type: void
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE19get_untyped_deleterEv
pub fn stub_473f34() -> ! {
    todo!("0x473f34 boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_untyped_deleter(void)")
}

// 0x473f38 — __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::resize(unsigned long,RBX::DataModelArbiter::ConcurrencyModel)")]
// was: __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE6resizeEmS2_
pub fn stub_473f38() -> ! {
    todo!("0x473f38 std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::resize(unsigned long,RBX::DataModelArbiter::ConcurrencyModel)")
}

// 0x473f70 — __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::push_back(RBX::DataModelArbiter::ConcurrencyModel const&)")]
// was: __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE9push_backERKS2_
pub fn stub_473f70() -> ! {
    todo!("0x473f70 std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::push_back(RBX::DataModelArbiter::ConcurrencyModel const&)")
}

// 0x473f98 — __ZNSt6vectorIPKN3RBX4NameESaIS3_EE6resizeEmS3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>::resize(unsigned long,RBX::Name const*)")]
// was: __ZNSt6vectorIPKN3RBX4NameESaIS3_EE6resizeEmS3_
pub fn stub_473f98() -> ! {
    todo!("0x473f98 std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>::resize(unsigned long,RBX::Name const*)")
}

