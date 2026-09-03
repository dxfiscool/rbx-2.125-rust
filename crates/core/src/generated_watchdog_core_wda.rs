//! generated_watchdog_core_wda — 120 core stubs (watchdog core-wda, global-deduped, EA-sorted gap filler)
//! Source: ida/export.json (85545 funcs) filtered core namespace (RBX|SystemAddress|RakNet|TaskScheduler|signals, boost::shared_ptr sanitized) NOT in /tmp/global_eas.txt, EA-sorted asc, next 120 gap.
//! Format: // 0xADDR — mangled + #[doc(alias="mangled")] + pub fn stub_0xADDR() { todo!("0xADDR") }
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes/backticks removed.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x2b8f38 — __ZNK3RBX10Reflection13EventDescBaseINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias="RBX::Reflection::EventDescBase<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias="__ZNK3RBX10Reflection13EventDescBaseINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0x2b8f38() { todo!("0x2b8f38 __ZNK3RBX10Reflection13EventDescBaseINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE") }

// 0x2b8f4c — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13disconnectAllEv
#[doc(alias="rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
#[doc(alias="__ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13disconnectAllEv")]
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x2b8f4c() { todo!("0x2b8f4c __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13disconnectAllEv") }

// 0x2b91b8 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsS5_RKNS_10shared_ptrINS1_8InstanceEEENS6_IS3_EENS_3argILi1EEENSC_ILi2EEENSC_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISI_T0_T1_T2_T3_EENSG_9list_av_4IT4_T5_T6_T7_E4typeEEEMSL_FSI_SM_SN_SO_ESR_SS_ST_SU_
#[doc(alias="boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias="__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsS5_RKNS_10shared_ptrINS1_8InstanceEEENS6_IS3_EENS_3argILi1EEENSC_ILi2EEENSC_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISI_T0_T1_T2_T3_EENSG_9list_av_4IT4_T5_T6_T7_E4typeEEEMSL_FSI_SM_SN_SO_ESR_SS_ST_SU_")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2b91b8() { todo!("0x2b91b8 __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsS5_RKNS_10shared_ptrINS1_8InstanceEEENS6_IS3_EENS_3argILi1EEENSC_ILi2EEENSC_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISI_T0_T1_T2_T3_EENSG_9list_av_4IT4_T5_T6_T7_E4typeEEEMSL_FSI_SM_SN_SO_ESR_SS_ST_SU_") }

// 0x2b92d4 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3ISsSsN5boost10shared_ptrINS_8InstanceEEEEEvRKT_RKT0_RKT1_
#[doc(alias="void RBX::Reflection::GenericSlotWrapper::execute3<std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>(std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias="__ZN3RBX10Reflection18GenericSlotWrapper8execute3ISsSsN5boost10shared_ptrINS_8InstanceEEEEEvRKT_RKT0_RKT1_")]
pub fn stub_0x2b92d4() { todo!("0x2b92d4 __ZN3RBX10Reflection18GenericSlotWrapper8execute3ISsSsN5boost10shared_ptrINS_8InstanceEEEEEvRKT_RKT0_RKT1_") }

// 0x2b9460 — __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE5clearEv
#[doc(alias="boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::clear(void)")]
#[doc(alias="__ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE5clearEv")]
pub fn stub_0x2b9460() { todo!("0x2b9460 __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE5clearEv") }

// 0x2b948c — __ZN5boost8functionIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSF_RKS4_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
#[doc(alias="__ZN5boost8functionIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSF_RKS4_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2b948c() { todo!("0x2b948c __ZN5boost8functionIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSF_RKS4_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE") }

// 0x2b9570 — __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
#[doc(alias="__ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2b9570() { todo!("0x2b9570 __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE") }

// 0x2b9658 — __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEvT_
#[doc(alias="void boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
#[doc(alias="__ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEvT_")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2b9658() { todo!("0x2b9658 __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEvT_") }

// 0x2b9750 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE
#[doc(alias="boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias="__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x2b9750() { todo!("0x2b9750 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE") }

// 0x2b976c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEvSsSsSE_E6invokeERNS1_15function_bufferESsSsSE_
#[doc(alias="boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias="__ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEvSsSsSE_E6invokeERNS1_15function_bufferESsSsSE_")]
pub fn stub_0x2b976c() { todo!("0x2b976c __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEvSsSsSE_E6invokeERNS1_15function_bufferESsSsSE_") }

// 0x2b978c — __ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias="bool boost::detail::function::basic_vtable3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias="__ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferE")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2b978c() { todo!("0x2b978c __ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferE") }

// 0x2b9874 — __ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias="bool boost::detail::function::basic_vtable3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias="__ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2b9874() { todo!("0x2b9874 __ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE") }

// 0x2b9958 — __ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias="void boost::detail::function::basic_vtable3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias="__ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x2b9958() { todo!("0x2b9958 __ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE") }

