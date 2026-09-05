// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: EA-sorted asc next 150 not yet in datamodel (strict RBX::Instance|RBX::DataModel|RBX::Workspace 0 remaining, fallback gap filler)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 150 stubs | range 0x23e5dc..0x248104 | strict 10215 complete (0 remaining), datamodel distinct 29652->29802
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias where needed

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x23e5dc — __ZN3RBX9Debugable7doCrashEPKc
// type: int __fastcall(RBX::Debugable *this, const char *)
#[doc(alias = "RBX::Debugable::doCrash(char const*)")]
pub use rbx_core::generated_core_shard_bw::stub_23e5dc as stub_23e5dc;

// 0x23e5f8 — __ZN3RBX9Debugable4dumpERSo
// type: int __fastcall(RBX::Debugable *this, std::ostream *)
#[doc(alias = "RBX::Debugable::dump(std::ostream &)")]
pub use rbx_core::generated_core_shard_hs::stub_0x23e5f8 as stub_23e5f8;

// 0x23e638 — __ZN3RBX15DebugNameStringC1EPKci
// type: _QWORD *__fastcall(_QWORD *this, const char *, unsigned int)
#[doc(alias = "RBX::DebugNameString::DebugNameString(char const*,int)")]
pub use rbx_core::generated_core_shard_bw::stub_23e638 as stub_23e638;

// 0x23e644 — __ZN3RBX15DebugNameString16getNameIncrementEPKc
// type: char *__fastcall(RBX::DebugNameString *this, const char *)
#[doc(alias = "RBX::DebugNameString::getNameIncrement(char const*)")]
pub use rbx_core::generated_core_shard_bw::stub_23e644 as stub_23e644;

// 0x23e66c — __Z10DebugBreakv
// type: void __fastcall __noreturn()
#[doc(alias = "DebugBreak(void)")]
pub use rbx_core::generated_core_shard_ac::stub_0x23e66c as stub_23e66c;

// 0x23e678 — __ZN3RBX3Log9timeStampERSt14basic_ofstreamIcSt11char_traitsIcEEb
// type: int __fastcall(std::ostream *, int)
#[doc(alias = "RBX::Log::timeStamp(std::basic_ofstream<char,std::char_traits<char>> &,bool)")]
pub use rbx_core::generated_core_shard_hs::stub_0x23e678 as stub_23e678;

// 0x23e988 — __ZN3RBX3Log10writeEntryENS0_8SeverityEPKc
// type: int __fastcall(int, int, const char *)
#[doc(alias = "RBX::Log::writeEntry(RBX::Log::Severity,char const*)")]
pub use rbx_core::generated_core_shard_bw::stub_23e988 as stub_23e988;

// 0x23ea18 — __ZN3RBX3Log9formatMemEj
// type: int __fastcall(RBX::Log *this, unsigned int)
#[doc(alias = "RBX::Log::formatMem(unsigned int)")]
pub use rbx_core::generated_core_shard_bw::stub_23ea18 as stub_23ea18;

// 0x23eb48 — __ZN3RBX3Log10formatTimeEd
// type: int __fastcall(RBX::Log *this, double)
#[doc(alias = "RBX::Log::formatTime(double)")]
pub use rbx_core::generated_core_shard_bw::stub_23eb48 as stub_23eb48;

// 0x23ec00 — __Z11initBaseLogv
// type: void __fastcall()
#[doc(alias = "initBaseLog(void)")]
pub use rbx_core::generated_core_shard_ac::stub_0x23ec00 as stub_23ec00;

// 0x23ec04 — __ZN5boost9date_time23gregorian_calendar_baseINS0_19year_month_day_baseINS_9gregorian9greg_yearENS3_10greg_monthENS3_8greg_dayEEEjE15from_day_numberEj
// type: _WORD *__fastcall(_WORD *result, int)
#[doc(alias = "boost::date_time::gregorian_calendar_base<boost::date_time::year_month_day_base<boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day>,unsigned int>::from_day_number(unsigned int)")]
pub use rbx_core::generated_core_shard_hs::stub_0x23ec04 as stub_23ec04;

// 0x23ecfc — __ZN5boost9date_time12second_clockINS_10posix_time5ptimeEE11create_timeEP2tm
// type: int __fastcall(_DWORD *, __int64 *)
#[doc(alias = "boost::date_time::second_clock<boost::posix_time::ptime>::create_time(tm *)")]
pub use rbx_core::generated_core_shard_hs::stub_0x23ecfc as stub_23ecfc;

// 0x23ef20 — __ZNK5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEE4dateEv
// type: unsigned int __fastcall(__int64 *)
#[doc(alias = "boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::date(void)const")]
pub use rbx_core::generated_core_shard_hs::stub_0x23ef20 as stub_23ef20;

// 0x23f068 — __GLOBAL__I_a_43
// type: int()
#[doc(alias = "global constructor keyed to_a_43")]
// was: global constructor keyed to_a_43
pub use rbx_core::generated_core_shard_ac::stub_0x23f068 as stub_23f068;

// 0x23f294 — __Z8RBXCRASHv
// type: int __fastcall(RBX::Debugable *)
#[doc(alias = "RBXCRASH(void)")]
pub use rbx_core::generated_core_shard_ac::stub_0x23f294 as stub_23f294;

