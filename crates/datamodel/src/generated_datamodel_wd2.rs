// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace (10215) exhausted — gap filler EA-sorted asc distinct not yet in crates/datamodel/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x446db0..0x464890 | gap filler EA-sorted ascending next 120 after 0x446d80 (shard WD1)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; ` and  stripped from alias
// Shard: wd2 EA-sorted ascending continuation after datamodel_shard_WD1 (global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x446db0 — __ZNSt6vectorISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_
// type: int __fastcall(int, char *, int *)
#[doc(alias = "std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<boost::condition_variable *,boost::mutex *>*,std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>>,std::pair<boost::condition_variable *,boost::mutex *> const&)")]
#[doc(alias = "__ZNSt6vectorISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_")]
pub fn stub_0x446db0() -> ! {
    todo!("0x446db0 std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<boost::condition_variable *,boost::mutex *>*,std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>>,std::pair<boost::condition_variable *,boost::mutex *> const&)")
}

// 0x446ea8 — __ZNSt12_Vector_baseISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE11_M_allocateEm")]
pub fn stub_0x446ea8() -> ! {
    todo!("0x446ea8 std::_Vector_base<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::_M_allocate(unsigned long)")
}

// 0x446ec0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairIPN5boost18condition_variableEPNS4_5mutexEESA_EET0_T_SC_SB_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::pair<boost::condition_variable *,boost::mutex *> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *>(std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairIPN5boost18condition_variableEPNS4_5mutexEESA_EET0_T_SC_SB_")]
pub fn stub_0x446ec0() -> ! {
    todo!("0x446ec0 std::pair<boost::condition_variable *,boost::mutex *> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *>(std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *)")
}

// 0x446f08 — __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEC2INS1_11thread_dataINS_9function0IvEEEEEEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::function0<void>>>(boost::detail::thread_data<boost::function0<void>> *)")]
#[doc(alias = "__ZN5boost10shared_ptrINS_6detail16thread_data_baseEEC2INS1_11thread_dataINS_9function0IvEEEEEEPT_")]
pub fn stub_0x446f08() -> ! {
    todo!("0x446f08 boost::shared_ptr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::function0<void>>>(boost::detail::thread_data<boost::function0<void>> *)")
}

// 0x446ff0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_0x446ff0() -> ! {
    todo!("0x446ff0 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x447000 — __ZN5boost3_bi5list3INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEENS2_IbEEEC2ES5_S8_S9_
// type: int __fastcall(int, int, int, unsigned __int8)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>::list3(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEENS2_IbEEEC2ES5_S8_S9_")]
pub fn stub_0x447000() -> ! {
    todo!("0x447000 boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>::list3(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>)")
}

// 0x4470d0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x4470d0() -> ! {
    todo!("0x4470d0 __ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")
}

// 0x4471fc — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>)")]
#[doc(alias = "__ZN5boost9function0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEEvT_")]
pub fn stub_0x4471fc() -> ! {
    todo!("0x4471fc void boost::function0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>)")
}