// 0x2b9a2c — __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKSsSI_RKNS3_INS4_8InstanceEEEEENS0_5list3IRSsSP_RSK_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias="void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<std::string &,std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&> &,boost::_bi::list3<std::string &,std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
#[doc(alias="__ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKSsSI_RKNS3_INS4_8InstanceEEEEENS0_5list3IRSsSP_RSK_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x2b9a2c() { todo!("0x2b9a2c __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKSsSI_RKNS3_INS4_8InstanceEEEEENS0_5list3IRSsSP_RSK_EEEEvNS0_4typeIvEERT_RT0_i") }

// 0x2b9a54 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias="boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias="__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0x2b9a54() { todo!("0x2b9a54 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE") }

// 0x2b9bac — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
#[doc(alias="rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&)")]
#[doc(alias="__ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_")]
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x2b9bac() { todo!("0x2b9bac __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_") }

// 0x2b9ca0 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE
#[doc(alias="rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
#[doc(alias="__ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE")]
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x2b9ca0() { todo!("0x2b9ca0 __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE") }

// 0x2b9eac — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSEPSA_
#[doc(alias="rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
#[doc(alias="__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSEPSA_")]
pub fn stub_0x2b9eac() { todo!("0x2b9eac __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSEPSA_") }

// 0x2b9ed0 — __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_EC2IPS9_EERKSC_T_
#[doc(alias="rbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>*)")]
#[doc(alias="__ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_EC2IPS9_EERKSC_T_")]
pub fn stub_0x2b9ed0() { todo!("0x2b9ed0 __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_EC2IPS9_EERKSC_T_") }

// 0x2b9fcc — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias="rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
#[doc(alias="__ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED1Ev")]
pub fn stub_0x2b9fcc() { todo!("0x2b9fcc __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED1Ev") }

// 0x2ba0dc — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
#[doc(alias="rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
#[doc(alias="__ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED0Ev")]
pub fn stub_0x2ba0dc() { todo!("0x2ba0dc __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED0Ev") }

// 0x2ba20c — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot10disconnectEv
#[doc(alias="rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")]
#[doc(alias="__ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot10disconnectEv")]
pub fn stub_0x2ba20c() { todo!("0x2ba20c __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot10disconnectEv") }

// 0x2ba31c — __ZNK3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot9connectedEv
#[doc(alias="rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
#[doc(alias="__ZNK3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot9connectedEv")]
pub fn stub_0x2ba31c() { todo!("0x2ba31c __ZNK3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot9connectedEv") }

// 0x2ba328 — __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_E4callESsSsS7_
#[doc(alias="rbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias="__ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_E4callESsSsS7_")]
pub fn stub_0x2ba328() { todo!("0x2ba328 __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_E4callESsSsS7_") }

// 0x2ba518 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_E4callESsSsS7_
#[doc(alias="non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias="__ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_E4callESsSsS7_")]
pub fn stub_0x2ba518() { todo!("0x2ba518 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_E4callESsSsS7_") }

// 0x2ba520 — __ZNK5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEclESsSsS4_
#[doc(alias="boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
#[doc(alias="__ZNK5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEclESsSsS4_")]
pub fn stub_0x2ba520() { todo!("0x2ba520 __ZNK5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEclESsSsS4_") }

// 0x2ba750 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE6removeEPNS8_4slotE
#[doc(alias="rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
#[doc(alias="__ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE6removeEPNS8_4slotE")]
// type: int __fastcall(int, char *)
pub fn stub_0x2ba750() { todo!("0x2ba750 __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE6removeEPNS8_4slotE") }

// 0x2ba840 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot22safe_static_init_mutexEv
#[doc(alias="rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
#[doc(alias="__ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x2ba840() { todo!("0x2ba840 __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot22safe_static_init_mutexEv") }

// 0x2ba844 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot24safe_static_do_get_mutexEv
#[doc(alias="rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias="__ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0x2ba844() { todo!("0x2ba844 __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot24safe_static_do_get_mutexEv") }

// 0x2ba934 — __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_ED1Ev
#[doc(alias="rbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
#[doc(alias="__ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_ED1Ev")]
pub fn stub_0x2ba934() { todo!("0x2ba934 __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_ED1Ev") }

// 0x2baa44 — __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_ED0Ev
#[doc(alias="rbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
#[doc(alias="__ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_ED0Ev")]
pub fn stub_0x2baa44() { todo!("0x2baa44 __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_ED0Ev") }

// 0x2bab74 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD1Ev
#[doc(alias="rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
#[doc(alias="__ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD1Ev")]
pub fn stub_0x2bab74() { todo!("0x2bab74 __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD1Ev") }

// 0x2baba0 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD0Ev
#[doc(alias="rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
#[doc(alias="__ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD0Ev")]
pub fn stub_0x2baba0() { todo!("0x2baba0 __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD0Ev") }

// 0x2bac74 — __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE13assign_to_ownERKS5_
#[doc(alias="boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to_own(boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>> const&)")]
#[doc(alias="__ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE13assign_to_ownERKS5_")]
pub fn stub_0x2bac74() { todo!("0x2bac74 __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE13assign_to_ownERKS5_") }

// 0x2bb394 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EEC2EMS2_FSA_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::ScriptContext::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias="__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EEC2EMS2_FSA_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
pub fn stub_0x2bb394() { todo!("0x2bb394 __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EEC2EMS2_FSA_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE") }

