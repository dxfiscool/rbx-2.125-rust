//! core bg8 — 100 core stubs EA-sorted asc distinct not yet in rbx_core nor global set.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua) global distinct not yet in crates/rbx_core/src nor /tmp/global_eas.txt — next 100 uncovered after 0x755834 (bg7 max) -> 0x914a9c..0x93b818.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone(void)const")]
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE5cloneEv")]
// 0x914a9c — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE5cloneEv
// type: 
pub fn stub_914a9c() -> ! {
    todo!("0x914a9c __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::rethrow(void)const")]
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE7rethrowEv")]
// 0x914b58 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE7rethrowEv
// type: int(void)
pub fn stub_914b58() -> ! {
    todo!("0x914b58 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE7rethrowEv")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::~clone_impl()")]
#[doc(alias = "__ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED0Ev")]
// 0x914c88 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED0Ev
// type: 
pub fn stub_914c88() -> ! {
    todo!("0x914c88 __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED0Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone(void)const")]
#[doc(alias = "__ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE5cloneEv")]
// 0x914ca0 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE5cloneEv
// type: 
pub fn stub_914ca0() -> ! {
    todo!("0x914ca0 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE5cloneEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::rethrow(void)const")]
#[doc(alias = "__ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE7rethrowEv")]
// 0x914cac — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE7rethrowEv
// type: 
pub fn stub_914cac() -> ! {
    todo!("0x914cac __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE7rethrowEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::~clone_impl()")]
#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED0Ev")]
// 0x914cbc — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED0Ev
// type: 
pub fn stub_914cbc() -> ! {
    todo!("0x914cbc __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_lexical_cast>::~error_info_injector()")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED0Ev")]
// 0x914cd8 — __ZN5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED0Ev
// type: 
pub fn stub_914cd8() -> ! {
    todo!("0x914cd8 __ZN5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED0Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::bad_lexical_cast>::~error_info_injector()")]
#[doc(alias = "__ZThn12_N5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED0Ev")]
// 0x914cec — __ZThn12_N5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED0Ev
// type: 
pub fn stub_914cec() -> ! {
    todo!("0x914cec __ZThn12_N5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone_tag)")]
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEEC1ERKS5_NS5_9clone_tagE")]
// 0x914d04 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEEC1ERKS5_NS5_9clone_tagE
// type: 
pub fn stub_914d04() -> ! {
    todo!("0x914d04 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEEC1ERKS5_NS5_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone_impl(boost::exception_detail::error_info_injector<boost::bad_lexical_cast> const&)")]
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEEC1ERKS4_")]
// 0x914e40 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEEC1ERKS4_
// type: 
pub fn stub_914e40() -> ! {
    todo!("0x914e40 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEEC1ERKS4_")
}

#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<int>(int &)")]
#[doc(alias = "__ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIiEEbRT_")]
// 0x914f7c — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIiEEbRT_
// type: int(void)
pub fn stub_914f7c() -> ! {
    todo!("0x914f7c __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIiEEbRT_")
}

#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned int,char>(unsigned int &,char const*,char const*)")]
#[doc(alias = "__ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEjcEEbRT0_PKT1_S8_")]
// 0x914fdc — __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEjcEEbRT0_PKT1_S8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_914fdc() -> ! {
    todo!("0x914fdc __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEjcEEbRT0_PKT1_S8_")
}

#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS0_IFvbEEENS0_IFvSsEEEEENS7_5list5INS7_5valueIPSC_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS0_IFvbEEENS0_IFvSsEEEEENS7_5list5INS7_5valueIPSC_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
// 0x915374 — __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS0_IFvbEEENS0_IFvSsEEEEENS7_5list5INS7_5valueIPSC_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_915374() -> ! {
    todo!("0x915374 __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS0_IFvbEEENS0_IFvSsEEEEENS7_5list5INS7_5valueIPSC_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
// 0x9154d0 — __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_9154d0() -> ! {
    todo!("0x9154d0 __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_")]
// 0x91562c — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_91562c() -> ! {
    todo!("0x91562c __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE")]