// 0x447338 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEE6manageERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEE6manageERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x447338() -> ! {
    todo!("0x447338 boost::detail::function::functor_manager<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x4473b8 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(void (__fastcall **)(int *, int))
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_0x4473b8() -> ! {
    todo!("0x4473b8 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x447410 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS5_5list1INS5_5valueISsEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS5_5list1INS5_5valueISsEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x447410() -> ! {
    todo!("0x447410 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")
}

// 0x44753c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS5_5list1INS5_5valueISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS5_5list1INS5_5valueISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x44753c() -> ! {
    todo!("0x44753c bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x447674 — __ZN5boost3_bi5list1INS0_5valueISsEEEC2ES3_
// type: std::string *__fastcall(std::string *, const std::string *)
#[doc(alias = "boost::_bi::list1<boost::_bi::value<std::string>>::list1(boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi5list1INS0_5valueISsEEEC2ES3_")]
pub fn stub_0x447674() -> ! {
    todo!("0x447674 boost::_bi::list1<boost::_bi::value<std::string>>::list1(boost::_bi::value<std::string>)")
}

// 0x447794 — __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS0_IFvSsEEESA_ENS7_5list4INS_3argILi1EEENSE_ILi2EEENS7_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS0_IFvSsEEESA_ENS7_5list4INS_3argILi1EEENSE_ILi2EEENS7_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x447794() -> ! {
    todo!("0x447794 __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS0_IFvSsEEESA_ENS7_5list4INS_3argILi1EEENSE_ILi2EEENS7_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

// 0x4478e0 — __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x4478e0() -> ! {
    todo!("0x4478e0 __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

// 0x447a30 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEEvT_
// type: void __fastcall(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEEvT_")]
pub fn stub_0x447a30() -> ! {
    todo!("0x447a30 void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")
}

// 0x447b90 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x447b90() -> ! {
    todo!("0x447b90 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x447bac — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_
// type: int __fastcall(int *, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_")]
pub fn stub_0x447bac() -> ! {
    todo!("0x447bac boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")
}

// 0x447bcc — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x447bcc() -> ! {
    todo!("0x447bcc bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")
}

// 0x447d1c — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x447d1c() -> ! {
    todo!("0x447d1c bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x447e68 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, _DWORD *, _DWORD *, int, void *, int, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x447e68() -> ! {
    todo!("0x447e68 void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x447f68 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EclIPFvPSsPSt9exceptionS8_S8_ENS0_5list2IRSC_RSE_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int, void (__fastcall **)(int, int, _DWORD *, _DWORD *), int **)
#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EclIPFvPSsPSt9exceptionS8_S8_ENS0_5list2IRSC_RSE_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x447f68() -> ! {
    todo!("0x447f68 void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")
}

// 0x44806c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, void *, int, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x44806c() -> ! {
    todo!("0x44806c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x448218 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EC2ES3_S4_S9_S9_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, int)
#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
#[doc(alias = "__ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EC2ES3_S4_S9_S9_")]
pub fn stub_0x448218() -> ! {
    todo!("0x448218 boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")
}

// 0x44830c — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EC2ES3_S4_S9_S9_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EC2ES3_S4_S9_S9_")]
pub fn stub_0x44830c() -> ! {
    todo!("0x44830c boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")
}

// 0x448400 — __ZN5boost9function1IvbE13assign_to_ownERKS1_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function1<void,bool>::assign_to_own(boost::function1<void,bool> const&)")]
#[doc(alias = "__ZN5boost9function1IvbE13assign_to_ownERKS1_")]
pub fn stub_0x448400() -> ! {
    todo!("0x448400 boost::function1<void,bool>::assign_to_own(boost::function1<void,bool> const&)")
}

// 0x4489f8 — __ZN3RBX4Name7declareILZNS_13sAssetServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sAssetServiceEEEERKS0_v")]
pub fn stub_0x4489f8() -> ! {
    todo!("0x4489f8 __ZN3RBX4Name7declareILZNS_13sAssetServiceEEEERKS0_v")
}

// 0x448a3c — __ZN3RBX4Name13callDoDeclareILZNS_13sAssetServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sAssetServiceEEEEvv")]
pub fn stub_0x448a3c() -> ! {
    todo!("0x448a3c __ZN3RBX4Name13callDoDeclareILZNS_13sAssetServiceEEEEvv")
}

// 0x448a40 — __ZN3RBX4Name9doDeclareILZNS_13sAssetServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sAssetServiceEEEERKS0_v")]
pub fn stub_0x448a40() -> ! {
    todo!("0x448a40 __ZN3RBX4Name9doDeclareILZNS_13sAssetServiceEEEERKS0_v")
}

// 0x448cc8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12AssetServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AssetService,RBX::AssetService>(rbx_core::SharedPtr<RBX::AssetService> const*,RBX::AssetService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12AssetServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x448cc8() -> ! {
    todo!("0x448cc8 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AssetService,RBX::AssetService>(boost::shared_ptr<RBX::AssetService> const*,RBX::AssetService *)const")
}

// 0x449180 — __ZN3RBX4Name13callDoDeclareILZNS_14sScriptServiceEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sScriptServiceEEEEvv")]
pub fn stub_0x449180() -> ! {
    todo!("0x449180 __ZN3RBX4Name13callDoDeclareILZNS_14sScriptServiceEEEEvv")
}

// 0x44918c — __ZN3RBX13ScriptServiceD1Ev
// type: void __fastcall(RBX::ScriptService *__hidden this)
#[doc(alias = "RBX::ScriptService::~ScriptService()")]
#[doc(alias = "__ZN3RBX13ScriptServiceD1Ev")]
pub fn stub_0x44918c() -> ! {
    todo!("0x44918c RBX::ScriptService::~ScriptService()")
}

// 0x449270 — __ZN3RBX13ScriptServiceD0Ev
// type: void __fastcall(RBX::ScriptService *__hidden this)
#[doc(alias = "RBX::ScriptService::~ScriptService()")]
#[doc(alias = "__ZN3RBX13ScriptServiceD0Ev")]
pub fn stub_0x449270() -> ! {
    todo!("0x449270 RBX::ScriptService::~ScriptService()")
}

// 0x44936c — __ZThn32_N3RBX13ScriptServiceD1Ev
// type: void __fastcall(RBX::ScriptService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ScriptService::~ScriptService()")]
#[doc(alias = "__ZThn32_N3RBX13ScriptServiceD1Ev")]
pub fn stub_0x44936c() -> ! {
    todo!("0x44936c non-virtual thunk toRBX::ScriptService::~ScriptService()")
}

// 0x449450 — __ZThn32_N3RBX13ScriptServiceD0Ev
// type: void __fastcall(RBX::ScriptService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ScriptService::~ScriptService()")]
#[doc(alias = "__ZThn32_N3RBX13ScriptServiceD0Ev")]
pub fn stub_0x449450() -> ! {
    todo!("0x449450 non-virtual thunk toRBX::ScriptService::~ScriptService()")
}

// 0x44954c — __ZThn36_N3RBX13ScriptServiceD1Ev
// type: void __fastcall(RBX::ScriptService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ScriptService::~ScriptService()")]
#[doc(alias = "__ZThn36_N3RBX13ScriptServiceD1Ev")]
pub fn stub_0x44954c() -> ! {
    todo!("0x44954c non-virtual thunk toRBX::ScriptService::~ScriptService()")
}

// 0x449630 — __ZThn36_N3RBX13ScriptServiceD0Ev
// type: void __fastcall(RBX::ScriptService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ScriptService::~ScriptService()")]
#[doc(alias = "__ZThn36_N3RBX13ScriptServiceD0Ev")]
pub fn stub_0x449630() -> ! {
    todo!("0x449630 non-virtual thunk toRBX::ScriptService::~ScriptService()")
}

// 0x449728 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EED2Ev")]
pub fn stub_0x449728() -> ! {
    todo!("0x449728 std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>::~vector()")
}

// 0x449bd8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ScriptServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScriptService,RBX::ScriptService>(rbx_core::SharedPtr<RBX::ScriptService> const*,RBX::ScriptService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ScriptServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x449bd8() -> ! {
    todo!("0x449bd8 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScriptService,RBX::ScriptService>(boost::shared_ptr<RBX::ScriptService> const*,RBX::ScriptService *)const")
}

// 0x44a068 — __ZN3RBX4Name7declareILZNS_21sContextActionServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_21sContextActionServiceEEEERKS0_v")]
pub fn stub_0x44a068() -> ! {
    todo!("0x44a068 __ZN3RBX4Name7declareILZNS_21sContextActionServiceEEEERKS0_v")
}

// 0x44a0ac — __ZN3RBX4Name13callDoDeclareILZNS_21sContextActionServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_21sContextActionServiceEEEEvv")]
pub fn stub_0x44a0ac() -> ! {
    todo!("0x44a0ac __ZN3RBX4Name13callDoDeclareILZNS_21sContextActionServiceEEEEvv")
}

// 0x44a0b0 — __ZN3RBX4Name9doDeclareILZNS_21sContextActionServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sContextActionServiceEEEERKS0_v")]
pub fn stub_0x44a0b0() -> ! {
    todo!("0x44a0b0 __ZN3RBX4Name9doDeclareILZNS_21sContextActionServiceEEEERKS0_v")
}

// 0x44a338 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ContextActionServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContextActionService,RBX::ContextActionService>(rbx_core::SharedPtr<RBX::ContextActionService> const*,RBX::ContextActionService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ContextActionServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x44a338() -> ! {
    todo!("0x44a338 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContextActionService,RBX::ContextActionService>(boost::shared_ptr<RBX::ContextActionService> const*,RBX::ContextActionService *)const")
}

// 0x44a8f0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9FWServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FWService,RBX::FWService>(rbx_core::SharedPtr<RBX::FWService> const*,RBX::FWService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9FWServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x44a8f0() -> ! {
    todo!("0x44a8f0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FWService,RBX::FWService>(boost::shared_ptr<RBX::FWService> const*,RBX::FWService *)const")
}

// 0x44ab28 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network7PlayersES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Players,RBX::Network::Players>(rbx_core::SharedPtr<RBX::Network::Players> const*,RBX::Network::Players *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network7PlayersES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x44ab28() -> ! {
    todo!("0x44ab28 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Players,RBX::Network::Players>(boost::shared_ptr<RBX::Network::Players> const*,RBX::Network::Players *)const")
}

// 0x44ae78 — __ZN3RBX4Name7declareILZNS_22sPersonalServerServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_22sPersonalServerServiceEEEERKS0_v")]
pub fn stub_0x44ae78() -> ! {
    todo!("0x44ae78 __ZN3RBX4Name7declareILZNS_22sPersonalServerServiceEEEERKS0_v")
}

// 0x44aebc — __ZN3RBX4Name13callDoDeclareILZNS_22sPersonalServerServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_22sPersonalServerServiceEEEEvv")]
pub fn stub_0x44aebc() -> ! {
    todo!("0x44aebc __ZN3RBX4Name13callDoDeclareILZNS_22sPersonalServerServiceEEEEvv")
}

// 0x44aec0 — __ZN3RBX4Name9doDeclareILZNS_22sPersonalServerServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_22sPersonalServerServiceEEEERKS0_v")]
pub fn stub_0x44aec0() -> ! {
    todo!("0x44aec0 __ZN3RBX4Name9doDeclareILZNS_22sPersonalServerServiceEEEERKS0_v")
}

// 0x44b148 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_21PersonalServerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PersonalServerService,RBX::PersonalServerService>(rbx_core::SharedPtr<RBX::PersonalServerService> const*,RBX::PersonalServerService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_21PersonalServerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x44b148() -> ! {
    todo!("0x44b148 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PersonalServerService,RBX::PersonalServerService>(boost::shared_ptr<RBX::PersonalServerService> const*,RBX::PersonalServerService *)const")
}

// 0x44b678 — __ZN3RBX4Name7declareILZNS_16sTeleportServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sTeleportServiceEEEERKS0_v")]
pub fn stub_0x44b678() -> ! {
    todo!("0x44b678 __ZN3RBX4Name7declareILZNS_16sTeleportServiceEEEERKS0_v")
}

// 0x44b6c0 — __ZN3RBX4Name9doDeclareILZNS_16sTeleportServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sTeleportServiceEEEERKS0_v")]
pub fn stub_0x44b6c0() -> ! {
    todo!("0x44b6c0 __ZN3RBX4Name9doDeclareILZNS_16sTeleportServiceEEEERKS0_v")
}

// 0x44b948 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15TeleportServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TeleportService,RBX::TeleportService>(rbx_core::SharedPtr<RBX::TeleportService> const*,RBX::TeleportService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15TeleportServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x44b948() -> ! {
    todo!("0x44b948 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TeleportService,RBX::TeleportService>(boost::shared_ptr<RBX::TeleportService> const*,RBX::TeleportService *)const")
}