// 0x23f2a0 — __Z8RBXCRASHPKc
// type: int __fastcall(RBX::Debugable *, const char *)
#[doc(alias = "RBXCRASH(char const*)")]
pub use rbx_core::generated_core_shard_ac::stub_0x23f2a0 as stub_23f2a0;

// 0x23f2ac — __ZN3RBX12boost_detail8init_fooEv
// type: void __fastcall(RBX::boost_detail *this)
#[doc(alias = "RBX::boost_detail::init_foo(void)")]
pub use rbx_core::generated_core_shard_hs::stub_0x23f2ac as stub_23f2ac;

// 0x23f42c — __ZN3RBX15set_thread_nameEPKc
// type: void __fastcall(RBX *this, const char *, int, int)
#[doc(alias = "RBX::set_thread_name(char const*)")]
pub use rbx_core::generated_core_shard_bw::stub_23f42c as stub_23f42c;

// 0x23f50c — __ZN3RBX14thread_wrapperERKN5boost9function0IvEEPKc
// type: void __fastcall(_DWORD *, int *, int)
#[doc(alias = "RBX::thread_wrapper(boost::function0<void> const&,char const*)")]
pub use rbx_core::generated_core_shard_hs::stub_0x23f50c as stub_23f50c;

// 0x23f8f0 — __ZN3RBXL15thread_functionERKN5boost9function0IvEESs
// type: void __fastcall(int, int *, int, int)
#[doc(alias = "RBX::thread_function(boost::function0<void> const&,std::string)")]
pub use rbx_core::generated_core_shard_hs::stub_0x23f8f0 as stub_23f8f0;

// 0x23fa10 — __ZN3RBX13worker_threadC1ERKN5boost9function0INS0_11work_resultEEEPKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::worker_thread::worker_thread(boost::function0<RBX::worker_thread::work_result> const&,char const*)")]
pub use rbx_core::generated_core_shard_hs::stub_0x23fa10 as stub_23fa10;

// 0x23fa1c — __ZN3RBX13worker_threadC2ERKN5boost9function0INS0_11work_resultEEEPKc
// type: int __fastcall(int, int *, boost::detail::sp_counted_base *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, char, int, boost::detail::sp_counted_base *, int, int, int, int, int, pthread_mutex_t *, int, int, int, int, int, int)
#[doc(alias = "RBX::worker_thread::worker_thread(boost::function0<RBX::worker_thread::work_result> const&,char const*)")]
pub use rbx_core::generated_core_shard_hs::stub_0x23fa1c as stub_23fa1c;

// 0x23ffb0 — __ZN3RBX13worker_thread10threadProcEN5boost10shared_ptrINS0_4dataEEERKNS1_9function0INS0_11work_resultEEE
// type: void __fastcall(boost::mutex **, _DWORD *)
#[doc(alias = "RBX::worker_thread::threadProc(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&)")]
pub use rbx_core::generated_core_shard_hs::stub_0x23ffb0 as stub_23ffb0;

// 0x2400f4 — __ZN3RBX13worker_threadD1Ev
// type: void __fastcall(RBX::worker_thread *__hidden this)
#[doc(alias = "RBX::worker_thread::~worker_thread()")]
pub use rbx_reflection::generated_refl_27::stub_0x2400f4 as stub_2400f4;

// 0x240100 — __ZN3RBX13worker_threadD2Ev
// type: void __fastcall(boost::mutex **this)
#[doc(alias = "RBX::worker_thread::~worker_thread()")]
pub use rbx_reflection::generated_refl_27::stub_0x240100 as stub_240100;

// 0x2402c4 — __ZN3RBX13worker_thread4wakeEv
// type: void __fastcall(boost::mutex **this)
#[doc(alias = "RBX::worker_thread::wake(void)")]
pub use rbx_core::generated_core_shard_bw::stub_2402c4 as stub_2402c4;

// 0x2403cc — __ZN5boost19thread_specific_ptrISsED1Ev
#[doc(alias = "boost::thread_specific_ptr<std::string>::~thread_specific_ptr()")]
pub use rbx_reflection::generated_refl_27::stub_0x2403cc as stub_2403cc;

// 0x2403d8 — __ZN5boost19thread_specific_ptrISsE5resetEPSs
// type: void __fastcall(int *, const void *)
#[doc(alias = "boost::thread_specific_ptr<std::string>::reset(std::string *)")]
pub use rbx_core::generated_core_shard_hs::stub_0x2403d8 as stub_2403d8;

// 0x2404f4 — __ZN5boost4bindIvRKNS_9function0IvEESsS2_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
// type: void __fastcall(double *, int, int *, const std::string *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list_av_2<boost::function0<void>,std::string>::type> boost::bind<void,boost::function0<void> const&,std::string,boost::function0<void>,std::string>(void (*)(boost::function0<void> const&,std::string),boost::function0<void>,std::string)")]
pub use rbx_core::generated_core_shard_hs::stub_0x2404f4 as stub_2404f4;

// 0x2407fc — __ZN5boost4bindIvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS3_11work_resultEEES5_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_
// type: void __fastcall(boost::detail::sp_counted_base *, int, int *, int, int, int, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list_av_2<boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>::type> boost::bind<void,boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&,boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>(void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>)")]
pub use rbx_core::generated_core_shard_hs::stub_0x2407fc as stub_2407fc;