// 0x2bb498 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED0Ev
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias="__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED0Ev")]
pub fn stub_0x2bb498() { todo!("0x2bb498 __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED0Ev") }

// 0x2bb54c — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias="__ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x2bb54c() { todo!("0x2bb54c __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE") }

// 0x2bb570 — __ZN3RBX10Reflection11Call0HelperINS_13ScriptContextEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_
#[doc(alias="RBX::Reflection::Call0Helper<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::ScriptContext::*)(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::ScriptContext*,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::ScriptContext::*)(void),RBX::Reflection::Variant&)")]
#[doc(alias="__ZN3RBX10Reflection11Call0HelperINS_13ScriptContextEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_")]
pub fn stub_0x2bb570() { todo!("0x2bb570 __ZN3RBX10Reflection11Call0HelperINS_13ScriptContextEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_") }

// 0x2bb658 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKSt6vectorINS1_10Reflection7VariantESaIS9_EEEEEERS3_RKT_
#[doc(alias="rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const&)")]
#[doc(alias="__ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKSt6vectorINS1_10Reflection7VariantESaIS9_EEEEEERS3_RKT_")]
pub fn stub_0x2bb658() { todo!("0x2bb658 __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKSt6vectorINS1_10Reflection7VariantESaIS9_EEEEEERS3_RKT_") }

// 0x2bb6c0 — __ZN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS4_EEEaSERKS8_
#[doc(alias="rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>::operator=(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const&)")]
#[doc(alias="__ZN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS4_EEEaSERKS8_")]
pub fn stub_0x2bb6c0() { todo!("0x2bb6c0 __ZN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS4_EEEaSERKS8_") }

// 0x2bb6f8 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE14construct_funcEPKcPc
#[doc(alias="rbx::implementation::typed_holder<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::construct_func(char const*,char *)")]
#[doc(alias="__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE14construct_funcEPKcPc")]
pub fn stub_0x2bb6f8() { todo!("0x2bb6f8 __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE14construct_funcEPKcPc") }

// 0x2bb720 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EEC2EMS2_FS7_bEPKcSD_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ScriptContext::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias="__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EEC2EMS2_FS7_bEPKcSD_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x2bb720() { todo!("0x2bb720 __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EEC2EMS2_FS7_bEPKcSD_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE") }

// 0x2bb8cc — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias="__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x2bb8cc() { todo!("0x2bb8cc __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EE16declareSignatureEPKcNS0_7VariantE") }

// 0x2bb8fc — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EED0Ev
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::~BoundFuncDesc()")]
#[doc(alias="__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EED0Ev")]
pub fn stub_0x2bb8fc() { todo!("0x2bb8fc __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EED0Ev") }

// 0x2bb9d0 — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias="__ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x2bb9d0() { todo!("0x2bb9d0 __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE") }

// 0x2bba10 — __ZN3RBX10Reflection11Call1HelperINS_13ScriptContextEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEbEbS7_E4callEPS2_S9_RNS0_7VariantERKb
#[doc(alias="RBX::Reflection::Call1Helper<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ScriptContext::*)(bool),bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::call(RBX::ScriptContext*,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ScriptContext::*)(bool),RBX::Reflection::Variant &,bool const&)")]
#[doc(alias="__ZN3RBX10Reflection11Call1HelperINS_13ScriptContextEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEbEbS7_E4callEPS2_S9_RNS0_7VariantERKb")]
pub fn stub_0x2bba10() { todo!("0x2bba10 __ZN3RBX10Reflection11Call1HelperINS_13ScriptContextEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEbEbS7_E4callEPS2_S9_RNS0_7VariantERKb") }

// 0x2bc2a8 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EEC2EMS2_FviS6_SsEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::BoundFuncDesc(void (RBX::ScriptContext::*)(int,rbx_core::SharedPtr<RBX::Instance>,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias="__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EEC2EMS2_FviS6_SsEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x2bc2a8() { todo!("0x2bc2a8 __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EEC2EMS2_FviS6_SsEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE") }

// 0x2bc4c4 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias="__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_")]
pub fn stub_0x2bc4c4() { todo!("0x2bc4c4 __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_") }

// 0x2bc530 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EED0Ev
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::~BoundFuncDesc()")]
#[doc(alias="__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EED0Ev")]
pub fn stub_0x2bc530() { todo!("0x2bc530 __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EED0Ev") }

// 0x2bc658 — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias="__ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x2bc658() { todo!("0x2bc658 __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE") }

