//! platform — generated_plat_bi — 150 stubs EA-sorted asc global gap filler not yet in platform
//! Source: ida/export.json (85545 funcs) global gap filler next 150 EA-sorted asc not yet stubbed in platform
//! Distinct stub_ 30750/85545 -> 30900/85545 | uncovered 54795 -> 54645 (platform)
//! Batch: 150 stubs | range 0x23ffb0..0x248bb0 | rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x23ffb0 — __ZN3RBX13worker_thread10threadProcEN5boost10shared_ptrINS0_4dataEEERKNS1_9function0INS0_11work_resultEEE
// type: void __fastcall(boost::mutex **, _DWORD *)
// was: RBX::worker_thread::threadProc(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::worker_thread::threadProc(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&)")]
pub fn stub_23ffb0() -> ! {
    todo!("0x23ffb0 RBX::worker_thread::threadProc(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&)")
}

// 0x2400f4 — __ZN3RBX13worker_threadD1Ev
// type: void __fastcall(RBX::worker_thread *__hidden this)
#[doc(alias = "RBX::worker_thread::~worker_thread()")]
pub fn stub_2400f4() -> ! {
    todo!("0x2400f4 RBX::worker_thread::~worker_thread()")
}

// 0x240100 — __ZN3RBX13worker_threadD2Ev
// type: void __fastcall(boost::mutex **this)
#[doc(alias = "RBX::worker_thread::~worker_thread()")]
pub fn stub_240100() -> ! {
    todo!("0x240100 RBX::worker_thread::~worker_thread()")
}

// 0x2402c4 — __ZN3RBX13worker_thread4wakeEv
// type: void __fastcall(boost::mutex **this)
#[doc(alias = "RBX::worker_thread::wake(void)")]
pub fn stub_2402c4() -> ! {
    todo!("0x2402c4 RBX::worker_thread::wake(void)")
}

// 0x2403cc — __ZN5boost19thread_specific_ptrISsED1Ev
#[doc(alias = "boost::thread_specific_ptr<std::string>::~thread_specific_ptr()")]
pub fn stub_2403cc() -> ! {
    todo!("0x2403cc boost::thread_specific_ptr<std::string>::~thread_specific_ptr()")
}

// 0x2403d8 — __ZN5boost19thread_specific_ptrISsE5resetEPSs
// type: void __fastcall(int *, const void *)
#[doc(alias = "boost::thread_specific_ptr<std::string>::reset(std::string *)")]
pub fn stub_2403d8() -> ! {
    todo!("0x2403d8 boost::thread_specific_ptr<std::string>::reset(std::string *)")
}

// 0x2404f4 — __ZN5boost4bindIvRKNS_9function0IvEESsS2_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
// type: void __fastcall(double *, int, int *, const std::string *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list_av_2<boost::function0<void>,std::string>::type> boost::bind<void,boost::function0<void> const&,std::string,boost::function0<void>,std::string>(void (*)(boost::function0<void> const&,std::string),boost::function0<void>,std::string)")]
pub fn stub_2404f4() -> ! {
    todo!("0x2404f4 boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list_av_2<boost::function0<void>,std::string>::type> boost::bind<void,boost::function0<void> const&,std::string,boost::function0<void>,std::string>(void (*)(boost::function0<void> const&,std::string),boost::function0<void>,std::string)")
}

// 0x2407fc — __ZN5boost4bindIvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS3_11work_resultEEES5_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_
// type: void __fastcall(boost::detail::sp_counted_base *, int, int *, int, int, int, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list_av_2<boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>::type> boost::bind<void,boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&,boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>(void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&,rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>(void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>)")]
pub fn stub_2407fc() -> ! {
    todo!("0x2407fc boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&,rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>(void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>)")
}

// 0x240a54 — __ZN5boost22condition_variable_any4waitINS_11unique_lockINS_5mutexEEEEEvRT_
// type: void __fastcall(int, int)
#[doc(alias = "void boost::condition_variable_any::wait<boost::unique_lock<boost::mutex>>(boost::unique_lock<boost::mutex> &)")]
pub fn stub_240a54() -> ! {
    todo!("0x240a54 void boost::condition_variable_any::wait<boost::unique_lock<boost::mutex>>(boost::unique_lock<boost::mutex> &)")
}

// 0x240c80 — __ZN5boost15throw_exceptionINS_15condition_errorEEEvRKT_
// type: void __fastcall __noreturn(_QWORD *)
#[doc(alias = "void boost::throw_exception<boost::condition_error>(boost::condition_error const&)")]
pub fn stub_240c80() -> ! {
    todo!("0x240c80 void boost::throw_exception<boost::condition_error>(boost::condition_error const&)")
}

// 0x241040 — __ZN5boost15condition_errorD1Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::condition_error::~condition_error()")]
pub fn stub_241040() -> ! {
    todo!("0x241040 boost::condition_error::~condition_error()")
}

// 0x2410a0 — __ZN5boost15condition_errorD0Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::condition_error::~condition_error()")]
pub fn stub_2410a0() -> ! {
    todo!("0x2410a0 boost::condition_error::~condition_error()")
}

// 0x241108 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED1Ev
// type: std::runtime_error *__fastcall(std::runtime_error *)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()")]
pub fn stub_241108() -> ! {
    todo!("0x241108 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()")
}

// 0x241214 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_15condition_errorEED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk to boost::exception_detail::error_info_injector<boost::condition_error>::~error_info_injector()")]
pub fn stub_241214() -> ! {
    todo!("0x241214 non-virtual thunk to boost::exception_detail::error_info_injector<boost::condition_error>::~error_info_injector()")
}

