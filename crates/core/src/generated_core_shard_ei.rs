//! core shard EI — 100 core stubs EA-sorted, lowest uncovered 0x91199c..0x926bc0 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EH 0x911000).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "RBX::StringConverter<RBX::AssetService::AccessType>::convertToValue(std::string const&,RBX::AssetService::AccessType&)")]
// 0x91199c — __ZN3RBX15StringConverterINS_12AssetService10AccessTypeEE14convertToValueERKSsRS2_
pub fn stub_91199c() -> ! {
    todo!("0x91199c __ZN3RBX15StringConverterINS_12AssetService10AccessTypeEE14convertToValueERKSsRS2_")
}

#[doc(alias = "RBX::AssetService::AssetService(void)")]
// 0x9119e8 — __ZN3RBX12AssetServiceC1Ev
pub fn stub_9119e8() -> ! {
    todo!("0x9119e8 __ZN3RBX12AssetServiceC1Ev")
}

#[doc(alias = "RBX::AssetService::AssetService(void)")]
// 0x9119ec — __ZN3RBX12AssetServiceC2Ev
pub fn stub_9119ec() -> ! {
    todo!("0x9119ec __ZN3RBX12AssetServiceC2Ev")
}

#[doc(alias = "RBX::enumToString(RBX::AssetService::AccessType)")]
// 0x911c70 — __ZN3RBX12enumToStringENS_12AssetService10AccessTypeE
pub fn stub_911c70() -> ! {
    todo!("0x911c70 __ZN3RBX12enumToStringENS_12AssetService10AccessTypeE")
}

#[doc(alias = "RBX::AssetService::httpPostHelper(std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x91210c — __ZN3RBX12AssetService14httpPostHelperEPSsPSt9exceptionN5boost8functionIFvbEEENS5_IFvSsEEE
pub fn stub_91210c() -> ! {
    todo!("0x91210c __ZN3RBX12AssetService14httpPostHelperEPSsPSt9exceptionN5boost8functionIFvbEEENS5_IFvSsEEE")
}