// 0x2bc7f0 — __ZN3RBX10Reflection11Call3HelperINS_13ScriptContextEMS2_FviN5boost10shared_ptrINS_8InstanceEEESsEiS6_SsvE4callEPS2_S8_RNS0_7VariantERKiRKS6_RKSs
#[doc(alias="RBX::Reflection::Call3Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(int,rbx_core::SharedPtr<RBX::Instance>,std::string),int,rbx_core::SharedPtr<RBX::Instance>,std::string,void>::call(RBX::ScriptContext*,void (RBX::ScriptContext::*)(int,rbx_core::SharedPtr<RBX::Instance>,std::string),RBX::Reflection::Variant &,int const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&)")]
#[doc(alias="__ZN3RBX10Reflection11Call3HelperINS_13ScriptContextEMS2_FviN5boost10shared_ptrINS_8InstanceEEESsEiS6_SsvE4callEPS2_S8_RNS0_7VariantERKiRKS6_RKSs")]
pub fn stub_0x2bc7f0() { todo!("0x2bc7f0 __ZN3RBX10Reflection11Call3HelperINS_13ScriptContextEMS2_FviN5boost10shared_ptrINS_8InstanceEEESsEiS6_SsvE4callEPS2_S8_RNS0_7VariantERKiRKS6_RKSs") }

// 0x2bc988 — __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrINS_8InstanceEEELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrIS7_EEPNS3_10disable_ifINS3_7is_sameIS7_NS4_IKNS0_5TupleEEEEEvE4typeE
#[doc(alias="rbx_core::SharedPtr<RBX::Instance> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<RBX::Instance>,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<RBX::Instance>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias="__ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrINS_8InstanceEEELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrIS7_EEPNS3_10disable_ifINS3_7is_sameIS7_NS4_IKNS0_5TupleEEEEEvE4typeE")]
// type: int __fastcall(int, int, int)
pub fn stub_0x2bc988() { todo!("0x2bc988 __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrINS_8InstanceEEELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrIS7_EEPNS3_10disable_ifINS3_7is_sameIS7_NS4_IKNS0_5TupleEEEEEvE4typeE") }

// 0x2bcba0 — __ZN3RBX10Reflection9ArgHelper10try_objectILi2ENS_8InstanceEEEbRNS0_18FunctionDescriptor9ArgumentsERN5boost10shared_ptrIT0_EEPNS7_9enable_ifINS7_10is_base_ofINS0_13DescribedBaseES9_EEvE4typeE
#[doc(alias="bool RBX::Reflection::ArgHelper::try_object<2,RBX::Instance>(RBX::Reflection::FunctionDescriptor::Arguments &,rbx_core::SharedPtr<RBX::Instance> &,boost::enable_if<boost::is_base_of<RBX::Reflection::DescribedBase,RBX::Instance>,void>::type *)")]
#[doc(alias="__ZN3RBX10Reflection9ArgHelper10try_objectILi2ENS_8InstanceEEEbRNS0_18FunctionDescriptor9ArgumentsERN5boost10shared_ptrIT0_EEPNS7_9enable_ifINS7_10is_base_ofINS0_13DescribedBaseES9_EEvE4typeE")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2bcba0() { todo!("0x2bcba0 __ZN3RBX10Reflection9ArgHelper10try_objectILi2ENS_8InstanceEEEbRNS0_18FunctionDescriptor9ArgumentsERN5boost10shared_ptrIT0_EEPNS7_9enable_ifINS7_10is_base_ofINS0_13DescribedBaseES9_EEvE4typeE") }

// 0x2bceb4 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS2_FvSsS6_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::ScriptContext::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias="__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS2_FvSsS6_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x2bceb4() { todo!("0x2bceb4 __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS2_FvSsS6_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE") }

// 0x2bd080 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias="__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_")]
pub fn stub_0x2bd080() { todo!("0x2bd080 __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_") }

// 0x2bd0cc — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
#[doc(alias="__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev")]
pub fn stub_0x2bd0cc() { todo!("0x2bd0cc __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev") }

// 0x2bd1e4 — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias="RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias="__ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x2bd1e4() { todo!("0x2bd1e4 __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE") }

// 0x2bd368 — __ZN3RBX10Reflection11Call2HelperINS_13ScriptContextEMS2_FvSsN5boost10shared_ptrINS_8InstanceEEEESsS6_vE4callEPS2_S8_RNS0_7VariantERKSsRKS6_
#[doc(alias="RBX::Reflection::Call2Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),std::string,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::ScriptContext*,void (RBX::ScriptContext::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias="__ZN3RBX10Reflection11Call2HelperINS_13ScriptContextEMS2_FvSsN5boost10shared_ptrINS_8InstanceEEEESsS6_vE4callEPS2_S8_RNS0_7VariantERKSsRKS6_")]
// type: int __fastcall(int, int, int, int, std::string *, int)
pub fn stub_0x2bd368() { todo!("0x2bd368 __ZN3RBX10Reflection11Call2HelperINS_13ScriptContextEMS2_FvSsN5boost10shared_ptrINS_8InstanceEEEESsS6_vE4callEPS2_S8_RNS0_7VariantERKSsRKS6_") }

// 0x2bd4f0 — __ZN3RBX10Reflection9ArgHelper6getArgISsLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias="std::string RBX::Reflection::ArgHelper::getArg<std::string,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias="__ZN3RBX10Reflection9ArgHelper6getArgISsLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0x2bd4f0() { todo!("0x2bd4f0 __ZN3RBX10Reflection9ArgHelper6getArgISsLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE") }

