//! core bg4 — 120 core stubs EA-sorted asc distinct not yet in rbx_core or core (union).
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua) global distinct not yet in crates/rbx_core/src nor crates/core/src — next 120 uncovered after 0x9ff2a8 (watchdog 7H max) -> 0xa39050..0xa92d24.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network20sGuidRegistryServiceEEEEvv")]
// 0xa39050 — __ZN3RBX4Name13callDoDeclareILZNS_7Network20sGuidRegistryServiceEEEEvv
// type: void()
pub fn stub_a39050() -> ! {
    todo!("0xa39050 __ZN3RBX4Name13callDoDeclareILZNS_7Network20sGuidRegistryServiceEEEEvv")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::GuidRegistryService>(void)")]
// 0xa39124 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network19GuidRegistryServiceEEEvv
// type: void()
pub fn stub_a39124() -> ! {
    todo!("0xa39124 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network19GuidRegistryServiceEEEvv")
}

#[doc(alias = "RBX::Network::ChatMessage::ChatMessage(RBX::Network::ChatMessage const&)")]
// 0xa394f0 — __ZN3RBX7Network11ChatMessageC2ERKS1_
// type: RBX::Network::ChatMessage *__fastcall(RBX::Network::ChatMessage *this, const RBX::Network::ChatMessage *)
pub fn stub_a394f0() -> ! {
    todo!("0xa394f0 __ZN3RBX7Network11ChatMessageC2ERKS1_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot> &)")]
// 0xa39718 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
// type: int __fastcall(int, int32_t **)
pub fn stub_a39718() -> ! {
    todo!("0xa39718 __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::mutex(void)")]
// 0xa3992c — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE5mutexEv
// type: int __fastcall(int, int, int, int)
pub fn stub_a3992c() -> ! {
    todo!("0xa3992c __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE5mutexEv")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot> const&)")]
// 0xa39a40 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEEaSERKSC_
// type: int32_t **__fastcall(int32_t **, int32_t **)
pub fn stub_a39a40() -> ! {
    todo!("0xa39a40 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEEaSERKSC_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::safe_static_init_mutex(void)")]
// 0xa39af4 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE22safe_static_init_mutexEv
// type: void()
pub fn stub_a39af4() -> ! {
    todo!("0xa39af4 __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE22safe_static_init_mutexEv")
}

#[doc(alias = "__gnu_cxx::new_allocator<RBX::Network::AbuseReport>::destroy(RBX::Network::AbuseReport*)")]
// 0xa3a87c — __ZN9__gnu_cxx13new_allocatorIN3RBX7Network11AbuseReportEE7destroyEPS3_
// type: int __fastcall(int, int)
pub fn stub_a3a87c() -> ! {
    todo!("0xa3a87c __ZN9__gnu_cxx13new_allocatorIN3RBX7Network11AbuseReportEE7destroyEPS3_")
}

#[doc(alias = "std::deque<RBX::Network::AbuseReport,std::allocator<RBX::Network::AbuseReport>>::_M_push_back_aux(RBX::Network::AbuseReport const&)")]
// 0xa3aad0 — __ZNSt5dequeIN3RBX7Network11AbuseReportESaIS2_EE16_M_push_back_auxERKS2_
// type: void __fastcall(_DWORD *, int *, int, int, int, int, int, int, int, char, char, int, int, int, int, int, void *, int)
pub fn stub_a3aad0() -> ! {
    todo!("0xa3aad0 __ZNSt5dequeIN3RBX7Network11AbuseReportESaIS2_EE16_M_push_back_auxERKS2_")
}

#[doc(alias = "std::deque<RBX::Network::AbuseReport,std::allocator<RBX::Network::AbuseReport>>::_M_reallocate_map(unsigned long,bool)")]
// 0xa3af34 — __ZNSt5dequeIN3RBX7Network11AbuseReportESaIS2_EE17_M_reallocate_mapEmb
// type: char *__fastcall(void **, unsigned int, int)
pub fn stub_a3af34() -> ! {
    todo!("0xa3af34 __ZNSt5dequeIN3RBX7Network11AbuseReportESaIS2_EE17_M_reallocate_mapEmb")
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>::operator()(RBX::Network::AbuseReport&,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&)const")]
// 0xa3b00c — __ZNK5boost4_mfi3mf2IvN3RBX7Network11AbuseReportENS_10shared_ptrINS3_6PlayerEEERKNS3_11ChatMessageEEclERS4_S7_SA_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_a3b00c() -> ! {
    todo!("0xa3b00c __ZNK5boost4_mfi3mf2IvN3RBX7Network11AbuseReportENS_10shared_ptrINS3_6PlayerEEERKNS3_11ChatMessageEEclERS4_S7_SA_")
}

#[doc(alias = "boost::_bi::storage3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>::storage3(boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>)")]
// 0xa3b288 — __ZN5boost3_bi8storage3INS_17reference_wrapperIN3RBX7Network11AbuseReportEEENS0_5valueINS_10shared_ptrINS4_6PlayerEEEEENS_3argILi1EEEEC2ES6_SB_SD_
// type: _DWORD *__fastcall(_DWORD *, int, int *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_a3b288() -> ! {
    todo!("0xa3b288 __ZN5boost3_bi8storage3INS_17reference_wrapperIN3RBX7Network11AbuseReportEEENS0_5valueINS_10shared_ptrINS4_6PlayerEEEEENS_3argILi1EEEEC2ES6_SB_SD_")
}

#[doc(alias = "__ZN5boost9function0IN3RBX13worker_thread11work_resultEEC2INS_3_bi6bind_tIS3_PFS3_NS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsENS6_5list2INS6_5valueISC_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0xa3b504 — __ZN5boost9function0IN3RBX13worker_thread11work_resultEEC2INS_3_bi6bind_tIS3_PFS3_NS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsENS6_5list2INS6_5valueISC_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
pub fn stub_a3b504() -> ! {
    todo!("0xa3b504 __ZN5boost9function0IN3RBX13worker_thread11work_resultEEC2INS_3_bi6bind_tIS3_PFS3_NS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsENS6_5list2INS6_5valueISC_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<RBX::worker_thread::work_result>::assign_to<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>>)")]
