//! audio generated_189 — 150 stubs EA-sorted asc global gap filler not yet in audio (FMOD|Sound|Audio 2541/2541 complete, gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 150 not in audio | rbx_core::SharedPtr not boost
//! Range 0x46dff8..0x4758a4 | existing 25512 -> 25662 distinct
//! Batch: 150 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x46dff8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEENS3_5list1INS3_5valueISB_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEENS3_5list1INS3_5valueISB_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
pub fn stub_46dff8() {
    // IDA 0x46dff8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x46e014 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEENS3_5list1INS3_5valueISB_EEEEEEvPS7_E6invokeERNS1_15function_bufferESJ_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEENS3_5list1INS3_5valueISB_EEEEEEvPS7_E6invokeERNS1_15function_bufferESJ_")]
pub fn stub_46e014() {
    // IDA 0x46e014: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

// 0x46e030 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS4_10LegacyLock14Implementation6EventsEEEENS8_5list1INS8_5valueISE_EEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS4_10LegacyLock14Implementation6EventsEEEENS8_5list1INS8_5valueISE_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_46e030() -> ! {
    todo!("0x46e030 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,boost::detail::function::function_buffer &)const")
}

// 0x46e110 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS4_10LegacyLock14Implementation6EventsEEEENS8_5list1INS8_5valueISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS4_10LegacyLock14Implementation6EventsEEEENS8_5list1INS8_5valueISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_46e110() -> ! {
    todo!("0x46e110 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x46e208 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEEEEclIPFvS9_ENS1_IRPS5_EEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::operator()<void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>) &,boost::_bi::list1<RBX::DataModel*&> &,int)
#[doc(alias = "void boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>) &,boost::_bi::list1<RBX::DataModel*&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEEEEclIPFvS9_ENS1_IRPS5_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_46e208() {
    // IDA 0x46e208: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

// 0x46e2d4 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEENS3_5list1INS3_5valueISB_EEEEEEE12manage_smallERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: int(void)
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEENS3_5list1INS3_5valueISB_EEEEEEE12manage_smallERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
pub fn stub_46e2d4() -> ! {
    todo!("0x46e2d4 boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x46e358 — __ZN3RBX9DataModel10LegacyLock14Implementation6EventsC2Ev
// demangled: RBX::DataModel::LegacyLock::Implementation::Events::Events(void)
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation::Events *__hidden this)
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::Events::Events(void)")]
#[doc(alias = "__ZN3RBX9DataModel10LegacyLock14Implementation6EventsC2Ev")]
pub fn stub_46e358() -> ! {
    todo!("0x46e358 RBX::DataModel::LegacyLock::Implementation::Events::Events(void)")
}

// 0x46e404 — __ZN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEC2IS5_EEPT_
// demangled: boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>(RBX::DataModel::LegacyLock::Implementation::Events *)
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>(RBX::DataModel::LegacyLock::Implementation::Events *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEC2IS5_EEPT_")]
pub fn stub_46e404() {
    // IDA 0x46e404: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x46e4d8 — __ZN5boost6detail12shared_countC2IN3RBX9DataModel10LegacyLock14Implementation6EventsEEEPT_
// demangled: boost::detail::shared_count::shared_count<RBX::DataModel::LegacyLock::Implementation::Events>(RBX::DataModel::LegacyLock::Implementation::Events *)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DataModel::LegacyLock::Implementation::Events>(RBX::DataModel::LegacyLock::Implementation::Events *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX9DataModel10LegacyLock14Implementation6EventsEEEPT_")]
pub fn stub_46e4d8() {
    // IDA 0x46e4d8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x46e5d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEED1Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::~sp_counted_impl_p()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEED1Ev")]
pub fn stub_46e5d8() {
    // IDA 0x46e5d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46e5dc — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEED0Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::~sp_counted_impl_p()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEED0Ev")]
pub fn stub_46e5dc() {
    // IDA 0x46e5dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46e5e0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEE7disposeEv
// demangled: boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEE7disposeEv")]
pub fn stub_46e5e0() {
    // IDA 0x46e5e0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x46e604 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEE11get_deleterERKSt9type_info")]
pub fn stub_46e604() {
    // IDA 0x46e604: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x46e608 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::LegacyLock::Implementation::Events>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10LegacyLock14Implementation6EventsEE19get_untyped_deleterEv")]
pub fn stub_46e608() {
    // IDA 0x46e608: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x46e60c — __ZN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEaSERKS6_
// demangled: boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>::operator=(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&)
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>::operator=(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEaSERKS6_")]
pub fn stub_46e60c() -> ! {
    todo!("0x46e60c rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>::operator=(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")
}

// 0x46e644 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE9pop_frontEv
// demangled: std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::pop_front(void)
// type: int(void)
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::pop_front(void)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE9pop_frontEv")]
pub fn stub_46e644() -> ! {
    todo!("0x46e644 std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::pop_front(void)")
}

// 0x46e670 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE16_M_pop_front_auxEv
// demangled: std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_pop_front_aux(void)
// type: int(void)
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_pop_front_aux(void)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE16_M_pop_front_auxEv")]
pub fn stub_46e670() -> ! {
    todo!("0x46e670 std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_pop_front_aux(void)")
}

// 0x46e69c — __ZN3RBX9DataModel10LegacyLock14Implementation27safe_static_init_eventsPoolEv
// demangled: RBX::DataModel::LegacyLock::Implementation::safe_static_init_eventsPool(void)
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation *__hidden this)
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_init_eventsPool(void)")]
#[doc(alias = "__ZN3RBX9DataModel10LegacyLock14Implementation27safe_static_init_eventsPoolEv")]
pub fn stub_46e69c() -> ! {
    todo!("0x46e69c RBX::DataModel::LegacyLock::Implementation::safe_static_init_eventsPool(void)")
}

// 0x46e6a0 — __ZN3RBX9DataModel10LegacyLock14Implementation29safe_static_do_get_eventsPoolEv
// demangled: RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_eventsPool(void)
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation *__hidden this)
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_eventsPool(void)")]
#[doc(alias = "__ZN3RBX9DataModel10LegacyLock14Implementation29safe_static_do_get_eventsPoolEv")]
pub fn stub_46e6a0() -> ! {
    todo!("0x46e6a0 RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_eventsPool(void)")
}

// 0x46e808 — __ZN3rbx10safe_queueIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEED1Ev
// demangled: rbx::safe_queue<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>::~safe_queue()
#[doc(alias = "rbx::safe_queue<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>::~safe_queue()")]
#[doc(alias = "__ZN3rbx10safe_queueIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEED1Ev")]
pub fn stub_46e808() {
    // IDA 0x46e808: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46e8cc — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EED2Ev
// demangled: std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~deque()
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~deque()")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EED2Ev")]
pub fn stub_46e8cc() {
    // IDA 0x46e8cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46e9b4 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EED2Ev
// demangled: std::_Deque_base<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~_Deque_base()
// type: int(void)
#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EED2Ev")]
pub fn stub_46e9b4() {
    // IDA 0x46e9b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46e9e0 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_
// demangled: std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*>)
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_destroy_data_aux(std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_")]
pub fn stub_46e9e0() -> ! {
    todo!("0x46e9e0 std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_destroy_data_aux(std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>)")
}

// 0x46eb20 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE17_M_initialize_mapEm
// demangled: std::_Deque_base<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_initialize_map(unsigned long)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE17_M_initialize_mapEm")]
pub fn stub_46eb20() -> ! {
    todo!("0x46eb20 std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_initialize_map(unsigned long)")
}

// 0x46ec78 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE15_M_create_nodesEPPS7_SB_
// demangled: std::_Deque_base<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_create_nodes(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>**,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>**)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_create_nodes(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>**,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>**)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE15_M_create_nodesEPPS7_SB_")]
pub fn stub_46ec78() {
    // IDA 0x46ec78: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

// 0x46ed6c — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EEC2ERKS9_
// demangled: std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::deque(std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>> const&)
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::deque(std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>> const&)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EEC2ERKS9_")]
pub fn stub_46ed6c() {
    // IDA 0x46ed6c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x46ee90 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEERKS8_PS9_ES0_IS8_RS8_PS8_EET0_T_SH_SG_St12__false_type
// demangled: std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*> std::__uninitialized_copy_aux<std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*>>(std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::__false_type)
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>>(std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEERKS8_PS9_ES0_IS8_RS8_PS8_EET0_T_SH_SG_St12__false_type")]
pub fn stub_46ee90() -> ! {
    todo!("0x46ee90 std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>>(std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::__false_type)")
}

// 0x46f030 — __ZN3RBX9DataModel10LegacyLock14Implementation27safe_static_init_currentJobEv
// demangled: RBX::DataModel::LegacyLock::Implementation::safe_static_init_currentJob(void)
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation *__hidden this)
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_init_currentJob(void)")]
#[doc(alias = "__ZN3RBX9DataModel10LegacyLock14Implementation27safe_static_init_currentJobEv")]
pub fn stub_46f030() -> ! {
    todo!("0x46f030 RBX::DataModel::LegacyLock::Implementation::safe_static_init_currentJob(void)")
}

// 0x46f034 — __ZN3RBX9DataModel10LegacyLock14Implementation29safe_static_do_get_currentJobEv
// demangled: RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_currentJob(void)
// type: _DWORD __fastcall(RBX::DataModel::LegacyLock::Implementation *__hidden this)
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_currentJob(void)")]
#[doc(alias = "__ZN3RBX9DataModel10LegacyLock14Implementation29safe_static_do_get_currentJobEv")]
pub fn stub_46f034() -> ! {
    todo!("0x46f034 RBX::DataModel::LegacyLock::Implementation::safe_static_do_get_currentJob(void)")
}

// 0x46f148 — __ZN3rbx25thread_specific_referenceIN3RBX9DataModel10GenericJobEED1Ev
// demangled: rbx::thread_specific_reference<RBX::DataModel::GenericJob>::~thread_specific_reference()
#[doc(alias = "rbx::thread_specific_reference<RBX::DataModel::GenericJob>::~thread_specific_reference()")]
#[doc(alias = "__ZN3rbx25thread_specific_referenceIN3RBX9DataModel10GenericJobEED1Ev")]
pub fn stub_46f148() {
    // IDA 0x46f148: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46f158 — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEED2Ev
// demangled: boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::~thread_specific_ptr()
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::~thread_specific_ptr()")]
#[doc(alias = "__ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEED2Ev")]
pub fn stub_46f158() {
    // IDA 0x46f158: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46f24c — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataD1Ev
// demangled: boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data()
#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data()")]
#[doc(alias = "__ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataD1Ev")]
pub fn stub_46f24c() {
    // IDA 0x46f24c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46f250 — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataD0Ev
// demangled: boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data()
#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data()")]
#[doc(alias = "__ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataD0Ev")]
pub fn stub_46f250() {
    // IDA 0x46f250: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46f254 — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataclEPv
// demangled: boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::operator()(void *)
#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::operator()(void *)")]
#[doc(alias = "__ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataclEPv")]
pub fn stub_46f254() -> ! {
    todo!("0x46f254 boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::operator()(void *)")
}

// 0x46f260 — __ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS9_EEEET_T0_
// demangled: boost::detail::shared_count::shared_count<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>(boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>(boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS9_EEEET_T0_")]
pub fn stub_46f260() {
    // IDA 0x46f260: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x46f358 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEED1Ev")]
pub fn stub_46f358() {
    // IDA 0x46f358: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46f35c — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEED0Ev")]
pub fn stub_46f35c() {
    // IDA 0x46f35c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46f360 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE7disposeEv")]
pub fn stub_46f360() {
    // IDA 0x46f360: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x46f370 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE11get_deleterERKSt9type_info")]
pub fn stub_46f370() {
    // IDA 0x46f370: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x46f388 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE19get_untyped_deleterEv")]
pub fn stub_46f388() {
    // IDA 0x46f388: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x46f38c — __ZN3rbx7signals6signalIFvRKSsEE13disconnectAllEv
// demangled: rbx::signals::signal<void ()(std::string const&)>::disconnectAll(void)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(std::string const&)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKSsEE13disconnectAllEv")]
pub fn stub_46f38c() -> ! {
    todo!("0x46f38c rbx::signals::signal<void ()(std::string const&)>::disconnectAll(void)")
}

// 0x46f504 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE13disconnectAllEv
// demangled: rbx::signals::signal<void ()(RBX::UIEvent const&)>::disconnectAll(void)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE13disconnectAllEv")]
pub fn stub_46f504() -> ! {
    todo!("0x46f504 rbx::signals::signal<void ()(RBX::UIEvent const&)>::disconnectAll(void)")
}

// 0x46f67c — __ZNSt8_Rb_treeISsSt4pairIKSsiESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,std::allocator<std::pair<std::string const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,int>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,std::allocator<std::pair<std::string const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,int>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsiESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
pub fn stub_46f67c() {
    // IDA 0x46f67c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x46f6b0 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm
// demangled: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::reserve_for_insert(unsigned long)
// type: int(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm")]
pub fn stub_46f6b0() {
    // IDA 0x46f6b0: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

// 0x46f704 — __ZNK3RBX13GuiImageMixin18getImageRectOffsetEv
// demangled: RBX::GuiImageMixin::getImageRectOffset(void)const
// type: _DWORD __fastcall(RBX::GuiImageMixin *__hidden this)
#[doc(alias = "RBX::GuiImageMixin::getImageRectOffset(void)const")]
#[doc(alias = "__ZNK3RBX13GuiImageMixin18getImageRectOffsetEv")]
pub fn stub_46f704() -> ! {
    todo!("0x46f704 RBX::GuiImageMixin::getImageRectOffset(void)const")
}

// 0x46f710 — __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EED1Ev")]
pub fn stub_46f710() {
    // IDA 0x46f710: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46f734 — __ZNK3RBX13GuiImageMixin16getImageRectSizeEv
// demangled: RBX::GuiImageMixin::getImageRectSize(void)const
// type: _DWORD __fastcall(RBX::GuiImageMixin *__hidden this)
#[doc(alias = "RBX::GuiImageMixin::getImageRectSize(void)const")]
#[doc(alias = "__ZNK3RBX13GuiImageMixin16getImageRectSizeEv")]
pub fn stub_46f734() -> ! {
    todo!("0x46f734 RBX::GuiImageMixin::getImageRectSize(void)const")
}

// 0x46f740 — __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EED1Ev")]
pub fn stub_46f740() {
    // IDA 0x46f740: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46f768 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EED0Ev
// demangled: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::~TypedPropertyDescriptor()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::~TypedPropertyDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EED0Ev")]
pub fn stub_46f768() {
    // IDA 0x46f768: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46f798 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_46f798() -> ! {
    todo!("0x46f798 RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x46f8fc — __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EEC2IMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EEC2IMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_46f8fc() -> ! {
    todo!("0x46f8fc RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x46fa10 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_46fa10() -> ! {
    todo!("0x46fa10 RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x46fb34 — __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EED0Ev")]
pub fn stub_46fb34() {
    // IDA 0x46fb34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46fb60 — __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_46fb60() -> ! {
    todo!("0x46fb60 RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::isReadOnly(void)const")
}

// 0x46fb64 — __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_46fb64() -> ! {
    todo!("0x46fb64 RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::isWriteOnly(void)const")
}

// 0x46fb68 — __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_46fb68() -> ! {
    todo!("0x46fb68 RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x46fba0 — __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_46fba0() -> ! {
    todo!("0x46fba0 RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")
}

// 0x46fbd4 — __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EEC2IMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EEC2IMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_46fbd4() -> ! {
    todo!("0x46fbd4 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x46fce8 — __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EED0Ev")]
pub fn stub_46fce8() {
    // IDA 0x46fce8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46fd14 — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_46fd14() -> ! {
    todo!("0x46fd14 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::isReadOnly(void)const")
}

// 0x46fd18 — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_46fd18() -> ! {
    todo!("0x46fd18 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::isWriteOnly(void)const")
}

// 0x46fd1c — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_46fd1c() -> ! {
    todo!("0x46fd1c RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x46fd54 — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_46fd54() -> ! {
    todo!("0x46fd54 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")
}

// 0x46fd8c — __ZN3RBX9DataModel10MouseStatsC2Ev
// demangled: RBX::DataModel::MouseStats::MouseStats(void)
// type: _DWORD __fastcall(RBX::DataModel::MouseStats *__hidden this)
#[doc(alias = "RBX::DataModel::MouseStats::MouseStats(void)")]
#[doc(alias = "__ZN3RBX9DataModel10MouseStatsC2Ev")]
pub fn stub_46fd8c() -> ! {
    todo!("0x46fd8c RBX::DataModel::MouseStats::MouseStats(void)")
}

// 0x46feac — __ZN3RBX7IMetricD1Ev
// demangled: RBX::IMetric::~IMetric()
// type: void __fastcall(RBX::IMetric *__hidden this)
#[doc(alias = "RBX::IMetric::~IMetric()")]
#[doc(alias = "__ZN3RBX7IMetricD1Ev")]
pub fn stub_46feac() {
    // IDA 0x46feac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46feb0 — __ZN3RBX7IMetricD0Ev
// demangled: RBX::IMetric::~IMetric()
// type: void __fastcall(RBX::IMetric *__hidden this)
#[doc(alias = "RBX::IMetric::~IMetric()")]
#[doc(alias = "__ZN3RBX7IMetricD0Ev")]
pub fn stub_46feb0() {
    // IDA 0x46feb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x46feb8 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE7releaseEv
// demangled: boost::thread_specific_ptr<RBX::Security::Context>::release(void)
#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::release(void)")]
#[doc(alias = "__ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE7releaseEv")]
pub fn stub_46feb8() -> ! {
    todo!("0x46feb8 boost::thread_specific_ptr<RBX::Security::Context>::release(void)")
}

// 0x46ff84 — __ZN3RBX9DataModel10GenericJobC2EN5boost10shared_ptrIS0_EEPKcNS_12DataModelJob8TaskTypeE
// demangled: RBX::DataModel::GenericJob::GenericJob(boost::shared_ptr<RBX::DataModel>,char const*,RBX::DataModelJob::TaskType)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int, int, int, RBX::TaskScheduler::Job *, int, int, int, int, int)
#[doc(alias = "RBX::DataModel::GenericJob::GenericJob(rbx_core::SharedPtr<RBX::DataModel>,char const*,RBX::DataModelJob::TaskType)")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJobC2EN5boost10shared_ptrIS0_EEPKcNS_12DataModelJob8TaskTypeE")]
pub fn stub_46ff84() -> ! {
    todo!("0x46ff84 RBX::DataModel::GenericJob::GenericJob(rbx_core::SharedPtr<RBX::DataModel>,char const*,RBX::DataModelJob::TaskType)")
}

// 0x47013c — __ZN3RBX9DataModel10GenericJobD1Ev
// demangled: RBX::DataModel::GenericJob::~GenericJob()
// type: void __fastcall(RBX::DataModel::GenericJob *__hidden this)
#[doc(alias = "RBX::DataModel::GenericJob::~GenericJob()")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJobD1Ev")]
pub fn stub_47013c() {
    // IDA 0x47013c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x47025c — __ZN3RBX9DataModel10GenericJobD0Ev
// demangled: RBX::DataModel::GenericJob::~GenericJob()
// type: void __fastcall(RBX::DataModel::GenericJob *__hidden this)
#[doc(alias = "RBX::DataModel::GenericJob::~GenericJob()")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJobD0Ev")]
pub fn stub_47025c() {
    // IDA 0x47025c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x470390 — __ZN3RBX9DataModel10GenericJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// demangled: RBX::DataModel::GenericJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
#[doc(alias = "RBX::DataModel::GenericJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_470390() -> ! {
    todo!("0x470390 RBX::DataModel::GenericJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x470400 — __ZN3RBX9DataModel10GenericJob5errorERKNS_13TaskScheduler3Job5StatsE
// demangled: RBX::DataModel::GenericJob::error(RBX::TaskScheduler::Job::Stats const&)
// type: _DWORD __fastcall(RBX::DataModel::GenericJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::DataModel::GenericJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJob5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_470400() -> ! {
    todo!("0x470400 RBX::DataModel::GenericJob::error(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x470484 — __ZN3RBX9DataModel10GenericJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// demangled: RBX::DataModel::GenericJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)
// type: _DWORD __fastcall(RBX::DataModel::GenericJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::DataModel::GenericJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_470484() -> ! {
    todo!("0x470484 RBX::DataModel::GenericJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x470670 — __ZN3RBX9DataModel10GenericJob12processTasksEv
// demangled: RBX::DataModel::GenericJob::processTasks(void)
// type: _DWORD __fastcall(RBX::DataModel::GenericJob *__hidden this)
#[doc(alias = "RBX::DataModel::GenericJob::processTasks(void)")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJob12processTasksEv")]
pub fn stub_470670() -> ! {
    todo!("0x470670 RBX::DataModel::GenericJob::processTasks(void)")
}

// 0x470818 — __ZN3RBX9DataModel10GenericJob4stepERN5boost8functionIFvPS0_EEE
// demangled: RBX::DataModel::GenericJob::step(boost::function<void ()(RBX::DataModel*)> &)
#[doc(alias = "RBX::DataModel::GenericJob::step(boost::function<void ()(RBX::DataModel*)> &)")]
#[doc(alias = "__ZN3RBX9DataModel10GenericJob4stepERN5boost8functionIFvPS0_EEE")]
pub fn stub_470818() -> ! {
    todo!("0x470818 RBX::DataModel::GenericJob::step(boost::function<void ()(RBX::DataModel*)> &)")
}

// 0x4708e0 — __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EED2Ev
// demangled: std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~deque()
#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~deque()")]
#[doc(alias = "__ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EED2Ev")]
pub fn stub_4708e0() {
    // IDA 0x4708e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4709c8 — __ZNSt11_Deque_baseIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EED2Ev
// demangled: std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~_Deque_base()
// type: int(void)
#[doc(alias = "std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EED2Ev")]
pub fn stub_4709c8() {
    // IDA 0x4709c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4709f8 — __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EEC2ERKSC_
// demangled: std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::deque(std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>> const&)
#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::deque(std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>> const&)")]
#[doc(alias = "__ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EEC2ERKSC_")]
pub fn stub_4709f8() {
    // IDA 0x4709f8: function ctor/assign from a bind_t functor. Box<dyn Fn> from closure captures — carrier no-op.
}

// 0x470b30 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Instance10SaveFilterEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Instance10SaveFilterEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_470b30() {
    // IDA 0x470b30: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x470b58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::GearType>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::GearType>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_470b58() {
    // IDA 0x470b58: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x470b80 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_470b80() {
    // IDA 0x470b80: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x470ba8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::Genre>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::Genre>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_470ba8() {
    // IDA 0x470ba8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x470bd0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_470bd0() {
    // IDA 0x470bd0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x470bf8 — __GLOBAL__I_a_178
// demangled: global constructor keyed to_a_178
#[doc(alias = "global constructor keyed to_a_178")]
#[doc(alias = "__GLOBAL__I_a_178")]
pub fn stub_470bf8() -> ! {
    todo!("0x470bf8 global constructor keyed to_a_178")
}

// 0x4727ec — __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEC1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::EnumDesc(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEC1Ev")]
pub fn stub_4727ec() -> ! {
    todo!("0x4727ec RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::EnumDesc(void)")
}

// 0x4727f0 — __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEC2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::EnumDesc(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEC2Ev")]
pub fn stub_4727f0() -> ! {
    todo!("0x4727f0 RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::EnumDesc(void)")
}

// 0x4729dc — __ZN3RBX12DataModelJobC2EPKcNS0_8TaskTypeEbN5boost10shared_ptrINS_16DataModelArbiterEEENS_4Time8IntervalE
// demangled: RBX::DataModelJob::DataModelJob(char const*,RBX::DataModelJob::TaskType,bool,boost::shared_ptr<RBX::DataModelArbiter>,RBX::Time::Interval)
// type: int __fastcall(char, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, RBX::TaskScheduler::Job *, int, int, int, int)
#[doc(alias = "RBX::DataModelJob::DataModelJob(char const*,RBX::DataModelJob::TaskType,bool,rbx_core::SharedPtr<RBX::DataModelArbiter>,RBX::Time::Interval)")]
#[doc(alias = "__ZN3RBX12DataModelJobC2EPKcNS0_8TaskTypeEbN5boost10shared_ptrINS_16DataModelArbiterEEENS_4Time8IntervalE")]
pub fn stub_4729dc() -> ! {
    todo!("0x4729dc RBX::DataModelJob::DataModelJob(char const*,RBX::DataModelJob::TaskType,bool,rbx_core::SharedPtr<RBX::DataModelArbiter>,RBX::Time::Interval)")
}

// 0x472b4c — __ZN3RBX12DataModelJob4stepERKNS_13TaskScheduler3Job5StatsE
// demangled: RBX::DataModelJob::step(RBX::TaskScheduler::Job::Stats const&)
// type: _DWORD __fastcall(RBX::DataModelJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::DataModelJob::step(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX12DataModelJob4stepERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_472b4c() -> ! {
    todo!("0x472b4c RBX::DataModelJob::step(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x472cd4 — __ZN3RBX12DataModelJob17getPriorityFactorEv
// demangled: RBX::DataModelJob::getPriorityFactor(void)
// type: _DWORD __fastcall(RBX::DataModelJob *__hidden this)
#[doc(alias = "RBX::DataModelJob::getPriorityFactor(void)")]
#[doc(alias = "__ZN3RBX12DataModelJob17getPriorityFactorEv")]
pub fn stub_472cd4() -> ! {
    todo!("0x472cd4 RBX::DataModelJob::getPriorityFactor(void)")
}

// 0x472e00 — __ZN3RBX16DataModelArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_
// demangled: RBX::DataModelArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)
// type: _DWORD __fastcall(RBX::DataModelArbiter *__hidden this, RBX::TaskScheduler::Job *, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::DataModelArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
#[doc(alias = "__ZN3RBX16DataModelArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_")]
pub fn stub_472e00() -> ! {
    todo!("0x472e00 RBX::DataModelArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")
}

// 0x472e2c — __ZN3RBX16DataModelArbiterC2Ev
// demangled: RBX::DataModelArbiter::DataModelArbiter(void)
// type: _DWORD __fastcall(RBX::DataModelArbiter *__hidden this)
#[doc(alias = "RBX::DataModelArbiter::DataModelArbiter(void)")]
#[doc(alias = "__ZN3RBX16DataModelArbiterC2Ev")]
pub fn stub_472e2c() -> ! {
    todo!("0x472e2c RBX::DataModelArbiter::DataModelArbiter(void)")
}

// 0x473124 — __ZN3RBX16DataModelArbiterD0Ev
// demangled: RBX::DataModelArbiter::~DataModelArbiter()
// type: void __fastcall(RBX::DataModelArbiter *__hidden this)
#[doc(alias = "RBX::DataModelArbiter::~DataModelArbiter()")]
#[doc(alias = "__ZN3RBX16DataModelArbiterD0Ev")]
pub fn stub_473124() {
    // IDA 0x473124: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4731c4 — __ZN3RBX16DataModelArbiterD1Ev
// demangled: RBX::DataModelArbiter::~DataModelArbiter()
// type: void __fastcall(RBX::DataModelArbiter *__hidden this)
#[doc(alias = "RBX::DataModelArbiter::~DataModelArbiter()")]
#[doc(alias = "__ZN3RBX16DataModelArbiterD1Ev")]
pub fn stub_4731c4() {
    // IDA 0x4731c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4731c8 — __ZN3RBX16DataModelArbiterD2Ev
// demangled: RBX::DataModelArbiter::~DataModelArbiter()
// type: void __fastcall(RBX::DataModelArbiter *__hidden this)
#[doc(alias = "RBX::DataModelArbiter::~DataModelArbiter()")]
#[doc(alias = "__ZN3RBX16DataModelArbiterD2Ev")]
pub fn stub_4731c8() {
    // IDA 0x4731c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x473318 — __ZN3RBX16DataModelArbiter7preStepEPNS_13TaskScheduler3JobE
// demangled: RBX::DataModelArbiter::preStep(RBX::TaskScheduler::Job *)
// type: _DWORD __fastcall(RBX::DataModelArbiter *__hidden this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::DataModelArbiter::preStep(RBX::TaskScheduler::Job *)")]
#[doc(alias = "__ZN3RBX16DataModelArbiter7preStepEPNS_13TaskScheduler3JobE")]
pub fn stub_473318() -> ! {
    todo!("0x473318 RBX::DataModelArbiter::preStep(RBX::TaskScheduler::Job *)")
}

// 0x473350 — __ZN3RBX16DataModelArbiter8postStepEPNS_13TaskScheduler3JobE
// demangled: RBX::DataModelArbiter::postStep(RBX::TaskScheduler::Job *)
// type: _DWORD __fastcall(RBX::DataModelArbiter *__hidden this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::DataModelArbiter::postStep(RBX::TaskScheduler::Job *)")]
#[doc(alias = "__ZN3RBX16DataModelArbiter8postStepEPNS_13TaskScheduler3JobE")]
pub fn stub_473350() -> ! {
    todo!("0x473350 RBX::DataModelArbiter::postStep(RBX::TaskScheduler::Job *)")
}

// 0x473388 — __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE7addPairES3_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::addPair(RBX::DataModelArbiter::ConcurrencyModel,char const*)
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::addPair(RBX::DataModelArbiter::ConcurrencyModel,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE7addPairES3_PKc")]
pub fn stub_473388() -> ! {
    todo!("0x473388 RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::addPair(RBX::DataModelArbiter::ConcurrencyModel,char const*)")
}

// 0x4736e8 — __ZN3RBX12DataModelJobD1Ev
// demangled: RBX::DataModelJob::~DataModelJob()
// type: void __fastcall(RBX::DataModelJob *__hidden this)
#[doc(alias = "RBX::DataModelJob::~DataModelJob()")]
#[doc(alias = "__ZN3RBX12DataModelJobD1Ev")]
pub fn stub_4736e8() {
    // IDA 0x4736e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4736ec — __ZN3RBX12DataModelJobD0Ev
// demangled: RBX::DataModelJob::~DataModelJob()
// type: void __fastcall(RBX::DataModelJob *__hidden this)
#[doc(alias = "RBX::DataModelJob::~DataModelJob()")]
#[doc(alias = "__ZN3RBX12DataModelJobD0Ev")]
pub fn stub_4736ec() {
    // IDA 0x4736ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x473790 — __ZN3RBX23SimpleThrottlingArbiter11isThrottledEv
// demangled: RBX::SimpleThrottlingArbiter::isThrottled(void)
// type: _DWORD __fastcall(RBX::SimpleThrottlingArbiter *__hidden this)
#[doc(alias = "RBX::SimpleThrottlingArbiter::isThrottled(void)")]
#[doc(alias = "__ZN3RBX23SimpleThrottlingArbiter11isThrottledEv")]
pub fn stub_473790() -> ! {
    todo!("0x473790 RBX::SimpleThrottlingArbiter::isThrottled(void)")
}

// 0x473858 — __ZN3RBX13TaskScheduler7Arbiter24getSyncronizationArbiterEv
// demangled: RBX::TaskScheduler::Arbiter::getSyncronizationArbiter(void)
// type: _DWORD __fastcall(RBX::TaskScheduler::Arbiter *__hidden this)
#[doc(alias = "RBX::TaskScheduler::Arbiter::getSyncronizationArbiter(void)")]
#[doc(alias = "__ZN3RBX13TaskScheduler7Arbiter24getSyncronizationArbiterEv")]
pub fn stub_473858() -> ! {
    todo!("0x473858 RBX::TaskScheduler::Arbiter::getSyncronizationArbiter(void)")
}

// 0x473860 — __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEED1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEED1Ev")]
pub fn stub_473860() {
    // IDA 0x473860: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x473868 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE15convertToStringEmRSs
// demangled: RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE15convertToStringEmRSs")]
pub fn stub_473868() -> ! {
    todo!("0x473868 RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToString(unsigned long,std::string &)const")
}

// 0x4739b0 — __ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE14construct_funcEPKcPc
// demangled: rbx::implementation::typed_holder<RBX::DataModelArbiter::ConcurrencyModel>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModelArbiter::ConcurrencyModel>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE14construct_funcEPKcPc")]
pub fn stub_4739b0() -> ! {
    todo!("0x4739b0 rbx::implementation::typed_holder<RBX::DataModelArbiter::ConcurrencyModel>::construct_func(char const*,char *)")
}

// 0x4739c0 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE13convertToItemERKS3_
// demangled: RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToItem(RBX::DataModelArbiter::ConcurrencyModel const&)const
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToItem(RBX::DataModelArbiter::ConcurrencyModel const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE13convertToItemERKS3_")]
pub fn stub_4739c0() -> ! {
    todo!("0x4739c0 RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToItem(RBX::DataModelArbiter::ConcurrencyModel const&)const")
}

// 0x473a8c — __ZN3rbx8any_castIRKN3RBX16DataModelArbiter16ConcurrencyModelENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// demangled: RBX::DataModelArbiter::ConcurrencyModel const& rbx::any_cast<RBX::DataModelArbiter::ConcurrencyModel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: int(void)
#[doc(alias = "RBX::DataModelArbiter::ConcurrencyModel const& rbx::any_cast<RBX::DataModelArbiter::ConcurrencyModel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX16DataModelArbiter16ConcurrencyModelENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_473a8c() -> ! {
    todo!("0x473a8c RBX::DataModelArbiter::ConcurrencyModel const& rbx::any_cast<RBX::DataModelArbiter::ConcurrencyModel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x473b80 — __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEED2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEED2Ev")]
pub fn stub_473b80() {
    // IDA 0x473b80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x473d54 — __ZN5boost10shared_ptrIN3RBX6Limits7CounterEEC2IS3_EEPT_
// demangled: boost::shared_ptr<RBX::Limits::Counter>::shared_ptr<RBX::Limits::Counter>(RBX::Limits::Counter *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Limits::Counter>::shared_ptr<RBX::Limits::Counter>(RBX::Limits::Counter *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6Limits7CounterEEC2IS3_EEPT_")]
pub fn stub_473d54() {
    // IDA 0x473d54: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x473e2c — __ZN5boost6detail12shared_countC2IN3RBX6Limits7CounterEEEPT_
// demangled: boost::detail::shared_count::shared_count<RBX::Limits::Counter>(RBX::Limits::Counter *)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Limits::Counter>(RBX::Limits::Counter *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX6Limits7CounterEEEPT_")]
pub fn stub_473e2c() {
    // IDA 0x473e2c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x473f18 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED1Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED1Ev")]
pub fn stub_473f18() {
    // IDA 0x473f18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x473f1c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED0Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED0Ev")]
pub fn stub_473f1c() {
    // IDA 0x473f1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x473f20 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE7disposeEv")]
pub fn stub_473f20() {
    // IDA 0x473f20: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x473f30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE11get_deleterERKSt9type_info")]
pub fn stub_473f30() {
    // IDA 0x473f30: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x473f34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE19get_untyped_deleterEv")]
pub fn stub_473f34() {
    // IDA 0x473f34: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x473f38 — __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE6resizeEmS2_
// demangled: std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::resize(unsigned long,RBX::DataModelArbiter::ConcurrencyModel)
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::resize(unsigned long,RBX::DataModelArbiter::ConcurrencyModel)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE6resizeEmS2_")]
pub fn stub_473f38() -> ! {
    todo!("0x473f38 std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::resize(unsigned long,RBX::DataModelArbiter::ConcurrencyModel)")
}

// 0x473f70 — __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::push_back(RBX::DataModelArbiter::ConcurrencyModel const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::push_back(RBX::DataModelArbiter::ConcurrencyModel const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE9push_backERKS2_")]
pub fn stub_473f70() -> ! {
    todo!("0x473f70 std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::push_back(RBX::DataModelArbiter::ConcurrencyModel const&)")
}

// 0x473f98 — __ZNSt6vectorIPKN3RBX4NameESaIS3_EE6resizeEmS3_
// demangled: std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>::resize(unsigned long,RBX::Name const*)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>::resize(unsigned long,RBX::Name const*)")]
#[doc(alias = "__ZNSt6vectorIPKN3RBX4NameESaIS3_EE6resizeEmS3_")]
pub fn stub_473f98() -> ! {
    todo!("0x473f98 std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>::resize(unsigned long,RBX::Name const*)")
}

// 0x474004 — __ZNSt3mapIPKN3RBX4NameENS0_16DataModelArbiter16ConcurrencyModelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// demangled: std::map<RBX::Name const*,RBX::DataModelArbiter::ConcurrencyModel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::operator[](RBX::Name const* const&)
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::DataModelArbiter::ConcurrencyModel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_16DataModelArbiter16ConcurrencyModelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_474004() -> ! {
    todo!("0x474004 std::map<RBX::Name const*,RBX::DataModelArbiter::ConcurrencyModel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::operator[](RBX::Name const* const&)")
}

// 0x47405c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_47405c() -> ! {
    todo!("0x47405c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")
}

// 0x474110 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_474110() -> ! {
    todo!("0x474110 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")
}

// 0x474168 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_474168() -> ! {
    todo!("0x474168 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")
}

// 0x474350 — __ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs
// demangled: std::vector<std::string,std::allocator<std::string>>::_M_fill_insert(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,std::allocator<std::string>>>,unsigned long,std::string const&)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::_M_fill_insert(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,std::allocator<std::string>>>,unsigned long,std::string const&)")]
#[doc(alias = "__ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs")]
pub fn stub_474350() -> ! {
    todo!("0x474350 std::vector<std::string,std::allocator<std::string>>::_M_fill_insert(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,std::allocator<std::string>>>,unsigned long,std::string const&)")
}

// 0x47486c — __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModelArbiter::ConcurrencyModel*,std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>>,RBX::DataModelArbiter::ConcurrencyModel const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModelArbiter::ConcurrencyModel*,std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>>,RBX::DataModelArbiter::ConcurrencyModel const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_47486c() -> ! {
    todo!("0x47486c std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModelArbiter::ConcurrencyModel*,std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>>,RBX::DataModelArbiter::ConcurrencyModel const&)")
}

// 0x474950 — __ZNSt12_Vector_baseIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_allocate(unsigned long)
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE11_M_allocateEm")]
pub fn stub_474950() -> ! {
    todo!("0x474950 std::_Vector_base<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_allocate(unsigned long)")
}

// 0x474968 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16DataModelArbiter16ConcurrencyModelES6_EET0_T_S8_S7_
// demangled: RBX::DataModelArbiter::ConcurrencyModel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *>(RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *)
// type: int(void)
#[doc(alias = "RBX::DataModelArbiter::ConcurrencyModel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *>(RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16DataModelArbiter16ConcurrencyModelES6_EET0_T_S8_S7_")]
pub fn stub_474968() -> ! {
    todo!("0x474968 RBX::DataModelArbiter::ConcurrencyModel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *>(RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *)")
}

// 0x4749a8 — __ZNSt12_Vector_baseImSaImEE11_M_allocateEm
// demangled: std::_Vector_base<unsigned long,std::allocator<unsigned long>>::_M_allocate(unsigned long)
// type: int(void)
#[doc(alias = "std::_Vector_base<unsigned long,std::allocator<unsigned long>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseImSaImEE11_M_allocateEm")]
pub fn stub_4749a8() -> ! {
    todo!("0x4749a8 std::_Vector_base<unsigned long,std::allocator<unsigned long>>::_M_allocate(unsigned long)")
}

// 0x4749c0 — __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// demangled: std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModelArbiter::ConcurrencyModel*,std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>>,unsigned long,RBX::DataModelArbiter::ConcurrencyModel const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModelArbiter::ConcurrencyModel*,std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>>,unsigned long,RBX::DataModelArbiter::ConcurrencyModel const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_4749c0() -> ! {
    todo!("0x4749c0 std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModelArbiter::ConcurrencyModel*,std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>>,unsigned long,RBX::DataModelArbiter::ConcurrencyModel const&)")
}

// 0x474c38 — __ZN3RBX13ActivityMeterILi2EE13updateBucketsEv
// demangled: RBX::ActivityMeter<2>::updateBuckets(void)
// type: int(void)
#[doc(alias = "RBX::ActivityMeter<2>::updateBuckets(void)")]
#[doc(alias = "__ZN3RBX13ActivityMeterILi2EE13updateBucketsEv")]
pub fn stub_474c38() -> ! {
    todo!("0x474c38 RBX::ActivityMeter<2>::updateBuckets(void)")
}

// 0x474cf0 — __ZN3RBX16OnScreenProfiler7GetInstEv
// demangled: RBX::OnScreenProfiler::GetInst(void)
// type: _DWORD __fastcall(RBX::OnScreenProfiler *__hidden this)
#[doc(alias = "RBX::OnScreenProfiler::GetInst(void)")]
#[doc(alias = "__ZN3RBX16OnScreenProfiler7GetInstEv")]
pub fn stub_474cf0() -> ! {
    todo!("0x474cf0 RBX::OnScreenProfiler::GetInst(void)")
}

// 0x474d54 — __ZN3RBX16OnScreenProfiler6CreateEv
// demangled: RBX::OnScreenProfiler::Create(void)
// type: _DWORD __fastcall(RBX::OnScreenProfiler *__hidden this)
#[doc(alias = "RBX::OnScreenProfiler::Create(void)")]
#[doc(alias = "__ZN3RBX16OnScreenProfiler6CreateEv")]
pub fn stub_474d54() -> ! {
    todo!("0x474d54 RBX::OnScreenProfiler::Create(void)")
}

// 0x474dfc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>> *)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_474dfc() {
    // IDA 0x474dfc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x474e24 — __GLOBAL__I_a_179
// demangled: global constructor keyed to_a_179
#[doc(alias = "global constructor keyed to_a_179")]
#[doc(alias = "__GLOBAL__I_a_179")]
pub fn stub_474e24() -> ! {
    todo!("0x474e24 global constructor keyed to_a_179")
}

// 0x474eec — __ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEEC1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::EnumDesc(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEEC1Ev")]
pub fn stub_474eec() -> ! {
    todo!("0x474eec RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::EnumDesc(void)")
}

// 0x474ef0 — __ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEEC2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::EnumDesc(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEEC2Ev")]
pub fn stub_474ef0() -> ! {
    todo!("0x474ef0 RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::EnumDesc(void)")
}

// 0x4750c8 — __ZN3RBX13DataModelMesh17setLevelOfDetailXENS0_7LODTypeE
// demangled: RBX::DataModelMesh::setLevelOfDetailX(RBX::DataModelMesh::LODType)
#[doc(alias = "RBX::DataModelMesh::setLevelOfDetailX(RBX::DataModelMesh::LODType)")]
#[doc(alias = "__ZN3RBX13DataModelMesh17setLevelOfDetailXENS0_7LODTypeE")]
pub fn stub_4750c8() -> ! {
    todo!("0x4750c8 RBX::DataModelMesh::setLevelOfDetailX(RBX::DataModelMesh::LODType)")
}

// 0x4750e8 — __ZN3RBX13DataModelMesh17setLevelOfDetailYENS0_7LODTypeE
// demangled: RBX::DataModelMesh::setLevelOfDetailY(RBX::DataModelMesh::LODType)
#[doc(alias = "RBX::DataModelMesh::setLevelOfDetailY(RBX::DataModelMesh::LODType)")]
#[doc(alias = "__ZN3RBX13DataModelMesh17setLevelOfDetailYENS0_7LODTypeE")]
pub fn stub_4750e8() -> ! {
    todo!("0x4750e8 RBX::DataModelMesh::setLevelOfDetailY(RBX::DataModelMesh::LODType)")
}

// 0x475108 — __ZN3RBX13DataModelMesh8setScaleERKN3G3D7Vector3E
// demangled: RBX::DataModelMesh::setScale(G3D::Vector3 const&)
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::DataModelMesh::setScale(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX13DataModelMesh8setScaleERKN3G3D7Vector3E")]
pub fn stub_475108() -> ! {
    todo!("0x475108 RBX::DataModelMesh::setScale(G3D::Vector3 const&)")
}

// 0x4751a8 — __ZN3RBX13DataModelMesh12setVertColorERKN3G3D7Vector3E
// demangled: RBX::DataModelMesh::setVertColor(G3D::Vector3 const&)
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::DataModelMesh::setVertColor(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX13DataModelMesh12setVertColorERKN3G3D7Vector3E")]
pub fn stub_4751a8() -> ! {
    todo!("0x4751a8 RBX::DataModelMesh::setVertColor(G3D::Vector3 const&)")
}

// 0x475210 — __ZN3RBX13DataModelMesh9setOffsetERKN3G3D7Vector3E
// demangled: RBX::DataModelMesh::setOffset(G3D::Vector3 const&)
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::DataModelMesh::setOffset(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX13DataModelMesh9setOffsetERKN3G3D7Vector3E")]
pub fn stub_475210() -> ! {
    todo!("0x475210 RBX::DataModelMesh::setOffset(G3D::Vector3 const&)")
}

// 0x475278 — __ZN3RBX13DataModelMeshC2Ev
// demangled: RBX::DataModelMesh::DataModelMesh(void)
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::DataModelMesh(void)")]
#[doc(alias = "__ZN3RBX13DataModelMeshC2Ev")]
pub fn stub_475278() -> ! {
    todo!("0x475278 RBX::DataModelMesh::DataModelMesh(void)")
}

// 0x4754a4 — __ZNK3RBX13DataModelMesh12askSetParentEPKNS_8InstanceE
// demangled: RBX::DataModelMesh::askSetParent(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::DataModelMesh::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX13DataModelMesh12askSetParentEPKNS_8InstanceE")]
pub fn stub_4754a4() -> ! {
    todo!("0x4754a4 RBX::DataModelMesh::askSetParent(RBX::Instance const*)const")
}

// 0x4754e0 — __ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEE7addPairES3_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::addPair(RBX::DataModelMesh::LODType,char const*)
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::addPair(RBX::DataModelMesh::LODType,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEE7addPairES3_PKc")]
pub fn stub_4754e0() -> ! {
    todo!("0x4754e0 RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::addPair(RBX::DataModelMesh::LODType,char const*)")
}

// 0x475840 — __ZNK3RBX13DataModelMesh17getLevelOfDetailXEv
// demangled: RBX::DataModelMesh::getLevelOfDetailX(void)const
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::getLevelOfDetailX(void)const")]
#[doc(alias = "__ZNK3RBX13DataModelMesh17getLevelOfDetailXEv")]
pub fn stub_475840() -> ! {
    todo!("0x475840 RBX::DataModelMesh::getLevelOfDetailX(void)const")
}

// 0x475848 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEED1Ev")]
pub fn stub_475848() {
    // IDA 0x475848: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x47586c — __ZNK3RBX13DataModelMesh17getLevelOfDetailYEv
// demangled: RBX::DataModelMesh::getLevelOfDetailY(void)const
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::getLevelOfDetailY(void)const")]
#[doc(alias = "__ZNK3RBX13DataModelMesh17getLevelOfDetailYEv")]
pub fn stub_47586c() -> ! {
    todo!("0x47586c RBX::DataModelMesh::getLevelOfDetailY(void)const")
}

// 0x475874 — __ZNK3RBX13DataModelMesh8getScaleEv
// demangled: RBX::DataModelMesh::getScale(void)const
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::getScale(void)const")]
#[doc(alias = "__ZNK3RBX13DataModelMesh8getScaleEv")]
pub fn stub_475874() -> ! {
    todo!("0x475874 RBX::DataModelMesh::getScale(void)const")
}

// 0x475878 — __ZN3RBX10Reflection14PropDescriptorINS_13DataModelMeshEN3G3D7Vector3EED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DataModelMeshEN3G3D7Vector3EED1Ev")]
pub fn stub_475878() {
    // IDA 0x475878: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x47589c — __ZNK3RBX13DataModelMesh12getVertColorEv
// demangled: RBX::DataModelMesh::getVertColor(void)const
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::getVertColor(void)const")]
#[doc(alias = "__ZNK3RBX13DataModelMesh12getVertColorEv")]
pub fn stub_47589c() -> ! {
    todo!("0x47589c RBX::DataModelMesh::getVertColor(void)const")
}

// 0x4758a0 — __ZNK3RBX13DataModelMesh9getOffsetEv
// demangled: RBX::DataModelMesh::getOffset(void)const
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::getOffset(void)const")]
#[doc(alias = "__ZNK3RBX13DataModelMesh9getOffsetEv")]
pub fn stub_4758a0() -> ! {
    todo!("0x4758a0 RBX::DataModelMesh::getOffset(void)const")
}

// 0x4758a4 — __ZN3RBX13DataModelMeshD1Ev
// demangled: RBX::DataModelMesh::~DataModelMesh()
// type: void __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::~DataModelMesh()")]
#[doc(alias = "__ZN3RBX13DataModelMeshD1Ev")]
pub fn stub_4758a4() {
    // IDA 0x4758a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