// 0x2be070 — __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias="RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias="__ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x2be070() { todo!("0x2be070 __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE") }

// 0x2be2cc — __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias="RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")]
#[doc(alias="__ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_ED0Ev")]
pub fn stub_0x2be2cc() { todo!("0x2be2cc __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_ED0Ev") }

// 0x2be380 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias="RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias="__ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
pub fn stub_0x2be380() { todo!("0x2be380 __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE") }

// 0x2be4d4 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias="RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias="__ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
pub fn stub_0x2be4d4() { todo!("0x2be4d4 __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE") }

// 0x2be728 — __ZNK3RBX10Reflection13EventDescBaseINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias="RBX::Reflection::EventDescBase<RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias="__ZNK3RBX10Reflection13EventDescBaseINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0x2be728() { todo!("0x2be728 __ZNK3RBX10Reflection13EventDescBaseINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE") }

// 0x2be73c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13disconnectAllEv
#[doc(alias="rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
#[doc(alias="__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13disconnectAllEv")]
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x2be73c() { todo!("0x2be73c __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13disconnectAllEv") }

// 0x2be8b4 — __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EEclES6_SsS6_
#[doc(alias="rbx::signals::signal_with_args<3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias="__ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EEclES6_SsS6_")]
// type: void __fastcall(_DWORD *, shared_count *, std::string *, shared_count *, int, int, boost::detail::sp_counted_base *, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x2be8b4() { todo!("0x2be8b4 __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EEclES6_SsS6_") }

// 0x2beb34 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4nextERNS2_13intrusive_ptrINS8_4slotEEE
#[doc(alias="rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")]
#[doc(alias="__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4nextERNS2_13intrusive_ptrINS8_4slotEEE")]
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x2beb34() { todo!("0x2beb34 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4nextERNS2_13intrusive_ptrINS8_4slotEEE") }

// 0x2bec94 — __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE8fireItemEPNS0_6signalIS7_E4slotES6_SsS6_
#[doc(alias="rbx::signals::signal_with_args<3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias="__ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE8fireItemEPNS0_6signalIS7_E4slotES6_SsS6_")]
pub fn stub_0x2bec94() { todo!("0x2bec94 __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE8fireItemEPNS0_6signalIS7_E4slotES6_SsS6_") }

// 0x2bee54 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE8on_errorERSt9exception
#[doc(alias="rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")]
#[doc(alias="__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE8on_errorERSt9exception")]
pub fn stub_0x2bee54() { todo!("0x2bee54 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE8on_errorERSt9exception") }

// 0x2bee7c — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKSsS8_NS4_IS3_EENS_3argILi1EEENSC_ILi2EEENSC_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISI_T0_T1_T2_T3_EENSG_9list_av_4IT4_T5_T6_T7_E4typeEEEMSL_FSI_SM_SN_SO_ESR_SS_ST_SU_
#[doc(alias="boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias="__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKSsS8_NS4_IS3_EENS_3argILi1EEENSC_ILi2EEENSC_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISI_T0_T1_T2_T3_EENSG_9list_av_4IT4_T5_T6_T7_E4typeEEEMSL_FSI_SM_SN_SO_ESR_SS_ST_SU_")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2bee7c() { todo!("0x2bee7c __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKSsS8_NS4_IS3_EENS_3argILi1EEENSC_ILi2EEENSC_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISI_T0_T1_T2_T3_EENSG_9list_av_4IT4_T5_T6_T7_E4typeEEEMSL_FSI_SM_SN_SO_ESR_SS_ST_SU_") }

// 0x2bef98 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3IN5boost10shared_ptrINS_8InstanceEEESsS6_EEvRKT_RKT0_RKT1_
#[doc(alias="void RBX::Reflection::GenericSlotWrapper::execute3<rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias="__ZN3RBX10Reflection18GenericSlotWrapper8execute3IN5boost10shared_ptrINS_8InstanceEEESsS6_EEvRKT_RKT0_RKT1_")]
pub fn stub_0x2bef98() { todo!("0x2bef98 __ZN3RBX10Reflection18GenericSlotWrapper8execute3IN5boost10shared_ptrINS_8InstanceEEESsS6_EEvRKT_RKT0_RKT1_") }

// 0x2bf124 — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E5clearEv
#[doc(alias="boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::clear(void)")]
#[doc(alias="__ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E5clearEv")]
pub fn stub_0x2bf124() { todo!("0x2bf124 __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E5clearEv") }

// 0x2bf150 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSF_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
#[doc(alias="__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSF_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2bf150() { todo!("0x2bf150 __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSF_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE") }

// 0x2bf234 — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
#[doc(alias="__ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2bf234() { todo!("0x2bf234 __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE") }

// 0x2bf31c — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEvT_
#[doc(alias="void boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
#[doc(alias="__ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEvT_")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2bf31c() { todo!("0x2bf31c __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEvT_") }

// 0x2bf414 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE
#[doc(alias="boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias="__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x2bf414() { todo!("0x2bf414 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE") }