// 0xa3b8e0 — __ZN5boost9function0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS3_PFS3_NS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsENS6_5list2INS6_5valueISC_EENSG_ISsEEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
pub fn stub_a3b8e0() -> ! {
    todo!("0xa3b8e0 __ZN5boost9function0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS3_PFS3_NS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsENS6_5list2INS6_5valueISC_EENSG_ISsEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0xa3bccc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX13worker_thread11work_resultEPFS7_NS_10shared_ptrINS5_7Network13AbuseReporter4dataEEESsENS3_5list2INS3_5valueISC_EENSG_ISsEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
pub fn stub_a3bccc() -> ! {
    todo!("0xa3bccc __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX13worker_thread11work_resultEPFS7_NS_10shared_ptrINS5_7Network13AbuseReporter4dataEEESsENS3_5list2INS3_5valueISC_EENSG_ISsEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>>,RBX::worker_thread::work_result>::invoke(boost::detail::function::function_buffer &)")]
// 0xa3bcf0 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIN3RBX13worker_thread11work_resultEPFS7_NS_10shared_ptrINS5_7Network13AbuseReporter4dataEEESsENS3_5list2INS3_5valueISC_EENSG_ISsEEEEEES7_E6invokeERNS1_15function_bufferE
// type: int __fastcall(_DWORD *)
pub fn stub_a3bcf0() -> ! {
    todo!("0xa3bcf0 __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIN3RBX13worker_thread11work_resultEPFS7_NS_10shared_ptrINS5_7Network13AbuseReporter4dataEEESsENS3_5list2INS3_5valueISC_EENSG_ISsEEEEEES7_E6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<RBX::worker_thread::work_result>::assign_to<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// 0xa3bd04 — __ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_10shared_ptrINS3_7Network13AbuseReporter4dataEEESsENS8_5list2INS8_5valueISE_EENSI_ISsEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int)
pub fn stub_a3bd04() -> ! {
    todo!("0xa3bd04 __ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_10shared_ptrINS3_7Network13AbuseReporter4dataEEESsENS8_5list2INS8_5valueISE_EENSI_ISsEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<RBX::worker_thread::work_result>::assign_to<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xa3c0e0 — __ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_10shared_ptrINS3_7Network13AbuseReporter4dataEEESsENS8_5list2INS8_5valueISE_EENSI_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, void **)
pub fn stub_a3c0e0() -> ! {
    todo!("0xa3c0e0 __ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_10shared_ptrINS3_7Network13AbuseReporter4dataEEESsENS8_5list2INS8_5valueISE_EENSI_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "RBX::worker_thread::work_result boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>::operator()<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list0>(boost::_bi::type<RBX::worker_thread::work_result>,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string) &,boost::_bi::list0 &,long)")]
// 0xa3c56c — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX7Network13AbuseReporter4dataEEEEENS2_ISsEEEclINS4_13worker_thread11work_resultEPFSE_S8_SsENS0_5list0EEET_NS0_4typeISI_EERT0_RT1_l
// type: int __fastcall(int *, int (__fastcall **)(int *, int *))
pub fn stub_a3c56c() -> ! {
    todo!("0xa3c56c __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX7Network13AbuseReporter4dataEEEEENS2_ISsEEEclINS4_13worker_thread11work_resultEPFSE_S8_SsENS0_5list0EEET_NS0_4typeISI_EERT0_RT1_l")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xa3c868 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX13worker_thread11work_resultEPFS7_NS_10shared_ptrINS5_7Network13AbuseReporter4dataEEESsENS3_5list2INS3_5valueISC_EENSG_ISsEEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, int, int)
pub fn stub_a3c868() -> ! {
    todo!("0xa3c868 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX13worker_thread11work_resultEPFS7_NS_10shared_ptrINS5_7Network13AbuseReporter4dataEEESsENS3_5list2INS3_5valueISC_EENSG_ISsEEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>)")]
// 0xa3ca74 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX7Network13AbuseReporter4dataEEEEENS2_ISsEEEC2ES9_SA_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(pthread_mutex_t *, int *, const std::string *)
pub fn stub_a3ca74() -> ! {
    todo!("0xa3ca74 __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX7Network13AbuseReporter4dataEEEEENS2_ISsEEEC2ES9_SA_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Network::AbuseReporter::data>>,boost::_bi::value<std::string>)")]
// 0xa3cd70 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network13AbuseReporter4dataEEEEENS2_ISsEEEC2ES9_SA_
// type: int *__fastcall(int *, int *, const std::string *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_a3cd70() -> ! {
    todo!("0xa3cd70 __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network13AbuseReporter4dataEEEEENS2_ISsEEEC2ES9_SA_")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::AbuseReporter::data,RBX::Network::AbuseReporter::data>(boost::shared_ptr<RBX::Network::AbuseReporter::data> *,RBX::Network::AbuseReporter::data *,boost::detail::shared_count &)")]
// 0xa3cfec — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13AbuseReporter4dataES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_a3cfec() -> ! {
    todo!("0xa3cfec __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13AbuseReporter4dataES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "std::deque<RBX::Network::AbuseReport,std::allocator<RBX::Network::AbuseReport>>::~deque()")]
// 0xa3d1c0 — __ZNSt5dequeIN3RBX7Network11AbuseReportESaIS2_EED2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_a3d1c0() -> ! {
    todo!("0xa3d1c0 __ZNSt5dequeIN3RBX7Network11AbuseReportESaIS2_EED2Ev")
}

#[doc(alias = "std::deque<RBX::Network::AbuseReport,std::allocator<RBX::Network::AbuseReport>>::_M_destroy_data_aux(std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport&,RBX::Network::AbuseReport*>,std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport&,RBX::Network::AbuseReport*>)")]
// 0xa3d2e8 — __ZNSt5dequeIN3RBX7Network11AbuseReportESaIS2_EE19_M_destroy_data_auxESt15_Deque_iteratorIS2_RS2_PS2_ES8_
// type: int __fastcall(int, _DWORD *, int *)
pub fn stub_a3d2e8() -> ! {
    todo!("0xa3d2e8 __ZNSt5dequeIN3RBX7Network11AbuseReportESaIS2_EE19_M_destroy_data_auxESt15_Deque_iteratorIS2_RS2_PS2_ES8_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::AbuseReporter::data>::~sp_counted_impl_p()")]
// 0xa3d680 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13AbuseReporter4dataEED1Ev
// type: void()
pub fn stub_a3d680() -> ! {
    todo!("0xa3d680 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13AbuseReporter4dataEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::AbuseReporter::data>::~sp_counted_impl_p()")]
// 0xa3d684 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13AbuseReporter4dataEED0Ev
// type: void __fastcall(void *)
pub fn stub_a3d684() -> ! {
    todo!("0xa3d684 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13AbuseReporter4dataEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::AbuseReporter::data>::dispose(void)")]
// 0xa3d690 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13AbuseReporter4dataEE7disposeEv
// type: void __fastcall(int)
pub fn stub_a3d690() -> ! {
    todo!("0xa3d690 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13AbuseReporter4dataEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::AbuseReporter::data>::get_deleter(std::type_info const&)")]
// 0xa3d768 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13AbuseReporter4dataEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_a3d768() -> ! {
    todo!("0xa3d768 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13AbuseReporter4dataEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::AbuseReporter::data>::get_untyped_deleter(void)")]
// 0xa3d76c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13AbuseReporter4dataEE19get_untyped_deleterEv
// type: int()
pub fn stub_a3d76c() -> ! {
    todo!("0xa3d76c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13AbuseReporter4dataEE19get_untyped_deleterEv")
}

#[doc(alias = "std::list<RBX::Network::AbuseReport::Message,std::allocator<RBX::Network::AbuseReport::Message>>::_M_create_node(RBX::Network::AbuseReport::Message const&)")]
// 0xa3d770 — __ZNSt4listIN3RBX7Network11AbuseReport7MessageESaIS3_EE14_M_create_nodeERKS3_
// type: _DWORD *__fastcall(int, _DWORD *, int, int, void *, int, int, int, void *, int)
pub fn stub_a3d770() -> ! {
    todo!("0xa3d770 __ZNSt4listIN3RBX7Network11AbuseReport7MessageESaIS3_EE14_M_create_nodeERKS3_")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sNetworkSettingsEEEEvv")]
// 0xa3f640 — __ZN3RBX4Name13callDoDeclareILZNS_16sNetworkSettingsEEEEvv
// type: void()
pub fn stub_a3f640() -> ! {
    todo!("0xa3f640 __ZN3RBX4Name13callDoDeclareILZNS_16sNetworkSettingsEEEEvv")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>> *)")]
// 0xa50bc0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players14PlayerChatTypeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_a50bc0() -> ! {
    todo!("0xa50bc0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players14PlayerChatTypeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>> *)")]
// 0xa50be8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players10ChatOptionEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_a50be8() -> ! {
    todo!("0xa50be8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players10ChatOptionEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")
}

#[doc(alias = "std::list<RBX::Network::AbuseReport::Message,std::allocator<RBX::Network::AbuseReport::Message>>::list(std::list<RBX::Network::AbuseReport::Message,std::allocator<RBX::Network::AbuseReport::Message>> const&)")]
// 0xa50fcc — __ZNSt4listIN3RBX7Network11AbuseReport7MessageESaIS3_EEC2ERKS5_
// type: std::_List_node_base *__fastcall(std::_List_node_base *, void **, int, int, char, int, int, int, int)
pub fn stub_a50fcc() -> ! {
    todo!("0xa50fcc __ZNSt4listIN3RBX7Network11AbuseReport7MessageESaIS3_EEC2ERKS5_")
}

#[doc(alias = "RBX::Network::ChatMessage::~ChatMessage()")]
// 0xa51d24 — __ZN3RBX7Network11ChatMessageD2Ev
// type: void __fastcall(RBX::Network::ChatMessage *__hidden this)
pub fn stub_a51d24() -> ! {
    todo!("0xa51d24 __ZN3RBX7Network11ChatMessageD2Ev")
}

#[doc(alias = "std::list<RBX::Network::AbuseReport::Message,std::allocator<RBX::Network::AbuseReport::Message>>::operator=(std::list<RBX::Network::AbuseReport::Message,std::allocator<RBX::Network::AbuseReport::Message>> const&)")]
// 0xa5275c — __ZNSt4listIN3RBX7Network11AbuseReport7MessageESaIS3_EEaSERKS5_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_a5275c() -> ! {
    todo!("0xa5275c __ZNSt4listIN3RBX7Network11AbuseReport7MessageESaIS3_EEaSERKS5_")
}

#[doc(alias = "void std::list<RBX::Network::AbuseReport::Message,std::allocator<RBX::Network::AbuseReport::Message>>::insert<std::_List_const_iterator<RBX::Network::AbuseReport::Message>>(std::_List_iterator<RBX::Network::AbuseReport::Message>,std::_List_const_iterator<RBX::Network::AbuseReport::Message>,std::_List_const_iterator<RBX::Network::AbuseReport::Message>)")]
// 0xa52848 — __ZNSt4listIN3RBX7Network11AbuseReport7MessageESaIS3_EE6insertISt20_List_const_iteratorIS3_EEEvSt14_List_iteratorIS3_ET_SB_
// type: void __fastcall(int, std::_List_node_base *, int, int)
pub fn stub_a52848() -> ! {
    todo!("0xa52848 __ZNSt4listIN3RBX7Network11AbuseReport7MessageESaIS3_EE6insertISt20_List_const_iteratorIS3_EEEvSt14_List_iteratorIS3_ET_SB_")
}

#[doc(alias = "std::_Deque_base<RBX::Network::AbuseReport,std::allocator<RBX::Network::AbuseReport>>::_M_initialize_map(unsigned long)")]
// 0xa52ae4 — __ZNSt11_Deque_baseIN3RBX7Network11AbuseReportESaIS2_EE17_M_initialize_mapEm
// type: void __fastcall(_DWORD *, unsigned int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, void *, int)
pub fn stub_a52ae4() -> ! {
    todo!("0xa52ae4 __ZNSt11_Deque_baseIN3RBX7Network11AbuseReportESaIS2_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::deque<RBX::Network::AbuseReport,std::allocator<RBX::Network::AbuseReport>>::deque(std::deque<RBX::Network::AbuseReport,std::allocator<RBX::Network::AbuseReport>> const&)")]
// 0xa52cd0 — __ZNSt5dequeIN3RBX7Network11AbuseReportESaIS2_EEC2ERKS4_
// type: int __fastcall(int, _DWORD *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_a52cd0() -> ! {
    todo!("0xa52cd0 __ZNSt5dequeIN3RBX7Network11AbuseReportESaIS2_EEC2ERKS4_")
}

#[doc(alias = "std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport&,RBX::Network::AbuseReport*> std::__uninitialized_copy_aux<std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport const&,RBX::Network::AbuseReport const*>,std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport&,RBX::Network::AbuseReport*>>(std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport const&,RBX::Network::AbuseReport const*>,std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport const&,RBX::Network::AbuseReport const*>,std::_Deque_iterator<RBX::Network::AbuseReport,RBX::Network::AbuseReport&,RBX::Network::AbuseReport*>,std::__false_type)")]
// 0xa52e28 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3RBX7Network11AbuseReportERKS3_PS4_ES0_IS3_RS3_PS3_EET0_T_SC_SB_St12__false_type
// type: void __fastcall(_DWORD *, _DWORD *, int, int, struct _Unwind_Exception *lpuexcpt, _DWORD *, char, int, int, int, int, int, void *, int)
pub fn stub_a52e28() -> ! {
    todo!("0xa52e28 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3RBX7Network11AbuseReportERKS3_PS4_ES0_IS3_RS3_PS3_EET0_T_SC_SB_St12__false_type")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::disconnectAll(void)")]
// 0xa535ac — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13disconnectAllEv
// type: void __fastcall(_DWORD *)
pub fn stub_a535ac() -> ! {
    todo!("0xa535ac __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::disconnectAll(void)")]
// 0xa53764 — __ZN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE13disconnectAllEv
// type: void __fastcall(_DWORD *)
pub fn stub_a53764() -> ! {
    todo!("0xa53764 __ZN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE13disconnectAllEv")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::RoundRobinPhysicsSender(RBX::Network::Replicator &)")]
// 0xa7e0e8 — __ZN3RBX7Network23RoundRobinPhysicsSenderC1ERNS0_10ReplicatorE
// type: RBX::Network::RoundRobinPhysicsSender *__fastcall(RBX::Network::RoundRobinPhysicsSender *this, RBX::Network::Replicator *)
pub fn stub_a7e0e8() -> ! {
    todo!("0xa7e0e8 __ZN3RBX7Network23RoundRobinPhysicsSenderC1ERNS0_10ReplicatorE")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::step(void)")]
// 0xa7e360 — __ZN3RBX7Network23RoundRobinPhysicsSender4stepEv
// type: void __fastcall(RBX::Network::RoundRobinPhysicsSender *this, int, int, int)
pub fn stub_a7e360() -> ! {
    todo!("0xa7e360 __ZN3RBX7Network23RoundRobinPhysicsSender4stepEv")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::sendPacket(int,PacketPriority,RBX::Network::ReplicatorStats::PhysicsSenderStats *)")]
// 0xa7e468 — __ZN3RBX7Network23RoundRobinPhysicsSender10sendPacketEi14PacketPriorityPNS0_15ReplicatorStats18PhysicsSenderStatsE
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_a7e468() -> ! {
    todo!("0xa7e468 __ZN3RBX7Network23RoundRobinPhysicsSender10sendPacketEi14PacketPriorityPNS0_15ReplicatorStats18PhysicsSenderStatsE")
}

#[doc(alias = "int RBX::SendPhysics::reportSimJobs<RBX::Network::RoundRobinPhysicsSender::JobSender>(RBX::Network::RoundRobinPhysicsSender::JobSender &,RBX::SimJobTracker &,RBX::SimJob const*,int)")]
// 0xa7e9cc — __ZN3RBX11SendPhysics13reportSimJobsINS_7Network23RoundRobinPhysicsSender9JobSenderEEEiRT_RNS_13SimJobTrackerEPKNS_6SimJobEi
// type: int __fastcall(int, _DWORD *, RBX::SimJobTracker *, RBX::SimJob *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, pthread_mutex_t *, int, int, int, int)
pub fn stub_a7e9cc() -> ! {
    todo!("0xa7e9cc __ZN3RBX11SendPhysics13reportSimJobsINS_7Network23RoundRobinPhysicsSender9JobSenderEEEiRT_RNS_13SimJobTrackerEPKNS_6SimJobEi")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::~RoundRobinPhysicsSender()")]
// 0xa7ec08 — __ZN3RBX7Network23RoundRobinPhysicsSenderD1Ev
// type: void __fastcall(RBX::Network::RoundRobinPhysicsSender *__hidden this)
pub fn stub_a7ec08() -> ! {
    todo!("0xa7ec08 __ZN3RBX7Network23RoundRobinPhysicsSenderD1Ev")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::~RoundRobinPhysicsSender()")]
// 0xa7ecd4 — __ZN3RBX7Network23RoundRobinPhysicsSenderD0Ev
// type: void __fastcall(RBX::Network::RoundRobinPhysicsSender *__hidden this)
pub fn stub_a7ecd4() -> ! {
    todo!("0xa7ecd4 __ZN3RBX7Network23RoundRobinPhysicsSenderD0Ev")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::JobSender::closePacket(void)")]
// 0xa7ef60 — __ZN3RBX7Network23RoundRobinPhysicsSender9JobSender11closePacketEv
// type: void __fastcall(RBX::Network::RoundRobinPhysicsSender::JobSender *this)
pub fn stub_a7ef60() -> ! {
    todo!("0xa7ef60 __ZN3RBX7Network23RoundRobinPhysicsSender9JobSender11closePacketEv")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::JobSender::openPacket(void)")]
// 0xa7f320 — __ZN3RBX7Network23RoundRobinPhysicsSender9JobSender10openPacketEv
// type: void __fastcall(RakNet **this)
pub fn stub_a7f320() -> ! {
    todo!("0xa7f320 __ZN3RBX7Network23RoundRobinPhysicsSender9JobSender10openPacketEv")
}

#[doc(alias = "RBX::Network::Player::loadData(void)")]
// 0xa7fbf0 — __ZN3RBX7Network6Player8loadDataEv
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *)
pub fn stub_a7fbf0() -> ! {
    todo!("0xa7fbf0 __ZN3RBX7Network6Player8loadDataEv")
}

#[doc(alias = "RBX::Network::Player::saveData(void)")]
// 0xa802c8 — __ZN3RBX7Network6Player8saveDataEv
// type: void __fastcall(RBX::Network::PersistentDataStore **this, const RBX::Instance *)
pub fn stub_a802c8() -> ! {
    todo!("0xa802c8 __ZN3RBX7Network6Player8saveDataEv")
}

#[doc(alias = "RBX::Network::Player::saveLeaderboardData(void)")]
// 0xa80674 — __ZN3RBX7Network6Player19saveLeaderboardDataEv
// type: void __fastcall(RBX::Network::PersistentDataStore **this, const RBX::Instance *)
pub fn stub_a80674() -> ! {
    todo!("0xa80674 __ZN3RBX7Network6Player19saveLeaderboardDataEv")
}

#[doc(alias = "RBX::Network::Player::setHasGroupBuildTools(bool)")]
// 0xa80a28 — __ZN3RBX7Network6Player21setHasGroupBuildToolsEb
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
pub fn stub_a80a28() -> ! {
    todo!("0xa80a28 __ZN3RBX7Network6Player21setHasGroupBuildToolsEb")
}

#[doc(alias = "RBX::Network::Player::setPersonalServerRank(int)")]
// 0xa80a50 — __ZN3RBX7Network6Player21setPersonalServerRankEi
// type: _DWORD __fastcall(RBX::Network::Player *__hidden this, int)
pub fn stub_a80a50() -> ! {
    todo!("0xa80a50 __ZN3RBX7Network6Player21setPersonalServerRankEi")
}

#[doc(alias = "RBX::Network::Player::getWebPersonalServerRank(boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0xa80adc — __ZN3RBX7Network6Player24getWebPersonalServerRankEN5boost8functionIFvSsEEES5_
// type: void __fastcall(RBX::ServiceProvider *, int *, int *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, char, int, int, char, int, int, int, int)
pub fn stub_a80adc() -> ! {
    todo!("0xa80adc __ZN3RBX7Network6Player24getWebPersonalServerRankEN5boost8functionIFvSsEEES5_")
}

#[doc(alias = "RBX::Network::Player::getDataComplexity(void)const")]
// 0xa80ed4 — __ZNK3RBX7Network6Player17getDataComplexityEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_a80ed4() -> ! {
    todo!("0xa80ed4 __ZNK3RBX7Network6Player17getDataComplexityEv")
}

#[doc(alias = "RBX::Network::Player::setDataComplexityLimit(int)")]
// 0xa80ee4 — __ZN3RBX7Network6Player22setDataComplexityLimitEi
// type: int __fastcall(RBX::Network::Player *this, int)
pub fn stub_a80ee4() -> ! {
    todo!("0xa80ee4 __ZN3RBX7Network6Player22setDataComplexityLimitEi")
}

#[doc(alias = "RBX::Network::Player::loadString(std::string)")]
// 0xa81da0 — __ZN3RBX7Network6Player10loadStringESs
// type: void __fastcall(RBX::Network::PersistentDataStore *, int, const void **)
pub fn stub_a81da0() -> ! {
    todo!("0xa81da0 __ZN3RBX7Network6Player10loadStringESs")
}

#[doc(alias = "RBX::Network::Player::saveString(std::string,std::string)")]
// 0xa82018 — __ZN3RBX7Network6Player10saveStringESsSs
// type: void __fastcall(int, const std::string *, const std::string *)
pub fn stub_a82018() -> ! {
    todo!("0xa82018 __ZN3RBX7Network6Player10saveStringESsSs")
}

#[doc(alias = "RBX::Network::Player::loadBoolean(std::string)")]
// 0xa82300 — __ZN3RBX7Network6Player11loadBooleanESs
// type: int __fastcall(int, const void **, bool)
pub fn stub_a82300() -> ! {
    todo!("0xa82300 __ZN3RBX7Network6Player11loadBooleanESs")
}

#[doc(alias = "RBX::Network::Player::saveBoolean(std::string,bool)")]
// 0xa82574 — __ZN3RBX7Network6Player11saveBooleanESsb
// type: void __fastcall(int, const std::string *, int)
pub fn stub_a82574() -> ! {
    todo!("0xa82574 __ZN3RBX7Network6Player11saveBooleanESsb")
}

#[doc(alias = "RBX::Network::Player::loadNumber(std::string)")]
// 0xa8285c — __ZN3RBX7Network6Player10loadNumberESs
// type: __int64 __fastcall(int, const void **, bool)
pub fn stub_a8285c() -> ! {
    todo!("0xa8285c __ZN3RBX7Network6Player10loadNumberESs")
}

#[doc(alias = "RBX::Network::Player::saveNumber(std::string,double)")]
// 0xa82ad8 — __ZN3RBX7Network6Player10saveNumberESsd
// type: void __fastcall(int, const std::string *, _BOOL4, unsigned int)
pub fn stub_a82ad8() -> ! {
    todo!("0xa82ad8 __ZN3RBX7Network6Player10saveNumberESsd")
}

#[doc(alias = "RBX::Network::Player::luaLoadCharacter(bool)")]
// 0xa835ec — __ZN3RBX7Network6Player16luaLoadCharacterEb
// type: void __fastcall(RBX::Network::Player *this, const char *, int, const void *)
pub fn stub_a835ec() -> ! {
    todo!("0xa835ec __ZN3RBX7Network6Player16luaLoadCharacterEb")
}

#[doc(alias = "RBX::Network::Player::removeCharacter(void)")]
// 0xa837d8 — __ZN3RBX7Network6Player15removeCharacterEv
// type: void __fastcall(RBX::Network::Player *this, int, bool)
pub fn stub_a837d8() -> ! {
    todo!("0xa837d8 __ZN3RBX7Network6Player15removeCharacterEv")
}

#[doc(alias = "RBX::Network::Player::setUnder13(bool)")]
// 0xa83950 — __ZN3RBX7Network6Player10setUnder13Eb
// type: int __fastcall(int this, int)
pub fn stub_a83950() -> ! {
    todo!("0xa83950 __ZN3RBX7Network6Player10setUnder13Eb")
}

#[doc(alias = "RBX::Network::Player::setSuperSafeChat(bool)")]
// 0xa83960 — __ZN3RBX7Network6Player16setSuperSafeChatEb
// type: int __fastcall(RBX::Network::Player *this, int)
pub fn stub_a83960() -> ! {
    todo!("0xa83960 __ZN3RBX7Network6Player16setSuperSafeChatEb")
}

#[doc(alias = "RBX::Network::Player::setMembershipType(RBX::Network::Player::MembershipType)")]
// 0xa83998 — __ZN3RBX7Network6Player17setMembershipTypeENS1_14MembershipTypeE
// type: int __fastcall(RBX::Instance *, int)
pub fn stub_a83998() -> ! {
    todo!("0xa83998 __ZN3RBX7Network6Player17setMembershipTypeENS1_14MembershipTypeE")
}

#[doc(alias = "RBX::Network::Player::setAccountAge(int)")]
// 0xa839cc — __ZN3RBX7Network6Player13setAccountAgeEi
// type: int __fastcall(RBX::Network::Player *this, int)
pub fn stub_a839cc() -> ! {
    todo!("0xa839cc __ZN3RBX7Network6Player13setAccountAgeEi")
}

#[doc(alias = "RBX::Network::Player::kick(void)")]
// 0xa83a00 — __ZN3RBX7Network6Player4kickEv
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *)
pub fn stub_a83a00() -> ! {
    todo!("0xa83a00 __ZN3RBX7Network6Player4kickEv")
}

#[doc(alias = "RBX::Network::Player::setCharacterAppearance(std::string const&)")]
// 0xa84aec — __ZN3RBX7Network6Player22setCharacterAppearanceERKSs
// type: void __fastcall(RBX::Network::Player *this, const std::string *)
pub fn stub_a84aec() -> ! {
    todo!("0xa84aec __ZN3RBX7Network6Player22setCharacterAppearanceERKSs")
}

#[doc(alias = "RBX::Network::Player::setCanLoadCharacterAppearance(bool)")]
// 0xa85160 — __ZN3RBX7Network6Player29setCanLoadCharacterAppearanceEb
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
pub fn stub_a85160() -> ! {
    todo!("0xa85160 __ZN3RBX7Network6Player29setCanLoadCharacterAppearanceEb")
}

#[doc(alias = "RBX::Network::Player::removeCharacterAppearanceScript(void)")]
// 0xa85188 — __ZN3RBX7Network6Player31removeCharacterAppearanceScriptEv
// type: _DWORD __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_a85188() -> ! {
    todo!("0xa85188 __ZN3RBX7Network6Player31removeCharacterAppearanceScriptEv")
}

#[doc(alias = "RBX::Network::Player::setUserId(int)")]
// 0xa85408 — __ZN3RBX7Network6Player9setUserIdEi
// type: void __fastcall(RBX::Network::Player *this, int)
pub fn stub_a85408() -> ! {
    todo!("0xa85408 __ZN3RBX7Network6Player9setUserIdEi")
}

#[doc(alias = "RBX::Network::Player::getRoleInGroup(int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0xa85b14 — __ZN3RBX7Network6Player14getRoleInGroupEiN5boost8functionIFvSsEEES5_
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int)
pub fn stub_a85b14() -> ! {
    todo!("0xa85b14 __ZN3RBX7Network6Player14getRoleInGroupEiN5boost8functionIFvSsEEES5_")
}

#[doc(alias = "RBX::Network::Player::getSuperSafeChat(void)const")]
// 0xa85d98 — __ZNK3RBX7Network6Player16getSuperSafeChatEv
// type: bool __fastcall(RBX::Network::Player *this)
pub fn stub_a85d98() -> ! {
    todo!("0xa85d98 __ZNK3RBX7Network6Player16getSuperSafeChatEv")
}

#[doc(alias = "RBX::Network::Player::getChatMode(void)const")]
// 0xa85dc0 — __ZNK3RBX7Network6Player11getChatModeEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_a85dc0() -> ! {
    todo!("0xa85dc0 __ZNK3RBX7Network6Player11getChatModeEv")
}

#[doc(alias = "RBX::Network::Player::setTeamColor(RBX::BrickColor)")]
// 0xa85de8 — __ZN3RBX7Network6Player12setTeamColorENS_10BrickColorE
// type: int __fastcall(int, int)
pub fn stub_a85de8() -> ! {
    todo!("0xa85de8 __ZN3RBX7Network6Player12setTeamColorENS_10BrickColorE")
}

#[doc(alias = "RBX::Network::Player::setNeutral(bool)")]
// 0xa85e44 — __ZN3RBX7Network6Player10setNeutralEb
// type: int __fastcall(RBX::Network::Player *this, int)
pub fn stub_a85e44() -> ! {
    todo!("0xa85e44 __ZN3RBX7Network6Player10setNeutralEb")
}

#[doc(alias = "RBX::Network::Player::setCameraMode(RBX::Camera::CameraMode)")]
// 0xa85ea4 — __ZN3RBX7Network6Player13setCameraModeENS_6Camera10CameraModeE
pub fn stub_a85ea4() -> ! {
    todo!("0xa85ea4 __ZN3RBX7Network6Player13setCameraModeENS_6Camera10CameraModeE")
}

#[doc(alias = "RBX::Network::Player::Player(void)")]
// 0xa85ee4 — __ZN3RBX7Network6PlayerC1Ev
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_a85ee4() -> ! {
    todo!("0xa85ee4 __ZN3RBX7Network6PlayerC1Ev")
}

#[doc(alias = "RBX::Network::Player::Player(void)")]
// 0xa85ef0 — __ZN3RBX7Network6PlayerC2Ev
// type: RBX::Instance *__fastcall(RBX::Network::Player *this)
pub fn stub_a85ef0() -> ! {
    todo!("0xa85ef0 __ZN3RBX7Network6PlayerC2Ev")
}

#[doc(alias = "RBX::Network::Player::~Player()")]
// 0xa86cf8 — __ZN3RBX7Network6PlayerD0Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_a86cf8() -> ! {
    todo!("0xa86cf8 __ZN3RBX7Network6PlayerD0Ev")
}

#[doc(alias = "RBX::Network::Player::~Player()")]
// 0xa86d98 — __ZN3RBX7Network6PlayerD1Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_a86d98() -> ! {
    todo!("0xa86d98 __ZN3RBX7Network6PlayerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Player::~Player()")]
// 0xa86da4 — __ZThn32_N3RBX7Network6PlayerD0Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_a86da4() -> ! {
    todo!("0xa86da4 __ZThn32_N3RBX7Network6PlayerD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Player::~Player()")]
// 0xa86e48 — __ZThn36_N3RBX7Network6PlayerD0Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_a86e48() -> ! {
    todo!("0xa86e48 __ZThn36_N3RBX7Network6PlayerD0Ev")
}

#[doc(alias = "RBX::Network::Player::~Player()")]
// 0xa86eec — __ZN3RBX7Network6PlayerD2Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_a86eec() -> ! {
    todo!("0xa86eec __ZN3RBX7Network6PlayerD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Player::~Player()")]
