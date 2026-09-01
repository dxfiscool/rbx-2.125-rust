//! platform generated_191 — next 120 stubs EA-sorted ascending
//! Filter: ViewController|UIApplication|Platform|iOS strict (1119 total, 0 uncovered) + global filler EA-sorted ascending (next 120 after 0xf28934) | rbx_core::SharedPtr not boost
//! Batch: 120 stubs | range 0xf28944..0xf291c4 | rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0xf28944 — j___ZN5boost6detail12shared_countC2IN3RBX13TaskScheduler6ThreadEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskScheduler::Thread>(RBX::TaskScheduler::Thread *)")]
pub fn stub_f28944() -> ! {
    todo!("0xf28944 boost::detail::shared_count::shared_count<RBX::TaskScheduler::Thread>(RBX::TaskScheduler::Thread *)")
}

// 0xf28954 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_f28954() -> ! {
    todo!("0xf28954 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf28964 — j___ZN5boost6thread10timed_joinERKNS_10posix_time5ptimeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::thread::timed_join(boost::posix_time::ptime const&)")]
pub fn stub_f28964() -> ! {
    todo!("0xf28964 boost::thread::timed_join(boost::posix_time::ptime const&)")
}

// 0xf28974 — j___ZN5boost6thread17do_try_join_untilERK8timespec
// type: _DWORD __fastcall(boost::thread *__hidden this, const timespec *)
#[doc(alias = "boost::thread::do_try_join_until(timespec const&)")]
pub fn stub_f28974() -> ! {
    todo!("0xf28974 boost::thread::do_try_join_until(timespec const&)")
}

// 0xf28984 — j___ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE14subtract_timesERKS5_S8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::subtract_times(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&)")]
pub fn stub_f28984() -> ! {
    todo!("0xf28984 boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::subtract_times(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&)")
}

// 0xf28994 — j___ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE17add_time_durationERKS5_NS3_13time_durationE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::add_time_duration(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::posix_time::time_duration)")]
pub fn stub_f28994() -> ! {
    todo!("0xf28994 boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::add_time_duration(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::posix_time::time_duration)")
}

// 0xf289a4 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>)")]
pub fn stub_f289a4() -> ! {
    todo!("0xf289a4 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>)")
}

// 0xf289c4 — j___ZN5boost9gregorian16bad_day_of_monthC2Ev
// type: _DWORD __fastcall(boost::gregorian::bad_day_of_month *__hidden this)
#[doc(alias = "boost::gregorian::bad_day_of_month::bad_day_of_month(void)")]
pub fn stub_f289c4() -> ! {
    todo!("0xf289c4 boost::gregorian::bad_day_of_month::bad_day_of_month(void)")
}

// 0xf289d4 — j___ZN5boost9gregorian4dateC2ENS0_9greg_yearENS0_10greg_monthENS0_8greg_dayE
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::gregorian::date::date(boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day)")]
pub fn stub_f289d4() -> ! {
    todo!("0xf289d4 boost::gregorian::date::date(boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day)")
}

// 0xf289e4 — j___ZN5boost9gregorian8bad_yearC2Ev
// type: _DWORD __fastcall(boost::gregorian::bad_year *__hidden this)
#[doc(alias = "boost::gregorian::bad_year::bad_year(void)")]
pub fn stub_f289e4() -> ! {
    todo!("0xf289e4 boost::gregorian::bad_year::bad_year(void)")
}

// 0xf289f4 — j___ZN5boost9gregorian9bad_monthC2Ev
// type: _DWORD __fastcall(boost::gregorian::bad_month *__hidden this)
#[doc(alias = "boost::gregorian::bad_month::bad_month(void)")]
pub fn stub_f289f4() -> ! {
    todo!("0xf289f4 boost::gregorian::bad_month::bad_month(void)")
}

// 0xf28a04 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE7rethrowEv
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::rethrow(void)const")]
pub fn stub_f28a04() -> ! {
    todo!("0xf28a04 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::rethrow(void)const")
}