// 0x2bf430 — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEvSC_SsSC_E6invokeERNS1_15function_bufferESC_SsSC_
#[doc(alias="boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias="__ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEvSC_SsSC_E6invokeERNS1_15function_bufferESC_SsSC_")]
pub fn stub_0x2bf430() { todo!("0x2bf430 __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEvSC_SsSC_E6invokeERNS1_15function_bufferESC_SsSC_") }

// 0x2bf450 — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias="bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias="__ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferE")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2bf450() { todo!("0x2bf450 __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferE") }

// 0x2bf538 — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias="bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias="__ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2bf538() { todo!("0x2bf538 __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE") }

// 0x2bf61c — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias="void boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias="__ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x2bf61c() { todo!("0x2bf61c __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE") }

// 0x2bf6f0 — __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKNS3_INS4_8InstanceEEERKSsSK_EENS0_5list3IRSI_RSsSP_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias="void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&> &,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
#[doc(alias="__ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKNS3_INS4_8InstanceEEERKSsSK_EENS0_5list3IRSI_RSsSP_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x2bf6f0() { todo!("0x2bf6f0 __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKNS3_INS4_8InstanceEEERKSsSK_EENS0_5list3IRSI_RSsSP_EEEEvNS0_4typeIvEERT_RT0_i") }

// 0x2bf718 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias="boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias="__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0x2bf718() { todo!("0x2bf718 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE") }

// 0x2bf870 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
#[doc(alias="rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&)")]
#[doc(alias="__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_")]
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x2bf870() { todo!("0x2bf870 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_") }

// 0x2bf964 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_EC2IPS9_EERKSC_T_
#[doc(alias="rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>*)")]
#[doc(alias="__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_EC2IPS9_EERKSC_T_")]
pub fn stub_0x2bf964() { todo!("0x2bf964 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_EC2IPS9_EERKSC_T_") }

// 0x2bfa60 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias="rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
#[doc(alias="__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_8functionIS7_EEED1Ev")]
pub fn stub_0x2bfa60() { todo!("0x2bfa60 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_8functionIS7_EEED1Ev") }

// 0x2bfb70 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_8functionIS7_EEED0Ev
#[doc(alias="rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
#[doc(alias="__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_8functionIS7_EEED0Ev")]
// type: void __fastcall(_DWORD *)
pub fn stub_0x2bfb70() { todo!("0x2bfb70 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_8functionIS7_EEED0Ev") }

// 0x2bfca0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_E4callES7_SsS7_
#[doc(alias="rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias="__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_E4callES7_SsS7_")]
pub fn stub_0x2bfca0() { todo!("0x2bfca0 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_E4callES7_SsS7_") }

// 0x2bfe48 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_E4callES7_SsS7_
#[doc(alias="non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias="__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_E4callES7_SsS7_")]
pub fn stub_0x2bfe48() { todo!("0x2bfe48 __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_E4callES7_SsS7_") }

// 0x2bfe50 — __ZNK5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EclES4_SsS4_
#[doc(alias="boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
#[doc(alias="__ZNK5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EclES4_SsS4_")]
pub fn stub_0x2bfe50() { todo!("0x2bfe50 __ZNK5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EclES4_SsS4_") }

// 0x2c0038 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_ED1Ev
#[doc(alias="rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
#[doc(alias="__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_ED1Ev")]
pub fn stub_0x2c0038() { todo!("0x2c0038 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_ED1Ev") }

// 0x2c0148 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_ED0Ev
#[doc(alias="rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
#[doc(alias="__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_ED0Ev")]
pub fn stub_0x2c0148() { todo!("0x2c0148 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_ED0Ev") }

// 0x2c0278 — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E13assign_to_ownERKS5_
#[doc(alias="boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to_own(boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>> const&)")]
#[doc(alias="__ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E13assign_to_ownERKS5_")]
pub fn stub_0x2c0278() { todo!("0x2c0278 __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E13assign_to_ownERKS5_") }

// 0x2c02a8 — __ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias="boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance,200u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias="__ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_0x2c02a8() { todo!("0x2c02a8 __ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv") }

// 0x2c190c — __ZN5boost13intrusive_ptrIN3RBX3Lua6detail13LiveThreadRefEEaSERKS5_
#[doc(alias="rbx_core::SharedPtr<RBX::Lua::detail::LiveThreadRef>::operator=(rbx_core::SharedPtr<RBX::Lua::detail::LiveThreadRef> const&)")]
#[doc(alias="__ZN5boost13intrusive_ptrIN3RBX3Lua6detail13LiveThreadRefEEaSERKS5_")]
pub fn stub_0x2c190c() { todo!("0x2c190c __ZN5boost13intrusive_ptrIN3RBX3Lua6detail13LiveThreadRefEEaSERKS5_") }

// 0x2c1930 — __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua6detail13LiveThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE
#[doc(alias="void rbx_core::SharedPtr_release<RBX::Lua::detail::LiveThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::detail::LiveThreadRef,int,0> const*)")]
#[doc(alias="__ZN5boost21intrusive_ptr_releaseIN3RBX3Lua6detail13LiveThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE")]
pub fn stub_0x2c1930() { todo!("0x2c1930 __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua6detail13LiveThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE") }