// 0xa87d2c — __ZThn32_N3RBX7Network6PlayerD1Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_a87d2c() -> ! {
    todo!("0xa87d2c __ZThn32_N3RBX7Network6PlayerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Player::~Player()")]
// 0xa87d38 — __ZThn36_N3RBX7Network6PlayerD1Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_a87d38() -> ! {
    todo!("0xa87d38 __ZThn36_N3RBX7Network6PlayerD1Ev")
}

#[doc(alias = "RBX::Network::Player::reportStat(std::string)")]
// 0xa87d5c — __ZN3RBX7Network6Player10reportStatESs
// type: void __fastcall(int, const std::string *)
pub fn stub_a87d5c() -> ! {
    todo!("0xa87d5c __ZN3RBX7Network6Player10reportStatESs")
}

#[doc(alias = "RBX::Network::Player::setWebPersonalServerRank(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xa88570 — __ZN3RBX7Network6Player24setWebPersonalServerRankEiN5boost8functionIFvbEEENS3_IFvSsEEE
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, char, int, int, char, int, int, int, int)
pub fn stub_a88570() -> ! {
    todo!("0xa88570 __ZN3RBX7Network6Player24setWebPersonalServerRankEiN5boost8functionIFvbEEENS3_IFvSsEEE")
}

#[doc(alias = "RBX::Network::Player::waitForDataReady(boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xa8896c — __ZN3RBX7Network6Player16waitForDataReadyEN5boost8functionIFvbEEENS3_IFvSsEEE
// type: int __fastcall(int, int)
pub fn stub_a8896c() -> ! {
    todo!("0xa8896c __ZN3RBX7Network6Player16waitForDataReadyEN5boost8functionIFvbEEENS3_IFvSsEEE")
}