// 0x91579c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
// type: 
pub fn stub_91579c() -> ! {
    todo!("0x91579c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_")]
// 0x9157b8 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_9157b8() -> ! {
    todo!("0x9157b8 __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE")]
// 0x9157dc — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_9157dc() -> ! {
    todo!("0x9157dc __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x91593c — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, void *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_91593c() -> ! {
    todo!("0x91593c __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x915a98 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
pub fn stub_915a98() -> ! {
    todo!("0x915a98 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>> &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0x915ba4 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, char, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, int, int, int)
pub fn stub_915ba4() -> ! {
    todo!("0x915ba4 __ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::operator()(RBX::AssetService*,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_")]
// 0x915cb4 — __ZNK5boost4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, int, int, int)
pub fn stub_915cb4() -> ! {
    todo!("0x915cb4 __ZNK5boost4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x915dd4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_915dd4() -> ! {
    todo!("0x915dd4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")]
// 0x915f8c — __ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
// type: 
pub fn stub_915f8c() -> ! {
    todo!("0x915f8c __ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")]
// 0x916088 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
// type: 
pub fn stub_916088() -> ! {
    todo!("0x916088 __ZN5boost3_bi8storage5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")
}

#[doc(alias = "RBX::AssetService::AccessType * rbx::any_cast<RBX::AssetService::AccessType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX12AssetService10AccessTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x917198 — __ZN3rbx8any_castIN3RBX12AssetService10AccessTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: 
pub fn stub_917198() -> ! {
    todo!("0x917198 __ZN3rbx8any_castIN3RBX12AssetService10AccessTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::AssetService::AccessType & rbx::any_cast<RBX::AssetService::AccessType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x9171f0 — __ZN3rbx8any_castIRN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_9171f0() -> ! {
    todo!("0x9171f0 __ZN3rbx8any_castIRN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::resize(unsigned long,RBX::AssetService::AccessType)")]
#[doc(alias = "__ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE6resizeEmS2_")]
// 0x9172e0 — __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE6resizeEmS2_
// type: int(void)
pub fn stub_9172e0() -> ! {
    todo!("0x9172e0 __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::push_back(RBX::AssetService::AccessType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE9push_backERKS2_")]
// 0x917314 — __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE9push_backERKS2_
// type: int(void)
pub fn stub_917314() -> ! {
    todo!("0x917314 __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::AssetService::AccessType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_12AssetService10AccessTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0x91733c — __ZNSt3mapIPKN3RBX4NameENS0_12AssetService10AccessTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
pub fn stub_91733c() -> ! {
    todo!("0x91733c __ZNSt3mapIPKN3RBX4NameENS0_12AssetService10AccessTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::pair<RBX::Name const* const,RBX::AssetService::AccessType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0x917394 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_917394() -> ! {
    todo!("0x917394 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::AssetService::AccessType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0x917448 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
pub fn stub_917448() -> ! {
    todo!("0x917448 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::AssetService::AccessType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0x9174a0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
pub fn stub_9174a0() -> ! {
    todo!("0x9174a0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AssetService::AccessType*,std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>>,RBX::AssetService::AccessType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x917508 — __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_917508() -> ! {
    todo!("0x917508 __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX12AssetService10AccessTypeESaIS2_EE11_M_allocateEm")]
// 0x9175ec — __ZNSt12_Vector_baseIN3RBX12AssetService10AccessTypeESaIS2_EE11_M_allocateEm
// type: int(void)
pub fn stub_9175ec() -> ! {
    todo!("0x9175ec __ZNSt12_Vector_baseIN3RBX12AssetService10AccessTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::AssetService::AccessType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AssetService::AccessType *,RBX::AssetService::AccessType *>(RBX::AssetService::AccessType *,RBX::AssetService::AccessType *,RBX::AssetService::AccessType *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12AssetService10AccessTypeES6_EET0_T_S8_S7_")]
// 0x917604 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12AssetService10AccessTypeES6_EET0_T_S8_S7_
// type: int(void)
pub fn stub_917604() -> ! {
    todo!("0x917604 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12AssetService10AccessTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::AssetService::AccessType*,std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>>,unsigned long,RBX::AssetService::AccessType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0x917640 — __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
pub fn stub_917640() -> ! {
    todo!("0x917640 __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::utf8_encode(std::string const&)")]
#[doc(alias = "__ZN3RBX11utf8_encodeERKSs")]
// 0x919e14 — __ZN3RBX11utf8_encodeERKSs
// type: _DWORD __fastcall(RBX *__hidden this, const std::string *)
pub fn stub_919e14() -> ! {
    todo!("0x919e14 __ZN3RBX11utf8_encodeERKSs")
}

#[doc(alias = "RBX::utf8_decode(std::string const&)")]
#[doc(alias = "__ZN3RBX11utf8_decodeERKSs")]
// 0x919e20 — __ZN3RBX11utf8_decodeERKSs
// type: _DWORD __fastcall(RBX *__hidden this, const std::string *)
pub fn stub_919e20() -> ! {
    todo!("0x919e20 __ZN3RBX11utf8_decodeERKSs")
}

#[doc(alias = "RBX::RemoteFunction::processDelayedInvocations(void)")]
#[doc(alias = "__ZN3RBX14RemoteFunction25processDelayedInvocationsEv")]
// 0x91a6cc — __ZN3RBX14RemoteFunction25processDelayedInvocationsEv
// type: _DWORD __fastcall(RBX::RemoteFunction *__hidden this)
pub fn stub_91a6cc() -> ! {
    todo!("0x91a6cc __ZN3RBX14RemoteFunction25processDelayedInvocationsEv")
}

#[doc(alias = "RBX::DelayedInvocationQueue::push(boost::function<void ()(void)> const&)")]
#[doc(alias = "__ZN3RBX22DelayedInvocationQueue4pushERKN5boost8functionIFvvEEE")]
// 0x91aff4 — __ZN3RBX22DelayedInvocationQueue4pushERKN5boost8functionIFvvEEE
// type: 
pub fn stub_91aff4() -> ! {
    todo!("0x91aff4 __ZN3RBX22DelayedInvocationQueue4pushERKN5boost8functionIFvvEEE")
}

#[doc(alias = "RBX::DelayedInvocationQueue::process(void)")]
#[doc(alias = "__ZN3RBX22DelayedInvocationQueue7processEv")]
// 0x91b014 — __ZN3RBX22DelayedInvocationQueue7processEv
// type: _DWORD __fastcall(RBX::DelayedInvocationQueue *__hidden this)
pub fn stub_91b014() -> ! {
    todo!("0x91b014 __ZN3RBX22DelayedInvocationQueue7processEv")
}

#[doc(alias = "RBX::RemoteFunction::RemoteFunction(void)")]
#[doc(alias = "__ZN3RBX14RemoteFunctionC2Ev")]
// 0x91b1a4 — __ZN3RBX14RemoteFunctionC2Ev
// type: _DWORD __fastcall(RBX::RemoteFunction *__hidden this)
pub fn stub_91b1a4() -> ! {
    todo!("0x91b1a4 __ZN3RBX14RemoteFunctionC2Ev")
}

#[doc(alias = "RBX::RemoteFunction::localError(int,std::string)")]
#[doc(alias = "__ZN3RBX14RemoteFunction10localErrorEiSs")]
// 0x91b654 — __ZN3RBX14RemoteFunction10localErrorEiSs
// type: 
pub fn stub_91b654() -> ! {
    todo!("0x91b654 __ZN3RBX14RemoteFunction10localErrorEiSs")
}

#[doc(alias = "RBX::RemoteFunction::consumeRemoteInvocation(int,RBX::RemoteFunction::RemoteInvocation &)")]
#[doc(alias = "__ZN3RBX14RemoteFunction23consumeRemoteInvocationEiRNS0_16RemoteInvocationE")]
// 0x91c640 — __ZN3RBX14RemoteFunction23consumeRemoteInvocationEiRNS0_16RemoteInvocationE
// type: 
pub fn stub_91c640() -> ! {
    todo!("0x91c640 __ZN3RBX14RemoteFunction23consumeRemoteInvocationEiRNS0_16RemoteInvocationE")
}

#[doc(alias = "RBX::RemoteEvent::RemoteEvent(void)")]
#[doc(alias = "__ZN3RBX11RemoteEventC2Ev")]
// 0x91cf14 — __ZN3RBX11RemoteEventC2Ev
// type: _DWORD __fastcall(RBX::RemoteEvent *__hidden this)
pub fn stub_91cf14() -> ! {
    todo!("0x91cf14 __ZN3RBX11RemoteEventC2Ev")
}

#[doc(alias = "std::vector<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::push_back(boost::function<void ()(void)> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE9push_backERKS3_")]
// 0x91db1c — __ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE9push_backERKS3_
// type: 
pub fn stub_91db1c() -> ! {
    todo!("0x91db1c __ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE9push_backERKS3_")
}

#[doc(alias = "std::map<int,RBX::RemoteFunction::RemoteInvocation,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::operator[](int const&)")]
#[doc(alias = "__ZNSt3mapIiN3RBX14RemoteFunction16RemoteInvocationESt4lessIiESaISt4pairIKiS2_EEEixERS6_")]
// 0x91db60 — __ZNSt3mapIiN3RBX14RemoteFunction16RemoteInvocationESt4lessIiESaISt4pairIKiS2_EEEixERS6_
// type: 
pub fn stub_91db60() -> ! {
    todo!("0x91db60 __ZNSt3mapIiN3RBX14RemoteFunction16RemoteInvocationESt4lessIiESaISt4pairIKiS2_EEEixERS6_")
}

#[doc(alias = "RBX::RemoteFunction::~RemoteFunction()")]
#[doc(alias = "__ZN3RBX14RemoteFunctionD1Ev")]
// 0x91e75c — __ZN3RBX14RemoteFunctionD1Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
pub fn stub_91e75c() -> ! {
    todo!("0x91e75c __ZN3RBX14RemoteFunctionD1Ev")
}

#[doc(alias = "RBX::RemoteFunction::~RemoteFunction()")]
#[doc(alias = "__ZN3RBX14RemoteFunctionD0Ev")]
// 0x91e760 — __ZN3RBX14RemoteFunctionD0Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
pub fn stub_91e760() -> ! {
    todo!("0x91e760 __ZN3RBX14RemoteFunctionD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteFunction::~RemoteFunction()")]
#[doc(alias = "__ZThn32_N3RBX14RemoteFunctionD1Ev")]
// 0x91e810 — __ZThn32_N3RBX14RemoteFunctionD1Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
pub fn stub_91e810() -> ! {
    todo!("0x91e810 __ZThn32_N3RBX14RemoteFunctionD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteFunction::~RemoteFunction()")]
#[doc(alias = "__ZThn32_N3RBX14RemoteFunctionD0Ev")]
// 0x91e818 — __ZThn32_N3RBX14RemoteFunctionD0Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
pub fn stub_91e818() -> ! {
    todo!("0x91e818 __ZThn32_N3RBX14RemoteFunctionD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteFunction::~RemoteFunction()")]
#[doc(alias = "__ZThn36_N3RBX14RemoteFunctionD1Ev")]
// 0x91e8cc — __ZThn36_N3RBX14RemoteFunctionD1Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
pub fn stub_91e8cc() -> ! {
    todo!("0x91e8cc __ZThn36_N3RBX14RemoteFunctionD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteFunction::~RemoteFunction()")]
#[doc(alias = "__ZThn36_N3RBX14RemoteFunctionD0Ev")]
// 0x91e8d4 — __ZThn36_N3RBX14RemoteFunctionD0Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
pub fn stub_91e8d4() -> ! {
    todo!("0x91e8d4 __ZThn36_N3RBX14RemoteFunctionD0Ev")
}

#[doc(alias = "RBX::RemoteEvent::~RemoteEvent()")]
#[doc(alias = "__ZN3RBX11RemoteEventD1Ev")]
// 0x91e978 — __ZN3RBX11RemoteEventD1Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
pub fn stub_91e978() -> ! {
    todo!("0x91e978 __ZN3RBX11RemoteEventD1Ev")
}

#[doc(alias = "RBX::RemoteEvent::~RemoteEvent()")]
#[doc(alias = "__ZN3RBX11RemoteEventD0Ev")]
// 0x91e97c — __ZN3RBX11RemoteEventD0Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
pub fn stub_91e97c() -> ! {
    todo!("0x91e97c __ZN3RBX11RemoteEventD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteEvent::~RemoteEvent()")]
#[doc(alias = "__ZThn32_N3RBX11RemoteEventD1Ev")]
// 0x91ea2c — __ZThn32_N3RBX11RemoteEventD1Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
pub fn stub_91ea2c() -> ! {
    todo!("0x91ea2c __ZThn32_N3RBX11RemoteEventD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteEvent::~RemoteEvent()")]
#[doc(alias = "__ZThn32_N3RBX11RemoteEventD0Ev")]
// 0x91ea34 — __ZThn32_N3RBX11RemoteEventD0Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
pub fn stub_91ea34() -> ! {
    todo!("0x91ea34 __ZThn32_N3RBX11RemoteEventD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteEvent::~RemoteEvent()")]
#[doc(alias = "__ZThn36_N3RBX11RemoteEventD1Ev")]
// 0x91eae8 — __ZThn36_N3RBX11RemoteEventD1Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
pub fn stub_91eae8() -> ! {
    todo!("0x91eae8 __ZThn36_N3RBX11RemoteEventD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteEvent::~RemoteEvent()")]
#[doc(alias = "__ZThn36_N3RBX11RemoteEventD0Ev")]
// 0x91eaf0 — __ZThn36_N3RBX11RemoteEventD0Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
pub fn stub_91eaf0() -> ! {
    todo!("0x91eaf0 __ZThn36_N3RBX11RemoteEventD0Ev")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// 0x926bc0 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_926bc0() -> ! {
    todo!("0x926bc0 __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
// 0x926c74 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: 
pub fn stub_926c74() -> ! {
    todo!("0x926c74 __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert_unique(std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_")]
// 0x926cc0 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_
// type: 
pub fn stub_926cc0() -> ! {
    todo!("0x926cc0 __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_create_node(std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE14_M_create_nodeERKS5_")]
// 0x926d28 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE14_M_create_nodeERKS5_
// type: 
pub fn stub_926d28() -> ! {
    todo!("0x926d28 __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::erase(std::_Rb_tree_iterator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E")]
// 0x926e44 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_926e44() -> ! {
    todo!("0x926e44 __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>::destroy(std::pair<int const,RBX::RemoteFunction::RemoteInvocation>*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorISt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEEE7destroyEPS6_")]
// 0x926e6c — __ZN9__gnu_cxx13new_allocatorISt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEEE7destroyEPS6_
// type: 
pub fn stub_926e6c() -> ! {
    todo!("0x926e6c __ZN9__gnu_cxx13new_allocatorISt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEEE7destroyEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,std::string)>::slot>::operator=(rbx::signals::signal<void ()(int,std::string)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviSsEE4slotEEaSEPS6_")]
// 0x9271a4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviSsEE4slotEEaSEPS6_
// type: int(void)
pub fn stub_9271a4() -> ! {
    todo!("0x9271a4 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviSsEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,std::string)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,std::string)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviSsEE4slotEEaSERKS7_")]
// 0x9271c8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviSsEE4slotEEaSERKS7_
// type: int(void)
pub fn stub_9271c8() -> ! {
    todo!("0x9271c8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviSsEE4slotEEaSERKS7_")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list2<int &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string> &,boost::_bi::list2<int &,std::string &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX14RemoteFunctionEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_iSsEENS0_5list2IRiRSsEEEEvNS0_4typeIvEERT_RT0_i")]
// 0x92754c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX14RemoteFunctionEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_iSsEENS0_5list2IRiRSsEEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
pub fn stub_92754c() -> ! {
    todo!("0x92754c __ZN5boost3_bi5list3INS0_5valueIPN3RBX14RemoteFunctionEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_iSsEENS0_5list2IRiRSsEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>::operator()(RBX::RemoteFunction*,int,std::string)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEclEPS3_iSs")]
// 0x927674 — __ZNK5boost4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEclEPS3_iSs
// type: 
pub fn stub_927674() -> ! {
    todo!("0x927674 __ZNK5boost4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEclEPS3_iSs")
}

#[doc(alias = "std::vector<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::function<void ()(void)>*,std::vector<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>>,boost::function<void ()(void)> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
// 0x929300 — __ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: 
pub fn stub_929300() -> ! {
    todo!("0x929300 __ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "std::_Vector_base<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN5boost8functionIFvvEEESaIS3_EE11_M_allocateEm")]
// 0x929658 — __ZNSt12_Vector_baseIN5boost8functionIFvvEEESaIS3_EE11_M_allocateEm
// type: 
pub fn stub_929658() -> ! {
    todo!("0x929658 __ZNSt12_Vector_baseIN5boost8functionIFvvEEESaIS3_EE11_M_allocateEm")
}

#[doc(alias = "boost::function<void ()(void)> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::function<void ()(void)> *,boost::function<void ()(void)> *>(boost::function<void ()(void)> *,boost::function<void ()(void)> *,boost::function<void ()(void)> *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost8functionIFvvEEES7_EET0_T_S9_S8_")]
// 0x929670 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost8functionIFvvEEES7_EET0_T_S9_S8_
// type: 
pub fn stub_929670() -> ! {
    todo!("0x929670 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost8functionIFvvEEES7_EET0_T_S9_S8_")
}

#[doc(alias = "std::vector<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EED2Ev")]
// 0x9296c0 — __ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EED2Ev
// type: 
pub fn stub_9296c0() -> ! {
    todo!("0x9296c0 __ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EED2Ev")
}

#[doc(alias = "boost::function2<void,int,std::string>::clear(void)")]
#[doc(alias = "__ZN5boost9function2IviSsE5clearEv")]
// 0x92eea0 — __ZN5boost9function2IviSsE5clearEv
// type: 
pub fn stub_92eea0() -> ! {
    todo!("0x92eea0 __ZN5boost9function2IviSsE5clearEv")
}

#[doc(alias = "boost::function2<void,int,std::string>::operator()(int,std::string)const")]
#[doc(alias = "__ZNK5boost9function2IviSsEclEiSs")]
// 0x92fb2c — __ZNK5boost9function2IviSsEclEiSs
// type: 
pub fn stub_92fb2c() -> ! {
    todo!("0x92fb2c __ZNK5boost9function2IviSsEclEiSs")
}

#[doc(alias = "boost::function2<void,int,std::string>::assign_to_own(boost::function2<void,int,std::string> const&)")]
#[doc(alias = "__ZN5boost9function2IviSsE13assign_to_ownERKS1_")]
// 0x92fec4 — __ZN5boost9function2IviSsE13assign_to_ownERKS1_
// type: 
pub fn stub_92fec4() -> ! {
    todo!("0x92fec4 __ZN5boost9function2IviSsE13assign_to_ownERKS1_")
}

#[doc(alias = "RBX::RemoteEvent::~RemoteEvent()")]
#[doc(alias = "__ZN3RBX11RemoteEventD2Ev")]
// 0x937ef8 — __ZN3RBX11RemoteEventD2Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
pub fn stub_937ef8() -> ! {
    todo!("0x937ef8 __ZN3RBX11RemoteEventD2Ev")
}

#[doc(alias = "RBX::RemoteFunction::~RemoteFunction()")]
#[doc(alias = "__ZN3RBX14RemoteFunctionD2Ev")]
// 0x93807c — __ZN3RBX14RemoteFunctionD2Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
pub fn stub_93807c() -> ! {
    todo!("0x93807c __ZN3RBX14RemoteFunctionD2Ev")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0x93863c — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
pub fn stub_93863c() -> ! {
    todo!("0x93863c __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "RBX::MemoryStats::freeMemoryBytes(void)")]
#[doc(alias = "__ZN3RBX11MemoryStats15freeMemoryBytesEv")]
// 0x938d58 — __ZN3RBX11MemoryStats15freeMemoryBytesEv
// type: _DWORD __fastcall(RBX::MemoryStats *__hidden this)
pub fn stub_938d58() -> ! {
    todo!("0x938d58 __ZN3RBX11MemoryStats15freeMemoryBytesEv")
}

#[doc(alias = "RBX::MemoryStats::slowGetMemoryPoolAvailability(void)")]
#[doc(alias = "__ZN3RBX11MemoryStats29slowGetMemoryPoolAvailabilityEv")]
// 0x938d88 — __ZN3RBX11MemoryStats29slowGetMemoryPoolAvailabilityEv
// type: _DWORD __fastcall(RBX::MemoryStats *__hidden this)
pub fn stub_938d88() -> ! {
    todo!("0x938d88 __ZN3RBX11MemoryStats29slowGetMemoryPoolAvailabilityEv")
}

#[doc(alias = "RBX::MemoryStats::releaseAllPoolMemory(void)")]
#[doc(alias = "__ZN3RBX11MemoryStats20releaseAllPoolMemoryEv")]
// 0x938db0 — __ZN3RBX11MemoryStats20releaseAllPoolMemoryEv
// type: _DWORD __fastcall(RBX::MemoryStats *__hidden this)
pub fn stub_938db0() -> ! {
    todo!("0x938db0 __ZN3RBX11MemoryStats20releaseAllPoolMemoryEv")
}

#[doc(alias = "RBX::MemoryStats::slowCheckMemoryLevel(unsigned int)")]
#[doc(alias = "__ZN3RBX11MemoryStats20slowCheckMemoryLevelEj")]
// 0x938dd8 — __ZN3RBX11MemoryStats20slowCheckMemoryLevelEj
// type: _DWORD __fastcall(RBX::MemoryStats *__hidden this, unsigned int)
pub fn stub_938dd8() -> ! {
    todo!("0x938dd8 __ZN3RBX11MemoryStats20slowCheckMemoryLevelEj")
}

#[doc(alias = "FLog::FastLogS(unsigned char,char const*,std::string const&)")]
#[doc(alias = "__ZN4FLog8FastLogSEhPKcRKSs")]
// 0x9392fc — __ZN4FLog8FastLogSEhPKcRKSs
// type: _DWORD __fastcall(FLog *__hidden this, unsigned __int8, const char *, const std::string *)
pub fn stub_9392fc() -> ! {
    todo!("0x9392fc __ZN4FLog8FastLogSEhPKcRKSs")
}

#[doc(alias = "FLog::ForEachVariable(void (*)(std::string const&,std::string const&,void *),void *,FastVarType)")]
#[doc(alias = "__ZN4FLog15ForEachVariableEPFvRKSsS1_PvES2_11FastVarType")]
// 0x9396b4 — __ZN4FLog15ForEachVariableEPFvRKSsS1_PvES2_11FastVarType
// type: int __fastcall(int, int, int, int, int, int, int, int, char, char, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_9396b4() -> ! {
    todo!("0x9396b4 __ZN4FLog15ForEachVariableEPFvRKSsS1_PvES2_11FastVarType")
}

#[doc(alias = "FLog::visitVariable(std::pair<std::string const,FLog::IValueGetSet *>,void (*)(std::string const&,std::string const&,void *),void *)")]
#[doc(alias = "__ZN4FLogL13visitVariableESt4pairIKSsPNS_12IValueGetSetEEPFvRS1_S5_PvES6_")]
// 0x939980 — __ZN4FLogL13visitVariableESt4pairIKSsPNS_12IValueGetSetEEPFvRS1_S5_PvES6_
// type: 
pub fn stub_939980() -> ! {
    todo!("0x939980 __ZN4FLogL13visitVariableESt4pairIKSsPNS_12IValueGetSetEEPFvRS1_S5_PvES6_")
}

#[doc(alias = "FLog::SetValue(std::string const&,std::string const&,FastVarType,bool)")]
#[doc(alias = "__ZN4FLog8SetValueERKSsS1_11FastVarTypeb")]
// 0x939b38 — __ZN4FLog8SetValueERKSsS1_11FastVarTypeb
// type: 
pub fn stub_939b38() -> ! {
    todo!("0x939b38 __ZN4FLog8SetValueERKSsS1_11FastVarTypeb")
}

#[doc(alias = "FLog::GetValue(std::string const&,std::string &)")]
#[doc(alias = "__ZN4FLog8GetValueERKSsRSs")]
// 0x939c78 — __ZN4FLog8GetValueERKSsRSs
// type: _DWORD __fastcall(FLog *__hidden this, const std::string *, std::string *)
pub fn stub_939c78() -> ! {
    todo!("0x939c78 __ZN4FLog8GetValueERKSsRSs")
}

#[doc(alias = "FLog::SetValueFromServer(std::string const&,std::string const&)")]
#[doc(alias = "__ZN4FLog18SetValueFromServerERKSsS1_")]
// 0x939dec — __ZN4FLog18SetValueFromServerERKSsS1_
// type: _DWORD __fastcall(FLog *__hidden this, const std::string *, const std::string *)
pub fn stub_939dec() -> ! {
    todo!("0x939dec __ZN4FLog18SetValueFromServerERKSsS1_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEEixERS5_")]
// 0x93ac68 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEEixERS5_
// type: 
pub fn stub_93ac68() -> ! {
    todo!("0x93ac68 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEEixERS5_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")]
// 0x93af28 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: 
pub fn stub_93af28() -> ! {
    todo!("0x93af28 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")]
// 0x93b0d0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// type: 
pub fn stub_93b0d0() -> ! {
    todo!("0x93b0d0 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>>>::construct(void)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSsEEEEE9constructEv")]
// 0x93b180 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSsEEEEE9constructEv
// type: int(void)
pub fn stub_93b180() -> ! {
    todo!("0x93b180 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSsEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,FLog::IValueGetSet *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,FLog::IValueGetSet *>>,std::string,FLog::IValueGetSet *,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_")]
// 0x93b240 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_
// type: 
pub fn stub_93b240() -> ! {
    todo!("0x93b240 __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>> *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_")]
// 0x93b2dc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_
// type: 
pub fn stub_93b2dc() -> ! {
    todo!("0x93b2dc __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,FLog::IValueGetSet *>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,FLog::IValueGetSet *>>,std::string,FLog::IValueGetSet *,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<std::pair<std::string const,FLog::IValueGetSet *>>(std::string const&,std::pair<std::string const,FLog::IValueGetSet *> &&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE12emplace_implIJS9_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEbERS5_DpOT_")]
// 0x93b3f8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE12emplace_implIJS9_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEbERS5_DpOT_
// type: int __fastcall(int, int, int, int, char, char, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_93b3f8() -> ! {
    todo!("0x93b3f8 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE12emplace_implIJS9_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEbERS5_DpOT_")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,FLog::IValueGetSet *>>>>::construct_with_value<std::pair<std::string const,FLog::IValueGetSet *>>(std::pair<std::string const,FLog::IValueGetSet *> &&)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN4FLog12IValueGetSetEEEEEE20construct_with_valueIJS9_EEEvDpOT_")]
// 0x93b5d4 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN4FLog12IValueGetSetEEEEEE20construct_with_valueIJS9_EEEvDpOT_
// type: int __fastcall(int, std::string *)
pub fn stub_93b5d4() -> ! {
    todo!("0x93b5d4 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN4FLog12IValueGetSetEEEEEE20construct_with_valueIJS9_EEEvDpOT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,FLog::IValueGetSet *>>,std::string,FLog::IValueGetSet *,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")]
// 0x93b670 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: 
pub fn stub_93b670() -> ! {
    todo!("0x93b670 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,FLog::IValueGetSet *>>,std::string,FLog::IValueGetSet *,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")]
// 0x93b818 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// type: 
pub fn stub_93b818() -> ! {
    todo!("0x93b818 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN4FLog12IValueGetSetEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}
