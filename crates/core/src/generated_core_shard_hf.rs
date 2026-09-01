//! core shard HF — 100 core stubs EA-sorted, 0xf5a744..0xf5b714 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HE 0xf5a714).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HE 0xf5a714 (0xf5a744..0xf5b714, 20914->21014 covered, 904 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(int,int,bool)>::operator()(int,int,bool)")]
// 0xf5a744 — j___ZN3rbx7signals16signal_with_argsILi3EFviibEEclEiib
pub fn stub_0xf5a744() -> ! {
    todo!("0xf5a744 j___ZN3rbx7signals16signal_with_argsILi3EFviibEEclEiib")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::disconnectAll(void)")]
// 0xf5a8e4 — j___ZN3rbx7signals6signalIFvSsiiEE13disconnectAllEv
pub fn stub_0xf5a8e4() -> ! {
    todo!("0xf5a8e4 j___ZN3rbx7signals6signalIFvSsiiEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::insert(rbx::signals::signal<void ()(std::string,int,int)>::slot *)")]
// 0xf5a8f4 — j___ZN3rbx7signals6signalIFvSsiiEE6insertEPNS3_4slotE
pub fn stub_0xf5a8f4() -> ! {
    todo!("0xf5a8f4 j___ZN3rbx7signals6signalIFvSsiiEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::remove(rbx::signals::signal<void ()(std::string,int,int)>::slot *)")]
// 0xf5a904 — j___ZN3rbx7signals6signalIFvSsiiEE6removeEPNS3_4slotE
pub fn stub_0xf5a904() -> ! {
    todo!("0xf5a904 j___ZN3rbx7signals6signalIFvSsiiEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::disconnectAll(void)")]
// 0xf5a914 — j___ZN3rbx7signals6signalIFviibEE13disconnectAllEv
pub fn stub_0xf5a914() -> ! {
    todo!("0xf5a914 j___ZN3rbx7signals6signalIFviibEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::safe_static_do_get_mutex(void)")]
// 0xf5a924 — j___ZN3rbx7signals6signalIFviibEE24safe_static_do_get_mutexEv
pub fn stub_0xf5a924() -> ! {
    todo!("0xf5a924 j___ZN3rbx7signals6signalIFviibEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,bool)>::slot> &)")]
// 0xf5a934 — j___ZN3rbx7signals6signalIFviibEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_0xf5a934() -> ! {
    todo!("0xf5a934 j___ZN3rbx7signals6signalIFviibEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::slot::safe_static_do_get_mutex(void)")]
// 0xf5a944 — j___ZN3rbx7signals6signalIFviibEE4slot24safe_static_do_get_mutexEv
pub fn stub_0xf5a944() -> ! {
    todo!("0xf5a944 j___ZN3rbx7signals6signalIFviibEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::insert(rbx::signals::signal<void ()(int,int,bool)>::slot *)")]
// 0xf5a954 — j___ZN3rbx7signals6signalIFviibEE6insertEPNS3_4slotE
pub fn stub_0xf5a954() -> ! {
    todo!("0xf5a954 j___ZN3rbx7signals6signalIFviibEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::remove(rbx::signals::signal<void ()(int,int,bool)>::slot *)")]
// 0xf5a964 — j___ZN3rbx7signals6signalIFviibEE6removeEPNS3_4slotE
pub fn stub_0xf5a964() -> ! {
    todo!("0xf5a964 j___ZN3rbx7signals6signalIFviibEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int,bool)>::connect<boost::function<void ()(int,int,bool)>>(boost::function<void ()(int,int,bool)> const&)")]
// 0xf5a974 — j___ZN3rbx7signals6signalIFviibEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_0xf5a974() -> ! {
    todo!("0xf5a974 j___ZN3rbx7signals6signalIFviibEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,bool)>::on_error(std::exception &)")]
// 0xf5a984 — j___ZN3rbx7signals6signalIFviibEE8on_errorERSt9exception
pub fn stub_0xf5a984() -> ! {
    todo!("0xf5a984 j___ZN3rbx7signals6signalIFviibEE8on_errorERSt9exception")
}