// 0x44be78 — __ZN3RBX4Name7declareILZNS_15sCookiesServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sCookiesServiceEEEERKS0_v")]
pub fn stub_0x44be78() -> ! {
    todo!("0x44be78 __ZN3RBX4Name7declareILZNS_15sCookiesServiceEEEERKS0_v")
}

// 0x44bebc — __ZN3RBX4Name13callDoDeclareILZNS_15sCookiesServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sCookiesServiceEEEEvv")]
pub fn stub_0x44bebc() -> ! {
    todo!("0x44bebc __ZN3RBX4Name13callDoDeclareILZNS_15sCookiesServiceEEEEvv")
}

// 0x44bec0 — __ZN3RBX4Name9doDeclareILZNS_15sCookiesServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sCookiesServiceEEEERKS0_v")]
pub fn stub_0x44bec0() -> ! {
    todo!("0x44bec0 __ZN3RBX4Name9doDeclareILZNS_15sCookiesServiceEEEERKS0_v")
}

// 0x44c148 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CookiesServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CookiesService,RBX::CookiesService>(rbx_core::SharedPtr<RBX::CookiesService> const*,RBX::CookiesService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CookiesServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x44c148() -> ! {
    todo!("0x44c148 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CookiesService,RBX::CookiesService>(boost::shared_ptr<RBX::CookiesService> const*,RBX::CookiesService *)const")
}