// 0x240a54 — __ZN5boost22condition_variable_any4waitINS_11unique_lockINS_5mutexEEEEEvRT_
// type: void __fastcall(int, int)
#[doc(alias = "void boost::condition_variable_any::wait<boost::unique_lock<boost::mutex>>(boost::unique_lock<boost::mutex> &)")]
pub use rbx_core::generated_core_shard_hs::stub_0x240a54 as stub_240a54;

// 0x240c80 — __ZN5boost15throw_exceptionINS_15condition_errorEEEvRKT_
// type: void __fastcall __noreturn(_QWORD *)
#[doc(alias = "void boost::throw_exception<boost::condition_error>(boost::condition_error const&)")]
pub use rbx_core::generated_core_shard_hs::stub_0x240c80 as stub_240c80;

// 0x241040 — __ZN5boost15condition_errorD1Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::condition_error::~condition_error()")]
pub use rbx_reflection::generated_refl_27::stub_0x241040 as stub_241040;

// 0x2410a0 — __ZN5boost15condition_errorD0Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::condition_error::~condition_error()")]
pub use rbx_reflection::generated_refl_27::stub_0x2410a0 as stub_2410a0;

// 0x241108 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED1Ev
// type: std::runtime_error *__fastcall(std::runtime_error *)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()")]
pub use rbx_reflection::generated_refl_27::stub_0x241108 as stub_241108;

// 0x241214 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_15condition_errorEED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::condition_error>::~error_info_injector()")]
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::condition_error>::~error_info_injector()
pub use rbx_reflection::generated_refl_27::stub_0x241214 as stub_241214;

// 0x241324 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()")]
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()
pub use rbx_reflection::generated_refl_27::stub_0x241324 as stub_241324;

// 0x241430 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEE5cloneEv
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::clone(void)const")]
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::clone(void)const
pub use rbx_core::generated_core_shard_hs::stub_0x241430 as stub_241430;

// 0x241444 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS0_INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSH_ISB_EEEEEEEEvT_
// type: void __fastcall(int, int, int, int, char, int, boost::detail::sp_counted_base *, int, int, int, int, char, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>)")]
pub use rbx_core::generated_core_shard_hs::stub_0x241444 as stub_241444;

// 0x241798 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub use rbx_core::generated_core_shard_hs::stub_0x241798 as stub_241798;

// 0x2417bc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(_DWORD *)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub use rbx_core::generated_core_shard_hs::stub_0x2417bc as stub_2417bc;

// 0x2417d0 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS9_11work_resultEEEENS5_5list2INS5_5valueISB_EENSK_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, double *, _DWORD *, int, boost::detail::sp_counted_base *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub use rbx_core::generated_core_shard_hs::stub_0x2417d0 as stub_2417d0;

// 0x241aac — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEclIPFvS7_RKSB_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, int))
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::operator()<void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&) &,boost::_bi::list0 &,int)")]
pub use rbx_core::generated_core_shard_hs::stub_0x241aac as stub_241aac;

// 0x241bbc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub use rbx_core::generated_core_shard_hs::stub_0x241bbc as stub_241bbc;

// 0x241df4 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
pub use rbx_core::generated_core_shard_hs::stub_0x241df4 as stub_241df4;

// 0x241f98 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// type: int __fastcall(int, int *, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
pub use rbx_core::generated_core_shard_hs::stub_0x241f98 as stub_241f98;

// 0x242144 — __ZN5boost3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS4_11work_resultEEEENS0_5list2INS0_5valueIS6_EENSF_IS9_EEEEEC2ESD_RKSI_
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>::bind_t(void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>> const&)")]
pub use rbx_core::generated_core_shard_hs::stub_0x242144 as stub_242144;

// 0x242284 — __ZN5boost6detail20sp_pointer_constructIN3RBX13worker_thread4dataES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, boost::detail::sp_counted_base **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::worker_thread::data,RBX::worker_thread::data>(boost::shared_ptr<RBX::worker_thread::data> *,RBX::worker_thread::data *,boost::detail::shared_count &)")]
pub use rbx_core::generated_core_shard_hs::stub_0x242284 as stub_242284;

// 0x2423c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()")]
pub use rbx_reflection::generated_refl_27::stub_0x2423c8 as stub_2423c8;

// 0x2423cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()")]
pub use rbx_reflection::generated_refl_27::stub_0x2423cc as stub_2423cc;

// 0x2423d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::dispose(void)")]
pub use rbx_core::generated_core_shard_hs::stub_0x2423d8 as stub_2423d8;

// 0x2424bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_deleter(std::type_info const&)")]
pub use rbx_core::generated_core_shard_hs::stub_0x2424bc as stub_2424bc;

// 0x2424c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_untyped_deleter(void)")]
pub use rbx_core::generated_core_shard_hs::stub_0x2424c0 as stub_2424c0;

// 0x2424c4 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_SsENS3_5list2INS3_5valueIS1_EENSA_ISsEEEEEEEEvT_
// type: void __fastcall(_DWORD *, double *)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>)")]
pub use rbx_core::generated_core_shard_hs::stub_0x2424c4 as stub_2424c4;