#[doc(alias = "RBX::MarketplaceService::CurrencyType * rbx::any_cast<RBX::MarketplaceService::CurrencyType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf5a994 — j___ZN3rbx8any_castIN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0xf5a994() -> ! {
    todo!("0xf5a994 j___ZN3rbx8any_castIN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::MarketplaceService::CurrencyType const& rbx::any_cast<RBX::MarketplaceService::CurrencyType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf5a9a4 — j___ZN3rbx8any_castIRKN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0xf5a9a4() -> ! {
    todo!("0xf5a9a4 j___ZN3rbx8any_castIRKN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::MarketplaceService::CurrencyType & rbx::any_cast<RBX::MarketplaceService::CurrencyType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf5a9b4 — j___ZN3rbx8any_castIRN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0xf5a9b4() -> ! {
    todo!("0xf5a9b4 j___ZN3rbx8any_castIRN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::function<void ()(std::string,int,int)>,3,void ()(std::string,int,int)>::call(std::string,int,int)")]
// 0xf5aa24 — j___ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsii
pub fn stub_0xf5aa24() -> ! {
    todo!("0xf5aa24 j___ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsii")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::function<void ()(std::string,int,int)>,3,void ()(std::string,int,int)>::callable<rbx::signals::signal<void ()(std::string,int,int)>*>(boost::function<void ()(std::string,int,int)> const&,rbx::signals::signal<void ()(std::string,int,int)>*)")]
// 0xf5aa34 — j___ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_
pub fn stub_0xf5aa34() -> ! {
    todo!("0xf5aa34 j___ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,bool)>::slot,boost::function<void ()(int,int,bool)>,3,void ()(int,int,bool)>::callable<rbx::signals::signal<void ()(int,int,bool)>*>(boost::function<void ()(int,int,bool)> const&,rbx::signals::signal<void ()(int,int,bool)>*)")]
// 0xf5aa44 — j___ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_
pub fn stub_0xf5aa44() -> ! {
    todo!("0xf5aa44 j___ZN3rbx8callableINS_7signals6signalIFviibEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,bool)>::slot>::operator=(rbx::signals::signal<void ()(int,int,bool)>::slot*)")]
// 0xf5aad4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviibEE4slotEEaSEPS6_
pub fn stub_0xf5aad4() -> ! {
    todo!("0xf5aad4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviibEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,bool)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,bool)>::slot> const&)")]
// 0xf5aae4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviibEE4slotEEaSERKS7_
pub fn stub_0xf5aae4() -> ! {
    todo!("0xf5aae4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviibEE4slotEEaSERKS7_")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0xf5ab84 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
pub fn stub_0xf5ab84() -> ! {
    todo!("0xf5ab84 j___ZN5boost3_bi5list5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>> &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0xf5ab94 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0xf5ab94() -> ! {
    todo!("0xf5ab94 j___ZN5boost3_bi5list5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0xf5abd4 — j___ZN5boost3_bi8storage5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
pub fn stub_0xf5abd4() -> ! {
    todo!("0xf5abd4 j___ZN5boost3_bi8storage5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_5<RBX::MarketplaceService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,RBX::MarketplaceService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (RBX::MarketplaceService::*)(std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),RBX::MarketplaceService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xf5ac34 — j___ZN5boost4bindIvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENS6_IFvSsEEEPS2_NS_3argILi1EEENSC_ILi2EEES8_SA_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISH_T0_T1_T2_T3_T4_EENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSK_FSH_SL_SM_SN_SO_ESR_SS_ST_SU_SV_
pub fn stub_0xf5ac34() -> ! {
    todo!("0xf5ac34 j___ZN5boost4bindIvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENS6_IFvSsEEEPS2_NS_3argILi1EEENSC_ILi2EEES8_SA_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISH_T0_T1_T2_T3_T4_EENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSK_FSH_SL_SM_SN_SO_ESR_SS_ST_SU_SV_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf5acb4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0xf5acb4() -> ! {
    todo!("0xf5acb4 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0xf5adb4 — j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_
pub fn stub_0xf5adb4() -> ! {
    todo!("0xf5adb4 j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_")
}

#[doc(alias = "boost::function3<void,std::string,int,int>::clear(void)")]
// 0xf5ae14 — j___ZN5boost9function3IvSsiiE5clearEv
pub fn stub_0xf5ae14() -> ! {
    todo!("0xf5ae14 j___ZN5boost9function3IvSsiiE5clearEv")
}

#[doc(alias = "boost::function3<void,int,int,bool>::assign_to_own(boost::function3<void,int,int,bool> const&)")]
// 0xf5ae44 — j___ZN5boost9function3IviibE13assign_to_ownERKS1_
pub fn stub_0xf5ae44() -> ! {
    todo!("0xf5ae44 j___ZN5boost9function3IviibE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::function3<void,int,int,bool>::clear(void)")]
// 0xf5ae54 — j___ZN5boost9function3IviibE5clearEv
pub fn stub_0xf5ae54() -> ! {
    todo!("0xf5ae54 j___ZN5boost9function3IviibE5clearEv")
}

#[doc(alias = "boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::operator()(RBX::MarketplaceService*,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)const")]
// 0xf5af54 — j___ZNK5boost4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_
pub fn stub_0xf5af54() -> ! {
    todo!("0xf5af54 j___ZNK5boost4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf5afc4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_0xf5afc4() -> ! {
    todo!("0xf5afc4 j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0xf5afd4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_0xf5afd4() -> ! {
    todo!("0xf5afd4 j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf5afe4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_0xf5afe4() -> ! {
    todo!("0xf5afe4 j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::function3<void,int,int,bool>::operator()(int,int,bool)const")]
// 0xf5b0a4 — j___ZNK5boost9function3IviibEclEiib
pub fn stub_0xf5b0a4() -> ! {
    todo!("0xf5b0a4 j___ZNK5boost9function3IviibEclEiib")
}

#[doc(alias = "std::_Vector_base<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::_M_allocate(unsigned long)")]
// 0xf5b0c4 — j___ZNSt12_Vector_baseIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE11_M_allocateEm
pub fn stub_0xf5b0c4() -> ! {
    todo!("0xf5b0c4 j___ZNSt12_Vector_baseIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::MarketplaceService::CurrencyType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::MarketplaceService::CurrencyType *,RBX::MarketplaceService::CurrencyType *>(RBX::MarketplaceService::CurrencyType *,RBX::MarketplaceService::CurrencyType *,RBX::MarketplaceService::CurrencyType *)")]
// 0xf5b0d4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18MarketplaceService12CurrencyTypeES6_EET0_T_S8_S7_
pub fn stub_0xf5b0d4() -> ! {
    todo!("0xf5b0d4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18MarketplaceService12CurrencyTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::MarketplaceService::CurrencyType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::operator[](RBX::Name const* const&)")]
// 0xf5b0e4 — j___ZNSt3mapIPKN3RBX4NameENS0_18MarketplaceService12CurrencyTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf5b0e4() -> ! {
    todo!("0xf5b0e4 j___ZNSt3mapIPKN3RBX4NameENS0_18MarketplaceService12CurrencyTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::MarketplaceService::CurrencyType*,std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>>,RBX::MarketplaceService::CurrencyType const&)")]
// 0xf5b0f4 — j___ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf5b0f4() -> ! {
    todo!("0xf5b0f4 j___ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::MarketplaceService::CurrencyType*,std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>>,unsigned long,RBX::MarketplaceService::CurrencyType const&)")]
// 0xf5b104 — j___ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf5b104() -> ! {
    todo!("0xf5b104 j___ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::resize(unsigned long,RBX::MarketplaceService::CurrencyType)")]
// 0xf5b114 — j___ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE6resizeEmS2_
pub fn stub_0xf5b114() -> ! {
    todo!("0xf5b114 j___ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::push_back(RBX::MarketplaceService::CurrencyType const&)")]
// 0xf5b124 — j___ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE9push_backERKS2_
pub fn stub_0xf5b124() -> ! {
    todo!("0xf5b124 j___ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType> const&)")]
// 0xf5b134 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf5b134() -> ! {
    todo!("0xf5b134 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType> const&)")]
// 0xf5b144 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf5b144() -> ! {
    todo!("0xf5b144 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>> *)")]
// 0xf5b154 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0xf5b154() -> ! {
    todo!("0xf5b154 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType> const&)")]
// 0xf5b164 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf5b164() -> ! {
    todo!("0xf5b164 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char *,std::string> std::transform<__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::to_lowerF<char>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::to_lowerF<char>)")]
// 0xf5b174 — j___ZSt9transformIN9__gnu_cxx17__normal_iteratorIPcSsEES3_N5boost9algorithm6detail9to_lowerFIcEEET0_T_SA_S9_T1_
pub fn stub_0xf5b174() -> ! {
    todo!("0xf5b174 j___ZSt9transformIN9__gnu_cxx17__normal_iteratorIPcSsEES3_N5boost9algorithm6detail9to_lowerFIcEEET0_T_SA_S9_T1_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiBase>::operator=(rbx_core::SharedPtr<RBX::GuiBase> const&)")]
// 0xf5b1d4 — j___ZN5boost10shared_ptrIN3RBX7GuiBaseEEaSERKS3_
pub fn stub_0xf5b1d4() -> ! {
    todo!("0xf5b1d4 j___ZN5boost10shared_ptrIN3RBX7GuiBaseEEaSERKS3_")
}

#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::_M_allocate(unsigned long)")]
// 0xf5b1f4 — j___ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE11_M_allocateEm
pub fn stub_0xf5b1f4() -> ! {
    todo!("0xf5b1f4 j___ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::_Vector_base(unsigned long,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>> const&)")]
// 0xf5b204 — j___ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EEC2EmRKS5_
pub fn stub_0xf5b204() -> ! {
    todo!("0xf5b204 j___ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EEC2EmRKS5_")
}

#[doc(alias = "std::_Vector_base<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>>::_M_allocate(unsigned long)")]
// 0xf5b214 — j___ZNSt12_Vector_baseISt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS5_EESaIS7_EE11_M_allocateEm
pub fn stub_0xf5b214() -> ! {
    todo!("0xf5b214 j___ZNSt12_Vector_baseISt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS5_EESaIS7_EE11_M_allocateEm")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiBase> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::GuiBase> *,rbx_core::SharedPtr<RBX::GuiBase> *>(rbx_core::SharedPtr<RBX::GuiBase> *,rbx_core::SharedPtr<RBX::GuiBase> *,rbx_core::SharedPtr<RBX::GuiBase> *)")]
// 0xf5b224 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX7GuiBaseEEES8_EET0_T_SA_S9_
pub fn stub_0xf5b224() -> ! {
    todo!("0xf5b224 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX7GuiBaseEEES8_EET0_T_SA_S9_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> *,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> *>(std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> *,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> *,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> *)")]
// 0xf5b234 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS8_EESB_EET0_T_SD_SC_
pub fn stub_0xf5b234() -> ! {
    todo!("0xf5b234 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS8_EESB_EET0_T_SD_SC_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiBase>* std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::GuiBase> const*,rbx_core::SharedPtr<RBX::GuiBase>*>(rbx_core::SharedPtr<RBX::GuiBase> const*,rbx_core::SharedPtr<RBX::GuiBase> const*,rbx_core::SharedPtr<RBX::GuiBase>*)")]
// 0xf5b244 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN5boost10shared_ptrIN3RBX7GuiBaseEEEPS7_EET0_T_SC_SB_
pub fn stub_0xf5b244() -> ! {
    todo!("0xf5b244 j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN5boost10shared_ptrIN3RBX7GuiBaseEEEPS7_EET0_T_SC_SB_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiBase> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::GuiBase> *,rbx_core::SharedPtr<RBX::GuiBase> *>(rbx_core::SharedPtr<RBX::GuiBase> *,rbx_core::SharedPtr<RBX::GuiBase> *,rbx_core::SharedPtr<RBX::GuiBase> *)")]
// 0xf5b254 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX7GuiBaseEEES8_EET0_T_SA_S9_
pub fn stub_0xf5b254() -> ! {
    todo!("0xf5b254 j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX7GuiBaseEEES8_EET0_T_SA_S9_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::GuiBase>*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>,rbx_core::SharedPtr<RBX::GuiBase> const&)")]
// 0xf5b264 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
pub fn stub_0xf5b264() -> ! {
    todo!("0xf5b264 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::GuiBase>*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>,unsigned long,rbx_core::SharedPtr<RBX::GuiBase> const&)")]
// 0xf5b274 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
pub fn stub_0xf5b274() -> ! {
    todo!("0xf5b274 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::GuiBase>*)")]
// 0xf5b284 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE15_M_erase_at_endEPS4_
pub fn stub_0xf5b284() -> ! {
    todo!("0xf5b284 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE15_M_erase_at_endEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiBase>* std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::GuiBase> const*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>>(unsigned long,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::GuiBase> const*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::GuiBase> const*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>)")]
// 0xf5b294 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS4_S6_EEEEPS4_mT_SE_
pub fn stub_0xf5b294() -> ! {
    todo!("0xf5b294 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS4_S6_EEEEPS4_mT_SE_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::resize(unsigned long,rbx_core::SharedPtr<RBX::GuiBase>)")]
// 0xf5b2a4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE6resizeEmS4_
pub fn stub_0xf5b2a4() -> ! {
    todo!("0xf5b2a4 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE6resizeEmS4_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::push_back(rbx_core::SharedPtr<RBX::GuiBase> const&)")]
// 0xf5b2b4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE9push_backERKS4_
pub fn stub_0xf5b2b4() -> ! {
    todo!("0xf5b2b4 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE9push_backERKS4_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::vector(std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> const&)")]
// 0xf5b2c4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EEC2ERKS6_
pub fn stub_0xf5b2c4() -> ! {
    todo!("0xf5b2c4 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EEC2ERKS6_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::~vector()")]
// 0xf5b2d4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EED2Ev
pub fn stub_0xf5b2d4() -> ! {
    todo!("0xf5b2d4 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EED2Ev")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::operator=(std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> const&)")]
// 0xf5b2e4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EEaSERKS6_
pub fn stub_0xf5b2e4() -> ! {
    todo!("0xf5b2e4 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EEaSERKS6_")
}

#[doc(alias = "std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>*,std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>>>,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> const&)")]
// 0xf5b2f4 — j___ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_
pub fn stub_0xf5b2f4() -> ! {
    todo!("0xf5b2f4 j___ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_")
}

#[doc(alias = "std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>>::push_back(std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> const&)")]
// 0xf5b304 — j___ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EE9push_backERKS6_
pub fn stub_0xf5b304() -> ! {
    todo!("0xf5b304 j___ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EE9push_backERKS6_")
}

#[doc(alias = "std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>>::~vector()")]
// 0xf5b314 — j___ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EED2Ev
pub fn stub_0xf5b314() -> ! {
    todo!("0xf5b314 j___ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EED2Ev")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<RBX::GuiBase> *,unsigned long,rbx_core::SharedPtr<RBX::GuiBase>>(rbx_core::SharedPtr<RBX::GuiBase> *,unsigned long,rbx_core::SharedPtr<RBX::GuiBase> const&,std::__false_type)")]
// 0xf5b324 — j___ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX7GuiBaseEEEmS4_EvT_T0_RKT1_St12__false_type
pub fn stub_0xf5b324() -> ! {
    todo!("0xf5b324 j___ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX7GuiBaseEEEmS4_EvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Tool> RBX::shared_from<RBX::Tool>(RBX::Tool*)")]
// 0xf5b384 — j___ZN3RBX11shared_fromINS_4ToolEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_0xf5b384() -> ! {
    todo!("0xf5b384 j___ZN3RBX11shared_fromINS_4ToolEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::ContextActionService::~ContextActionService()")]
// 0xf5b3a4 — j___ZN3RBX20ContextActionServiceD2Ev
pub fn stub_0xf5b3a4() -> ! {
    todo!("0xf5b3a4 j___ZN3RBX20ContextActionServiceD2Ev")
}

#[doc(alias = "std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::vector(unsigned long,RBX::Voxel::Cell const&,std::allocator<RBX::Voxel::Cell> const&)")]
// 0xf5b504 — j___ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EEC2EmRKS2_RKS3_
pub fn stub_0xf5b504() -> ! {
    todo!("0xf5b504 j___ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EEC2EmRKS2_RKS3_")
}

#[doc(alias = "RBX::Voxel::Grid::Chunk::updateCountOfNonEmptyCells(int)")]
// 0xf5b514 — j___ZN3RBX5Voxel4Grid5Chunk26updateCountOfNonEmptyCellsEi
pub fn stub_0xf5b514() -> ! {
    todo!("0xf5b514 j___ZN3RBX5Voxel4Grid5Chunk26updateCountOfNonEmptyCellsEi")
}

#[doc(alias = "RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::insert(RBX::SpatialRegion::Id const&)")]
// 0xf5b534 — j___ZN3RBX5Voxel8ChunkMapINS0_4Grid5ChunkEE6insertERKNS_13SpatialRegion2IdE
pub fn stub_0xf5b534() -> ! {
    todo!("0xf5b534 j___ZN3RBX5Voxel8ChunkMapINS0_4Grid5ChunkEE6insertERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::ChunkMap(void)")]
// 0xf5b544 — j___ZN3RBX5Voxel8ChunkMapINS0_4Grid5ChunkEEC2Ev
pub fn stub_0xf5b544() -> ! {
    todo!("0xf5b544 j___ZN3RBX5Voxel8ChunkMapINS0_4Grid5ChunkEEC2Ev")
}

#[doc(alias = "RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::find(RBX::SpatialRegion::Id const&)const")]
// 0xf5b554 — j___ZNK3RBX5Voxel8ChunkMapINS0_4Grid5ChunkEE4findERKNS_13SpatialRegion2IdE
pub fn stub_0xf5b554() -> ! {
    todo!("0xf5b554 j___ZNK3RBX5Voxel8ChunkMapINS0_4Grid5ChunkEE4findERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue>>::_M_allocate(unsigned long)")]
// 0xf5b564 — j___ZNSt12_Vector_baseIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EE11_M_allocateEm
pub fn stub_0xf5b564() -> ! {
    todo!("0xf5b564 j___ZNSt12_Vector_baseIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue>>::_Vector_base(unsigned long,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue> const&)")]
// 0xf5b574 — j___ZNSt12_Vector_baseIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EEC2EmRKS7_
pub fn stub_0xf5b574() -> ! {
    todo!("0xf5b574 j___ZNSt12_Vector_baseIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EEC2EmRKS7_")
}

#[doc(alias = "std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::vector(std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>> const&)")]
// 0xf5b584 — j___ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EEC2ERKS4_
pub fn stub_0xf5b584() -> ! {
    todo!("0xf5b584 j___ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EEC2ERKS4_")
}

#[doc(alias = "std::vector<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue>>::vector(unsigned long,RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue const&,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue> const&)")]
// 0xf5b594 — j___ZNSt6vectorIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EEC2EmRKS6_RKS7_
pub fn stub_0xf5b594() -> ! {
    todo!("0xf5b594 j___ZNSt6vectorIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EEC2EmRKS6_RKS7_")
}

#[doc(alias = "std::vector<unsigned char,std::allocator<unsigned char>>::vector(std::vector<unsigned char,std::allocator<unsigned char>> const&)")]
// 0xf5b5a4 — j___ZNSt6vectorIhSaIhEEC2ERKS1_
pub fn stub_0xf5b5a4() -> ! {
    todo!("0xf5b5a4 j___ZNSt6vectorIhSaIhEEC2ERKS1_")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue *,unsigned long,RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue>(RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue *,unsigned long,RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue const&,std::__false_type)")]
// 0xf5b5b4 — j___ZSt26__uninitialized_fill_n_auxIPN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueEmS6_EvT_T0_RKT1_St12__false_type
pub fn stub_0xf5b5b4() -> ! {
    todo!("0xf5b5b4 j___ZSt26__uninitialized_fill_n_auxIPN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueEmS6_EvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "std::_Vector_base<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>::_M_allocate(unsigned long)")]
// 0xf5b5e4 — j___ZNSt12_Vector_baseIN3RBX11InputObject13UserInputTypeESaIS2_EE11_M_allocateEm
pub fn stub_0xf5b5e4() -> ! {
    todo!("0xf5b5e4 j___ZNSt12_Vector_baseIN3RBX11InputObject13UserInputTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>::_M_allocate(unsigned long)")]
// 0xf5b5f4 — j___ZNSt12_Vector_baseIN3RBX11InputObject14UserInputStateESaIS2_EE11_M_allocateEm
pub fn stub_0xf5b5f4() -> ! {
    todo!("0xf5b5f4 j___ZNSt12_Vector_baseIN3RBX11InputObject14UserInputStateESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::InputObject::UserInputType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::InputObject::UserInputType *,RBX::InputObject::UserInputType *>(RBX::InputObject::UserInputType *,RBX::InputObject::UserInputType *,RBX::InputObject::UserInputType *)")]
// 0xf5b604 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11InputObject13UserInputTypeES6_EET0_T_S8_S7_
pub fn stub_0xf5b604() -> ! {
    todo!("0xf5b604 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11InputObject13UserInputTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::InputObject::UserInputState * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::InputObject::UserInputState *,RBX::InputObject::UserInputState *>(RBX::InputObject::UserInputState *,RBX::InputObject::UserInputState *,RBX::InputObject::UserInputState *)")]
// 0xf5b614 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11InputObject14UserInputStateES6_EET0_T_S8_S7_
pub fn stub_0xf5b614() -> ! {
    todo!("0xf5b614 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11InputObject14UserInputStateES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::InputObject::UserInputType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::operator[](RBX::Name const* const&)")]
// 0xf5b624 — j___ZNSt3mapIPKN3RBX4NameENS0_11InputObject13UserInputTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf5b624() -> ! {
    todo!("0xf5b624 j___ZNSt3mapIPKN3RBX4NameENS0_11InputObject13UserInputTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::InputObject::UserInputState,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::operator[](RBX::Name const* const&)")]
// 0xf5b634 — j___ZNSt3mapIPKN3RBX4NameENS0_11InputObject14UserInputStateESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf5b634() -> ! {
    todo!("0xf5b634 j___ZNSt3mapIPKN3RBX4NameENS0_11InputObject14UserInputStateESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::InputObject::UserInputType*,std::vector<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>>,RBX::InputObject::UserInputType const&)")]
// 0xf5b644 — j___ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf5b644() -> ! {
    todo!("0xf5b644 j___ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::InputObject::UserInputType*,std::vector<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>>,unsigned long,RBX::InputObject::UserInputType const&)")]
// 0xf5b654 — j___ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf5b654() -> ! {
    todo!("0xf5b654 j___ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>::resize(unsigned long,RBX::InputObject::UserInputType)")]
// 0xf5b664 — j___ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE6resizeEmS2_
pub fn stub_0xf5b664() -> ! {
    todo!("0xf5b664 j___ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>::push_back(RBX::InputObject::UserInputType const&)")]
// 0xf5b674 — j___ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE9push_backERKS2_
pub fn stub_0xf5b674() -> ! {
    todo!("0xf5b674 j___ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::InputObject::UserInputState*,std::vector<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>>,RBX::InputObject::UserInputState const&)")]
// 0xf5b684 — j___ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf5b684() -> ! {
    todo!("0xf5b684 j___ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::InputObject::UserInputState*,std::vector<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>>,unsigned long,RBX::InputObject::UserInputState const&)")]
// 0xf5b694 — j___ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf5b694() -> ! {
    todo!("0xf5b694 j___ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>::resize(unsigned long,RBX::InputObject::UserInputState)")]
// 0xf5b6a4 — j___ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE6resizeEmS2_
pub fn stub_0xf5b6a4() -> ! {
    todo!("0xf5b6a4 j___ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>::push_back(RBX::InputObject::UserInputState const&)")]
// 0xf5b6b4 — j___ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE9push_backERKS2_
pub fn stub_0xf5b6b4() -> ! {
    todo!("0xf5b6b4 j___ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::InputObject::UserInputType> const&)")]
// 0xf5b6c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf5b6c4() -> ! {
    todo!("0xf5b6c4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType> const&)")]
// 0xf5b6d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf5b6d4() -> ! {
    todo!("0xf5b6d4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType> const&)")]
// 0xf5b6e4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf5b6e4() -> ! {
    todo!("0xf5b6e4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::InputObject::UserInputState> const&)")]
// 0xf5b6f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf5b6f4() -> ! {
    todo!("0xf5b6f4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState> const&)")]
// 0xf5b704 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf5b704() -> ! {
    todo!("0xf5b704 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState> const&)")]
// 0xf5b714 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf5b714() -> ! {
    todo!("0xf5b714 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