#[doc(alias = "RBX::Network::Player::renderStreamedRegion(RBX::Adorn *)")]
// 0xa8899c — __ZN3RBX7Network6Player20renderStreamedRegionEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Network::Player *__hidden this, RBX::Adorn *)
pub fn stub_a8899c() -> ! {
    todo!("0xa8899c __ZN3RBX7Network6Player20renderStreamedRegionEPNS_5AdornE")
}

#[doc(alias = "RBX::Network::Player::renderDPhysicsRegion(RBX::Adorn *)")]
// 0xa889c4 — __ZN3RBX7Network6Player20renderDPhysicsRegionEPNS_5AdornE
// type: void __fastcall(RBX::Network::Player *this, RBX::Adorn *)
pub fn stub_a889c4() -> ! {
    todo!("0xa889c4 __ZN3RBX7Network6Player20renderDPhysicsRegionEPNS_5AdornE")
}

#[doc(alias = "RBX::Network::Player::getConstCharacterRoot(void)const")]
// 0xa88c1c — __ZNK3RBX7Network6Player21getConstCharacterRootEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_a88c1c() -> ! {
    todo!("0xa88c1c __ZNK3RBX7Network6Player21getConstCharacterRootEv")
}

#[doc(alias = "RBX::Network::Player::setSimulationRadius(float)")]
// 0xa88c54 — __ZN3RBX7Network6Player19setSimulationRadiusEf
// type: int __fastcall(int this, float)
pub fn stub_a88c54() -> ! {
    todo!("0xa88c54 __ZN3RBX7Network6Player19setSimulationRadiusEf")
}