// 0x241324 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()")]
pub fn stub_241324() -> ! {
    todo!("0x241324 non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()")
}

// 0x241430 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEE5cloneEv
#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::clone(void)const")]
pub fn stub_241430() -> ! {
    todo!("0x241430 virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::clone(void)const")
}

// 0x241444 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS0_INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSH_ISB_EEEEEEEEvT_
// type: void __fastcall(int, int, int, int, char, int, boost::detail::sp_counted_base *, int, int, int, int, char, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>)")]
pub fn stub_241444() -> ! {
    todo!("0x241444 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>)")
}

// 0x241798 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_241798() -> ! {
    todo!("0x241798 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x2417bc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(_DWORD *)
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,void>::invoke(boost::detail::function::function_buffer &) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_2417bc() -> ! {
    todo!("0x2417bc boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x2417d0 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS9_11work_resultEEEENS5_5list2INS5_5valueISB_EENSK_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, double *, _DWORD *, int, boost::detail::sp_counted_base *, int, int, int, int, void *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_2417d0() -> ! {
    todo!("0x2417d0 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x241aac — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEclIPFvS7_RKSB_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, int))
// was: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::operator()<void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&) &,boost::_bi::list0 &,int) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&) &,boost::_bi::list0 &,int)")]
pub fn stub_241aac() -> ! {
    todo!("0x241aac void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&) &,boost::_bi::list0 &,int)")
}

// 0x241bbc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_241bbc() -> ! {
    todo!("0x241bbc boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x241df4 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
pub fn stub_241df4() -> ! {
    todo!("0x241df4 boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")
}

// 0x241f98 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// type: int __fastcall(int, int *, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
pub fn stub_241f98() -> ! {
    todo!("0x241f98 boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")
}

// 0x242144 — __ZN5boost3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS4_11work_resultEEEENS0_5list2INS0_5valueIS6_EENSF_IS9_EEEEEC2ESD_RKSI_
// type: int __fastcall(int, int, int)
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>::bind_t(void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>::bind_t(void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>> const&)")]
pub fn stub_242144() -> ! {
    todo!("0x242144 boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>::bind_t(void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>> const&)")
}

// 0x242284 — __ZN5boost6detail20sp_pointer_constructIN3RBX13worker_thread4dataES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, boost::detail::sp_counted_base **, int, void *, int)
// was: void boost::detail::sp_pointer_construct<RBX::worker_thread::data,RBX::worker_thread::data>(boost::shared_ptr<RBX::worker_thread::data> *,RBX::worker_thread::data *,boost::detail::shared_count &) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::worker_thread::data,RBX::worker_thread::data>(rbx_core::SharedPtr<RBX::worker_thread::data> *,RBX::worker_thread::data *,boost::detail::shared_count &)")]
pub fn stub_242284() -> ! {
    todo!("0x242284 void boost::detail::sp_pointer_construct<RBX::worker_thread::data,RBX::worker_thread::data>(rbx_core::SharedPtr<RBX::worker_thread::data> *,RBX::worker_thread::data *,boost::detail::shared_count &)")
}

// 0x2423c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()")]
pub fn stub_2423c8() -> ! {
    todo!("0x2423c8 boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()")
}

// 0x2423cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()")]
pub fn stub_2423cc() -> ! {
    todo!("0x2423cc boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()")
}

// 0x2423d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::dispose(void)")]
pub fn stub_2423d8() -> ! {
    todo!("0x2423d8 boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::dispose(void)")
}

// 0x2424bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_deleter(std::type_info const&)")]
pub fn stub_2424bc() -> ! {
    todo!("0x2424bc boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_deleter(std::type_info const&)")
}

// 0x2424c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_untyped_deleter(void)")]
pub fn stub_2424c0() -> ! {
    todo!("0x2424c0 boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_untyped_deleter(void)")
}

// 0x2424c4 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_SsENS3_5list2INS3_5valueIS1_EENSA_ISsEEEEEEEEvT_
// type: void __fastcall(_DWORD *, double *)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>)")]
pub fn stub_2424c4() -> ! {
    todo!("0x2424c4 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>)")
}

// 0x242818 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_242818() -> ! {
    todo!("0x242818 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x24283c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEvE6invokeERNS1_15function_bufferE
// type: void __fastcall(void (__fastcall ***)(_DWORD, int *))
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_24283c() -> ! {
    todo!("0x24283c boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x242958 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS5_5list2INS5_5valueIS8_EENSE_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, double *, void **)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_242958() -> ! {
    todo!("0x242958 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x242be8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_242be8() -> ! {
    todo!("0x242be8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x242e08 — __ZN5boost3_bi5list2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, const std::string *)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")]
pub fn stub_242e08() -> ! {
    todo!("0x242e08 boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")
}

// 0x242fc0 — __ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_
// type: _DWORD *__fastcall(_DWORD *, int *, const std::string *)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")]
pub fn stub_242fc0() -> ! {
    todo!("0x242fc0 boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")
}

// 0x24316c — __ZN5boost19thread_specific_ptrISsED2Ev
// type: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::thread_specific_ptr<std::string>::~thread_specific_ptr()")]
pub fn stub_24316c() -> ! {
    todo!("0x24316c boost::thread_specific_ptr<std::string>::~thread_specific_ptr()")
}

// 0x243260 — __ZN5boost19thread_specific_ptrISsE11delete_dataD1Ev
// type: void()
#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::~delete_data()")]
pub fn stub_243260() -> ! {
    todo!("0x243260 boost::thread_specific_ptr<std::string>::delete_data::~delete_data()")
}

// 0x243264 — __ZN5boost19thread_specific_ptrISsE11delete_dataD0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::~delete_data()")]
pub fn stub_243264() -> ! {
    todo!("0x243264 boost::thread_specific_ptr<std::string>::delete_data::~delete_data()")
}

// 0x243270 — __ZN5boost19thread_specific_ptrISsE11delete_dataclEPv
// type: void __fastcall(int, int *)
#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::operator()(void *)")]
pub fn stub_243270() -> ! {
    todo!("0x243270 boost::thread_specific_ptr<std::string>::delete_data::operator()(void *)")
}

// 0x2432c4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()")]
pub fn stub_2432c4() -> ! {
    todo!("0x2432c4 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()")
}

// 0x2432c8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()")]
pub fn stub_2432c8() -> ! {
    todo!("0x2432c8 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()")
}

// 0x2432d4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::dispose(void)")]
pub fn stub_2432d4() -> ! {
    todo!("0x2432d4 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::dispose(void)")
}

// 0x2432e8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_deleter(std::type_info const&)")]
pub fn stub_2432e8() -> ! {
    todo!("0x2432e8 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_deleter(std::type_info const&)")
}

// 0x243300 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_untyped_deleter(void)")]
pub fn stub_243300() -> ! {
    todo!("0x243300 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_untyped_deleter(void)")
}

// 0x243304 — __ZN5boost22condition_variable_anyC2Ev
// type: boost::condition_variable_any *__fastcall(boost::condition_variable_any *this)
#[doc(alias = "boost::condition_variable_any::condition_variable_any(void)")]
pub fn stub_243304() -> ! {
    todo!("0x243304 boost::condition_variable_any::condition_variable_any(void)")
}

// 0x2434dc — __GLOBAL__I_a_44
#[doc(alias = "global constructor keyed to_a_44")]
pub fn stub_2434dc() -> ! {
    todo!("0x2434dc global constructor keyed to'_a_44")
}

// 0x2435a4 — __ZN3RBX6CEvent4WaitEv
// type: int __fastcall(RBX::CEvent *this, int, int)
#[doc(alias = "RBX::CEvent::Wait(void)")]
pub fn stub_2435a4() -> ! {
    todo!("0x2435a4 RBX::CEvent::Wait(void)")
}

// 0x2435b4 — __ZN3RBX6CEvent19WaitForSingleObjectERS0_i
// type: int __fastcall(RBX::CEvent *this, int, int)
#[doc(alias = "RBX::CEvent::WaitForSingleObject(RBX::CEvent&,int)")]
pub fn stub_2435b4() -> ! {
    todo!("0x2435b4 RBX::CEvent::WaitForSingleObject(RBX::CEvent&,int)")
}

// 0x24381c — __ZN3RBX6CEvent4WaitEi
// type: bool __fastcall(RBX::CEvent *this, int, int)
#[doc(alias = "RBX::CEvent::Wait(int)")]
pub fn stub_24381c() -> ! {
    todo!("0x24381c RBX::CEvent::Wait(int)")
}

// 0x243830 — __ZN3RBX6CEventD1Ev
// type: void __fastcall(RBX::CEvent *__hidden this)
#[doc(alias = "RBX::CEvent::~CEvent()")]
pub fn stub_243830() -> ! {
    todo!("0x243830 RBX::CEvent::~CEvent()")
}

// 0x24383c — __ZN3RBX6CEventD2Ev
// type: void __fastcall(RBX::CEvent *__hidden this)
#[doc(alias = "RBX::CEvent::~CEvent()")]
pub fn stub_24383c() -> ! {
    todo!("0x24383c RBX::CEvent::~CEvent()")
}

// 0x243944 — __ZN3RBX6CEventC1Eb
// type: RBX::CEvent *__fastcall(RBX::CEvent *this, bool)
#[doc(alias = "RBX::CEvent::CEvent(bool)")]
pub fn stub_243944() -> ! {
    todo!("0x243944 RBX::CEvent::CEvent(bool)")
}

// 0x243a30 — __ZN3RBX6CEvent3SetEv
// type: void __fastcall(RBX::CEvent *this)
#[doc(alias = "RBX::CEvent::Set(void)")]
pub fn stub_243a30() -> ! {
    todo!("0x243a30 RBX::CEvent::Set(void)")
}

// 0x243b84 — __ZN5boost18condition_variable13do_wait_untilERNS_11unique_lockINS_5mutexEEERK8timespec
// type: int __fastcall(int, int, const timespec *)
#[doc(alias = "boost::condition_variable::do_wait_until(boost::unique_lock<boost::mutex> &,timespec const&)")]
pub fn stub_243b84() -> ! {
    todo!("0x243b84 boost::condition_variable::do_wait_until(boost::unique_lock<boost::mutex> &,timespec const&)")
}

// 0x243dd0 — __GLOBAL__I_a_45
#[doc(alias = "global constructor keyed to_a_45")]
pub fn stub_243dd0() -> ! {
    todo!("0x243dd0 global constructor keyed to'_a_45")
}

// 0x243e98 — __ZN3RBX6Limits9CountableC2Ev
// type: RBX::Limits::Countable *__fastcall(RBX::Limits::Countable *this, int, int, int)
#[doc(alias = "RBX::Limits::Countable::Countable(void)")]
pub fn stub_243e98() -> ! {
    todo!("0x243e98 RBX::Limits::Countable::Countable(void)")
}

// 0x244088 — __ZN3RBX6Limits7Counter3addEPNS0_9CountableE
// type: void __fastcall(int32_t *, volatile int *)
#[doc(alias = "RBX::Limits::Counter::add(RBX::Limits::Countable *)")]
pub fn stub_244088() -> ! {
    todo!("0x244088 RBX::Limits::Counter::add(RBX::Limits::Countable *)")
}

// 0x244200 — __ZN3RBX6Limits9CountableD2Ev
// type: void __fastcall(int32_t **this, volatile int *)
#[doc(alias = "RBX::Limits::Countable::~Countable()")]
pub fn stub_244200() -> ! {
    todo!("0x244200 RBX::Limits::Countable::~Countable()")
}

// 0x2442c4 — __ZN3RBX6Limits7Counter15getCurrentCountEv
// type: _DWORD __fastcall(RBX::Limits::Counter *__hidden this)
#[doc(alias = "RBX::Limits::Counter::getCurrentCount(void)")]
pub fn stub_2442c4() -> ! {
    todo!("0x2442c4 RBX::Limits::Counter::getCurrentCount(void)")
}

// 0x244358 — __ZN3RBX6Limits7Counter6canAddEi
// type: bool __fastcall(RBX::Limits::Counter *this, int)
#[doc(alias = "RBX::Limits::Counter::canAdd(int)")]
pub fn stub_244358() -> ! {
    todo!("0x244358 RBX::Limits::Counter::canAdd(int)")
}

// 0x244384 — __ZN3RBX6Limits7Counter9ActivatorC1EN5boost10shared_ptrIS1_EE
// was: RBX::Limits::Counter::Activator::Activator(boost::shared_ptr<RBX::Limits::Counter>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::Limits::Counter::Activator::Activator(rbx_core::SharedPtr<RBX::Limits::Counter>)")]
pub fn stub_244384() -> ! {
    todo!("0x244384 RBX::Limits::Counter::Activator::Activator(rbx_core::SharedPtr<RBX::Limits::Counter>)")
}

// 0x244390 — __ZN3RBX6Limits7Counter9ActivatorC2EN5boost10shared_ptrIS1_EE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, void *, int, int, int, int)
// was: RBX::Limits::Counter::Activator::Activator(boost::shared_ptr<RBX::Limits::Counter>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::Limits::Counter::Activator::Activator(rbx_core::SharedPtr<RBX::Limits::Counter>)")]
pub fn stub_244390() -> ! {
    todo!("0x244390 RBX::Limits::Counter::Activator::Activator(rbx_core::SharedPtr<RBX::Limits::Counter>)")
}

// 0x2445fc — __ZN3RBX6Limits7Counter9ActivatorD1Ev
// type: void __fastcall(RBX::Limits::Counter::Activator *__hidden this)
#[doc(alias = "RBX::Limits::Counter::Activator::~Activator()")]
pub fn stub_2445fc() -> ! {
    todo!("0x2445fc RBX::Limits::Counter::Activator::~Activator()")
}

// 0x244608 — __ZN3RBX6Limits7Counter9ActivatorD2Ev
// type: void __fastcall(RBX::Limits::Counter::Activator *this, int, int, int)
#[doc(alias = "RBX::Limits::Counter::Activator::~Activator()")]
pub fn stub_244608() -> ! {
    todo!("0x244608 RBX::Limits::Counter::Activator::~Activator()")
}

// 0x24480c — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE5resetEPS5_
// type: void __fastcall(int *, const void *)
// was: boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::reset(boost::shared_ptr<RBX::Limits::Counter>*) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::reset(rbx_core::SharedPtr<RBX::Limits::Counter>*)")]
pub fn stub_24480c() -> ! {
    todo!("0x24480c boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::reset(rbx_core::SharedPtr<RBX::Limits::Counter>*)")
}

// 0x244928 — __ZN3RBX6Limits7Counter24safe_static_init_currentEv
// type: int __fastcall(RBX::Limits::Counter *this)
#[doc(alias = "RBX::Limits::Counter::safe_static_init_current(void)")]
pub fn stub_244928() -> ! {
    todo!("0x244928 RBX::Limits::Counter::safe_static_init_current(void)")
}

// 0x244934 — __ZN3RBX6Limits7Counter26safe_static_do_get_currentEv
// type: int *__fastcall(RBX::Limits::Counter *this)
#[doc(alias = "RBX::Limits::Counter::safe_static_do_get_current(void)")]
pub fn stub_244934() -> ! {
    todo!("0x244934 RBX::Limits::Counter::safe_static_do_get_current(void)")
}

// 0x244ab8 — __ZN3rbx26thread_specific_shared_ptrIN3RBX6Limits7CounterEED1Ev
#[doc(alias = "rbx::thread_specific_shared_ptr<RBX::Limits::Counter>::~thread_specific_shared_ptr()")]
pub fn stub_244ab8() -> ! {
    todo!("0x244ab8 rbx::thread_specific_shared_ptr<RBX::Limits::Counter>::~thread_specific_shared_ptr()")
}

// 0x244ac8 — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEED2Ev
// type: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::~thread_specific_ptr() (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::~thread_specific_ptr()")]
pub fn stub_244ac8() -> ! {
    todo!("0x244ac8 boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::~thread_specific_ptr()")
}

// 0x244bbc — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD1Ev
// type: void()
// was: boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data::~delete_data() (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::~delete_data()")]
pub fn stub_244bbc() -> ! {
    todo!("0x244bbc boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::~delete_data()")
}

// 0x244bc0 — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD0Ev
// type: void __fastcall(void *)
// was: boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data::~delete_data() (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::~delete_data()")]
pub fn stub_244bc0() -> ! {
    todo!("0x244bc0 boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::~delete_data()")
}

// 0x244bcc — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataclEPv
// type: void __fastcall(int, _DWORD *)
// was: boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data::operator()(void *) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::operator()(void *)")]
pub fn stub_244bcc() -> ! {
    todo!("0x244bcc boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::operator()(void *)")
}

// 0x244c74 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED1Ev
// type: void()
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd() (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()")]
pub fn stub_244c74() -> ! {
    todo!("0x244c74 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()")
}

// 0x244c78 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED0Ev
// type: void __fastcall(void *)
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd() (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()")]
pub fn stub_244c78() -> ! {
    todo!("0x244c78 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()")
}

// 0x244c84 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE7disposeEv
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::dispose(void) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::dispose(void)")]
pub fn stub_244c84() -> ! {
    todo!("0x244c84 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::dispose(void)")
}

// 0x244c98 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::get_deleter(std::type_info const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_deleter(std::type_info const&)")]
pub fn stub_244c98() -> ! {
    todo!("0x244c98 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_deleter(std::type_info const&)")
}

// 0x244cb0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE19get_untyped_deleterEv
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::get_untyped_deleter(void) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_untyped_deleter(void)")]
pub fn stub_244cb0() -> ! {
    todo!("0x244cb0 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_untyped_deleter(void)")
}

// 0x244cb4 — __GLOBAL__I_a_46
#[doc(alias = "global constructor keyed to_a_46")]
pub fn stub_244cb4() -> ! {
    todo!("0x244cb4 global constructor keyed to'_a_46")
}

// 0x244d7c — __ZN3RBX16roblox_allocator6mallocEm
// type: void *__fastcall(size_t this, unsigned int)
#[doc(alias = "RBX::roblox_allocator::malloc(unsigned long)")]
pub fn stub_244d7c() -> ! {
    todo!("0x244d7c RBX::roblox_allocator::malloc(unsigned long)")
}

// 0x244dac — __ZN3RBX16roblox_allocator4freeEPc
// type: void __fastcall(RBX::roblox_allocator *this, char *)
#[doc(alias = "RBX::roblox_allocator::free(char *)")]
pub fn stub_244dac() -> ! {
    todo!("0x244dac RBX::roblox_allocator::free(char *)")
}

// 0x244db8 — __ZNSt6vectorIPmSaIS0_EED1Ev
// type: void **__fastcall(void **)
#[doc(alias = "std::vector<unsigned long *,std::allocator<unsigned long *>>::~vector()")]
pub fn stub_244db8() -> ! {
    todo!("0x244db8 std::vector<unsigned long *,std::allocator<unsigned long *>>::~vector()")
}

// 0x244dcc — __ZNSt6vectorIPFbvESaIS1_EED1Ev
// type: void **__fastcall(void **)
#[doc(alias = "std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::~vector()")]
pub fn stub_244dcc() -> ! {
    todo!("0x244dcc std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::~vector()")
}

// 0x244de0 — __GLOBAL__I_a_47
// type: int()
#[doc(alias = "global constructor keyed to_a_47")]
pub fn stub_244de0() -> ! {
    todo!("0x244de0 global constructor keyed to'_a_47")
}

// 0x244e94 — __ZNK3rbx7signals10connection10disconnectEv
// type: void __fastcall(int32_t **this)
#[doc(alias = "rbx::signals::connection::disconnect(void)const")]
pub fn stub_244e94() -> ! {
    todo!("0x244e94 rbx::signals::connection::disconnect(void)const")
}

// 0x244fd4 — __ZNK3rbx7signals10connection9connectedEv
// type: int __fastcall(rbx::signals::connection *this)
#[doc(alias = "rbx::signals::connection::connected(void)const")]
pub fn stub_244fd4() -> ! {
    todo!("0x244fd4 rbx::signals::connection::connected(void)const")
}

// 0x245118 — __ZNK3rbx7signals10connectioneqERKS1_
// type: bool __fastcall(int32_t, int32_t **, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection::operator==(rbx::signals::connection const&)const")]
pub fn stub_245118() -> ! {
    todo!("0x245118 rbx::signals::connection::operator==(rbx::signals::connection const&)const")
}

// 0x2452d0 — __ZNK3rbx7signals10connectionneERKS1_
// type: bool __fastcall(int32_t, int32_t **, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection::operator!=(rbx::signals::connection const&)const")]
pub fn stub_2452d0() -> ! {
    todo!("0x2452d0 rbx::signals::connection::operator!=(rbx::signals::connection const&)const")
}

// 0x245488 — __ZN3rbx7signals10connectionaSERKS1_
// type: int *__fastcall(int *, int *)
#[doc(alias = "rbx::signals::connection::operator=(rbx::signals::connection const&)")]
pub fn stub_245488() -> ! {
    todo!("0x245488 rbx::signals::connection::operator=(rbx::signals::connection const&)")
}

// 0x24551c — __ZN5boost8functionIFvRSt9exceptionEED1Ev
// type: int *__fastcall(int *)
#[doc(alias = "boost::function<void ()(std::exception &)>::~function()")]
pub fn stub_24551c() -> ! {
    todo!("0x24551c boost::function<void ()(std::exception &)>::~function()")
}

// 0x245544 — __ZN4Init14initStaticDataEv
// type: void __fastcall(Init *this)
#[doc(alias = "Init::initStaticData(void)")]
pub fn stub_245544() -> ! {
    todo!("0x245544 Init::initStaticData(void)")
}

// 0x245548 — __GLOBAL__I_a_48
#[doc(alias = "global constructor keyed to_a_48")]
pub fn stub_245548() -> ! {
    todo!("0x245548 global constructor keyed to'_a_48")
}

// 0x2456a0 — __ZN3RBX5Tasks12SequenceBase11isInhibitedEPNS_13TaskScheduler3JobE
// type: bool __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::SequenceBase::isInhibited(RBX::TaskScheduler::Job *)")]
pub fn stub_2456a0() -> ! {
    todo!("0x2456a0 RBX::Tasks::SequenceBase::isInhibited(RBX::TaskScheduler::Job *)")
}

// 0x2456d8 — __ZN3RBX5Tasks12SequenceBase7advanceEv
// type: int __fastcall(RBX::Tasks::SequenceBase *this)
#[doc(alias = "RBX::Tasks::SequenceBase::advance(void)")]
pub fn stub_2456d8() -> ! {
    todo!("0x2456d8 RBX::Tasks::SequenceBase::advance(void)")
}

// 0x245708 — __ZN3RBX5Tasks12SequenceBase7onAddedEPNS_13TaskScheduler3JobE
// type: void __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::SequenceBase::onAdded(RBX::TaskScheduler::Job *)")]
pub fn stub_245708() -> ! {
    todo!("0x245708 RBX::Tasks::SequenceBase::onAdded(RBX::TaskScheduler::Job *)")
}

// 0x2457f0 — __ZN3RBX5Tasks12SequenceBase9onRemovedEPNS_13TaskScheduler3JobE
// type: int __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::SequenceBase::onRemoved(RBX::TaskScheduler::Job *)")]
pub fn stub_2457f0() -> ! {
    todo!("0x2457f0 RBX::Tasks::SequenceBase::onRemoved(RBX::TaskScheduler::Job *)")
}

// 0x245848 — __ZNSt6vectorIPN3RBX13TaskScheduler3JobESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: void *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job **,std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>>,RBX::TaskScheduler::Job * const&)")]
pub fn stub_245848() -> ! {
    todo!("0x245848 std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job **,std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>>,RBX::TaskScheduler::Job * const&)")
}

// 0x245940 — __GLOBAL__I_a_49
#[doc(alias = "global constructor keyed to_a_49")]
pub fn stub_245940() -> ! {
    todo!("0x245940 global constructor keyed to'_a_49")
}

// 0x245a08 — __ZNK3RBX13TaskScheduler30getSchedulerDutyCyclePerThreadEv
// type: __int64 __fastcall(RBX::TaskScheduler *this)
#[doc(alias = "RBX::TaskScheduler::getSchedulerDutyCyclePerThread(void)const")]
pub fn stub_245a08() -> ! {
    todo!("0x245a08 RBX::TaskScheduler::getSchedulerDutyCyclePerThread(void)const")
}

// 0x245ab0 — __ZN3RBX16ExclusiveArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_
// type: int __fastcall(RBX::ExclusiveArbiter *this, RBX::TaskScheduler::Job *, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::ExclusiveArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
pub fn stub_245ab0() -> ! {
    todo!("0x245ab0 RBX::ExclusiveArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")
}

// 0x245b68 — __ZN3RBX13TaskScheduler11static_initEv
// type: void __fastcall(RBX::TaskScheduler *this, int, int, int)
#[doc(alias = "RBX::TaskScheduler::static_init(void)")]
pub fn stub_245b68() -> ! {
    todo!("0x245b68 RBX::TaskScheduler::static_init(void)")
}

// 0x245c64 — __ZN3RBX13TaskSchedulerD1Ev
// type: void __fastcall(RBX::TaskScheduler *__hidden this)
#[doc(alias = "RBX::TaskScheduler::~TaskScheduler()")]
pub fn stub_245c64() -> ! {
    todo!("0x245c64 RBX::TaskScheduler::~TaskScheduler()")
}

// 0x245c70 — __ZN3RBX13TaskScheduler9singletonEv
// type: _DWORD __fastcall(RBX::TaskScheduler *__hidden this)
#[doc(alias = "RBX::TaskScheduler::singleton(void)")]
pub fn stub_245c70() -> ! {
    todo!("0x245c70 RBX::TaskScheduler::singleton(void)")
}

// 0x245c94 — __ZN3RBX13TaskSchedulerC2Ev
// type: int __fastcall(RBX::TaskScheduler *this, int, int)
#[doc(alias = "RBX::TaskScheduler::TaskScheduler(void)")]
pub fn stub_245c94() -> ! {
    todo!("0x245c94 RBX::TaskScheduler::TaskScheduler(void)")
}

// 0x246308 — __ZN3RBX13TaskScheduler21sampleRunningJobCountEv
// type: bool __fastcall(RBX::TaskScheduler *this, int, int)
#[doc(alias = "RBX::TaskScheduler::sampleRunningJobCount(void)")]
pub fn stub_246308() -> ! {
    todo!("0x246308 RBX::TaskScheduler::sampleRunningJobCount(void)")
}

// 0x246358 — __ZN3RBX13TaskSchedulerD2Ev
// type: void __fastcall(RBX::TaskScheduler *this, int, int, const void *)
#[doc(alias = "RBX::TaskScheduler::~TaskScheduler()")]
pub fn stub_246358() -> ! {
    todo!("0x246358 RBX::TaskScheduler::~TaskScheduler()")
}

// 0x2467d0 — __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEEbNS1_8functionIFvvEEE
// type: void __fastcall(int, int *, unsigned __int8, int)
// was: RBX::TaskScheduler::remove(boost::shared_ptr<RBX::TaskScheduler::Job>,bool,boost::function<void ()(void)>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,bool,boost::function<void ()(void)>)")]
pub fn stub_2467d0() -> ! {
    todo!("0x2467d0 RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,bool,boost::function<void ()(void)>)")
}

// 0x246a48 — __ZN3RBX13TaskScheduler6removeERKN5boost10shared_ptrINS0_3JobEEENS2_INS_6CEventEEE
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int)
// was: RBX::TaskScheduler::remove(boost::shared_ptr<RBX::TaskScheduler::Job> const&,boost::shared_ptr<RBX::CEvent>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&,rbx_core::SharedPtr<RBX::CEvent>)")]
pub fn stub_246a48() -> ! {
    todo!("0x246a48 RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&,rbx_core::SharedPtr<RBX::CEvent>)")
}

// 0x246da8 — __ZN3RBX13TaskScheduler10rescheduleEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, RBX::TaskScheduler::Job **)
// was: RBX::TaskScheduler::reschedule(boost::shared_ptr<RBX::TaskScheduler::Job>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::TaskScheduler::reschedule(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
pub fn stub_246da8() -> ! {
    todo!("0x246da8 RBX::TaskScheduler::reschedule(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")
}

// 0x246e98 — __ZN3RBX13TaskScheduler11scheduleJobERNS0_3JobE
// type: int __fastcall(RBX::TaskScheduler *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::TaskScheduler::scheduleJob(RBX::TaskScheduler::Job &)")]
pub fn stub_246e98() -> ! {
    todo!("0x246e98 RBX::TaskScheduler::scheduleJob(RBX::TaskScheduler::Job &)")
}

// 0x246f90 — __ZN3RBX13TaskScheduler3addEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
// was: RBX::TaskScheduler::add(boost::shared_ptr<RBX::TaskScheduler::Job>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::TaskScheduler::add(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
pub fn stub_246f90() -> ! {
    todo!("0x246f90 RBX::TaskScheduler::add(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")
}

// 0x24710c — __ZN3RBX13TaskScheduler20incrementThreadCountEv
// type: int __fastcall(int32_t *this, volatile int *)
#[doc(alias = "RBX::TaskScheduler::incrementThreadCount(void)")]
pub fn stub_24710c() -> ! {
    todo!("0x24710c RBX::TaskScheduler::incrementThreadCount(void)")
}

// 0x24711c — __ZN3RBX13TaskScheduler20decrementThreadCountEv
// type: int __fastcall(int32_t *this, volatile int *)
#[doc(alias = "RBX::TaskScheduler::decrementThreadCount(void)")]
pub fn stub_24711c() -> ! {
    todo!("0x24711c RBX::TaskScheduler::decrementThreadCount(void)")
}

// 0x247130 — __ZNK3RBX13TaskScheduler20getShortestSleepTimeEv
// type: int __fastcall(RBX::TaskScheduler *this, int)
#[doc(alias = "RBX::TaskScheduler::getShortestSleepTime(void)const")]
pub fn stub_247130() -> ! {
    todo!("0x247130 RBX::TaskScheduler::getShortestSleepTime(void)const")
}

// 0x247154 — __ZN3RBX13TaskScheduler16wakeSleepingJobsEv
// type: int __fastcall(RBX::TaskScheduler *this)
#[doc(alias = "RBX::TaskScheduler::wakeSleepingJobs(void)")]
pub fn stub_247154() -> ! {
    todo!("0x247154 RBX::TaskScheduler::wakeSleepingJobs(void)")
}

// 0x247220 — __ZN3RBX13TaskScheduler12findJobToRunEN5boost10shared_ptrINS0_6ThreadEEE
// type: void __fastcall(RBX::TaskScheduler::Job **, int, int *, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: RBX::TaskScheduler::findJobToRun(boost::shared_ptr<RBX::TaskScheduler::Thread>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::TaskScheduler::findJobToRun(rbx_core::SharedPtr<RBX::TaskScheduler::Thread>)")]
pub fn stub_247220() -> ! {
    todo!("0x247220 RBX::TaskScheduler::findJobToRun(rbx_core::SharedPtr<RBX::TaskScheduler::Thread>)")
}

// 0x247bd8 — __ZN3rbx25thread_specific_referenceIN3RBX13TaskScheduler3JobEED1Ev
#[doc(alias = "rbx::thread_specific_reference<RBX::TaskScheduler::Job>::~thread_specific_reference()")]
pub fn stub_247bd8() -> ! {
    todo!("0x247bd8 rbx::thread_specific_reference<RBX::TaskScheduler::Job>::~thread_specific_reference()")
}

// 0x247be8 — __ZNK3RBX13TaskScheduler3Job12getDebugNameEv
// type: void __fastcall(RBX::TaskScheduler::Job *this, int)
#[doc(alias = "RBX::TaskScheduler::Job::getDebugName(void)const")]
pub fn stub_247be8() -> ! {
    todo!("0x247be8 RBX::TaskScheduler::Job::getDebugName(void)const")
}

// 0x247db0 — __ZN3RBX14RunningAverageIidE6sampleEi
// type: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::RunningAverage<int,double>::sample(int)")]
pub fn stub_247db0() -> ! {
    todo!("0x247db0 RBX::RunningAverage<int,double>::sample(int)")
}

// 0x247e74 — __ZN3RBX16ExclusiveArbiter11arbiterNameEv
// type: int __fastcall(RBX::ExclusiveArbiter *this)
#[doc(alias = "RBX::ExclusiveArbiter::arbiterName(void)")]
pub fn stub_247e74() -> ! {
    todo!("0x247e74 RBX::ExclusiveArbiter::arbiterName(void)")
}

// 0x247e90 — __ZN3RBX16ExclusiveArbiter11isThrottledEv
// type: int __fastcall(RBX::ExclusiveArbiter *this)
#[doc(alias = "RBX::ExclusiveArbiter::isThrottled(void)")]
pub fn stub_247e90() -> ! {
    todo!("0x247e90 RBX::ExclusiveArbiter::isThrottled(void)")
}

// 0x247e94 — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
// was: boost::shared_ptr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(boost::weak_ptr<RBX::TaskScheduler::Job> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(rbx_core::Weak<RBX::TaskScheduler::Job> const&)")]
pub fn stub_247e94() -> ! {
    todo!("0x247e94 rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(boost::weak_ptr<RBX::TaskScheduler::Job> const&)")
}

// 0x247fac — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, _Rb_tree_node_base *)
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>)")]
pub fn stub_247fac() -> ! {
    todo!("0x247fac std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>)")
}

// 0x248020 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<boost::shared_ptr<RBX::TaskScheduler::Job>> *) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::TaskScheduler::Job>> *)")]
pub fn stub_248020() -> ! {
    todo!("0x248020 std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::TaskScheduler::Job>> *)")
}

// 0x248050 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, _DWORD *, unsigned int M_parent, int)
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_insert_unique(boost::shared_ptr<RBX::TaskScheduler::Job> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_248050() -> ! {
    todo!("0x248050 std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")
}

// 0x248104 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_create_node(boost::shared_ptr<RBX::TaskScheduler::Job> const&) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_create_node(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_248104() -> ! {
    todo!("0x248104 std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_create_node(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")
}

// 0x248224 — __ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CEvent>(RBX::CEvent *)")]
pub fn stub_248224() -> ! {
    todo!("0x248224 boost::detail::shared_count::shared_count<RBX::CEvent>(RBX::CEvent *)")
}

// 0x24831c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()")]
pub fn stub_24831c() -> ! {
    todo!("0x24831c boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()")
}

// 0x248320 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()")]
pub fn stub_248320() -> ! {
    todo!("0x248320 boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()")
}

// 0x24832c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::dispose(void)")]
pub fn stub_24832c() -> ! {
    todo!("0x24832c boost::detail::sp_counted_impl_p<RBX::CEvent>::dispose(void)")
}

// 0x24834c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::get_deleter(std::type_info const&)")]
pub fn stub_24834c() -> ! {
    todo!("0x24834c boost::detail::sp_counted_impl_p<RBX::CEvent>::get_deleter(std::type_info const&)")
}

// 0x248350 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::get_untyped_deleter(void)")]
pub fn stub_248350() -> ! {
    todo!("0x248350 boost::detail::sp_counted_impl_p<RBX::CEvent>::get_untyped_deleter(void)")
}

// 0x248358 — __ZN5boost6detail11thread_dataINS_9function0IvEEED1Ev
// type: int __fastcall(boost::detail::thread_data_base *)
#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::~thread_data()")]
pub fn stub_248358() -> ! {
    todo!("0x248358 boost::detail::thread_data<boost::function0<void>>::~thread_data()")
}

// 0x248448 — __ZN5boost18condition_variableC2Ev
// type: boost::condition_variable *__fastcall(boost::condition_variable *this)
#[doc(alias = "boost::condition_variable::condition_variable(void)")]
pub fn stub_248448() -> ! {
    todo!("0x248448 boost::condition_variable::condition_variable(void)")
}

// 0x248620 — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_9function0IvEEEEEEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
// was: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const")]
pub fn stub_248620() -> ! {
    todo!("0x248620 void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const")
}

// 0x248778 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::get_untyped_deleter(void)")]
pub fn stub_248778() -> ! {
    todo!("0x248778 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::get_untyped_deleter(void)")
}

// 0x24877c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_24877c() -> ! {
    todo!("0x24877c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x2487dc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_2487dc() -> ! {
    todo!("0x2487dc boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x2487f8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&)")]
pub fn stub_2487f8() -> ! {
    todo!("0x2487f8 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&)")
}

// 0x248938 — __ZN5boost9function0IvE5dummy7nonnullEv
// type: void()
#[doc(alias = "boost::function0<void>::dummy::nonnull(void)")]
pub fn stub_248938() -> ! {
    todo!("0x248938 boost::function0<void>::dummy::nonnull(void)")
}

// 0x248940 — __ZN3RBX5mutexC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *this)
#[doc(alias = "RBX::mutex::mutex(void)")]
pub fn stub_248940() -> ! {
    todo!("0x248940 RBX::mutex::mutex(void)")
}

// 0x248a8c — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev
// type: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::~thread_specific_ptr()")]
pub fn stub_248a8c() -> ! {
    todo!("0x248a8c boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::~thread_specific_ptr()")
}

// 0x248b80 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD1Ev
// type: void()
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()")]
pub fn stub_248b80() -> ! {
    todo!("0x248b80 boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()")
}

// 0x248b84 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()")]
pub fn stub_248b84() -> ! {
    todo!("0x248b84 boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()")
}

// 0x248b90 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataclEPv
// type: void __fastcall(int, void *)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::operator()(void *)")]
pub fn stub_248b90() -> ! {
    todo!("0x248b90 boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::operator()(void *)")
}

// 0x248ba0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()")]
pub fn stub_248ba0() -> ! {
    todo!("0x248ba0 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()")
}

// 0x248ba4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()")]
pub fn stub_248ba4() -> ! {
    todo!("0x248ba4 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()")
}

// 0x248bb0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::dispose(void)")]
pub fn stub_248bb0() -> ! {
    todo!("0x248bb0 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::dispose(void)")
}