// 0x242818 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub use rbx_core::generated_core_shard_hs::stub_0x242818 as stub_242818;

// 0x24283c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEvE6invokeERNS1_15function_bufferE
// type: void __fastcall(void (__fastcall ***)(_DWORD, int *))
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub use rbx_core::generated_core_shard_hs::stub_0x24283c as stub_24283c;

// 0x242958 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS5_5list2INS5_5valueIS8_EENSE_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, double *, void **)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub use rbx_core::generated_core_shard_hs::stub_0x242958 as stub_242958;

// 0x242be8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub use rbx_core::generated_core_shard_hs::stub_0x242be8 as stub_242be8;

// 0x242e08 — __ZN5boost3_bi5list2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, const std::string *)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")]
pub use rbx_core::generated_core_shard_hs::stub_0x242e08 as stub_242e08;

// 0x242fc0 — __ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_
// type: _DWORD *__fastcall(_DWORD *, int *, const std::string *)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")]
pub use rbx_core::generated_core_shard_hs::stub_0x242fc0 as stub_242fc0;

// 0x24316c — __ZN5boost19thread_specific_ptrISsED2Ev
// type: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::thread_specific_ptr<std::string>::~thread_specific_ptr()")]
pub use rbx_reflection::generated_refl_27::stub_0x24316c as stub_24316c;

// 0x243260 — __ZN5boost19thread_specific_ptrISsE11delete_dataD1Ev
// type: void()
#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::~delete_data()")]
pub use rbx_reflection::generated_refl_27::stub_0x243260 as stub_243260;

// 0x243264 — __ZN5boost19thread_specific_ptrISsE11delete_dataD0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::~delete_data()")]
pub use rbx_reflection::generated_refl_27::stub_0x243264 as stub_243264;

// 0x243270 — __ZN5boost19thread_specific_ptrISsE11delete_dataclEPv
// type: void __fastcall(int, int *)
#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::operator()(void *)")]
pub use rbx_core::generated_core_shard_hs::stub_0x243270 as stub_243270;

// 0x2432c4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()")]
pub use rbx_reflection::generated_refl_27::stub_0x2432c4 as stub_2432c4;

// 0x2432c8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()")]
pub use rbx_reflection::generated_refl_27::stub_0x2432c8 as stub_2432c8;

// 0x2432d4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::dispose(void)")]
pub use rbx_core::generated_core_shard_hs::stub_0x2432d4 as stub_2432d4;

// 0x2432e8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_deleter(std::type_info const&)")]
pub use rbx_core::generated_core_shard_hs::stub_0x2432e8 as stub_2432e8;

// 0x243300 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_untyped_deleter(void)")]
pub use rbx_core::generated_core_shard_hs::stub_0x243300 as stub_243300;

// 0x243304 — __ZN5boost22condition_variable_anyC2Ev
// type: boost::condition_variable_any *__fastcall(boost::condition_variable_any *this)
#[doc(alias = "boost::condition_variable_any::condition_variable_any(void)")]
pub use rbx_core::generated_core_shard_hs::stub_0x243304 as stub_243304;

// 0x2434dc — __GLOBAL__I_a_44
#[doc(alias = "global constructor keyed to_a_44")]
// was: global constructor keyed to_a_44
pub use rbx_core::generated_core_shard_js::stub_2434dc as stub_2434dc;

// 0x2435a4 — __ZN3RBX6CEvent4WaitEv
// type: int __fastcall(RBX::CEvent *this, int, int)
#[doc(alias = "RBX::CEvent::Wait(void)")]
pub use rbx_core::generated_core_shard_hs::stub_0x2435a4 as stub_2435a4;

// 0x2435b4 — __ZN3RBX6CEvent19WaitForSingleObjectERS0_i
// type: int __fastcall(RBX::CEvent *this, int, int)
#[doc(alias = "RBX::CEvent::WaitForSingleObject(RBX::CEvent&,int)")]
pub use rbx_core::generated_core_shard_hs::stub_0x2435b4 as stub_2435b4;

// 0x24381c — __ZN3RBX6CEvent4WaitEi
// type: bool __fastcall(RBX::CEvent *this, int, int)
#[doc(alias = "RBX::CEvent::Wait(int)")]
pub use rbx_core::boost_skeletons::stub_24381c as stub_24381c;

// 0x243830 — __ZN3RBX6CEventD1Ev
// type: void __fastcall(RBX::CEvent *__hidden this)
#[doc(alias = "RBX::CEvent::~CEvent()")]
pub use rbx_reflection::generated_refl_27::stub_0x243830 as stub_243830;

// 0x24383c — __ZN3RBX6CEventD2Ev
// type: void __fastcall(RBX::CEvent *__hidden this)
#[doc(alias = "RBX::CEvent::~CEvent()")]
pub use rbx_reflection::generated_refl_27::stub_0x24383c as stub_24383c;

// 0x243944 — __ZN3RBX6CEventC1Eb
// type: RBX::CEvent *__fastcall(RBX::CEvent *this, bool)
#[doc(alias = "RBX::CEvent::CEvent(bool)")]
pub use rbx_core::boost_skeletons::stub_243944 as stub_243944;