// 0x4557a4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11ChatServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const*,RBX::ChatService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11ChatServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x4557a4() -> ! {
    todo!("0x4557a4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(boost::shared_ptr<RBX::ChatService> const*,RBX::ChatService *)const")
}

// 0x4559dc — __ZN3RBX4Name7declareILZNS_12sChatServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sChatServiceEEEERKS0_v")]
pub fn stub_0x4559dc() -> ! {
    todo!("0x4559dc __ZN3RBX4Name7declareILZNS_12sChatServiceEEEERKS0_v")
}

// 0x455a20 — __ZN3RBX4Name13callDoDeclareILZNS_12sChatServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sChatServiceEEEEvv")]
pub fn stub_0x455a20() -> ! {
    todo!("0x455a20 __ZN3RBX4Name13callDoDeclareILZNS_12sChatServiceEEEEvv")
}

// 0x455a24 — __ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v")]
pub fn stub_0x455a24() -> ! {
    todo!("0x455a24 __ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v")
}

// 0x45623c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10GuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const*,RBX::GuiService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10GuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x45623c() -> ! {
    todo!("0x45623c void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(boost::shared_ptr<RBX::GuiService> const*,RBX::GuiService *)const")
}

// 0x4566cc — __ZN3RBX4Name7declareILZNS_25sKeyframeSequenceProviderEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_25sKeyframeSequenceProviderEEEERKS0_v")]
pub fn stub_0x4566cc() -> ! {
    todo!("0x4566cc __ZN3RBX4Name7declareILZNS_25sKeyframeSequenceProviderEEEERKS0_v")
}