// 0x2c1af8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12LuaStatsItemEPNS_13ScriptContextEEEN5boost10shared_ptrIT_EET0_
#[doc(alias="rbx_core::SharedPtr<RBX::LuaStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::LuaStatsItem,RBX::ScriptContext *>(RBX::ScriptContext *)")]
#[doc(alias="__ZN3RBX9CreatableINS_8InstanceEE6createINS_12LuaStatsItemEPNS_13ScriptContextEEEN5boost10shared_ptrIT_EET0_")]
pub fn stub_0x2c1af8() { todo!("0x2c1af8 __ZN3RBX9CreatableINS_8InstanceEE6createINS_12LuaStatsItemEPNS_13ScriptContextEEEN5boost10shared_ptrIT_EET0_") }

// 0x2c2270 — __ZN5boost10shared_ptrIN3RBX12LuaStatsItemEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias="rbx_core::SharedPtr<RBX::LuaStatsItem>::shared_ptr<RBX::LuaStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias="__ZN5boost10shared_ptrIN3RBX12LuaStatsItemEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x2c2270() { todo!("0x2c2270 __ZN5boost10shared_ptrIN3RBX12LuaStatsItemEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_") }

// 0x2c2338 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LuaStatsItemES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias="void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaStatsItem,RBX::LuaStatsItem>(rbx_core::SharedPtr<RBX::LuaStatsItem> const*,RBX::LuaStatsItem *)const")]
#[doc(alias="__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LuaStatsItemES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x2c2338() { todo!("0x2c2338 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LuaStatsItemES6_EEvPKNS_10shared_ptrIT_EEPT0_") }

// 0x2c2424 — __ZN5boost6detail12shared_countC2IPN3RBX12LuaStatsItemENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias="boost::detail::shared_count::shared_count<RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias="__ZN5boost6detail12shared_countC2IPN3RBX12LuaStatsItemENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x2c2424() { todo!("0x2c2424 __ZN5boost6detail12shared_countC2IPN3RBX12LuaStatsItemENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_") }

// 0x2c252c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias="boost::detail::sp_counted_impl_pd<RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias="__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x2c252c() { todo!("0x2c252c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev") }

// 0x2c2530 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias="boost::detail::sp_counted_impl_pd<RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias="__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x2c2530() { todo!("0x2c2530 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev") }

// 0x2c2534 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias="boost::detail::sp_counted_impl_pd<RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias="__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x2c2534() { todo!("0x2c2534 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv") }

// 0x2c2554 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias="boost::detail::sp_counted_impl_pd<RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias="__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x2c2554() { todo!("0x2c2554 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info") }

// 0x2c256c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias="boost::detail::sp_counted_impl_pd<RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias="__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x2c256c() { todo!("0x2c256c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LuaStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv") }

// 0x2c2d40 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterIN3RBX10Reflection5TupleEEES6_EEPT_RKNS_10shared_ptrIT0_EE
#[doc(alias="rbx::detail::sp_ms_deleter<RBX::Reflection::Tuple> * boost::get_deleter<rbx::detail::sp_ms_deleter<RBX::Reflection::Tuple>,RBX::Reflection::Tuple>(rbx_core::SharedPtr<RBX::Reflection::Tuple> const&)")]
#[doc(alias="__ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterIN3RBX10Reflection5TupleEEES6_EEPT_RKNS_10shared_ptrIT0_EE")]
pub fn stub_0x2c2d40() { todo!("0x2c2d40 __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterIN3RBX10Reflection5TupleEEES6_EEPT_RKNS_10shared_ptrIT0_EE") }

// 0x2c2da0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection5TupleEN3rbx6detail13sp_ms_deleterIS4_EEED1Ev
#[doc(alias="boost::detail::sp_counted_impl_pd<RBX::Reflection::Tuple *,rbx::detail::sp_ms_deleter<RBX::Reflection::Tuple>>::~sp_counted_impl_pd()")]
#[doc(alias="__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection5TupleEN3rbx6detail13sp_ms_deleterIS4_EEED1Ev")]
pub fn stub_0x2c2da0() { todo!("0x2c2da0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection5TupleEN3rbx6detail13sp_ms_deleterIS4_EEED1Ev") }

// 0x2c2dd0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection5TupleEN3rbx6detail13sp_ms_deleterIS4_EEE11get_deleterERKSt9type_info
#[doc(alias="boost::detail::sp_counted_impl_pd<RBX::Reflection::Tuple *,rbx::detail::sp_ms_deleter<RBX::Reflection::Tuple>>::get_deleter(std::type_info const&)")]
#[doc(alias="__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection5TupleEN3rbx6detail13sp_ms_deleterIS4_EEE11get_deleterERKSt9type_info")]
pub fn stub_0x2c2dd0() { todo!("0x2c2dd0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection5TupleEN3rbx6detail13sp_ms_deleterIS4_EEE11get_deleterERKSt9type_info") }