// 0xf28a14 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler6ThreadEE22_internal_accept_ownerIS3_S3_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Thread>::_internal_accept_owner<RBX::TaskScheduler::Thread,RBX::TaskScheduler::Thread>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const*,RBX::TaskScheduler::Thread *)const")]
pub fn stub_f28a14() -> ! {
    todo!("0xf28a14 void boost::enable_shared_from_this<RBX::TaskScheduler::Thread>::_internal_accept_owner<RBX::TaskScheduler::Thread,RBX::TaskScheduler::Thread>(boost::shared_ptr<RBX::TaskScheduler::Thread> const*,RBX::TaskScheduler::Thread *)const")
}

// 0xf28a24 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_f28a24() -> ! {
    todo!("0xf28a24 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &)const")
}

// 0xf28a34 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_f28a34() -> ! {
    todo!("0xf28a34 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf28a44 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESA_EET0_T_SC_SB_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job const> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *)")]
pub fn stub_f28a44() -> ! {
    todo!("0xf28a44 boost::shared_ptr<RBX::TaskScheduler::Job const> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *>(boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *)")
}

// 0xf28a54 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES9_EET0_T_SB_SA_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *)")]
pub fn stub_f28a54() -> ! {
    todo!("0xf28a54 boost::shared_ptr<RBX::TaskScheduler::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *>(boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *)")
}

// 0xf28a64 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *)")]
pub fn stub_f28a64() -> ! {
    todo!("0xf28a64 boost::shared_ptr<RBX::TaskScheduler::Thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *>(boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *)")
}

// 0xf28a74 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *)")]
pub fn stub_f28a74() -> ! {
    todo!("0xf28a74 boost::shared_ptr<RBX::TaskScheduler::Thread> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *>(boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *)")
}

// 0xf28a84 — j___ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&)")]
pub fn stub_f28a84() -> ! {
    todo!("0xf28a84 std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::shared_ptr<RBX::TaskScheduler::Job const> const&)")
}

// 0xf28a94 — j___ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE9push_backERKS6_
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&)")]
pub fn stub_f28a94() -> ! {
    todo!("0xf28a94 std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Job const> const&)")
}

// 0xf28aa4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_f28aa4() -> ! {
    todo!("0xf28aa4 std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>>,boost::shared_ptr<RBX::TaskScheduler::Job> const&)")
}

// 0xf28ab4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_f28ab4() -> ! {
    todo!("0xf28ab4 std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Job> const&)")
}

// 0xf28ac4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&)")]
pub fn stub_f28ac4() -> ! {
    todo!("0xf28ac4 std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Thread>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::shared_ptr<RBX::TaskScheduler::Thread> const&)")
}

// 0xf28ad4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&)")]
pub fn stub_f28ad4() -> ! {
    todo!("0xf28ad4 std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Thread> const&)")
}

// 0xf28ae4 — j___ZN3RBX9TCriticalEjNS_10ConfidenceE
// type: __int64 __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::TCritical(unsigned int,RBX::Confidence)")]
pub fn stub_f28ae4() -> ! {
    todo!("0xf28ae4 RBX::TCritical(unsigned int,RBX::Confidence)")
}

// 0xf28af4 — j___ZN5boost9algorithm7trim_ifISsPFbcEEEvRT_T0_
// type: int __fastcall(std::string *this)
#[doc(alias = "void boost::algorithm::trim_if<std::string,bool (*)(char)>(std::string &,bool (*)(char))")]
pub fn stub_f28af4() -> ! {
    todo!("0xf28af4 void boost::algorithm::trim_if<std::string,bool (*)(char)>(std::string &,bool (*)(char))")
}

// 0xf28b04 — j___ZN3RBX10Reflection11Call1HelperINS_11HttpServiceEMS2_FN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsESsSI_E4callEPS2_SK_RS7_RSD_
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),std::string,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::HttpService*,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),RBX::Reflection::Variant&,std::string const&)")]
pub fn stub_f28b04() -> ! {
    todo!("0xf28b04 RBX::Reflection::Call1Helper<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::HttpService*,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),RBX::Reflection::Variant&,std::string const&)")
}

// 0xf28b14 — j___ZN3RBX10Reflection11Call1HelperINS_11HttpServiceEMS2_FSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEESI_SsE4callEPS2_SK_RS7_RKSI_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::HttpService,std::string (RBX::HttpService::*)(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,std::string>::call(RBX::HttpService*,std::string (RBX::HttpService::*)(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),RBX::Reflection::Variant&,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")]
pub fn stub_f28b14() -> ! {
    todo!("0xf28b14 RBX::Reflection::Call1Helper<RBX::HttpService,std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,std::string>::call(RBX::HttpService*,std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),RBX::Reflection::Variant&,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")
}