#[doc(alias = "RBX::Network::Player::setMaxSimulationRadius(float)")]
// 0xa88cb0 — __ZN3RBX7Network6Player22setMaxSimulationRadiusEf
// type: int __fastcall(int this, float32_t)
pub fn stub_a88cb0() -> ! {
    todo!("0xa88cb0 __ZN3RBX7Network6Player22setMaxSimulationRadiusEf")
}

#[doc(alias = "RBX::Network::Player::rebuildBackpack(void)")]
// 0xa88d60 — __ZN3RBX7Network6Player15rebuildBackpackEv
// type: void __fastcall(RBX::Instance **this, int, bool)
pub fn stub_a88d60() -> ! {
    todo!("0xa88d60 __ZN3RBX7Network6Player15rebuildBackpackEv")
}

#[doc(alias = "RBX::Network::Player::rebuildGui(void)")]
// 0xa8942c — __ZN3RBX7Network6Player10rebuildGuiEv
// type: void __fastcall(int **this, int, bool)
pub fn stub_a8942c() -> ! {
    todo!("0xa8942c __ZN3RBX7Network6Player10rebuildGuiEv")
}

#[doc(alias = "RBX::Network::Player::onCharacterDied(void)")]
// 0xa8993c — __ZN3RBX7Network6Player15onCharacterDiedEv
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *, bool)
pub fn stub_a8993c() -> ! {
    todo!("0xa8993c __ZN3RBX7Network6Player15onCharacterDiedEv")
}