// 0x243a30 — __ZN3RBX6CEvent3SetEv
// type: void __fastcall(RBX::CEvent *this)
#[doc(alias = "RBX::CEvent::Set(void)")]
pub use rbx_core::boost_skeletons::stub_243a30 as stub_243a30;

// 0x243b84 — __ZN5boost18condition_variable13do_wait_untilERNS_11unique_lockINS_5mutexEEERK8timespec
// type: int __fastcall(int, int, const timespec *)
#[doc(alias = "boost::condition_variable::do_wait_until(boost::unique_lock<boost::mutex> &,timespec const&)")]
pub use rbx_core::boost_skeletons::stub_243b84 as stub_243b84;

// 0x243dd0 — __GLOBAL__I_a_45
#[doc(alias = "global constructor keyed to_a_45")]
// was: global constructor keyed to_a_45
pub use rbx_core::generated_core_shard_js::stub_243dd0 as stub_243dd0;

// 0x243e98 — __ZN3RBX6Limits9CountableC2Ev
// type: RBX::Limits::Countable *__fastcall(RBX::Limits::Countable *this, int, int, int)
#[doc(alias = "RBX::Limits::Countable::Countable(void)")]
pub use rbx_core::boost_skeletons::stub_243e98 as stub_243e98;

// 0x244088 — __ZN3RBX6Limits7Counter3addEPNS0_9CountableE
// type: void __fastcall(int32_t *, volatile int *)
#[doc(alias = "RBX::Limits::Counter::add(RBX::Limits::Countable *)")]
pub use rbx_core::boost_skeletons::stub_244088 as stub_244088;

// 0x244200 — __ZN3RBX6Limits9CountableD2Ev
// type: void __fastcall(int32_t **this, volatile int *)
#[doc(alias = "RBX::Limits::Countable::~Countable()")]
pub use rbx_reflection::generated_refl_27::stub_0x244200 as stub_244200;

// 0x2442c4 — __ZN3RBX6Limits7Counter15getCurrentCountEv
// type: _DWORD __fastcall(RBX::Limits::Counter *__hidden this)
#[doc(alias = "RBX::Limits::Counter::getCurrentCount(void)")]
pub use rbx_core::boost_skeletons::stub_2442c4 as stub_2442c4;

// 0x244358 — __ZN3RBX6Limits7Counter6canAddEi
// type: bool __fastcall(RBX::Limits::Counter *this, int)
#[doc(alias = "RBX::Limits::Counter::canAdd(int)")]
pub use rbx_core::boost_skeletons::stub_244358 as stub_244358;

// 0x244384 — __ZN3RBX6Limits7Counter9ActivatorC1EN5boost10shared_ptrIS1_EE
#[doc(alias = "RBX::Limits::Counter::Activator::Activator(boost::shared_ptr<RBX::Limits::Counter>)")]
pub use rbx_core::boost_skeletons::stub_244384 as stub_244384;

// 0x244390 — __ZN3RBX6Limits7Counter9ActivatorC2EN5boost10shared_ptrIS1_EE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, void *, int, int, int, int)
#[doc(alias = "RBX::Limits::Counter::Activator::Activator(boost::shared_ptr<RBX::Limits::Counter>)")]
pub use rbx_core::boost_skeletons::stub_244390 as stub_244390;

// 0x2445fc — __ZN3RBX6Limits7Counter9ActivatorD1Ev
// type: void __fastcall(RBX::Limits::Counter::Activator *__hidden this)
#[doc(alias = "RBX::Limits::Counter::Activator::~Activator()")]
pub use rbx_reflection::generated_refl_27::stub_0x2445fc as stub_2445fc;

// 0x244608 — __ZN3RBX6Limits7Counter9ActivatorD2Ev
// type: void __fastcall(RBX::Limits::Counter::Activator *this, int, int, int)
#[doc(alias = "RBX::Limits::Counter::Activator::~Activator()")]
pub use rbx_reflection::generated_refl_27::stub_0x244608 as stub_244608;

// 0x24480c — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE5resetEPS5_
// type: void __fastcall(int *, const void *)
#[doc(alias = "boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::reset(boost::shared_ptr<RBX::Limits::Counter>*)")]
pub use rbx_core::boost_skeletons::stub_24480c as stub_24480c;

// 0x244928 — __ZN3RBX6Limits7Counter24safe_static_init_currentEv
// type: int __fastcall(RBX::Limits::Counter *this)
#[doc(alias = "RBX::Limits::Counter::safe_static_init_current(void)")]
pub use rbx_core::boost_skeletons::stub_244928 as stub_244928;

// 0x244934 — __ZN3RBX6Limits7Counter26safe_static_do_get_currentEv
// type: int *__fastcall(RBX::Limits::Counter *this)
#[doc(alias = "RBX::Limits::Counter::safe_static_do_get_current(void)")]
pub use rbx_core::boost_skeletons::stub_244934 as stub_244934;

// 0x244ab8 — __ZN3rbx26thread_specific_shared_ptrIN3RBX6Limits7CounterEED1Ev
#[doc(alias = "rbx::thread_specific_shared_ptr<RBX::Limits::Counter>::~thread_specific_shared_ptr()")]
pub use rbx_reflection::generated_refl_27::stub_0x244ab8 as stub_244ab8;