// 0x2c2de8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection5TupleEN3rbx6detail13sp_ms_deleterIS4_EEE19get_untyped_deleterEv
#[doc(alias="boost::detail::sp_counted_impl_pd<RBX::Reflection::Tuple *,rbx::detail::sp_ms_deleter<RBX::Reflection::Tuple>>::get_untyped_deleter(void)")]
#[doc(alias="__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection5TupleEN3rbx6detail13sp_ms_deleterIS4_EEE19get_untyped_deleterEv")]
pub fn stub_0x2c2de8() { todo!("0x2c2de8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection5TupleEN3rbx6detail13sp_ms_deleterIS4_EEE19get_untyped_deleterEv") }

// 0x2c59b0 — __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE11on_tostringERKS6_P9lua_State
#[doc(alias="RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_tostring(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node> const&,lua_State *)")]
#[doc(alias="__ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE11on_tostringERKS6_P9lua_State")]
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x2c59b0() { todo!("0x2c59b0 __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE11on_tostringERKS6_P9lua_State") }

// 0x2c59cc — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE11on_tostringERKSB_P9lua_State
#[doc(alias="RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::on_tostring(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> const&,lua_State *)")]
#[doc(alias="__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE11on_tostringERKSB_P9lua_State")]
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x2c59cc() { todo!("0x2c59cc __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE11on_tostringERKSB_P9lua_State") }

// 0x2c59e8 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE11on_tostringERKSF_P9lua_State
#[doc(alias="RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_tostring(rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,lua_State *)")]
#[doc(alias="__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE11on_tostringERKSF_P9lua_State")]
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x2c59e8() { todo!("0x2c59e8 __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE11on_tostringERKSF_P9lua_State") }

// 0x2c5ec8 — __ZN5boost13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEaSERKS4_
#[doc(alias="rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>::operator=(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef> const&)")]
#[doc(alias="__ZN5boost13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEaSERKS4_")]
pub fn stub_0x2c5ec8() { todo!("0x2c5ec8 __ZN5boost13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEaSERKS4_") }

// 0x2c7a20 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIiEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias="boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<int> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias="__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIiEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x2c7a20() { todo!("0x2c7a20 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIiEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info") }

// 0x2c7a38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIiEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias="boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<int> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias="__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIiEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x2c7a38() { todo!("0x2c7a38 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIiEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv") }

// 0x2c7a3c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5Stats14TypedStatsItemIbEEPKbEEN5boost10shared_ptrIT_EET0_
#[doc(alias="rbx_core::SharedPtr<RBX::Stats::TypedStatsItem<bool>> RBX::Creatable<RBX::Instance>::create<RBX::Stats::TypedStatsItem<bool>,bool const*>(bool const*)")]
#[doc(alias="__ZN3RBX9CreatableINS_8InstanceEE6createINS_5Stats14TypedStatsItemIbEEPKbEEN5boost10shared_ptrIT_EET0_")]
pub fn stub_0x2c7a3c() { todo!("0x2c7a3c __ZN3RBX9CreatableINS_8InstanceEE6createINS_5Stats14TypedStatsItemIbEEPKbEEN5boost10shared_ptrIT_EET0_") }

// 0x2c8094 — __ZN5boost10shared_ptrIN3RBX5Stats14TypedStatsItemIbEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias="rbx_core::SharedPtr<RBX::Stats::TypedStatsItem<bool>>::shared_ptr<RBX::Stats::TypedStatsItem<bool>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<bool> *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias="__ZN5boost10shared_ptrIN3RBX5Stats14TypedStatsItemIbEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x2c8094() { todo!("0x2c8094 __ZN5boost10shared_ptrIN3RBX5Stats14TypedStatsItemIbEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_") }

// 0x2c815c — __ZN5boost6detail12shared_countC2IPN3RBX5Stats14TypedStatsItemIbEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias="boost::detail::shared_count::shared_count<RBX::Stats::TypedStatsItem<bool> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<bool> *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias="__ZN5boost6detail12shared_countC2IPN3RBX5Stats14TypedStatsItemIbEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x2c815c() { todo!("0x2c815c __ZN5boost6detail12shared_countC2IPN3RBX5Stats14TypedStatsItemIbEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_") }

// 0x2c8268 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIbEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias="boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<bool> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias="__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIbEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x2c8268() { todo!("0x2c8268 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIbEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev") }

// 0x2c9f54 — __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE8on_indexERKS6_PKcP9lua_State
#[doc(alias="RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_index(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node> const&,char const*,lua_State *)")]
#[doc(alias="__ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE8on_indexERKS6_PKcP9lua_State")]
pub fn stub_0x2c9f54() { todo!("0x2c9f54 __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE8on_indexERKS6_PKcP9lua_State") }

// 0x2ca00c — __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE11on_newindexERS6_PKcP9lua_State
#[doc(alias="RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_newindex(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>&,char const*,lua_State *)")]
#[doc(alias="__ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE11on_newindexERS6_PKcP9lua_State")]
pub fn stub_0x2ca00c() { todo!("0x2ca00c __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE11on_newindexERS6_PKcP9lua_State") }