// 0x456710 — __ZN3RBX4Name13callDoDeclareILZNS_25sKeyframeSequenceProviderEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_25sKeyframeSequenceProviderEEEEvv")]
pub fn stub_0x456710() -> ! {
    todo!("0x456710 __ZN3RBX4Name13callDoDeclareILZNS_25sKeyframeSequenceProviderEEEEvv")
}

// 0x4568bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_24KeyframeSequenceProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_24KeyframeSequenceProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x4568bc() -> ! {
    todo!("0x4568bc void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(boost::shared_ptr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const")
}

// 0x456dec — __ZN3RBX4Name7declareILZNS_14sContentFilterEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sContentFilterEEEERKS0_v")]
pub fn stub_0x456dec() -> ! {
    todo!("0x456dec __ZN3RBX4Name7declareILZNS_14sContentFilterEEEERKS0_v")
}

// 0x456e30 — __ZN3RBX4Name13callDoDeclareILZNS_14sContentFilterEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sContentFilterEEEEvv")]
pub fn stub_0x456e30() -> ! {
    todo!("0x456e30 __ZN3RBX4Name13callDoDeclareILZNS_14sContentFilterEEEEvv")
}

// 0x456e34 — __ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v")]
pub fn stub_0x456e34() -> ! {
    todo!("0x456e34 __ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v")
}

// 0x4570bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ContentFilterES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const*,RBX::ContentFilter *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ContentFilterES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x4570bc() -> ! {
    todo!("0x4570bc void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const*,RBX::ContentFilter *)const")
}

// 0x457398 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14delete_bucketsEv
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14delete_bucketsEv")]
pub fn stub_0x457398() -> ! {
    todo!("0x457398 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void)")
}

// 0x4573e4 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEEC2EmRKS6_RKS8_RKSaINS1_8ptr_nodeIjEEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEEC2EmRKS6_RKS8_RKSaINS1_8ptr_nodeIjEEE")]
pub fn stub_0x4573e4() -> ! {
    todo!("0x4573e4 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&)")
}

// 0x457d60 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7GuiRootES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiRoot,RBX::GuiRoot>(rbx_core::SharedPtr<RBX::GuiRoot> const*,RBX::GuiRoot *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7GuiRootES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x457d60() -> ! {
    todo!("0x457d60 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiRoot,RBX::GuiRoot>(boost::shared_ptr<RBX::GuiRoot> const*,RBX::GuiRoot *)const")
}

// 0x4585f4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ChangeHistoryServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChangeHistoryService,RBX::ChangeHistoryService>(rbx_core::SharedPtr<RBX::ChangeHistoryService> const*,RBX::ChangeHistoryService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ChangeHistoryServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x4585f4() -> ! {
    todo!("0x4585f4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChangeHistoryService,RBX::ChangeHistoryService>(boost::shared_ptr<RBX::ChangeHistoryService> const*,RBX::ChangeHistoryService *)const")
}

// 0x45882c — __ZN3RBX4Name7declareILZNS_21sChangeHistoryServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_21sChangeHistoryServiceEEEERKS0_v")]
pub fn stub_0x45882c() -> ! {
    todo!("0x45882c __ZN3RBX4Name7declareILZNS_21sChangeHistoryServiceEEEERKS0_v")
}

// 0x458870 — __ZN3RBX4Name13callDoDeclareILZNS_21sChangeHistoryServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_21sChangeHistoryServiceEEEEvv")]
pub fn stub_0x458870() -> ! {
    todo!("0x458870 __ZN3RBX4Name13callDoDeclareILZNS_21sChangeHistoryServiceEEEEvv")
}

// 0x458874 — __ZN3RBX4Name9doDeclareILZNS_21sChangeHistoryServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sChangeHistoryServiceEEEERKS0_v")]
pub fn stub_0x458874() -> ! {
    todo!("0x458874 __ZN3RBX4Name9doDeclareILZNS_21sChangeHistoryServiceEEEERKS0_v")
}

// 0x458e44 — __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EED2Ev
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EED2Ev")]
pub fn stub_0x458e44() -> ! {
    todo!("0x458e44 std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::~vector()")
}