// 0x244ac8 — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEED2Ev
// type: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::~thread_specific_ptr()")]
pub use rbx_reflection::generated_refl_27::stub_0x244ac8 as stub_244ac8;

// 0x244bbc — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD1Ev
// type: void()
#[doc(alias = "boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data::~delete_data()")]
pub use rbx_reflection::generated_refl_27::stub_0x244bbc as stub_244bbc;

// 0x244bc0 — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data::~delete_data()")]
pub use rbx_reflection::generated_refl_27::stub_0x244bc0 as stub_244bc0;

// 0x244bcc — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataclEPv
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data::operator()(void *)")]
pub use rbx_core::boost_skeletons::stub_244bcc as stub_244bcc;

// 0x244c74 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()")]
pub use rbx_reflection::generated_refl_27::stub_0x244c74 as stub_244c74;

// 0x244c78 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()")]
pub use rbx_reflection::generated_refl_27::stub_0x244c78 as stub_244c78;

// 0x244c84 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::dispose(void)")]
pub use rbx_core::boost_skeletons::stub_244c84 as stub_244c84;

// 0x244c98 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::get_deleter(std::type_info const&)")]
pub use rbx_core::boost_skeletons::stub_244c98 as stub_244c98;

// 0x244cb0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::get_untyped_deleter(void)")]
pub use rbx_core::boost_skeletons::stub_244cb0 as stub_244cb0;

// 0x244cb4 — __GLOBAL__I_a_46
#[doc(alias = "global constructor keyed to_a_46")]
// was: global constructor keyed to_a_46
pub use rbx_core::generated_core_shard_js::stub_244cb4 as stub_244cb4;

// 0x244d7c — __ZN3RBX16roblox_allocator6mallocEm
// type: void *__fastcall(size_t this, unsigned int)
#[doc(alias = "RBX::roblox_allocator::malloc(unsigned long)")]
pub use rbx_core::boost_skeletons::stub_244d7c as stub_244d7c;

// 0x244dac — __ZN3RBX16roblox_allocator4freeEPc
// type: void __fastcall(RBX::roblox_allocator *this, char *)
#[doc(alias = "RBX::roblox_allocator::free(char *)")]
pub use rbx_core::boost_skeletons::stub_244dac as stub_244dac;

// 0x244db8 — __ZNSt6vectorIPmSaIS0_EED1Ev
// type: void **__fastcall(void **)
#[doc(alias = "std::vector<unsigned long *,std::allocator<unsigned long *>>::~vector()")]
pub use rbx_reflection::generated_refl_28::stub_0x244db8 as stub_244db8;

// 0x244dcc — __ZNSt6vectorIPFbvESaIS1_EED1Ev
// type: void **__fastcall(void **)
#[doc(alias = "std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::~vector()")]
pub use rbx_reflection::generated_refl_28::stub_0x244dcc as stub_244dcc;

// 0x244de0 — __GLOBAL__I_a_47
// type: int()
#[doc(alias = "global constructor keyed to_a_47")]
// was: global constructor keyed to_a_47
pub use rbx_core::generated_core_shard_js::stub_244de0 as stub_244de0;

// 0x244e94 — __ZNK3rbx7signals10connection10disconnectEv
// type: void __fastcall(int32_t **this)
#[doc(alias = "rbx::signals::connection::disconnect(void)const")]
pub use rbx_core::generated_core_shard_js::stub_244e94 as stub_244e94;

// 0x244fd4 — __ZNK3rbx7signals10connection9connectedEv
// type: int __fastcall(rbx::signals::connection *this)
#[doc(alias = "rbx::signals::connection::connected(void)const")]
pub use rbx_core::generated_core_shard_js::stub_244fd4 as stub_244fd4;

// 0x245118 — __ZNK3rbx7signals10connectioneqERKS1_
// type: bool __fastcall(int32_t, int32_t **, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection::operator==(rbx::signals::connection const&)const")]
pub use rbx_core::generated_core_shard_js::stub_245118 as stub_245118;

// 0x2452d0 — __ZNK3rbx7signals10connectionneERKS1_
// type: bool __fastcall(int32_t, int32_t **, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection::operator!=(rbx::signals::connection const&)const")]
pub use rbx_core::generated_core_shard_js::stub_2452d0 as stub_2452d0;

// 0x245488 — __ZN3rbx7signals10connectionaSERKS1_
// type: int *__fastcall(int *, int *)
#[doc(alias = "rbx::signals::connection::operator=(rbx::signals::connection const&)")]
pub use rbx_core::generated_core_shard_js::stub_245488 as stub_245488;

// 0x24551c — __ZN5boost8functionIFvRSt9exceptionEED1Ev
// type: int *__fastcall(int *)
#[doc(alias = "boost::function<void ()(std::exception &)>::~function()")]
pub use rbx_reflection::generated_refl_28::stub_0x24551c as stub_24551c;

// 0x245544 — __ZN4Init14initStaticDataEv
// type: void __fastcall(Init *this)
#[doc(alias = "Init::initStaticData(void)")]
pub use rbx_core::generated_core_shard_js::stub_245544 as stub_245544;