#[doc(alias = "RBX::Network::Player::onCharacterChangedFrontend(void)")]
// 0xa89e40 — __ZN3RBX7Network6Player26onCharacterChangedFrontendEv
// type: void __fastcall(RBX::Instance **this, RBX::Instance *, bool)
pub fn stub_a89e40() -> ! {
    todo!("0xa89e40 __ZN3RBX7Network6Player26onCharacterChangedFrontendEv")
}

#[doc(alias = "RBX::Network::Player::calculateNextSpawnLocation(RBX::ServiceProvider const*)")]
// 0xa8a3b8 — __ZN3RBX7Network6Player26calculateNextSpawnLocationEPKNS_15ServiceProviderE
// type: void __fastcall(RBX::Network::Player *this, const RBX::ServiceProvider *, int, int)
pub fn stub_a8a3b8() -> ! {
    todo!("0xa8a3b8 __ZN3RBX7Network6Player26calculateNextSpawnLocationEPKNS_15ServiceProviderE")
}

#[doc(alias = "RBX::Network::Player::loadCharacter(bool,std::string)")]
// 0xa8ad08 — __ZN3RBX7Network6Player13loadCharacterEbSs
// type: void __fastcall(int, RBX::Instance *, int)
pub fn stub_a8ad08() -> ! {
    todo!("0xa8ad08 __ZN3RBX7Network6Player13loadCharacterEbSs")
}