// 0x4597b4 — __ZN3RBX4Name7declareILZNS_6sVisitEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_6sVisitEEEERKS0_v")]
pub fn stub_0x4597b4() -> ! {
    todo!("0x4597b4 __ZN3RBX4Name7declareILZNS_6sVisitEEEERKS0_v")
}

// 0x4597f8 — __ZN3RBX4Name13callDoDeclareILZNS_6sVisitEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sVisitEEEEvv")]
pub fn stub_0x4597f8() -> ! {
    todo!("0x4597f8 __ZN3RBX4Name13callDoDeclareILZNS_6sVisitEEEEvv")
}

// 0x4597fc — __ZN3RBX4Name9doDeclareILZNS_6sVisitEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sVisitEEEERKS0_v")]
pub fn stub_0x4597fc() -> ! {
    todo!("0x4597fc __ZN3RBX4Name9doDeclareILZNS_6sVisitEEEERKS0_v")
}

// 0x459d14 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5VisitES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Visit,RBX::Visit>(rbx_core::SharedPtr<RBX::Visit> const*,RBX::Visit *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5VisitES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x459d14() -> ! {
    todo!("0x459d14 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Visit,RBX::Visit>(boost::shared_ptr<RBX::Visit> const*,RBX::Visit *)const")
}

// 0x45a690 — __ZN3rbx8any_castISsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::string rbx::any_cast<std::string,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castISsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x45a690() -> ! {
    todo!("0x45a690 std::string rbx::any_cast<std::string,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x45d108 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKbNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKbNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_")]
pub fn stub_0x45d108() -> ! {
    todo!("0x45d108 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")
}

// 0x45d228 — __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x45d228() -> ! {
    todo!("0x45d228 __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

// 0x45d310 — __ZN5boost9function1IvbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IvbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_0x45d310() -> ! {
    todo!("0x45d310 void boost::function1<void,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")
}