// 0x245548 — __GLOBAL__I_a_48
#[doc(alias = "global constructor keyed to_a_48")]
// was: global constructor keyed to_a_48
pub use rbx_core::generated_core_shard_js::stub_245548 as stub_245548;

// 0x2456a0 — __ZN3RBX5Tasks12SequenceBase11isInhibitedEPNS_13TaskScheduler3JobE
// type: bool __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::SequenceBase::isInhibited(RBX::TaskScheduler::Job *)")]
pub use rbx_core::boost_skeletons::stub_2456a0 as stub_2456a0;

// 0x2456d8 — __ZN3RBX5Tasks12SequenceBase7advanceEv
// type: int __fastcall(RBX::Tasks::SequenceBase *this)
#[doc(alias = "RBX::Tasks::SequenceBase::advance(void)")]
pub use rbx_core::boost_skeletons::stub_2456d8 as stub_2456d8;

// 0x245708 — __ZN3RBX5Tasks12SequenceBase7onAddedEPNS_13TaskScheduler3JobE
// type: void __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::SequenceBase::onAdded(RBX::TaskScheduler::Job *)")]
pub use rbx_core::boost_skeletons::stub_245708 as stub_245708;

// 0x2457f0 — __ZN3RBX5Tasks12SequenceBase9onRemovedEPNS_13TaskScheduler3JobE
// type: int __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::SequenceBase::onRemoved(RBX::TaskScheduler::Job *)")]
pub use rbx_core::boost_skeletons::stub_2457f0 as stub_2457f0;

// 0x245848 — __ZNSt6vectorIPN3RBX13TaskScheduler3JobESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: void *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job **,std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>>,RBX::TaskScheduler::Job * const&)")]
pub use rbx_core::boost_skeletons::stub_245848 as stub_245848;

// 0x245940 — __GLOBAL__I_a_49
#[doc(alias = "global constructor keyed to_a_49")]
// was: global constructor keyed to_a_49
pub use rbx_core::generated_core_shard_js::stub_245940 as stub_245940;

// 0x245a08 — __ZNK3RBX13TaskScheduler30getSchedulerDutyCyclePerThreadEv
// type: __int64 __fastcall(RBX::TaskScheduler *this)
#[doc(alias = "RBX::TaskScheduler::getSchedulerDutyCyclePerThread(void)const")]
pub use rbx_core::boost_skeletons::stub_245a08 as stub_245a08;

// 0x245ab0 — __ZN3RBX16ExclusiveArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_
// type: int __fastcall(RBX::ExclusiveArbiter *this, RBX::TaskScheduler::Job *, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::ExclusiveArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
pub use rbx_core::boost_skeletons::stub_245ab0 as stub_245ab0;

// 0x245b68 — __ZN3RBX13TaskScheduler11static_initEv
// type: void __fastcall(RBX::TaskScheduler *this, int, int, int)
#[doc(alias = "RBX::TaskScheduler::static_init(void)")]
pub use rbx_core::boost_skeletons::stub_245b68 as stub_245b68;

// 0x245c64 — __ZN3RBX13TaskSchedulerD1Ev
// type: void __fastcall(RBX::TaskScheduler *__hidden this)
#[doc(alias = "RBX::TaskScheduler::~TaskScheduler()")]
pub use rbx_reflection::generated_refl_28::stub_0x245c64 as stub_245c64;

// 0x245c70 — __ZN3RBX13TaskScheduler9singletonEv
// type: _DWORD __fastcall(RBX::TaskScheduler *__hidden this)
#[doc(alias = "RBX::TaskScheduler::singleton(void)")]
pub use rbx_core::boost_skeletons::stub_245c70 as stub_245c70;

// 0x245c94 — __ZN3RBX13TaskSchedulerC2Ev
// type: int __fastcall(RBX::TaskScheduler *this, int, int)
#[doc(alias = "RBX::TaskScheduler::TaskScheduler(void)")]
pub use rbx_core::boost_skeletons::stub_245c94 as stub_245c94;

// 0x246308 — __ZN3RBX13TaskScheduler21sampleRunningJobCountEv
// type: bool __fastcall(RBX::TaskScheduler *this, int, int)
#[doc(alias = "RBX::TaskScheduler::sampleRunningJobCount(void)")]
pub use rbx_core::boost_skeletons::stub_246308 as stub_246308;

// 0x246358 — __ZN3RBX13TaskSchedulerD2Ev
// type: void __fastcall(RBX::TaskScheduler *this, int, int, const void *)
#[doc(alias = "RBX::TaskScheduler::~TaskScheduler()")]
pub use rbx_reflection::generated_refl_28::stub_0x246358 as stub_246358;

// 0x2467d0 — __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEEbNS1_8functionIFvvEEE
// type: void __fastcall(int, int *, unsigned __int8, int)
#[doc(alias = "RBX::TaskScheduler::remove(boost::shared_ptr<RBX::TaskScheduler::Job>,bool,boost::function<void ()(void)>)")]
pub use rbx_core::boost_skeletons::stub_2467d0 as stub_2467d0;

// 0x246a48 — __ZN3RBX13TaskScheduler6removeERKN5boost10shared_ptrINS0_3JobEEENS2_INS_6CEventEEE
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::remove(boost::shared_ptr<RBX::TaskScheduler::Job> const&,boost::shared_ptr<RBX::CEvent>)")]
pub use rbx_core::boost_skeletons::stub_246a48 as stub_246a48;