#[doc(alias = "RBX::AssetService::getCreatorAssetIDHelper(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// 0x912280 — __ZN3RBX12AssetService23getCreatorAssetIDHelperEPSsPSt9exceptionN5boost8functionIFviEEENS5_IFvSsEEE
pub fn stub_912280() -> ! {
    todo!("0x912280 __ZN3RBX12AssetService23getCreatorAssetIDHelperEPSsPSt9exceptionN5boost8functionIFviEEENS5_IFvSsEEE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_5<RBX::AssetService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,RBX::AssetService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (RBX::AssetService::*)(std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),RBX::AssetService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x9132f8 — __ZN5boost4bindIvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENS6_IFvSsEEEPS2_NS_3argILi1EEENSC_ILi2EEES8_SA_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISH_T0_T1_T2_T3_T4_EENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSK_FSH_SL_SM_SN_SO_ESR_SS_ST_SU_SV_
pub fn stub_9132f8() -> ! {
    todo!("0x9132f8 __ZN5boost4bindIvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENS6_IFvSsEEEPS2_NS_3argILi1EEENSC_ILi2EEES8_SA_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISH_T0_T1_T2_T3_T4_EENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSK_FSH_SL_SM_SN_SO_ESR_SS_ST_SU_SV_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_5<RBX::AssetService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(int)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>,RBX::AssetService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(int)>,boost::function<void ()(std::string)>>(void (RBX::AssetService::*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),RBX::AssetService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// 0x9134c0 — __ZN5boost4bindIvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENS6_IFvSsEEEPS2_NS_3argILi1EEENSC_ILi2EEES8_SA_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISH_T0_T1_T2_T3_T4_EENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSK_FSH_SL_SM_SN_SO_ESR_SS_ST_SU_SV_
pub fn stub_9134c0() -> ! {
    todo!("0x9134c0 __ZN5boost4bindIvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENS6_IFvSsEEEPS2_NS_3argILi1EEENSC_ILi2EEES8_SA_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISH_T0_T1_T2_T3_T4_EENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSK_FSH_SL_SM_SN_SO_ESR_SS_ST_SU_SV_")
}

#[doc(alias = "RBX::AssetService::~AssetService()")]
// 0x913688 — __ZN3RBX12AssetServiceD1Ev
pub fn stub_913688() -> ! {
    todo!("0x913688 __ZN3RBX12AssetServiceD1Ev")
}

#[doc(alias = "RBX::AssetService::~AssetService()")]
// 0x9136d4 — __ZN3RBX12AssetServiceD0Ev
pub fn stub_9136d4() -> ! {
    todo!("0x9136d4 __ZN3RBX12AssetServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AssetService::~AssetService()")]
// 0x9137e0 — __ZThn32_N3RBX12AssetServiceD1Ev
// was: `non-virtual thunk to'RBX::AssetService::~AssetService()
pub fn stub_9137e0() -> ! {
    todo!("0x9137e0 __ZThn32_N3RBX12AssetServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AssetService::~AssetService()")]
// 0x913834 — __ZThn32_N3RBX12AssetServiceD0Ev
// was: `non-virtual thunk to'RBX::AssetService::~AssetService()
pub fn stub_913834() -> ! {
    todo!("0x913834 __ZThn32_N3RBX12AssetServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AssetService::~AssetService()")]
// 0x913940 — __ZThn36_N3RBX12AssetServiceD1Ev
// was: `non-virtual thunk to'RBX::AssetService::~AssetService()
pub fn stub_913940() -> ! {
    todo!("0x913940 __ZThn36_N3RBX12AssetServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AssetService::~AssetService()")]
// 0x913994 — __ZThn36_N3RBX12AssetServiceD0Ev
// was: `non-virtual thunk to'RBX::AssetService::~AssetService()
pub fn stub_913994() -> ! {
    todo!("0x913994 __ZThn36_N3RBX12AssetServiceD0Ev")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0x913d30 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS_8functionIFviEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_
pub fn stub_913d30() -> ! {
    todo!("0x913d30 __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS_8functionIFviEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x913ea0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
pub fn stub_913ea0() -> ! {
    todo!("0x913ea0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// 0x913ebc — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_
pub fn stub_913ebc() -> ! {
    todo!("0x913ebc __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0x913ee0 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFviEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_913ee0() -> ! {
    todo!("0x913ee0 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFviEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x914040 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFviEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_914040() -> ! {
    todo!("0x914040 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFviEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x91419c — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFviEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_91419c() -> ! {
    todo!("0x91419c __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFviEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>> &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0x9142a8 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFviEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_9142a8() -> ! {
    todo!("0x9142a8 __ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFviEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>::operator()(RBX::AssetService*,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>)const")]
// 0x9143b8 — __ZNK5boost4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_
pub fn stub_9143b8() -> ! {
    todo!("0x9143b8 __ZNK5boost4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x9144d8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_9144d8() -> ! {
    todo!("0x9144d8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x914690 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFviEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
pub fn stub_914690() -> ! {
    todo!("0x914690 __ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFviEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x91478c — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFviEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
pub fn stub_91478c() -> ! {
    todo!("0x91478c __ZN5boost3_bi8storage5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFviEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")
}

#[doc(alias = "void boost::throw_exception<boost::bad_lexical_cast>(boost::bad_lexical_cast const&)")]
// 0x91488c — __ZN5boost15throw_exceptionINS_16bad_lexical_castEEEvRKT_
pub fn stub_91488c() -> ! {
    todo!("0x91488c __ZN5boost15throw_exceptionINS_16bad_lexical_castEEEvRKT_")
}

#[doc(alias = "boost::bad_lexical_cast::~bad_lexical_cast()")]
// 0x91497c — __ZN5boost16bad_lexical_castD1Ev
pub fn stub_91497c() -> ! {
    todo!("0x91497c __ZN5boost16bad_lexical_castD1Ev")
}

#[doc(alias = "boost::bad_lexical_cast::~bad_lexical_cast()")]
// 0x914980 — __ZN5boost16bad_lexical_castD0Ev
pub fn stub_914980() -> ! {
    todo!("0x914980 __ZN5boost16bad_lexical_castD0Ev")
}

#[doc(alias = "boost::bad_lexical_cast::what(void)const")]
// 0x914994 — __ZNK5boost16bad_lexical_cast4whatEv
pub fn stub_914994() -> ! {
    todo!("0x914994 __ZNK5boost16bad_lexical_cast4whatEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::~clone_impl()")]
// 0x9149a0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED1Ev
pub fn stub_9149a0() -> ! {
    todo!("0x9149a0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_lexical_cast>::~error_info_injector()")]
// 0x9149b0 — __ZN5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED1Ev
pub fn stub_9149b0() -> ! {
    todo!("0x9149b0 __ZN5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_lexical_cast>::~error_info_injector()")]
// 0x9149b4 — __ZN5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED2Ev
pub fn stub_9149b4() -> ! {
    todo!("0x9149b4 __ZN5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED2Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::bad_lexical_cast>::~error_info_injector()")]
// 0x914a6c — __ZThn12_N5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::bad_lexical_cast>::~error_info_injector()
pub fn stub_914a6c() -> ! {
    todo!("0x914a6c __ZThn12_N5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED1Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::~clone_impl()")]
// 0x914a74 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::~clone_impl()
pub fn stub_914a74() -> ! {
    todo!("0x914a74 __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED1Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::~clone_impl()")]
// 0x914a7c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED1Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::~clone_impl()
pub fn stub_914a7c() -> ! {
    todo!("0x914a7c __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::~clone_impl()")]
// 0x914a88 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED0Ev
pub fn stub_914a88() -> ! {
    todo!("0x914a88 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone(void)const")]
// 0x914a9c — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE5cloneEv
pub fn stub_914a9c() -> ! {
    todo!("0x914a9c __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::rethrow(void)const")]
// 0x914b58 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE7rethrowEv
pub fn stub_914b58() -> ! {
    todo!("0x914b58 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE7rethrowEv")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::~clone_impl()")]
// 0x914c88 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED0Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::~clone_impl()
pub fn stub_914c88() -> ! {
    todo!("0x914c88 __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED0Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone(void)const")]
// 0x914ca0 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE5cloneEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone(void)const
pub fn stub_914ca0() -> ! {
    todo!("0x914ca0 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE5cloneEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::rethrow(void)const")]
// 0x914cac — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE7rethrowEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::rethrow(void)const
pub fn stub_914cac() -> ! {
    todo!("0x914cac __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE7rethrowEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::~clone_impl()")]
// 0x914cbc — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED0Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::~clone_impl()
pub fn stub_914cbc() -> ! {
    todo!("0x914cbc __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_lexical_cast>::~error_info_injector()")]
// 0x914cd8 — __ZN5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED0Ev
pub fn stub_914cd8() -> ! {
    todo!("0x914cd8 __ZN5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED0Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::bad_lexical_cast>::~error_info_injector()")]
// 0x914cec — __ZThn12_N5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED0Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::bad_lexical_cast>::~error_info_injector()
pub fn stub_914cec() -> ! {
    todo!("0x914cec __ZThn12_N5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone_tag)")]
// 0x914d04 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEEC1ERKS5_NS5_9clone_tagE
pub fn stub_914d04() -> ! {
    todo!("0x914d04 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEEC1ERKS5_NS5_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_lexical_cast>>::clone_impl(boost::exception_detail::error_info_injector<boost::bad_lexical_cast> const&)")]
// 0x914e40 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEEC1ERKS4_
pub fn stub_914e40() -> ! {
    todo!("0x914e40 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEEC1ERKS4_")
}

#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<int>(int &)")]
// 0x914f7c — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIiEEbRT_
pub fn stub_914f7c() -> ! {
    todo!("0x914f7c __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIiEEbRT_")
}

#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned int,char>(unsigned int &,char const*,char const*)")]
// 0x914fdc — __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEjcEEbRT0_PKT1_S8_
pub fn stub_914fdc() -> ! {
    todo!("0x914fdc __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEjcEEbRT0_PKT1_S8_")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0x91562c — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_
pub fn stub_91562c() -> ! {
    todo!("0x91562c __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x91579c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
pub fn stub_91579c() -> ! {
    todo!("0x91579c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// 0x9157b8 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_
pub fn stub_9157b8() -> ! {
    todo!("0x9157b8 __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0x9157dc — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_9157dc() -> ! {
    todo!("0x9157dc __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x91593c — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_91593c() -> ! {
    todo!("0x91593c __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x915a98 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_915a98() -> ! {
    todo!("0x915a98 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>> &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0x915ba4 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_915ba4() -> ! {
    todo!("0x915ba4 __ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::operator()(RBX::AssetService*,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)const")]
// 0x915cb4 — __ZNK5boost4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_
pub fn stub_915cb4() -> ! {
    todo!("0x915cb4 __ZNK5boost4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AssetService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::AssetService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x915dd4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_915dd4() -> ! {
    todo!("0x915dd4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x915f8c — __ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
pub fn stub_915f8c() -> ! {
    todo!("0x915f8c __ZN5boost3_bi5list5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<RBX::AssetService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x916088 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
pub fn stub_916088() -> ! {
    todo!("0x916088 __ZN5boost3_bi8storage5INS0_5valueIPN3RBX12AssetServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")
}

#[doc(alias = "RBX::AssetService::AccessType * rbx::any_cast<RBX::AssetService::AccessType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x917198 — __ZN3rbx8any_castIN3RBX12AssetService10AccessTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_917198() -> ! {
    todo!("0x917198 __ZN3rbx8any_castIN3RBX12AssetService10AccessTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::AssetService::AccessType & rbx::any_cast<RBX::AssetService::AccessType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x9171f0 — __ZN3rbx8any_castIRN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_9171f0() -> ! {
    todo!("0x9171f0 __ZN3rbx8any_castIRN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::resize(unsigned long,RBX::AssetService::AccessType)")]
// 0x9172e0 — __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE6resizeEmS2_
pub fn stub_9172e0() -> ! {
    todo!("0x9172e0 __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::push_back(RBX::AssetService::AccessType const&)")]
// 0x917314 — __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE9push_backERKS2_
pub fn stub_917314() -> ! {
    todo!("0x917314 __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::AssetService::AccessType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::operator[](RBX::Name const* const&)")]
// 0x91733c — __ZNSt3mapIPKN3RBX4NameENS0_12AssetService10AccessTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_91733c() -> ! {
    todo!("0x91733c __ZNSt3mapIPKN3RBX4NameENS0_12AssetService10AccessTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::pair<RBX::Name const* const,RBX::AssetService::AccessType> const&)")]
// 0x917394 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_917394() -> ! {
    todo!("0x917394 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::AssetService::AccessType> const&)")]
// 0x917448 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_917448() -> ! {
    todo!("0x917448 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::AssetService::AccessType> const&)")]
// 0x9174a0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_9174a0() -> ! {
    todo!("0x9174a0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AssetService::AccessType*,std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>>,RBX::AssetService::AccessType const&)")]
// 0x917508 — __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_917508() -> ! {
    todo!("0x917508 __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::_M_allocate(unsigned long)")]
// 0x9175ec — __ZNSt12_Vector_baseIN3RBX12AssetService10AccessTypeESaIS2_EE11_M_allocateEm
pub fn stub_9175ec() -> ! {
    todo!("0x9175ec __ZNSt12_Vector_baseIN3RBX12AssetService10AccessTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::AssetService::AccessType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AssetService::AccessType *,RBX::AssetService::AccessType *>(RBX::AssetService::AccessType *,RBX::AssetService::AccessType *,RBX::AssetService::AccessType *)")]
// 0x917604 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12AssetService10AccessTypeES6_EET0_T_S8_S7_
pub fn stub_917604() -> ! {
    todo!("0x917604 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12AssetService10AccessTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::AssetService::AccessType*,std::vector<RBX::AssetService::AccessType,std::allocator<RBX::AssetService::AccessType>>>,unsigned long,RBX::AssetService::AccessType const&)")]
// 0x917640 — __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_917640() -> ! {
    todo!("0x917640 __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::utf8_encode(std::string const&)")]
// 0x919e14 — __ZN3RBX11utf8_encodeERKSs
pub fn stub_919e14() -> ! {
    todo!("0x919e14 __ZN3RBX11utf8_encodeERKSs")
}

#[doc(alias = "RBX::utf8_decode(std::string const&)")]
// 0x919e20 — __ZN3RBX11utf8_decodeERKSs
pub fn stub_919e20() -> ! {
    todo!("0x919e20 __ZN3RBX11utf8_decodeERKSs")
}

#[doc(alias = "RBX::RemoteFunction::processDelayedInvocations(void)")]
// 0x91a6cc — __ZN3RBX14RemoteFunction25processDelayedInvocationsEv
pub fn stub_91a6cc() -> ! {
    todo!("0x91a6cc __ZN3RBX14RemoteFunction25processDelayedInvocationsEv")
}

#[doc(alias = "RBX::DelayedInvocationQueue::push(boost::function<void ()(void)> const&)")]
// 0x91aff4 — __ZN3RBX22DelayedInvocationQueue4pushERKN5boost8functionIFvvEEE
pub fn stub_91aff4() -> ! {
    todo!("0x91aff4 __ZN3RBX22DelayedInvocationQueue4pushERKN5boost8functionIFvvEEE")
}

#[doc(alias = "RBX::DelayedInvocationQueue::process(void)")]
// 0x91b014 — __ZN3RBX22DelayedInvocationQueue7processEv
pub fn stub_91b014() -> ! {
    todo!("0x91b014 __ZN3RBX22DelayedInvocationQueue7processEv")
}

#[doc(alias = "RBX::RemoteFunction::RemoteFunction(void)")]
// 0x91b1a4 — __ZN3RBX14RemoteFunctionC2Ev
pub fn stub_91b1a4() -> ! {
    todo!("0x91b1a4 __ZN3RBX14RemoteFunctionC2Ev")
}

#[doc(alias = "RBX::RemoteFunction::localError(int,std::string)")]
// 0x91b654 — __ZN3RBX14RemoteFunction10localErrorEiSs
pub fn stub_91b654() -> ! {
    todo!("0x91b654 __ZN3RBX14RemoteFunction10localErrorEiSs")
}

#[doc(alias = "RBX::RemoteFunction::consumeRemoteInvocation(int,RBX::RemoteFunction::RemoteInvocation &)")]
// 0x91c640 — __ZN3RBX14RemoteFunction23consumeRemoteInvocationEiRNS0_16RemoteInvocationE
pub fn stub_91c640() -> ! {
    todo!("0x91c640 __ZN3RBX14RemoteFunction23consumeRemoteInvocationEiRNS0_16RemoteInvocationE")
}

#[doc(alias = "RBX::RemoteFunction::remoteError(RBX::SystemAddress,int,std::string)")]
// 0x91cd94 — __ZN3RBX14RemoteFunction11remoteErrorENS_13SystemAddressEiSs
pub fn stub_91cd94() -> ! {
    todo!("0x91cd94 __ZN3RBX14RemoteFunction11remoteErrorENS_13SystemAddressEiSs")
}

#[doc(alias = "RBX::RemoteEvent::RemoteEvent(void)")]
// 0x91cf14 — __ZN3RBX11RemoteEventC2Ev
pub fn stub_91cf14() -> ! {
    todo!("0x91cf14 __ZN3RBX11RemoteEventC2Ev")
}

#[doc(alias = "std::vector<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::push_back(boost::function<void ()(void)> const&)")]
// 0x91db1c — __ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE9push_backERKS3_
pub fn stub_91db1c() -> ! {
    todo!("0x91db1c __ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE9push_backERKS3_")
}

#[doc(alias = "std::map<int,RBX::RemoteFunction::RemoteInvocation,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::operator[](int const&)")]
// 0x91db60 — __ZNSt3mapIiN3RBX14RemoteFunction16RemoteInvocationESt4lessIiESaISt4pairIKiS2_EEEixERS6_
pub fn stub_91db60() -> ! {
    todo!("0x91db60 __ZNSt3mapIiN3RBX14RemoteFunction16RemoteInvocationESt4lessIiESaISt4pairIKiS2_EEEixERS6_")
}

#[doc(alias = "RBX::RemoteFunction::~RemoteFunction()")]
// 0x91e75c — __ZN3RBX14RemoteFunctionD1Ev
pub fn stub_91e75c() -> ! {
    todo!("0x91e75c __ZN3RBX14RemoteFunctionD1Ev")
}

#[doc(alias = "RBX::RemoteFunction::~RemoteFunction()")]
// 0x91e760 — __ZN3RBX14RemoteFunctionD0Ev
pub fn stub_91e760() -> ! {
    todo!("0x91e760 __ZN3RBX14RemoteFunctionD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteFunction::~RemoteFunction()")]
// 0x91e810 — __ZThn32_N3RBX14RemoteFunctionD1Ev
// was: `non-virtual thunk to'RBX::RemoteFunction::~RemoteFunction()
pub fn stub_91e810() -> ! {
    todo!("0x91e810 __ZThn32_N3RBX14RemoteFunctionD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteFunction::~RemoteFunction()")]
// 0x91e818 — __ZThn32_N3RBX14RemoteFunctionD0Ev
// was: `non-virtual thunk to'RBX::RemoteFunction::~RemoteFunction()
pub fn stub_91e818() -> ! {
    todo!("0x91e818 __ZThn32_N3RBX14RemoteFunctionD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteFunction::~RemoteFunction()")]
// 0x91e8cc — __ZThn36_N3RBX14RemoteFunctionD1Ev
// was: `non-virtual thunk to'RBX::RemoteFunction::~RemoteFunction()
pub fn stub_91e8cc() -> ! {
    todo!("0x91e8cc __ZThn36_N3RBX14RemoteFunctionD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteFunction::~RemoteFunction()")]
// 0x91e8d4 — __ZThn36_N3RBX14RemoteFunctionD0Ev
// was: `non-virtual thunk to'RBX::RemoteFunction::~RemoteFunction()
pub fn stub_91e8d4() -> ! {
    todo!("0x91e8d4 __ZThn36_N3RBX14RemoteFunctionD0Ev")
}

#[doc(alias = "RBX::RemoteEvent::~RemoteEvent()")]
// 0x91e978 — __ZN3RBX11RemoteEventD1Ev
pub fn stub_91e978() -> ! {
    todo!("0x91e978 __ZN3RBX11RemoteEventD1Ev")
}

#[doc(alias = "RBX::RemoteEvent::~RemoteEvent()")]
// 0x91e97c — __ZN3RBX11RemoteEventD0Ev
pub fn stub_91e97c() -> ! {
    todo!("0x91e97c __ZN3RBX11RemoteEventD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteEvent::~RemoteEvent()")]
// 0x91ea2c — __ZThn32_N3RBX11RemoteEventD1Ev
// was: `non-virtual thunk to'RBX::RemoteEvent::~RemoteEvent()
pub fn stub_91ea2c() -> ! {
    todo!("0x91ea2c __ZThn32_N3RBX11RemoteEventD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteEvent::~RemoteEvent()")]
// 0x91ea34 — __ZThn32_N3RBX11RemoteEventD0Ev
// was: `non-virtual thunk to'RBX::RemoteEvent::~RemoteEvent()
pub fn stub_91ea34() -> ! {
    todo!("0x91ea34 __ZThn32_N3RBX11RemoteEventD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteEvent::~RemoteEvent()")]
// 0x91eae8 — __ZThn36_N3RBX11RemoteEventD1Ev
// was: `non-virtual thunk to'RBX::RemoteEvent::~RemoteEvent()
pub fn stub_91eae8() -> ! {
    todo!("0x91eae8 __ZThn36_N3RBX11RemoteEventD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RemoteEvent::~RemoteEvent()")]
// 0x91eaf0 — __ZThn36_N3RBX11RemoteEventD0Ev
// was: `non-virtual thunk to'RBX::RemoteEvent::~RemoteEvent()
pub fn stub_91eaf0() -> ! {
    todo!("0x91eaf0 __ZThn36_N3RBX11RemoteEventD0Ev")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<RBX::SystemAddress>,boost::_bi::value<int>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x92663c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS7_13SystemAddressEiSsEENS3_5list4INS3_5valueIPS8_EENSC_IS9_EENSC_IiEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
pub fn stub_92663c() -> ! {
    todo!("0x92663c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS7_13SystemAddressEiSsEENS3_5list4INS3_5valueIPS8_EENSC_IS9_EENSC_IiEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<RBX::SystemAddress>,boost::_bi::value<int>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")]
// 0x9266bc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS7_13SystemAddressEiSsEENS3_5list4INS3_5valueIPS8_EENSC_IS9_EENSC_IiEENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs
pub fn stub_9266bc() -> ! {
    todo!("0x9266bc __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS7_13SystemAddressEiSsEENS3_5list4INS3_5valueIPS8_EENSC_IS9_EENSC_IiEENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<RBX::SystemAddress>,boost::_bi::value<int>,boost::arg<1>>::operator()<boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string> &,boost::_bi::list1<std::string &> &,int)")]
// 0x9266d8 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS3_13SystemAddressEEENS2_IiEENS_3argILi1EEEEclINS_4_mfi3mf3IvS4_S7_iSsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_9266d8() -> ! {
    todo!("0x9266d8 __ZN5boost3_bi5list4INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS3_13SystemAddressEEENS2_IiEENS_3argILi1EEEEclINS_4_mfi3mf3IvS4_S7_iSsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string>::operator()(RBX::RemoteFunction*,RBX::SystemAddress,int,std::string)const")]
// 0x926804 — __ZNK5boost4_mfi3mf3IvN3RBX14RemoteFunctionENS2_13SystemAddressEiSsEclEPS3_S4_iSs
pub fn stub_926804() -> ! {
    todo!("0x926804 __ZNK5boost4_mfi3mf3IvN3RBX14RemoteFunctionENS2_13SystemAddressEiSsEclEPS3_S4_iSs")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
// 0x926bc0 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_926bc0() -> ! {
    todo!("0x926bc0 __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}