// 0xf28b24 — j___ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EE16declareSignatureEPKcS7_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_f28b24() -> ! {
    todo!("0xf28b24 RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf28b34 — j___ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EEC2EMS2_FSI_SsEPKcSO_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f28b34() -> ! {
    todo!("0xf28b34 RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf28b44 — j___ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EE16declareSignatureEPKcS7_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_f28b44() -> ! {
    todo!("0xf28b44 RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf28b54 — j___ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EEC2EMS2_FSsSI_EPKcSO_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::BoundFuncDesc(std::string (RBX::HttpService::*)(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f28b54() -> ! {
    todo!("0xf28b54 RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::BoundFuncDesc(std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf28b64 — j___ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_f28b64() -> ! {
    todo!("0xf28b64 RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf28b74 — j___ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EEC2EMS2_FvSsN5boost8functionIFvSsEEES8_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f28b74() -> ! {
    todo!("0xf28b74 RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf28b84 — j___ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EE16declareSignatureEPKcNS0_7VariantES7_S8_S7_S8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_f28b84() -> ! {
    todo!("0xf28b84 RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0xf28b94 — j___ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EEC2EMS2_FvSsSsS3_N5boost8functionIFvSsEEES9_EPKcSD_SD_SD_S3_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,char const*,RBX::HttpService::HttpContentType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f28b94() -> ! {
    todo!("0xf28b94 RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,char const*,RBX::HttpService::HttpContentType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf28ba4 — j___ZN3RBX10Reflection7Variant14genericConvertINS_11HttpService15HttpContentTypeEEERT_v
#[doc(alias = "RBX::HttpService::HttpContentType & RBX::Reflection::Variant::genericConvert<RBX::HttpService::HttpContentType>(void)")]
pub fn stub_f28ba4() -> ! {
    todo!("0xf28ba4 RBX::HttpService::HttpContentType & RBX::Reflection::Variant::genericConvert<RBX::HttpService::HttpContentType>(void)")
}

// 0xf28bb4 — j___ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE7addPairES3_PKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::addPair(RBX::HttpService::HttpContentType,char const*)")]
pub fn stub_f28bb4() -> ! {
    todo!("0xf28bb4 RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::addPair(RBX::HttpService::HttpContentType,char const*)")
}

// 0xf28bc4 — j___ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISJ_EEPNS3_10disable_ifINS3_7is_sameISJ_NS4_IKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_f28bc4() -> ! {
    todo!("0xf28bc4 boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::ArgHelper::getArg<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&,boost::disable_if<boost::is_same<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf28bd4 — j___ZN3RBX10Reflection9ArgHelper6getArgINS_11HttpService15HttpContentTypeELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::HttpService::HttpContentType RBX::Reflection::ArgHelper::getArg<RBX::HttpService::HttpContentType,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::HttpService::HttpContentType> const&,boost::disable_if<boost::is_same<RBX::HttpService::HttpContentType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_f28bd4() -> ! {
    todo!("0xf28bd4 RBX::HttpService::HttpContentType RBX::Reflection::ArgHelper::getArg<RBX::HttpService::HttpContentType,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::HttpService::HttpContentType> const&,boost::disable_if<boost::is_same<RBX::HttpService::HttpContentType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf28be4 — j___ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_11HttpService15HttpContentTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<3,RBX::HttpService::HttpContentType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::HttpService::HttpContentType &,boost::enable_if<boost::is_enum<RBX::HttpService::HttpContentType>,void>::type *)")]
pub fn stub_f28be4() -> ! {
    todo!("0xf28be4 bool RBX::Reflection::ArgHelper::try_enum<3,RBX::HttpService::HttpContentType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::HttpService::HttpContentType &,boost::enable_if<boost::is_enum<RBX::HttpService::HttpContentType>,void>::type *)")
}

// 0xf28bf4 — j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_11HttpServiceEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HttpService>(char const*,char const*,bool RBX::HttpService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f28bf4() -> ! {
    todo!("0xf28bf4 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HttpService>(char const*,char const*,bool RBX::HttpService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28c44 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_11HttpServiceEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)")]
pub fn stub_f28c44() -> ! {
    todo!("0xf28c44 boost::shared_ptr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)")
}

// 0xf28c54 — j___ZN3rbx8any_castIN3RBX11HttpService15HttpContentTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::HttpService::HttpContentType * rbx::any_cast<RBX::HttpService::HttpContentType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_f28c54() -> ! {
    todo!("0xf28c54 RBX::HttpService::HttpContentType * rbx::any_cast<RBX::HttpService::HttpContentType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0xf28c64 — j___ZN3rbx8any_castIRN3RBX11HttpService15HttpContentTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::HttpService::HttpContentType & rbx::any_cast<RBX::HttpService::HttpContentType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_f28c64() -> ! {
    todo!("0xf28c64 RBX::HttpService::HttpContentType & rbx::any_cast<RBX::HttpService::HttpContentType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf28c74 — j___ZN5boost10shared_ptrIN3RBX11HttpServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f28c74() -> ! {
    todo!("0xf28c74 boost::shared_ptr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf28c84 — j___ZN5boost6detail12shared_countC2IPN3RBX11HttpServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f28c84() -> ! {
    todo!("0xf28c84 boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf28ca4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11HttpServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpService,RBX::HttpService>(rbx_core::SharedPtr<RBX::HttpService> const*,RBX::HttpService *)const")]
pub fn stub_f28ca4() -> ! {
    todo!("0xf28ca4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpService,RBX::HttpService>(boost::shared_ptr<RBX::HttpService> const*,RBX::HttpService *)const")
}

// 0xf28cb4 — j___ZNSt12_Vector_baseIN3RBX11HttpService15HttpContentTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Vector_base<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_allocate(unsigned long)")]
pub fn stub_f28cb4() -> ! {
    todo!("0xf28cb4 std::_Vector_base<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_allocate(unsigned long)")
}

// 0xf28cc4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11HttpService15HttpContentTypeES6_EET0_T_S8_S7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::HttpService::HttpContentType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *>(RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *)")]
pub fn stub_f28cc4() -> ! {
    todo!("0xf28cc4 RBX::HttpService::HttpContentType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *>(RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *)")
}

// 0xf28cd4 — j___ZNSt3mapIPKN3RBX4NameENS0_11HttpService15HttpContentTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::HttpService::HttpContentType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_f28cd4() -> ! {
    todo!("0xf28cd4 std::map<RBX::Name const*,RBX::HttpService::HttpContentType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::operator[](RBX::Name const* const&)")
}

// 0xf28ce4 — j___ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,RBX::HttpService::HttpContentType const&)")]
pub fn stub_f28ce4() -> ! {
    todo!("0xf28ce4 std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,RBX::HttpService::HttpContentType const&)")
}

// 0xf28cf4 — j___ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(_DWORD)
#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,unsigned long,RBX::HttpService::HttpContentType const&)")]
pub fn stub_f28cf4() -> ! {
    todo!("0xf28cf4 std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,unsigned long,RBX::HttpService::HttpContentType const&)")
}

// 0xf28d04 — j___ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::resize(unsigned long,RBX::HttpService::HttpContentType)")]
pub fn stub_f28d04() -> ! {
    todo!("0xf28d04 std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::resize(unsigned long,RBX::HttpService::HttpContentType)")
}

// 0xf28d14 — j___ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::push_back(RBX::HttpService::HttpContentType const&)")]
pub fn stub_f28d14() -> ! {
    todo!("0xf28d14 std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::push_back(RBX::HttpService::HttpContentType const&)")
}

// 0xf28d24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")]
pub fn stub_f28d24() -> ! {
    todo!("0xf28d24 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")
}

// 0xf28d34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")]
pub fn stub_f28d34() -> ! {
    todo!("0xf28d34 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")
}

// 0xf28d44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")]
pub fn stub_f28d44() -> ! {
    todo!("0xf28d44 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")
}

// 0xf28d54 — j___ZN3RBX10Reflection14PropDescriptorINS_10PointLightEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PointLight,float>::PropDescriptor<float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float)>(char const*,char const*,float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f28d54() -> ! {
    todo!("0xf28d54 RBX::Reflection::PropDescriptor<RBX::PointLight,float>::PropDescriptor<float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float)>(char const*,char const*,float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28d64 — j___ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f28d64() -> ! {
    todo!("0xf28d64 RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28d74 — j___ZN3RBX10Reflection14PropDescriptorINS_5LightEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,bool>::PropDescriptor<bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool)>(char const*,char const*,bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f28d74() -> ! {
    todo!("0xf28d74 RBX::Reflection::PropDescriptor<RBX::Light,bool>::PropDescriptor<bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool)>(char const*,char const*,bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28d84 — j___ZN3RBX10Reflection14PropDescriptorINS_5LightEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,float>::PropDescriptor<float (RBX::Light::*)(void)const,void (RBX::Light::*)(float)>(char const*,char const*,float (RBX::Light::*)(void)const,void (RBX::Light::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f28d84() -> ! {
    todo!("0xf28d84 RBX::Reflection::PropDescriptor<RBX::Light,float>::PropDescriptor<float (RBX::Light::*)(void)const,void (RBX::Light::*)(float)>(char const*,char const*,float (RBX::Light::*)(void)const,void (RBX::Light::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28d94 — j___ZN3RBX10Reflection14PropDescriptorINS_9SpotLightEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SpotLight,float>::PropDescriptor<float (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(float)>(char const*,char const*,float (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f28d94() -> ! {
    todo!("0xf28d94 RBX::Reflection::PropDescriptor<RBX::SpotLight,float>::PropDescriptor<float (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(float)>(char const*,char const*,float (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28da4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_9SpotLightENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SpotLight,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(RBX::NormalId)>(char const*,char const*,RBX::NormalId (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(RBX::NormalId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f28da4() -> ! {
    todo!("0xf28da4 RBX::Reflection::EnumPropDescriptor<RBX::SpotLight,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(RBX::NormalId)>(char const*,char const*,RBX::NormalId (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(RBX::NormalId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28e44 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10PointLightEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::PointLight> RBX::Creatable<RBX::Instance>::create<RBX::PointLight>(void)")]
pub fn stub_f28e44() -> ! {
    todo!("0xf28e44 boost::shared_ptr<RBX::PointLight> RBX::Creatable<RBX::Instance>::create<RBX::PointLight>(void)")
}

// 0xf28e54 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_9SpotLightEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)")]
pub fn stub_f28e54() -> ! {
    todo!("0xf28e54 boost::shared_ptr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)")
}

// 0xf28e64 — j___ZN5boost10shared_ptrIN3RBX10PointLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::PointLight>::shared_ptr<RBX::PointLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f28e64() -> ! {
    todo!("0xf28e64 boost::shared_ptr<RBX::PointLight>::shared_ptr<RBX::PointLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf28e74 — j___ZN5boost10shared_ptrIN3RBX9SpotLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f28e74() -> ! {
    todo!("0xf28e74 boost::shared_ptr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf28e84 — j___ZN5boost6detail12shared_countC2IPN3RBX10PointLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f28e84() -> ! {
    todo!("0xf28e84 boost::detail::shared_count::shared_count<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf28e94 — j___ZN5boost6detail12shared_countC2IPN3RBX9SpotLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f28e94() -> ! {
    todo!("0xf28e94 boost::detail::shared_count::shared_count<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf28ea4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_9SpotLightENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SpotLight,RBX::NormalId>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_f28ea4() -> ! {
    todo!("0xf28ea4 RBX::Reflection::EnumPropDescriptor<RBX::SpotLight,RBX::NormalId>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf28eb4 — j___ZNK3RBX10Reflection8EnumDescINS_8NormalIdEE14convertToIndexES2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NormalId>::convertToIndex(RBX::NormalId)const")]
pub fn stub_f28eb4() -> ! {
    todo!("0xf28eb4 RBX::Reflection::EnumDesc<RBX::NormalId>::convertToIndex(RBX::NormalId)const")
}

// 0xf28ee4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10PointLightES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PointLight,RBX::PointLight>(rbx_core::SharedPtr<RBX::PointLight> const*,RBX::PointLight *)const")]
pub fn stub_f28ee4() -> ! {
    todo!("0xf28ee4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PointLight,RBX::PointLight>(boost::shared_ptr<RBX::PointLight> const*,RBX::PointLight *)const")
}

// 0xf28ef4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9SpotLightES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SpotLight,RBX::SpotLight>(rbx_core::SharedPtr<RBX::SpotLight> const*,RBX::SpotLight *)const")]
pub fn stub_f28ef4() -> ! {
    todo!("0xf28ef4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SpotLight,RBX::SpotLight>(boost::shared_ptr<RBX::SpotLight> const*,RBX::SpotLight *)const")
}

// 0xf28f04 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10declareSubEPS2_S4_
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::declareSub(RBX::Reflection::EventDescriptor*,RBX::Reflection::EventDescriptor*)")]
pub fn stub_f28f04() -> ! {
    todo!("0xf28f04 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::declareSub(RBX::Reflection::EventDescriptor*,RBX::Reflection::EventDescriptor*)")
}

// 0xf28f14 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10staticDataEv
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::staticData(void)")]
pub fn stub_f28f14() -> ! {
    todo!("0xf28f14 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::staticData(void)")
}

// 0xf28f24 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE7declareEPS2_
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::declare(RBX::Reflection::EventDescriptor*)")]
pub fn stub_f28f24() -> ! {
    todo!("0xf28f24 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::declare(RBX::Reflection::EventDescriptor*)")
}

// 0xf28f34 — j___ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<XmlAttribute,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_f28f34() -> ! {
    todo!("0xf28f34 boost::singleton_pool<XmlAttribute,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0xf28f44 — j___ZN5boost15throw_exceptionINS_21thread_resource_errorEEEvRKT_
// type: int __fastcall(int)
#[doc(alias = "void boost::throw_exception<boost::thread_resource_error>(boost::thread_resource_error const&)")]
pub fn stub_f28f44() -> ! {
    todo!("0xf28f44 void boost::throw_exception<boost::thread_resource_error>(boost::thread_resource_error const&)")
}

// 0xf28f54 — j___ZN5boost16exception_detail20copy_boost_exceptionEPNS_9exceptionEPKS1_
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::exception_detail::copy_boost_exception(boost::exception *,boost::exception const*)")]
pub fn stub_f28f54() -> ! {
    todo!("0xf28f54 boost::exception_detail::copy_boost_exception(boost::exception *,boost::exception const*)")
}

// 0xf28f64 — j___ZN5boost6detail12shared_countC1ERKS1_
// type: _DWORD __fastcall(boost::detail::shared_count *__hidden this, const shared_count *)
#[doc(alias = "boost::detail::shared_count::shared_count(boost::detail::shared_count const&)")]
pub fn stub_f28f64() -> ! {
    todo!("0xf28f64 boost::detail::shared_count::shared_count(boost::detail::shared_count const&)")
}

// 0xf28f74 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
pub fn stub_f28f74() -> ! {
    todo!("0xf28f74 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")
}

// 0xf28f84 — j___ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE7rethrowEv
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::rethrow(void)const")]
pub fn stub_f28f84() -> ! {
    todo!("0xf28f84 boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::rethrow(void)const")
}

// 0xf28f94 — j___ZNSt10_List_baseIN3RBX10Reflection19SignatureDescriptor4ItemESaIS3_EE8_M_clearEv
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_List_base<RBX::Reflection::SignatureDescriptor::Item,std::allocator<RBX::Reflection::SignatureDescriptor::Item>>::_M_clear(void)")]
pub fn stub_f28f94() -> ! {
    todo!("0xf28f94 std::_List_base<RBX::Reflection::SignatureDescriptor::Item,std::allocator<RBX::Reflection::SignatureDescriptor::Item>>::_M_clear(void)")
}

// 0xf28fa4 — j___ZNSt6vectorIPN3RBX10Reflection15EventDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int)
#[doc(alias = "std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor **,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,RBX::Reflection::EventDescriptor * const&)")]
pub fn stub_f28fa4() -> ! {
    todo!("0xf28fa4 std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor **,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,RBX::Reflection::EventDescriptor * const&)")
}

// 0xf28fb4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE7declareEPS2_
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::declare(RBX::Reflection::FunctionDescriptor*)")]
pub fn stub_f28fb4() -> ! {
    todo!("0xf28fb4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::declare(RBX::Reflection::FunctionDescriptor*)")
}

// 0xf28fc4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10declareSubEPS2_S4_
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declareSub(RBX::Reflection::YieldFunctionDescriptor*,RBX::Reflection::YieldFunctionDescriptor*)")]
pub fn stub_f28fc4() -> ! {
    todo!("0xf28fc4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declareSub(RBX::Reflection::YieldFunctionDescriptor*,RBX::Reflection::YieldFunctionDescriptor*)")
}

// 0xf28fd4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10staticDataEv
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::staticData(void)")]
pub fn stub_f28fd4() -> ! {
    todo!("0xf28fd4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::staticData(void)")
}

// 0xf28fe4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE7declareEPS2_
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declare(RBX::Reflection::YieldFunctionDescriptor*)")]
pub fn stub_f28fe4() -> ! {
    todo!("0xf28fe4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declare(RBX::Reflection::YieldFunctionDescriptor*)")
}

// 0xf28ff4 — j___ZN5boost6detail12shared_countC2INS_16exception_detail10clone_implINS3_10bad_alloc_EEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> *)")]
pub fn stub_f28ff4() -> ! {
    todo!("0xf28ff4 boost::detail::shared_count::shared_count<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> *)")
}

// 0xf29004 — j___ZN5boost6detail15sp_counted_base7releaseEv
// type: _DWORD __fastcall(boost::detail::sp_counted_base *__hidden this)
#[doc(alias = "boost::detail::sp_counted_base::release(void)")]
pub fn stub_f29004() -> ! {
    todo!("0xf29004 boost::detail::sp_counted_base::release(void)")
}

// 0xf29014 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
pub fn stub_f29014() -> ! {
    todo!("0xf29014 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")
}

// 0xf29024 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
pub fn stub_f29024() -> ! {
    todo!("0xf29024 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")
}

// 0xf29034 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
// type: int(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
pub fn stub_f29034() -> ! {
    todo!("0xf29034 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")
}

// 0xf29044 — j___ZNSt6vectorIPN3RBX10Reflection23YieldFunctionDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int)
#[doc(alias = "std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::YieldFunctionDescriptor **,std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>>,RBX::Reflection::YieldFunctionDescriptor * const&)")]
pub fn stub_f29044() -> ! {
    todo!("0xf29044 std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::YieldFunctionDescriptor **,std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>>,RBX::Reflection::YieldFunctionDescriptor * const&)")
}

// 0xf29054 — j___ZN3RBX10Reflection10DescriptorC2EPKcNS1_10AttributesE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::Descriptor::Descriptor(char const*,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f29054() -> ! {
    todo!("0xf29054 RBX::Reflection::Descriptor::Descriptor(char const*,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf29064 — j___ZN3RBX10Reflection15ClassDescriptorD2Ev
// type: void __fastcall(RBX::Reflection::ClassDescriptor *__hidden this)
#[doc(alias = "RBX::Reflection::ClassDescriptor::~ClassDescriptor()")]
pub fn stub_f29064() -> ! {
    todo!("0xf29064 RBX::Reflection::ClassDescriptor::~ClassDescriptor()")
}

// 0xf29074 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE12mergeMembersEPKS3_
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> const*)")]
pub fn stub_f29074() -> ! {
    todo!("0xf29074 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> const*)")
}

// 0xf29084 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEEC2EPS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>*)")]
pub fn stub_f29084() -> ! {
    todo!("0xf29084 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>*)")
}

// 0xf29094 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10declareSubEPS2_S4_
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declareSub(RBX::Reflection::CallbackDescriptor*,RBX::Reflection::CallbackDescriptor*)")]
pub fn stub_f29094() -> ! {
    todo!("0xf29094 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declareSub(RBX::Reflection::CallbackDescriptor*,RBX::Reflection::CallbackDescriptor*)")
}

// 0xf290a4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10staticDataEv
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::staticData(void)")]
pub fn stub_f290a4() -> ! {
    todo!("0xf290a4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::staticData(void)")
}

// 0xf290b4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE12mergeMembersEPKS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> const*)")]
pub fn stub_f290b4() -> ! {
    todo!("0xf290b4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> const*)")
}

// 0xf290c4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE7declareEPS2_
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declare(RBX::Reflection::CallbackDescriptor*)")]
pub fn stub_f290c4() -> ! {
    todo!("0xf290c4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declare(RBX::Reflection::CallbackDescriptor*)")
}

// 0xf290d4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEEC2EPS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>*)")]
pub fn stub_f290d4() -> ! {
    todo!("0xf290d4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>*)")
}

// 0xf290e4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE10declareSubEPS2_S4_
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::declareSub(RBX::Reflection::FunctionDescriptor*,RBX::Reflection::FunctionDescriptor*)")]
pub fn stub_f290e4() -> ! {
    todo!("0xf290e4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::declareSub(RBX::Reflection::FunctionDescriptor*,RBX::Reflection::FunctionDescriptor*)")
}

// 0xf290f4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE10staticDataEv
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::staticData(void)")]
pub fn stub_f290f4() -> ! {
    todo!("0xf290f4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::staticData(void)")
}

// 0xf29104 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE12mergeMembersEPKS3_
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> const*)")]
pub fn stub_f29104() -> ! {
    todo!("0xf29104 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> const*)")
}

// 0xf29114 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEEC2EPS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>*)")]
pub fn stub_f29114() -> ! {
    todo!("0xf29114 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>*)")
}

// 0xf29124 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE10declareSubEPS2_S4_
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::declareSub(RBX::Reflection::PropertyDescriptor*,RBX::Reflection::PropertyDescriptor*)")]
pub fn stub_f29124() -> ! {
    todo!("0xf29124 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::declareSub(RBX::Reflection::PropertyDescriptor*,RBX::Reflection::PropertyDescriptor*)")
}

// 0xf29134 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE10staticDataEv
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::staticData(void)")]
pub fn stub_f29134() -> ! {
    todo!("0xf29134 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::staticData(void)")
}

// 0xf29144 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE12mergeMembersEPKS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> const*)")]
pub fn stub_f29144() -> ! {
    todo!("0xf29144 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> const*)")
}

// 0xf29154 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE7declareEPS2_
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::declare(RBX::Reflection::PropertyDescriptor*)")]
pub fn stub_f29154() -> ! {
    todo!("0xf29154 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::declare(RBX::Reflection::PropertyDescriptor*)")
}

// 0xf29164 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEEC2EPS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>*)")]
pub fn stub_f29164() -> ! {
    todo!("0xf29164 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>*)")
}

// 0xf29174 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE12mergeMembersEPKS3_
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> const*)")]
pub fn stub_f29174() -> ! {
    todo!("0xf29174 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> const*)")
}

// 0xf29184 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEEC2EPS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>*)")]
pub fn stub_f29184() -> ! {
    todo!("0xf29184 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>*)")
}

// 0xf29194 — j___ZN5boost16exception_detail10bad_alloc_D2Ev
// type: void __fastcall(boost::exception_detail::bad_alloc_ *__hidden this)
#[doc(alias = "boost::exception_detail::bad_alloc_::~bad_alloc_()")]
pub fn stub_f29194() -> ! {
    todo!("0xf29194 boost::exception_detail::bad_alloc_::~bad_alloc_()")
}

// 0xf291a4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS5_NS5_9clone_tagE
// type: int __fastcall(int, int, int, int, std::exception *, std::string *, int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_tag)")]
pub fn stub_f291a4() -> ! {
    todo!("0xf291a4 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_tag)")
}

// 0xf291b4 — j___ZN5boost16exception_detail14bad_exception_D2Ev
// type: void __fastcall(boost::exception_detail::bad_exception_ *__hidden this)
#[doc(alias = "boost::exception_detail::bad_exception_::~bad_exception_()")]
pub fn stub_f291b4() -> ! {
    todo!("0xf291b4 boost::exception_detail::bad_exception_::~bad_exception_()")
}

// 0xf291c4 — j___ZN5boost16exception_detail27get_static_exception_objectINS0_10bad_alloc_EEENS_13exception_ptrEv
// type: int __fastcall(int, int, int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, char, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::exception_ptr boost::exception_detail::get_static_exception_object<boost::exception_detail::bad_alloc_>(void)")]
pub fn stub_f291c4() -> ! {
    todo!("0xf291c4 boost::exception_ptr boost::exception_detail::get_static_exception_object<boost::exception_detail::bad_alloc_>(void)")
}