// 0x246da8 — __ZN3RBX13TaskScheduler10rescheduleEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, RBX::TaskScheduler::Job **)
#[doc(alias = "RBX::TaskScheduler::reschedule(boost::shared_ptr<RBX::TaskScheduler::Job>)")]
pub use rbx_core::boost_skeletons::stub_246da8 as stub_246da8;

// 0x246e98 — __ZN3RBX13TaskScheduler11scheduleJobERNS0_3JobE
// type: int __fastcall(RBX::TaskScheduler *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::TaskScheduler::scheduleJob(RBX::TaskScheduler::Job &)")]
pub use rbx_core::boost_skeletons::stub_246e98 as stub_246e98;

// 0x246f90 — __ZN3RBX13TaskScheduler3addEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::add(boost::shared_ptr<RBX::TaskScheduler::Job>)")]
pub use rbx_core::boost_skeletons::stub_246f90 as stub_246f90;

// 0x24710c — __ZN3RBX13TaskScheduler20incrementThreadCountEv
// type: int __fastcall(int32_t *this, volatile int *)
#[doc(alias = "RBX::TaskScheduler::incrementThreadCount(void)")]
pub use rbx_core::boost_skeletons::stub_24710c as stub_24710c;

// 0x24711c — __ZN3RBX13TaskScheduler20decrementThreadCountEv
// type: int __fastcall(int32_t *this, volatile int *)
#[doc(alias = "RBX::TaskScheduler::decrementThreadCount(void)")]
pub use rbx_core::boost_skeletons::stub_24711c as stub_24711c;

// 0x247130 — __ZNK3RBX13TaskScheduler20getShortestSleepTimeEv
// type: int __fastcall(RBX::TaskScheduler *this, int)
#[doc(alias = "RBX::TaskScheduler::getShortestSleepTime(void)const")]
pub use rbx_core::boost_skeletons::stub_247130 as stub_247130;

// 0x247154 — __ZN3RBX13TaskScheduler16wakeSleepingJobsEv
// type: int __fastcall(RBX::TaskScheduler *this)
#[doc(alias = "RBX::TaskScheduler::wakeSleepingJobs(void)")]
pub use rbx_core::boost_skeletons::stub_247154 as stub_247154;

// 0x247220 — __ZN3RBX13TaskScheduler12findJobToRunEN5boost10shared_ptrINS0_6ThreadEEE
// type: void __fastcall(RBX::TaskScheduler::Job **, int, int *, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::findJobToRun(boost::shared_ptr<RBX::TaskScheduler::Thread>)")]
pub use rbx_core::boost_skeletons::stub_247220 as stub_247220;

// 0x247bd8 — __ZN3rbx25thread_specific_referenceIN3RBX13TaskScheduler3JobEED1Ev
#[doc(alias = "rbx::thread_specific_reference<RBX::TaskScheduler::Job>::~thread_specific_reference()")]
pub use rbx_reflection::generated_refl_28::stub_0x247bd8 as stub_247bd8;

// 0x247be8 — __ZNK3RBX13TaskScheduler3Job12getDebugNameEv
// type: void __fastcall(RBX::TaskScheduler::Job *this, int)
#[doc(alias = "RBX::TaskScheduler::Job::getDebugName(void)const")]
pub use rbx_core::boost_skeletons::stub_247be8 as stub_247be8;

// 0x247db0 — __ZN3RBX14RunningAverageIidE6sampleEi
// type: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::RunningAverage<int,double>::sample(int)")]
pub use rbx_core::boost_skeletons::stub_247db0 as stub_247db0;

// 0x247e74 — __ZN3RBX16ExclusiveArbiter11arbiterNameEv
// type: int __fastcall(RBX::ExclusiveArbiter *this)
#[doc(alias = "RBX::ExclusiveArbiter::arbiterName(void)")]
pub use rbx_core::boost_skeletons::stub_247e74 as stub_247e74;

// 0x247e90 — __ZN3RBX16ExclusiveArbiter11isThrottledEv
// type: int __fastcall(RBX::ExclusiveArbiter *this)
#[doc(alias = "RBX::ExclusiveArbiter::isThrottled(void)")]
pub use rbx_core::boost_skeletons::stub_247e90 as stub_247e90;

// 0x247e94 — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(boost::weak_ptr<RBX::TaskScheduler::Job> const&)")]
pub use rbx_core::boost_skeletons::stub_247e94 as stub_247e94;

// 0x247fac — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>)")]
pub use rbx_core::boost_skeletons::stub_247fac as stub_247fac;

// 0x248020 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<boost::shared_ptr<RBX::TaskScheduler::Job>> *)")]
pub use rbx_core::boost_skeletons::stub_248020 as stub_248020;

// 0x248050 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, _DWORD *, unsigned int M_parent, int)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_insert_unique(boost::shared_ptr<RBX::TaskScheduler::Job> const&)")]
pub use rbx_core::boost_skeletons::stub_248050 as stub_248050;

// 0x248104 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_create_node(boost::shared_ptr<RBX::TaskScheduler::Job> const&)")]
pub use rbx_core::boost_skeletons::stub_248104 as stub_248104;