#[doc(alias = "RBX::Network::Player::calculatesSpawnLocationEarly(void)const")]
// 0xa8cd24 — __ZNK3RBX7Network6Player28calculatesSpawnLocationEarlyEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_a8cd24() -> ! {
    todo!("0xa8cd24 __ZNK3RBX7Network6Player28calculatesSpawnLocationEarlyEv")
}

#[doc(alias = "RBX::Network::Player::onLocalPlayerNotIdle(RBX::ServiceProvider *)")]
// 0xa8cd48 — __ZN3RBX7Network6Player20onLocalPlayerNotIdleEPNS_15ServiceProviderE
// type: void __fastcall(RBX::Network::Player *this, RBX::ServiceProvider *, int, int)
pub fn stub_a8cd48() -> ! {
    todo!("0xa8cd48 __ZN3RBX7Network6Player20onLocalPlayerNotIdleEPNS_15ServiceProviderE")
}

#[doc(alias = "RBX::Network::Player::doPeriodicIdleCheck(void)")]
// 0xa8cdd0 — __ZN3RBX7Network6Player19doPeriodicIdleCheckEv
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *)
pub fn stub_a8cdd0() -> ! {
    todo!("0xa8cdd0 __ZN3RBX7Network6Player19doPeriodicIdleCheckEv")
}

