//! core shard lq — 100 core stubs EA-sorted, next uncovered fallback after shard lp (0x801ed0..0x82699c, lowest EA first).
//! Source: `ida/export.json` filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|FMOD|Lua (fallback 37271, 6088 uncovered -> 5988 after, distinct 38699->38799, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch].
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::TestService::startScripts(void)")]
#[doc(alias = "__ZN3RBX11TestService12startScriptsEv")]
// 0x801ed0 — __ZN3RBX11TestService12startScriptsEv
// type: _DWORD __fastcall(RBX::TestService *__hidden this)
pub fn stub_0x801ed0() -> ! {
    todo!("0x801ed0 __ZN3RBX11TestService12startScriptsEv")
}

#[doc(alias = "RBX::TestService::stopScripts(void)")]
#[doc(alias = "__ZN3RBX11TestService11stopScriptsEv")]
// 0x8025ac — __ZN3RBX11TestService11stopScriptsEv
// type: void __fastcall(const shared_count *this)
pub fn stub_0x8025ac() -> ! {
    todo!("0x8025ac __ZN3RBX11TestService11stopScriptsEv")
}

#[doc(alias = "RBX::TestService::onScriptEnded(int)")]
#[doc(alias = "__ZN3RBX11TestService13onScriptEndedEi")]
// 0x802818 — __ZN3RBX11TestService13onScriptEndedEi
// type: RBX::TestService *__fastcall(RBX::TestService *this, int)
pub fn stub_0x802818() -> ! {
    todo!("0x802818 __ZN3RBX11TestService13onScriptEndedEi")
}

#[doc(alias = "RBX::TestService::onScriptFailed(int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)")]
#[doc(alias = "__ZN3RBX11TestService14onScriptFailedEiPKcS2_N5boost10shared_ptrINS_10BaseScriptEEEi")]
// 0x802830 — __ZN3RBX11TestService14onScriptFailedEiPKcS2_N5boost10shared_ptrINS_10BaseScriptEEEi
// was: RBX::TestService::onScriptFailed(int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)
pub fn stub_0x802830() -> ! {
    todo!("0x802830 __ZN3RBX11TestService14onScriptFailedEiPKcS2_N5boost10shared_ptrINS_10BaseScriptEEEi")
}

#[doc(alias = "RBX::TestService::filterScript(std::string const&)")]
#[doc(alias = "__ZN3RBX11TestService12filterScriptERKSs")]
// 0x8029a8 — __ZN3RBX11TestService12filterScriptERKSs
// type: void __fastcall(RBX::TestService *this, const std::string *, const std::string *)
pub fn stub_0x8029a8() -> ! {
    todo!("0x8029a8 __ZN3RBX11TestService12filterScriptERKSs")
}

#[doc(alias = "__ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS8_E4typeEST_")]
// 0x804864 — __ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS8_E4typeEST_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, int, int, int, int)
// was: __ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS8_E4typeEST_
pub fn stub_0x804864() -> ! {
    todo!("0x804864 __ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS8_E4typeEST_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list_av_6<rbx_core::SharedPtr<RBX::TestService>,int,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int,rbx_core::SharedPtr<RBX::TestService>,int,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::TestService::*)(int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int),rbx_core::SharedPtr<RBX::TestService>,int,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX11TestServiceEiPKcS4_NS_10shared_ptrINS1_10BaseScriptEEEiNS5_IS2_EEiNS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISG_T0_T1_T2_T3_T4_T5_EENSE_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMSJ_FSG_SK_SL_SM_SN_SO_ESR_SS_ST_SU_SV_SW_")]
// 0x804978 — __ZN5boost4bindIvN3RBX11TestServiceEiPKcS4_NS_10shared_ptrINS1_10BaseScriptEEEiNS5_IS2_EEiNS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISG_T0_T1_T2_T3_T4_T5_EENSE_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMSJ_FSG_SK_SL_SM_SN_SO_ESR_SS_ST_SU_SV_SW_
// type: void __fastcall(int, int, int, const shared_count *, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>,boost::_bi::list_av_6<boost::shared_ptr<RBX::TestService>,int,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int,boost::shared_ptr<RBX::TestService>,int,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::TestService::*)(int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int),boost::shared_ptr<RBX::TestService>,int,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)
pub fn stub_0x804978() -> ! {
    todo!("0x804978 __ZN5boost4bindIvN3RBX11TestServiceEiPKcS4_NS_10shared_ptrINS1_10BaseScriptEEEiNS5_IS2_EEiNS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISG_T0_T1_T2_T3_T4_T5_EENSE_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMSJ_FSG_SK_SL_SM_SN_SO_ESR_SS_ST_SU_SV_SW_")
}