// 0x45d408 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x45d408() -> ! {
    todo!("0x45d408 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x45d428 — __ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x45d428() -> ! {
    todo!("0x45d428 void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x45d500 — __ZN3rbx7signals6signalIFvbEE6insertEPNS3_4slotE
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(bool)>::insert(rbx::signals::signal<void ()(bool)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE6insertEPNS3_4slotE")]
pub fn stub_0x45d500() -> ! {
    todo!("0x45d500 rbx::signals::signal<void ()(bool)>::insert(rbx::signals::signal<void ()(bool)>::slot *)")
}

// 0x45d710 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::callable<rbx::signals::signal<void ()(bool)>*>(boost::function<void ()(bool)> const&,rbx::signals::signal<void ()(bool)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_")]
pub fn stub_0x45d710() -> ! {
    todo!("0x45d710 rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::callable<rbx::signals::signal<void ()(bool)>*>(boost::function<void ()(bool)> const&,rbx::signals::signal<void ()(bool)>*)")
}

// 0x45d810 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb")]
pub fn stub_0x45d810() -> ! {
    todo!("0x45d810 rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")
}

// 0x45d818 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb")]
pub fn stub_0x45d818() -> ! {
    todo!("0x45d818 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")
}

// 0x45d820 — __ZN3rbx7signals6signalIFvbEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(bool)>::remove(rbx::signals::signal<void ()(bool)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE6removeEPNS3_4slotE")]
pub fn stub_0x45d820() -> ! {
    todo!("0x45d820 rbx::signals::signal<void ()(bool)>::remove(rbx::signals::signal<void ()(bool)>::slot *)")
}

// 0x45d910 — __ZN3rbx7signals6signalIFvbEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x45d910() -> ! {
    todo!("0x45d910 rbx::signals::signal<void ()(bool)>::slot::safe_static_init_mutex(void)")
}

// 0x45d918 — __ZN3rbx7signals6signalIFvbEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE4slotD0Ev")]
pub fn stub_0x45d918() -> ! {
    todo!("0x45d918 rbx::signals::signal<void ()(bool)>::slot::~slot()")
}

// 0x45dfb8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED1Ev")]
pub fn stub_0x45dfb8() -> ! {
    todo!("0x45dfb8 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()")
}

// 0x45e0c8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED0Ev")]
pub fn stub_0x45e0c8() -> ! {
    todo!("0x45e0c8 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()")
}

// 0x4634e4 — __ZN3RBX10Reflection17BoundCallbackDescIFbvEED0Ev
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::~BoundCallbackDesc()")]
#[doc(alias = "__ZN3RBX10Reflection17BoundCallbackDescIFbvEED0Ev")]
pub fn stub_0x4634e4() -> ! {
    todo!("0x4634e4 RBX::Reflection::BoundCallbackDesc<bool ()(void)>::~BoundCallbackDesc()")
}

// 0x4635f0 — __ZNK3RBX10Reflection16CallbackDescImplIFbvELi0EE18setGenericCallbackEPNS0_13DescribedBaseEN5boost10shared_ptrINS6_8functionIFNS7_INS0_5TupleEEENS7_IKS9_EEEEEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::setGenericCallback(RBX::Reflection::DescribedBase *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)const")]
#[doc(alias = "__ZNK3RBX10Reflection16CallbackDescImplIFbvELi0EE18setGenericCallbackEPNS0_13DescribedBaseEN5boost10shared_ptrINS6_8functionIFNS7_INS0_5TupleEEENS7_IKS9_EEEEEEE")]
pub fn stub_0x4635f0() -> ! {
    todo!("0x4635f0 RBX::Reflection::CallbackDescImpl<bool ()(void),0>::setGenericCallback(RBX::Reflection::DescribedBase *,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)const")
}

// 0x463730 — __ZNK3RBX10Reflection12CallbackDescIFbvEE13clearCallbackEPNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::CallbackDesc<bool ()(void)>::clearCallback(RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection12CallbackDescIFbvEE13clearCallbackEPNS0_13DescribedBaseE")]
pub fn stub_0x463730() -> ! {
    todo!("0x463730 RBX::Reflection::CallbackDesc<bool ()(void)>::clearCallback(RBX::Reflection::DescribedBase *)const")
}

// 0x4637f0 — __ZN5boost4bindIbNS_10shared_ptrINS_8functionIFNS1_IN3RBX10Reflection5TupleEEENS1_IKS5_EEEEEEESB_EENS_3_bi6bind_tIT_PFSE_T0_ENSC_9list_av_1IT1_E4typeEEESH_SJ_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list_av_1<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::type> boost::bind<bool,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
#[doc(alias = "__ZN5boost4bindIbNS_10shared_ptrINS_8functionIFNS1_IN3RBX10Reflection5TupleEEENS1_IKS5_EEEEEEESB_EENS_3_bi6bind_tIT_PFSE_T0_ENSC_9list_av_1IT1_E4typeEEESH_SJ_")]
pub fn stub_0x4637f0() -> ! {
    todo!("0x4637f0 boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list_av_1<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>::type> boost::bind<bool,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>(bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)")
}

// 0x463908 — __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EE11callGenericEN5boost10shared_ptrINS4_8functionIFNS5_INS0_5TupleEEENS5_IKS7_EEEEEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::callGeneric(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
#[doc(alias = "__ZN3RBX10Reflection16CallbackDescImplIFbvELi0EE11callGenericEN5boost10shared_ptrINS4_8functionIFNS5_INS0_5TupleEEENS5_IKS7_EEEEEEE")]
pub fn stub_0x463908() -> ! {
    todo!("0x463908 RBX::Reflection::CallbackDescImpl<bool ()(void),0>::callGeneric(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)")
}

// 0x463a5c — __ZN3RBX10Reflection12CallbackDescIFbvEE11callGenericIbEEN5boost10disable_ifINS5_7is_voidIT_EES8_E4typeENS5_10shared_ptrINS5_8functionIFNSC_INS0_5TupleEEENSC_IKSE_EEEEEEESF_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::disable_if<boost::is_void<bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::callGeneric<bool>(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
#[doc(alias = "__ZN3RBX10Reflection12CallbackDescIFbvEE11callGenericIbEEN5boost10disable_ifINS5_7is_voidIT_EES8_E4typeENS5_10shared_ptrINS5_8functionIFNSC_INS0_5TupleEEENSC_IKSE_EEEEEEESF_")]
pub fn stub_0x463a5c() -> ! {
    todo!("0x463a5c boost::disable_if<boost::is_void<bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::callGeneric<bool>(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Reflection::Tuple>)")
}

// 0x463b98 — __ZN3RBX10Reflection12CallbackDescIFbvEE13convertResultIbEEN5boost10disable_ifINS5_7is_sameINS5_10shared_ptrIKNS0_5TupleEEET_EESC_E4typeENS8_IS9_EE
// type: int(void)
#[doc(alias = "boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::convertResult<bool>(rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
#[doc(alias = "__ZN3RBX10Reflection12CallbackDescIFbvEE13convertResultIbEEN5boost10disable_ifINS5_7is_sameINS5_10shared_ptrIKNS0_5TupleEEET_EESC_E4typeENS8_IS9_EE")]
pub fn stub_0x463b98() -> ! {
    todo!("0x463b98 boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Reflection::Tuple const>,bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::convertResult<bool>(boost::shared_ptr<RBX::Reflection::Tuple>)")
}

// 0x463ce8 — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC2IS3_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC2IS3_EEPT_")]
pub fn stub_0x463ce8() -> ! {
    todo!("0x463ce8 boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")
}

// 0x463dc0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEED1Ev")]
pub fn stub_0x463dc0() -> ! {
    todo!("0x463dc0 boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::~sp_counted_impl_p()")
}

// 0x463dc8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE7disposeEv")]
pub fn stub_0x463dc8() -> ! {
    todo!("0x463dc8 boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::dispose(void)")
}

// 0x463e70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE19get_untyped_deleterEv")]
pub fn stub_0x463e70() -> ! {
    todo!("0x463e70 boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::get_untyped_deleter(void)")
}

// 0x463e74 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEC2ESE_
#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>::list1(boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>)")]
#[doc(alias = "__ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEC2ESE_")]
pub fn stub_0x463e74() -> ! {
    todo!("0x463e74 boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>::list1(boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>)")
}

// 0x463f54 — __ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x463f54() -> ! {
    todo!("0x463f54 __ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

// 0x464030 — __ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x464030() -> ! {
    todo!("0x464030 __ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

// 0x464110 — __ZN5boost9function0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>)")]
#[doc(alias = "__ZN5boost9function0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEEvT_")]
pub fn stub_0x464110() -> ! {
    todo!("0x464110 void boost::function0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>)")
}

// 0x464200 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x464200() -> ! {
    todo!("0x464200 boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x46421c — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEbE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,bool>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEbE6invokeERNS1_15function_bufferE")]
pub fn stub_0x46421c() -> ! {
    todo!("0x46421c boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>,bool>::invoke(boost::detail::function::function_buffer &)")
}

// 0x464230 — __ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x464230() -> ! {
    todo!("0x464230 bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &)const")
}

// 0x464310 — __ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x464310() -> ! {
    todo!("0x464310 bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x464408 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEclIbPFbSD_ENS0_5list0EEET_NS0_4typeISK_EERT0_RT1_l
#[doc(alias = "bool boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>::operator()<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list0>(boost::_bi::type<bool>,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>) &,boost::_bi::list0 &,long)")]
#[doc(alias = "__ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEclIbPFbSD_ENS0_5list0EEET_NS0_4typeISK_EERT0_RT1_l")]
pub fn stub_0x464408() -> ! {
    todo!("0x464408 bool boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>::operator()<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list0>(boost::_bi::type<bool>,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>) &,boost::_bi::list0 &,long)")
}

// 0x4644d8 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE12manage_smallERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// type: int(void)
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE12manage_smallERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x4644d8() -> ! {
    todo!("0x4644d8 boost::detail::function::functor_manager_common<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x46455c — __ZN3RBX10Reflection12CallbackDescIFbvEED1Ev
#[doc(alias = "RBX::Reflection::CallbackDesc<bool ()(void)>::~CallbackDesc()")]
#[doc(alias = "__ZN3RBX10Reflection12CallbackDescIFbvEED1Ev")]
pub fn stub_0x46455c() -> ! {
    todo!("0x46455c RBX::Reflection::CallbackDesc<bool ()(void)>::~CallbackDesc()")
}

// 0x464654 — __ZN3RBX10Reflection12CallbackDescIFbvEED0Ev
#[doc(alias = "RBX::Reflection::CallbackDesc<bool ()(void)>::~CallbackDesc()")]
#[doc(alias = "__ZN3RBX10Reflection12CallbackDescIFbvEED0Ev")]
pub fn stub_0x464654() -> ! {
    todo!("0x464654 RBX::Reflection::CallbackDesc<bool ()(void)>::~CallbackDesc()")
}

// 0x4647cc — __ZN5boost8functionIFbvEEaSERKS2_
#[doc(alias = "boost::function<bool ()(void)>::operator=(boost::function<bool ()(void)> const&)")]
#[doc(alias = "__ZN5boost8functionIFbvEEaSERKS2_")]
pub fn stub_0x4647cc() -> ! {
    todo!("0x4647cc boost::function<bool ()(void)>::operator=(boost::function<bool ()(void)> const&)")
}

// 0x464890 — __ZN5boost9function0IbE4swapERS1_
#[doc(alias = "boost::function0<bool>::swap(boost::function0<bool>&)")]
#[doc(alias = "__ZN5boost9function0IbE4swapERS1_")]
pub fn stub_0x464890() -> ! {
    todo!("0x464890 boost::function0<bool>::swap(boost::function0<bool>&)")
}