#[doc(alias = "RBX::Network::Player::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0xa8d370 — __ZN3RBX7Network6Player17onServiceProviderEPNS_15ServiceProviderES3_
// type: void __fastcall(RBX::Network::Player *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
pub fn stub_a8d370() -> ! {
    todo!("0xa8d370 __ZN3RBX7Network6Player17onServiceProviderEPNS_15ServiceProviderES3_")
}

#[doc(alias = "RBX::Network::Player::removeCharacterAppearance(void)")]
// 0xa8e338 — __ZN3RBX7Network6Player25removeCharacterAppearanceEv
// type: void __fastcall(RBX::Network::Player *this, int, bool)
pub fn stub_a8e338() -> ! {
    todo!("0xa8e338 __ZN3RBX7Network6Player25removeCharacterAppearanceEv")
}

#[doc(alias = "RBX::Network::Player::loadCharacterAppearance(bool)")]
// 0xa8e848 — __ZN3RBX7Network6Player23loadCharacterAppearanceEb
// type: void __fastcall(RBX::Network::Player *this, int, bool)
pub fn stub_a8e848() -> ! {
    todo!("0xa8e848 __ZN3RBX7Network6Player23loadCharacterAppearanceEb")
}

#[doc(alias = "RBX::Network::Player::doFirstSpawnLocationCalculation(RBX::ServiceProvider const*,std::string const&)")]
// 0xa90888 — __ZN3RBX7Network6Player31doFirstSpawnLocationCalculationEPKNS_15ServiceProviderERKSs
// type: void __fastcall(RBX::Network::Player *this, const RBX::ServiceProvider *, const std::string *)
pub fn stub_a90888() -> ! {
    todo!("0xa90888 __ZN3RBX7Network6Player31doFirstSpawnLocationCalculationEPKNS_15ServiceProviderERKSs")
}

#[doc(alias = "RBX::Network::Player::calculateNextSpawnLocationHelper(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*)")]
// 0xa90bdc — __ZN3RBX7Network6Player32calculateNextSpawnLocationHelperERN5boost8weak_ptrIS1_EEPKNS_15ServiceProviderE
// type: void __fastcall(int, const RBX::ServiceProvider *, int, int, int, pthread_mutex_t *, int, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_a90bdc() -> ! {
    todo!("0xa90bdc __ZN3RBX7Network6Player32calculateNextSpawnLocationHelperERN5boost8weak_ptrIS1_EEPKNS_15ServiceProviderE")
}

#[doc(alias = "RBX::Network::Player::calculateSpawnLocation(std::string const&)")]
// 0xa90dfc — __ZN3RBX7Network6Player22calculateSpawnLocationERKSs
// type: void __fastcall(RBX::Network::Player *this, const std::string *, const std::string *)
pub fn stub_a90dfc() -> ! {
    todo!("0xa90dfc __ZN3RBX7Network6Player22calculateSpawnLocationERKSs")
}

#[doc(alias = "RBX::Network::Player::checkContextReadyToSpawnCharacter(void)")]
// 0xa91220 — __ZN3RBX7Network6Player33checkContextReadyToSpawnCharacterEv
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *)
pub fn stub_a91220() -> ! {
    todo!("0xa91220 __ZN3RBX7Network6Player33checkContextReadyToSpawnCharacterEv")
}

#[doc(alias = "RBX::Network::Player::setupHumanoid(boost::shared_ptr<RBX::Humanoid>)")]
// 0xa919a0 — __ZN3RBX7Network6Player13setupHumanoidEN5boost10shared_ptrINS_8HumanoidEEE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_a919a0() -> ! {
    todo!("0xa919a0 __ZN3RBX7Network6Player13setupHumanoidEN5boost10shared_ptrINS_8HumanoidEEE")
}

#[doc(alias = "RBX::Network::Player::setName(std::string const&)")]
// 0xa92024 — __ZN3RBX7Network6Player7setNameERKSs
// type: void __fastcall(RBX::Network::Player *this, const std::string *)
pub fn stub_a92024() -> ! {
    todo!("0xa92024 __ZN3RBX7Network6Player7setNameERKSs")
}

#[doc(alias = "RBX::Network::Player::getPlayerBackpack(void)")]
// 0xa92150 — __ZN3RBX7Network6Player17getPlayerBackpackEv
// type: _UNKNOWN **__fastcall(RBX::Network::Player *this, int, int, int)
pub fn stub_a92150() -> ! {
    todo!("0xa92150 __ZN3RBX7Network6Player17getPlayerBackpackEv")
}

#[doc(alias = "RBX::Network::Player::isFriendsWith(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xa925a4 — __ZN3RBX7Network6Player13isFriendsWithEiN5boost8functionIFvbEEENS3_IFvSsEEE
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int, int *)
pub fn stub_a925a4() -> ! {
    todo!("0xa925a4 __ZN3RBX7Network6Player13isFriendsWithEiN5boost8functionIFvbEEENS3_IFvSsEEE")
}

#[doc(alias = "RBX::Network::Player::isBestFriendsWith(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xa92d24 — __ZN3RBX7Network6Player17isBestFriendsWithEiN5boost8functionIFvbEEENS3_IFvSsEEE
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int)
pub fn stub_a92d24() -> ! {
    todo!("0xa92d24 __ZN3RBX7Network6Player17isBestFriendsWithEiN5boost8functionIFvbEEENS3_IFvSsEEE")
}