#[doc(alias = "__ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// 0x807944 — __ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: __ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
pub fn stub_0x807944() -> ! {
    todo!("0x807944 __ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
// 0x807a2c — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
pub fn stub_0x807a2c() -> ! {
    todo!("0x807a2c __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")]
#[doc(alias = "__ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEEvT_")]
// 0x807b18 — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: void boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)
pub fn stub_0x807b18() -> ! {
    todo!("0x807b18 __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")]
// 0x807c14 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x807c14() -> ! {
    todo!("0x807c14 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::invoke(boost::detail::function::function_buffer &,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker4INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEvSA_SA_SD_iE6invokeERNS1_15function_bufferESA_SA_SD_i")]
// 0x807c30 — __ZN5boost6detail8function26void_function_obj_invoker4INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEvSA_SA_SD_iE6invokeERNS1_15function_bufferESA_SA_SD_i
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, int)
// was: boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::invoke(boost::detail::function::function_buffer &,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)
pub fn stub_0x807c30() -> ! {
    todo!("0x807c30 __ZN5boost6detail8function26void_function_obj_invoker4INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEvSA_SA_SD_iE6invokeERNS1_15function_bufferESA_SA_SD_i")
}

#[doc(alias = "bool boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEbT_RNS1_15function_bufferE")]
// 0x807c60 — __ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: bool boost::detail::function::basic_vtable4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x807c60() -> ! {
    todo!("0x807c60 __ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x807d4c — __ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: bool boost::detail::function::basic_vtable4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x807d4c() -> ! {
    todo!("0x807d4c __ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x807e34 — __ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, int, _DWORD *)
// was: void boost::detail::function::basic_vtable4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x807e34() -> ! {
    todo!("0x807e34 __ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list4<char const*&,char const*&,rbx_core::SharedPtr<RBX::BaseScript>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int> &,boost::_bi::list4<char const*&,char const*&,rbx_core::SharedPtr<RBX::BaseScript>&,int &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEclINS_4_mfi3mf5IvS5_iPKcSJ_NS3_INS4_10BaseScriptEEEiEENS0_5list4IRSJ_SO_RSL_RiEEEEvNS0_4typeIvEERT_RT0_i")]
// 0x807f0c — __ZN5boost3_bi5list6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEclINS_4_mfi3mf5IvS5_iPKcSJ_NS3_INS4_10BaseScriptEEEiEENS0_5list4IRSJ_SO_RSL_RiEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
// was: void boost::_bi::list6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>,boost::_bi::list4<char const*&,char const*&,boost::shared_ptr<RBX::BaseScript>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int> &,boost::_bi::list4<char const*&,char const*&,boost::shared_ptr<RBX::BaseScript>&,int &> &,int)
pub fn stub_0x807f0c() -> ! {
    todo!("0x807f0c __ZN5boost3_bi5list6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEclINS_4_mfi3mf5IvS5_iPKcSJ_NS3_INS4_10BaseScriptEEEiEENS0_5list4IRSJ_SO_RSL_RiEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "void boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::call<rbx_core::SharedPtr<RBX::TestService>,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>(rbx_core::SharedPtr<RBX::TestService> &,void const*,int &,char const* &,char const* &,rbx_core::SharedPtr<RBX::BaseScript> &,int &)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf5IvN3RBX11TestServiceEiPKcS5_NS_10shared_ptrINS2_10BaseScriptEEEiE4callINS6_IS3_EEiS5_S5_S8_iEEvRT_PKvRT0_RT1_RT2_RT3_RT4_")]
// 0x80800c — __ZNK5boost4_mfi3mf5IvN3RBX11TestServiceEiPKcS5_NS_10shared_ptrINS2_10BaseScriptEEEiE4callINS6_IS3_EEiS5_S5_S8_iEEvRT_PKvRT0_RT1_RT2_RT3_RT4_
// type: void __fastcall(char **, _DWORD *, int, int *, int *, boost::detail::sp_counted_base *, const shared_count *, _DWORD *, int, int)
// was: void boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::call<boost::shared_ptr<RBX::TestService>,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>(boost::shared_ptr<RBX::TestService> &,void const*,int &,char const* &,char const* &,boost::shared_ptr<RBX::BaseScript> &,int &)const
pub fn stub_0x80800c() -> ! {
    todo!("0x80800c __ZNK5boost4_mfi3mf5IvN3RBX11TestServiceEiPKcS5_NS_10shared_ptrINS2_10BaseScriptEEEiE4callINS6_IS3_EEiS5_S5_S8_iEEvRT_PKvRT0_RT1_RT2_RT3_RT4_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x80811c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x80811c() -> ! {
    todo!("0x80811c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "RBX::LibraryService::LibraryService(RBX::ScriptContext *)")]
#[doc(alias = "__ZN3RBX14LibraryServiceC1EPNS_13ScriptContextE")]
// 0x817ef4 — __ZN3RBX14LibraryServiceC1EPNS_13ScriptContextE
// type: _DWORD __fastcall(RBX::LibraryService *__hidden this, RBX::ScriptContext *)
pub fn stub_0x817ef4() -> ! {
    todo!("0x817ef4 __ZN3RBX14LibraryServiceC1EPNS_13ScriptContextE")
}

#[doc(alias = "RBX::LibraryService::LibraryService(RBX::ScriptContext *)")]
#[doc(alias = "__ZN3RBX14LibraryServiceC2EPNS_13ScriptContextE")]
// 0x817ef8 — __ZN3RBX14LibraryServiceC2EPNS_13ScriptContextE
// type: _DWORD __fastcall(RBX::LibraryService *__hidden this, RBX::ScriptContext *)
pub fn stub_0x817ef8() -> ! {
    todo!("0x817ef8 __ZN3RBX14LibraryServiceC2EPNS_13ScriptContextE")
}

#[doc(alias = "RBX::LibraryService::ContentReadyHelper(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
#[doc(alias = "__ZN3RBX14LibraryService18ContentReadyHelperEN5boost8weak_ptrINS_13ScriptContextEEESsSsNS_14AsyncHttpQueue13RequestResultEPSiNS1_10shared_ptrIKSsEE")]
// 0x818730 — __ZN3RBX14LibraryService18ContentReadyHelperEN5boost8weak_ptrINS_13ScriptContextEEESsSsNS_14AsyncHttpQueue13RequestResultEPSiNS1_10shared_ptrIKSsEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::LibraryService::ContentReadyHelper(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)
pub fn stub_0x818730() -> ! {
    todo!("0x818730 __ZN3RBX14LibraryService18ContentReadyHelperEN5boost8weak_ptrINS_13ScriptContextEEESsSsNS_14AsyncHttpQueue13RequestResultEPSiNS1_10shared_ptrIKSsEE")
}

#[doc(alias = "RBX::LibraryService::registerDevelopmentLibrary(std::string const&,rbx_core::SharedPtr<RBX::Script>)")]
#[doc(alias = "__ZN3RBX14LibraryService26registerDevelopmentLibraryERKSsN5boost10shared_ptrINS_6ScriptEEE")]
// 0x81957c — __ZN3RBX14LibraryService26registerDevelopmentLibraryERKSsN5boost10shared_ptrINS_6ScriptEEE
// was: RBX::LibraryService::registerDevelopmentLibrary(std::string const&,boost::shared_ptr<RBX::Script>)
pub fn stub_0x81957c() -> ! {
    todo!("0x81957c __ZN3RBX14LibraryService26registerDevelopmentLibraryERKSsN5boost10shared_ptrINS_6ScriptEEE")
}

#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::Script>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsN5boost10shared_ptrIN3RBX6ScriptEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_")]
// 0x81b960 — __ZNSt3mapISsN5boost10shared_ptrIN3RBX6ScriptEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_
// was: std::map<std::string,boost::shared_ptr<RBX::Script>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::operator[](std::string const&)
pub fn stub_0x81b960() -> ! {
    todo!("0x81b960 __ZNSt3mapISsN5boost10shared_ptrIN3RBX6ScriptEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script>::operator=(rbx_core::SharedPtr<RBX::Script> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6ScriptEEaSERKS3_")]
// 0x81bb7c — __ZN5boost10shared_ptrIN3RBX6ScriptEEaSERKS3_
// was: boost::shared_ptr<RBX::Script>::operator=(boost::shared_ptr<RBX::Script> const&)
pub fn stub_0x81bb7c() -> ! {
    todo!("0x81bb7c __ZN5boost10shared_ptrIN3RBX6ScriptEEaSERKS3_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_6<rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS2_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEES4_SsSsNS_3argILi1EEENSB_ILi2EEENSB_ILi3EEEEENS_3_bi6bind_tIT_PFSH_T0_T1_T2_T3_T4_T5_ENSF_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEESP_SR_SS_ST_SU_SV_SW_")]
// 0x81c20c — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS2_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEES4_SsSsNS_3argILi1EEENSB_ILi2EEENSB_ILi3EEEEENS_3_bi6bind_tIT_PFSH_T0_T1_T2_T3_T4_T5_ENSF_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEESP_SR_SS_ST_SU_SV_SW_
// type: int __fastcall(int, int, int, int, std::string *)
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list_av_6<boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>)
pub fn stub_0x81c20c() -> ! {
    todo!("0x81c20c __ZN5boost4bindIvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS2_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEES4_SsSsNS_3argILi1EEENSB_ILi2EEENSB_ILi3EEEEENS_3_bi6bind_tIT_PFSH_T0_T1_T2_T3_T4_T5_ENSF_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEESP_SR_SS_ST_SU_SV_SW_")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptContext> RBX::weak_from<RBX::ScriptContext>(RBX::ScriptContext*)")]
#[doc(alias = "__ZN3RBX9weak_fromINS_13ScriptContextEEEN5boost8weak_ptrIT_EEPS4_")]
// 0x81c550 — __ZN3RBX9weak_fromINS_13ScriptContextEEEN5boost8weak_ptrIT_EEPS4_
// was: boost::weak_ptr<RBX::ScriptContext> RBX::weak_from<RBX::ScriptContext>(RBX::ScriptContext*)
pub fn stub_0x81c550() -> ! {
    todo!("0x81c550 __ZN3RBX9weak_fromINS_13ScriptContextEEEN5boost8weak_ptrIT_EEPS4_")
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSB_5list6INSB_5valueISF_EENSJ_ISsEESL_NS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// 0x81ca80 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSB_5list6INSB_5valueISF_EENSJ_ISsEESL_NS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSB_5list6INSB_5valueISF_EENSJ_ISsEESL_NS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
pub fn stub_0x81ca80() -> ! {
    todo!("0x81ca80 __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSB_5list6INSB_5valueISF_EENSJ_ISsEESL_NS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
// 0x81cc30 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
pub fn stub_0x81cc30() -> ! {
    todo!("0x81cc30 __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_EC2ERKS9_")]
// 0x81cde4 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_EC2ERKS9_
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)
pub fn stub_0x81cde4() -> ! {
    todo!("0x81cde4 __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_EC2ERKS9_")
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEEvT_")]
// 0x81cf2c — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEEvT_
// was: void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)
pub fn stub_0x81cf2c() -> ! {
    todo!("0x81cf2c __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")]
// 0x81d0f0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x81d0f0() -> ! {
    todo!("0x81d0f0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEvSA_SB_SE_E6invokeERNS1_15function_bufferESA_SB_SE_")]
// 0x81d10c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEvSA_SB_SE_E6invokeERNS1_15function_bufferESA_SB_SE_
// was: boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)
pub fn stub_0x81d10c() -> ! {
    todo!("0x81d10c __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEvSA_SB_SE_E6invokeERNS1_15function_bufferESA_SB_SE_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEbT_RNS1_15function_bufferE")]
// 0x81d130 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x81d130() -> ! {
    todo!("0x81d130 __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x81d2e8 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x81d2e8() -> ! {
    todo!("0x81d2e8 __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x81d498 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x81d498() -> ! {
    todo!("0x81d498 __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclIPFvS6_SsSsNS4_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS0_5list3IRSG_RSH_RSK_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0x81d558 — __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclIPFvS6_SsSsNS4_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS0_5list3IRSG_RSH_RSK_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const>&> &,int)
pub fn stub_0x81d558() -> ! {
    todo!("0x81d558 __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclIPFvS6_SsSsNS4_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS0_5list3IRSG_RSH_RSK_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x81d7e4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x81d7e4() -> ! {
    todo!("0x81d7e4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::list6(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_S8_SA_SB_SC_")]
// 0x81d938 — __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_S8_SA_SB_SC_
// was: boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::list6(boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)
pub fn stub_0x81d938() -> ! {
    todo!("0x81d938 __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_S8_SA_SB_SC_")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias = "__ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_S8_SA_SB_SC_")]
// 0x81db6c — __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_S8_SA_SB_SC_
// was: boost::_bi::storage6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage6(boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)
pub fn stub_0x81db6c() -> ! {
    todo!("0x81db6c __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_S8_SA_SB_SC_")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_")]
// 0x81dda0 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_
// was: boost::_bi::storage5<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>)
pub fn stub_0x81dda0() -> ! {
    todo!("0x81dda0 __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_")]
// 0x81dfd4 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_
// was: boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>)
pub fn stub_0x81dfd4() -> ! {
    todo!("0x81dfd4 __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_EC2ES7_S8_S8_")]
// 0x81e208 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_EC2ES7_S8_S8_
// type: int __fastcall(int, int, int, int)
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)
pub fn stub_0x81e208() -> ! {
    todo!("0x81e208 __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_EC2ES7_S8_S8_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEEEC2ES7_S8_")]
// 0x81e3e4 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEEEC2ES7_S8_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>)
pub fn stub_0x81e3e4() -> ! {
    todo!("0x81e3e4 __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEEEC2ES7_S8_")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptContext>::weak_ptr<RBX::ScriptContext>(rbx_core::SharedPtr<RBX::ScriptContext> const&,boost::detail::sp_enable_if_convertible<RBX::ScriptContext,RBX::ScriptContext>::type)")]
#[doc(alias = "__ZN5boost8weak_ptrIN3RBX13ScriptContextEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")]
// 0x81e558 — __ZN5boost8weak_ptrIN3RBX13ScriptContextEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// was: boost::weak_ptr<RBX::ScriptContext>::weak_ptr<RBX::ScriptContext>(boost::shared_ptr<RBX::ScriptContext> const&,boost::detail::sp_enable_if_convertible<RBX::ScriptContext,RBX::ScriptContext>::type)
pub fn stub_0x81e558() -> ! {
    todo!("0x81e558 __ZN5boost8weak_ptrIN3RBX13ScriptContextEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")
}

#[doc(alias = "std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>::pair(std::string const&,rbx_core::SharedPtr<RBX::Script> const&)")]
#[doc(alias = "__ZNSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEEC2ERS0_RKS5_")]
// 0x81fc84 — __ZNSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEEC2ERS0_RKS5_
// was: std::pair<std::string const,boost::shared_ptr<RBX::Script>>::pair(std::string const&,boost::shared_ptr<RBX::Script> const&)
pub fn stub_0x81fc84() -> ! {
    todo!("0x81fc84 __ZNSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEEC2ERS0_RKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
// 0x81fd40 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, int, int)
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::pair<std::string const,boost::shared_ptr<RBX::Script>> const&)
pub fn stub_0x81fd40() -> ! {
    todo!("0x81fd40 __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")]
// 0x81fe2c — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int __fastcall(int, int, int, int)
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,boost::shared_ptr<RBX::Script>> const&)
pub fn stub_0x81fe2c() -> ! {
    todo!("0x81fe2c __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_")]
// 0x81fe7c — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int)
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::_M_insert_unique(std::pair<std::string const,boost::shared_ptr<RBX::Script>> const&)
pub fn stub_0x81fe7c() -> ! {
    todo!("0x81fe7c __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_")]
// 0x81ff00 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_
// type: _DWORD *__fastcall(int, const shared_count *, int, int, void *, int)
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::_M_create_node(std::pair<std::string const,boost::shared_ptr<RBX::Script>> const&)
pub fn stub_0x81ff00() -> ! {
    todo!("0x81ff00 __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::lower_bound(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_")]
// 0x820008 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::lower_bound(std::string const&)
pub fn stub_0x820008() -> ! {
    todo!("0x820008 __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::find(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_")]
// 0x820038 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_
// type: int __fastcall(int, std::string *this)
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::find(std::string const&)
pub fn stub_0x820038() -> ! {
    todo!("0x820038 __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_")
}

#[doc(alias = "luaB_pairs(lua_State *)")]
#[doc(alias = "__ZL10luaB_pairsP9lua_State")]
// 0x825644 — __ZL10luaB_pairsP9lua_State
pub fn stub_0x825644() -> ! {
    todo!("0x825644 __ZL10luaB_pairsP9lua_State")
}

#[doc(alias = "luaB_next(lua_State *)")]
#[doc(alias = "__ZL9luaB_nextP9lua_State")]
// 0x825674 — __ZL9luaB_nextP9lua_State
pub fn stub_0x825674() -> ! {
    todo!("0x825674 __ZL9luaB_nextP9lua_State")
}

#[doc(alias = "luaB_newproxy(lua_State *)")]
#[doc(alias = "__ZL13luaB_newproxyP9lua_State")]
// 0x8256a4 — __ZL13luaB_newproxyP9lua_State
pub fn stub_0x8256a4() -> ! {
    todo!("0x8256a4 __ZL13luaB_newproxyP9lua_State")
}

#[doc(alias = "luaB_assert(lua_State *)")]
#[doc(alias = "__ZL11luaB_assertP9lua_State")]
// 0x82571c — __ZL11luaB_assertP9lua_State
pub fn stub_0x82571c() -> ! {
    todo!("0x82571c __ZL11luaB_assertP9lua_State")
}

#[doc(alias = "luaB_collectgarbage(lua_State *)")]
#[doc(alias = "__ZL19luaB_collectgarbageP9lua_State")]
// 0x825768 — __ZL19luaB_collectgarbageP9lua_State
// type: int __fastcall(int)
pub fn stub_0x825768() -> ! {
    todo!("0x825768 __ZL19luaB_collectgarbageP9lua_State")
}

#[doc(alias = "luaB_dofile(lua_State *)")]
#[doc(alias = "__ZL11luaB_dofileP9lua_State")]
// 0x825808 — __ZL11luaB_dofileP9lua_State
pub fn stub_0x825808() -> ! {
    todo!("0x825808 __ZL11luaB_dofileP9lua_State")
}

#[doc(alias = "luaB_error(lua_State *)")]
#[doc(alias = "__ZL10luaB_errorP9lua_State")]
// 0x82584c — __ZL10luaB_errorP9lua_State
pub fn stub_0x82584c() -> ! {
    todo!("0x82584c __ZL10luaB_errorP9lua_State")
}

#[doc(alias = "luaB_gcinfo(lua_State *)")]
#[doc(alias = "__ZL11luaB_gcinfoP9lua_State")]
// 0x825894 — __ZL11luaB_gcinfoP9lua_State
pub fn stub_0x825894() -> ! {
    todo!("0x825894 __ZL11luaB_gcinfoP9lua_State")
}

#[doc(alias = "luaB_getfenv(lua_State *)")]
#[doc(alias = "__ZL12luaB_getfenvP9lua_State")]
// 0x8258b0 — __ZL12luaB_getfenvP9lua_State
pub fn stub_0x8258b0() -> ! {
    todo!("0x8258b0 __ZL12luaB_getfenvP9lua_State")
}

#[doc(alias = "luaB_getmetatable(lua_State *)")]
#[doc(alias = "__ZL17luaB_getmetatableP9lua_State")]
// 0x8258e8 — __ZL17luaB_getmetatableP9lua_State
pub fn stub_0x8258e8() -> ! {
    todo!("0x8258e8 __ZL17luaB_getmetatableP9lua_State")
}

#[doc(alias = "luaB_loadfile(lua_State *)")]
#[doc(alias = "__ZL13luaB_loadfileP9lua_State")]
// 0x825920 — __ZL13luaB_loadfileP9lua_State
pub fn stub_0x825920() -> ! {
    todo!("0x825920 __ZL13luaB_loadfileP9lua_State")
}

#[doc(alias = "luaB_load(lua_State *)")]
#[doc(alias = "__ZL9luaB_loadP9lua_State")]
// 0x825944 — __ZL9luaB_loadP9lua_State
// type: int __fastcall(int)
pub fn stub_0x825944() -> ! {
    todo!("0x825944 __ZL9luaB_loadP9lua_State")
}

#[doc(alias = "luaB_loadstring(lua_State *)")]
#[doc(alias = "__ZL15luaB_loadstringP9lua_State")]
// 0x825990 — __ZL15luaB_loadstringP9lua_State
pub fn stub_0x825990() -> ! {
    todo!("0x825990 __ZL15luaB_loadstringP9lua_State")
}

#[doc(alias = "luaB_pcall(lua_State *)")]
#[doc(alias = "__ZL10luaB_pcallP9lua_State")]
// 0x825b7c — __ZL10luaB_pcallP9lua_State
pub fn stub_0x825b7c() -> ! {
    todo!("0x825b7c __ZL10luaB_pcallP9lua_State")
}

#[doc(alias = "luaB_print(lua_State *)")]
#[doc(alias = "__ZL10luaB_printP9lua_State")]
// 0x825bc0 — __ZL10luaB_printP9lua_State
pub fn stub_0x825bc0() -> ! {
    todo!("0x825bc0 __ZL10luaB_printP9lua_State")
}

#[doc(alias = "luaB_rawequal(lua_State *)")]
#[doc(alias = "__ZL13luaB_rawequalP9lua_State")]
// 0x825c98 — __ZL13luaB_rawequalP9lua_State
pub fn stub_0x825c98() -> ! {
    todo!("0x825c98 __ZL13luaB_rawequalP9lua_State")
}

#[doc(alias = "luaB_rawget(lua_State *)")]
#[doc(alias = "__ZL11luaB_rawgetP9lua_State")]
// 0x825cc4 — __ZL11luaB_rawgetP9lua_State
pub fn stub_0x825cc4() -> ! {
    todo!("0x825cc4 __ZL11luaB_rawgetP9lua_State")
}

#[doc(alias = "luaB_rawset(lua_State *)")]
#[doc(alias = "__ZL11luaB_rawsetP9lua_State")]
// 0x825cf0 — __ZL11luaB_rawsetP9lua_State
pub fn stub_0x825cf0() -> ! {
    todo!("0x825cf0 __ZL11luaB_rawsetP9lua_State")
}

#[doc(alias = "luaB_select(lua_State *)")]
#[doc(alias = "__ZL11luaB_selectP9lua_State")]
// 0x825d24 — __ZL11luaB_selectP9lua_State
pub fn stub_0x825d24() -> ! {
    todo!("0x825d24 __ZL11luaB_selectP9lua_State")
}

#[doc(alias = "luaB_setfenv(lua_State *)")]
#[doc(alias = "__ZL12luaB_setfenvP9lua_State")]
// 0x825d8c — __ZL12luaB_setfenvP9lua_State
pub fn stub_0x825d8c() -> ! {
    todo!("0x825d8c __ZL12luaB_setfenvP9lua_State")
}

#[doc(alias = "luaB_setmetatable(lua_State *)")]
#[doc(alias = "__ZL17luaB_setmetatableP9lua_State")]
// 0x825e1c — __ZL17luaB_setmetatableP9lua_State
pub fn stub_0x825e1c() -> ! {
    todo!("0x825e1c __ZL17luaB_setmetatableP9lua_State")
}

#[doc(alias = "luaB_tonumber(lua_State *)")]
#[doc(alias = "__ZL13luaB_tonumberP9lua_State")]
// 0x825e88 — __ZL13luaB_tonumberP9lua_State
pub fn stub_0x825e88() -> ! {
    todo!("0x825e88 __ZL13luaB_tonumberP9lua_State")
}

#[doc(alias = "luaB_tostring(lua_State *)")]
#[doc(alias = "__ZL13luaB_tostringP9lua_State")]
// 0x825f64 — __ZL13luaB_tostringP9lua_State
pub fn stub_0x825f64() -> ! {
    todo!("0x825f64 __ZL13luaB_tostringP9lua_State")
}

#[doc(alias = "luaB_type(lua_State *)")]
#[doc(alias = "__ZL9luaB_typeP9lua_State")]
// 0x826024 — __ZL9luaB_typeP9lua_State
pub fn stub_0x826024() -> ! {
    todo!("0x826024 __ZL9luaB_typeP9lua_State")
}

#[doc(alias = "luaB_unpack(lua_State *)")]
#[doc(alias = "__ZL11luaB_unpackP9lua_State")]
// 0x82604c — __ZL11luaB_unpackP9lua_State
pub fn stub_0x82604c() -> ! {
    todo!("0x82604c __ZL11luaB_unpackP9lua_State")
}

#[doc(alias = "luaB_xpcall(lua_State *)")]
#[doc(alias = "__ZL11luaB_xpcallP9lua_State")]
// 0x8260e4 — __ZL11luaB_xpcallP9lua_State
pub fn stub_0x8260e4() -> ! {
    todo!("0x8260e4 __ZL11luaB_xpcallP9lua_State")
}

#[doc(alias = "getfunc(lua_State *,int)")]
#[doc(alias = "__ZL7getfuncP9lua_Statei")]
// 0x826130 — __ZL7getfuncP9lua_Statei
pub fn stub_0x826130() -> ! {
    todo!("0x826130 __ZL7getfuncP9lua_Statei")
}

#[doc(alias = "load_aux(lua_State *,int)")]
#[doc(alias = "__ZL8load_auxP9lua_Statei")]
// 0x8261ec — __ZL8load_auxP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x8261ec() -> ! {
    todo!("0x8261ec __ZL8load_auxP9lua_Statei")
}

#[doc(alias = "generic_reader(lua_State *,void *,unsigned long *)")]
#[doc(alias = "__ZL14generic_readerP9lua_StatePvPm")]
// 0x826210 — __ZL14generic_readerP9lua_StatePvPm
pub fn stub_0x826210() -> ! {
    todo!("0x826210 __ZL14generic_readerP9lua_StatePvPm")
}

#[doc(alias = "luaK_nil(FuncState *,int,int)")]
#[doc(alias = "__Z8luaK_nilP9FuncStateii")]
// 0x826350 — __Z8luaK_nilP9FuncStateii
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x826350() -> ! {
    todo!("0x826350 __Z8luaK_nilP9FuncStateii")
}

#[doc(alias = "luaK_codeABC(FuncState *,OpCode,int,int,int)")]
#[doc(alias = "__Z12luaK_codeABCP9FuncState6OpCodeiii")]
// 0x8263c8 — __Z12luaK_codeABCP9FuncState6OpCodeiii
// type: int()
pub fn stub_0x8263c8() -> ! {
    todo!("0x8263c8 __Z12luaK_codeABCP9FuncState6OpCodeiii")
}

#[doc(alias = "luaK_jump(FuncState *)")]
#[doc(alias = "__Z9luaK_jumpP9FuncState")]
// 0x8263e4 — __Z9luaK_jumpP9FuncState
pub fn stub_0x8263e4() -> ! {
    todo!("0x8263e4 __Z9luaK_jumpP9FuncState")
}

#[doc(alias = "luaK_codeABx(FuncState *,OpCode,int,unsigned int)")]
#[doc(alias = "__Z12luaK_codeABxP9FuncState6OpCodeij")]
// 0x826418 — __Z12luaK_codeABxP9FuncState6OpCodeij
pub fn stub_0x826418() -> ! {
    todo!("0x826418 __Z12luaK_codeABxP9FuncState6OpCodeij")
}

#[doc(alias = "luaK_concat(FuncState *,int *,int)")]
#[doc(alias = "__Z11luaK_concatP9FuncStatePii")]
// 0x82642c — __Z11luaK_concatP9FuncStatePii
pub fn stub_0x82642c() -> ! {
    todo!("0x82642c __Z11luaK_concatP9FuncStatePii")
}

#[doc(alias = "luaK_ret(FuncState *,int,int)")]
#[doc(alias = "__Z8luaK_retP9FuncStateii")]
// 0x82646c — __Z8luaK_retP9FuncStateii
pub fn stub_0x82646c() -> ! {
    todo!("0x82646c __Z8luaK_retP9FuncStateii")
}

#[doc(alias = "luaK_getlabel(FuncState *)")]
#[doc(alias = "__Z13luaK_getlabelP9FuncState")]
// 0x826488 — __Z13luaK_getlabelP9FuncState
pub fn stub_0x826488() -> ! {
    todo!("0x826488 __Z13luaK_getlabelP9FuncState")
}

#[doc(alias = "luaK_patchlist(FuncState *,int,int)")]
#[doc(alias = "__Z14luaK_patchlistP9FuncStateii")]
// 0x826490 — __Z14luaK_patchlistP9FuncStateii
pub fn stub_0x826490() -> ! {
    todo!("0x826490 __Z14luaK_patchlistP9FuncStateii")
}

#[doc(alias = "luaK_patchtohere(FuncState *,int)")]
#[doc(alias = "__Z16luaK_patchtohereP9FuncStatei")]
// 0x8264c0 — __Z16luaK_patchtohereP9FuncStatei
pub fn stub_0x8264c0() -> ! {
    todo!("0x8264c0 __Z16luaK_patchtohereP9FuncStatei")
}

#[doc(alias = "luaK_checkstack(FuncState *,int)")]
#[doc(alias = "__Z15luaK_checkstackP9FuncStatei")]
// 0x82657c — __Z15luaK_checkstackP9FuncStatei
pub fn stub_0x82657c() -> ! {
    todo!("0x82657c __Z15luaK_checkstackP9FuncStatei")
}

#[doc(alias = "luaK_reserveregs(FuncState *,int)")]
#[doc(alias = "__Z16luaK_reserveregsP9FuncStatei")]
// 0x8265b0 — __Z16luaK_reserveregsP9FuncStatei
pub fn stub_0x8265b0() -> ! {
    todo!("0x8265b0 __Z16luaK_reserveregsP9FuncStatei")
}

#[doc(alias = "luaK_stringK(FuncState *,TString *)")]
#[doc(alias = "__Z12luaK_stringKP9FuncStateP7TString")]
// 0x8265c4 — __Z12luaK_stringKP9FuncStateP7TString
pub fn stub_0x8265c4() -> ! {
    todo!("0x8265c4 __Z12luaK_stringKP9FuncStateP7TString")
}

#[doc(alias = "addk(FuncState *,lua_TValue *,lua_TValue *)")]
#[doc(alias = "__ZL4addkP9FuncStateP10lua_TValueS2_")]
// 0x8265dc — __ZL4addkP9FuncStateP10lua_TValueS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x8265dc() -> ! {
    todo!("0x8265dc __ZL4addkP9FuncStateP10lua_TValueS2_")
}

#[doc(alias = "luaK_numberK(FuncState *,double)")]
#[doc(alias = "__Z12luaK_numberKP9FuncStated")]
// 0x8266d0 — __Z12luaK_numberKP9FuncStated
pub fn stub_0x8266d0() -> ! {
    todo!("0x8266d0 __Z12luaK_numberKP9FuncStated")
}

#[doc(alias = "luaK_setreturns(FuncState *,expdesc *,int)")]
#[doc(alias = "__Z15luaK_setreturnsP9FuncStateP7expdesci")]
// 0x8266ec — __Z15luaK_setreturnsP9FuncStateP7expdesci
pub fn stub_0x8266ec() -> ! {
    todo!("0x8266ec __Z15luaK_setreturnsP9FuncStateP7expdesci")
}

#[doc(alias = "luaK_setoneret(FuncState *,expdesc *)")]
#[doc(alias = "__Z14luaK_setoneretP9FuncStateP7expdesc")]
// 0x82675c — __Z14luaK_setoneretP9FuncStateP7expdesc
pub fn stub_0x82675c() -> ! {
    todo!("0x82675c __Z14luaK_setoneretP9FuncStateP7expdesc")
}

#[doc(alias = "luaK_dischargevars(FuncState *,expdesc *)")]
#[doc(alias = "__Z18luaK_dischargevarsP9FuncStateP7expdesc")]
// 0x82679c — __Z18luaK_dischargevarsP9FuncStateP7expdesc
// type: int(void)
pub fn stub_0x82679c() -> ! {
    todo!("0x82679c __Z18luaK_dischargevarsP9FuncStateP7expdesc")
}

#[doc(alias = "luaK_exp2nextreg(FuncState *,expdesc *)")]
#[doc(alias = "__Z16luaK_exp2nextregP9FuncStateP7expdesc")]
// 0x826838 — __Z16luaK_exp2nextregP9FuncStateP7expdesc
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x826838() -> ! {
    todo!("0x826838 __Z16luaK_exp2nextregP9FuncStateP7expdesc")
}

#[doc(alias = "luaK_exp2anyreg(FuncState *,expdesc *)")]
#[doc(alias = "__Z15luaK_exp2anyregP9FuncStateP7expdesc")]
// 0x826960 — __Z15luaK_exp2anyregP9FuncStateP7expdesc
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x826960() -> ! {
    todo!("0x826960 __Z15luaK_exp2anyregP9FuncStateP7expdesc")
}

#[doc(alias = "luaK_exp2val(FuncState *,expdesc *)")]
#[doc(alias = "__Z12luaK_exp2valP9FuncStateP7expdesc")]
// 0x82699c — __Z12luaK_exp2valP9FuncStateP7expdesc
pub fn stub_0x82699c() -> ! {
    todo!("0x82699c __Z12luaK_exp2valP9FuncStateP7expdesc")
}
