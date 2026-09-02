//! core wdog_1788370647 — 100 core stubs EA-sorted asc gap filler distinct not yet in any crate.
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 uncovered core namespace (RBX::Core, boost replacements, atomic, TaskScheduler, no RBX deps).
//! Range: 0x681fd8..0x7e03a8 | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR() { todo!("0xADDR") }
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Tool::setTimerCallback(rbx_core::WeakPtr<RBX::Network::Player>)")]
// 0x681fd8 — __ZN3RBX4Tool16setTimerCallbackEN5boost8weak_ptrINS_7Network6PlayerEEE
// was: RBX::Tool::setTimerCallback(rbx_core::WeakPtr<RBX::Network::Player>)
pub fn stub_0x681fd8() -> ! {
    todo!("0x681fd8 __ZN3RBX4Tool16setTimerCallbackEN5boost8weak_ptrINS_7Network6PlayerEEE")
}

#[doc(alias = "RBX::Tool::moveOtherToolsToBackpack(rbx_core::WeakPtr<RBX::Network::Player>)")]
// 0x682190 — __ZN3RBX4Tool24moveOtherToolsToBackpackEN5boost8weak_ptrINS_7Network6PlayerEEE
// was: RBX::Tool::moveOtherToolsToBackpack(rbx_core::WeakPtr<RBX::Network::Player>)
pub fn stub_0x682190() -> ! {
    todo!("0x682190 __ZN3RBX4Tool24moveOtherToolsToBackpackEN5boost8weak_ptrINS_7Network6PlayerEEE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list_av_2<RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>::type> boost::bind<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>,RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>(void (RBX::Tool::*)(rbx_core::WeakPtr<RBX::Network::Player>),RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)")]
// 0x682e2c — __ZN5boost4bindIvN3RBX4ToolENS_8weak_ptrINS1_7Network6PlayerEEEPS2_S6_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISA_T0_T1_EENS8_9list_av_2IT2_T3_E4typeEEEMSD_FSA_SE_ESH_SI_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list_av_2<RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>::type> boost::bind<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>,RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>(voi
pub fn stub_0x682e2c() -> ! {
    todo!("0x682e2c __ZN5boost4bindIvN3RBX4ToolENS_8weak_ptrINS1_7Network6PlayerEEEPS2_S6_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISA_T0_T1_EENS8_9list_av_2IT2_T3_E4typeEEEMSD_FSA_SE_ESH_SI_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player>::shared_ptr<RBX::Network::Player>(rbx_core::WeakPtr<RBX::Network::Player> const&,boost::detail::sp_nothrow_tag)")]
// 0x683ee0 — __ZN5boost10shared_ptrIN3RBX7Network6PlayerEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: rbx_core::SharedPtr<RBX::Network::Player>::shared_ptr<RBX::Network::Player>(rbx_core::WeakPtr<RBX::Network::Player> const&,boost::detail::sp_nothrow_tag)
pub fn stub_0x683ee0() -> ! {
    todo!("0x683ee0 __ZN5boost10shared_ptrIN3RBX7Network6PlayerEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x683f5c — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_0x683f5c() -> ! {
    todo!("0x683f5c __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x684044 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
pub fn stub_0x684044() -> ! {
    todo!("0x684044 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>)")]
// 0x684130 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Too
pub fn stub_0x684130() -> ! {
    todo!("0x684130 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x68422c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer con
pub fn stub_0x68422c() -> ! {
    todo!("0x68422c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x684248 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::fun
pub fn stub_0x684248() -> ! {
    todo!("0x684248 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &)const")]
// 0x684260 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_
pub fn stub_0x684260() -> ! {
    todo!("0x684260 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x68434c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_
pub fn stub_0x68434c() -> ! {
    todo!("0x68434c __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x684434 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boo
pub fn stub_0x684434() -> ! {
    todo!("0x684434 __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::operator()<boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>> &,boost::_bi::list0 &,int)")]
// 0x68450c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEclINS_4_mfi3mf1IvS4_SA_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::operator()<boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Netw
pub fn stub_0x68450c() -> ! {
    todo!("0x68450c __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEclINS_4_mfi3mf1IvS4_SA_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>::operator()(RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)const")]
// 0x6845e0 — __ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS2_7Network6PlayerEEEEclEPS3_S7_
// was: boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>::operator()(RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)const
pub fn stub_0x6845e0() -> ! {
    todo!("0x6845e0 __ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS2_7Network6PlayerEEEEclEPS3_S7_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x6846c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer co
pub fn stub_0x6846c8() -> ! {
    todo!("0x6846c8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::list2(boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>)")]
// 0x684824 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEC2ES6_SB_
// was: boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::list2(boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>)
pub fn stub_0x684824() -> ! {
    todo!("0x684824 __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEC2ES6_SB_")
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS8_5list2INS8_5valueINS1_ISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x6850ac — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS8_5list2INS8_5valueINS1_ISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_0x6850ac() -> ! {
    todo!("0x6850ac __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS8_5list2INS8_5valueINS1_ISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS7_5list2INS7_5valueINS1_ISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x685190 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS7_5list2INS7_5valueINS1_ISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
pub fn stub_0x685190() -> ! {
    todo!("0x685190 __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS7_5list2INS7_5valueINS1_ISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIiLZNS_9sIntValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69dc6c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIiLZNS_9sIntValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_0x69dc6c() -> ! {
    todo!("0x69dc6c __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIiLZNS_9sIntValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIdLZNS_12sDoubleValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69dd1c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIdLZNS_12sDoubleValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_0x69dd1c() -> ! {
    todo!("0x69dd1c __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIdLZNS_12sDoubleValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIbLZNS_10sBoolValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69ddcc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIbLZNS_10sBoolValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_0x69ddcc() -> ! {
    todo!("0x69ddcc __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIbLZNS_10sBoolValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69de7c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_0x69de7c() -> ! {
    todo!("0x69de7c __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69df2c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_0x69df2c() -> ! {
    todo!("0x69df2c __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69dfdc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_0x69dfdc() -> ! {
    todo!("0x69dfdc __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_10BrickColorELZNS_16sBrickColorValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69e08c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_10BrickColorELZNS_16sBrickColorValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_0x69e08c() -> ! {
    todo!("0x69e08c __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_10BrickColorELZNS_16sBrickColorValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_6RbxRayELZNS_9sRayValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69e13c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_6RbxRayELZNS_9sRayValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_0x69e13c() -> ! {
    todo!("0x69e13c __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueINS_6RbxRayELZNS_9sRayValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIiLZNS_20sIntConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69e1ec — __ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIiLZNS_20sIntConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_0x69e1ec() -> ! {
    todo!("0x69e1ec __ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIiLZNS_20sIntConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIdLZNS_23sDoubleConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv")]
// 0x69e29c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIdLZNS_23sDoubleConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_0x69e29c() -> ! {
    todo!("0x69e29c __ZN3RBX9CreatableINS_8InstanceEE6createINS_16ConstrainedValueIdLZNS_23sDoubleConstrainedValueEEEEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX16ConstrainedValueIdLZNS1_23sDoubleConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x69f524 — __ZN5boost10shared_ptrIN3RBX16ConstrainedValueIdLZNS1_23sDoubleConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_0x69f524() -> ! {
    todo!("0x69f524 __ZN5boost10shared_ptrIN3RBX16ConstrainedValueIdLZNS1_23sDoubleConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX16ConstrainedValueIiLZNS1_20sIntConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6a09c4 — __ZN5boost10shared_ptrIN3RBX16ConstrainedValueIiLZNS1_20sIntConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_0x6a09c4() -> ! {
    todo!("0x6a09c4 __ZN5boost10shared_ptrIN3RBX16ConstrainedValueIiLZNS1_20sIntConstrainedValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5ValueINS1_6RbxRayELZNS1_9sRayValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6a20f0 — __ZN5boost10shared_ptrIN3RBX5ValueINS1_6RbxRayELZNS1_9sRayValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_0x6a20f0() -> ! {
    todo!("0x6a20f0 __ZN5boost10shared_ptrIN3RBX5ValueINS1_6RbxRayELZNS1_9sRayValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5ValueINS1_10BrickColorELZNS1_16sBrickColorValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6a37f0 — __ZN5boost10shared_ptrIN3RBX5ValueINS1_10BrickColorELZNS1_16sBrickColorValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_0x6a37f0() -> ! {
    todo!("0x6a37f0 __ZN5boost10shared_ptrIN3RBX5ValueINS1_10BrickColorELZNS1_16sBrickColorValueEEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5ValueIN3G3D6Color3ELZNS1_12sColor3ValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6a4f0c — __ZN5boost10shared_ptrIN3RBX5ValueIN3G3D6Color3ELZNS1_12sColor3ValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_0x6a4f0c() -> ! {
    todo!("0x6a4f0c __ZN5boost10shared_ptrIN3RBX5ValueIN3G3D6Color3ELZNS1_12sColor3ValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5ValueIN3G3D15CoordinateFrameELZNS1_12sCFrameValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6a6648 — __ZN5boost10shared_ptrIN3RBX5ValueIN3G3D15CoordinateFrameELZNS1_12sCFrameValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_0x6a6648() -> ! {
    todo!("0x6a6648 __ZN5boost10shared_ptrIN3RBX5ValueIN3G3D15CoordinateFrameELZNS1_12sCFrameValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5ValueIN3G3D7Vector3ELZNS1_13sVector3ValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6a7d4c — __ZN5boost10shared_ptrIN3RBX5ValueIN3G3D7Vector3ELZNS1_13sVector3ValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_0x6a7d4c() -> ! {
    todo!("0x6a7d4c __ZN5boost10shared_ptrIN3RBX5ValueIN3G3D7Vector3ELZNS1_13sVector3ValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5ValueIbLZNS1_10sBoolValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6a91ac — __ZN5boost10shared_ptrIN3RBX5ValueIbLZNS1_10sBoolValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_0x6a91ac() -> ! {
    todo!("0x6a91ac __ZN5boost10shared_ptrIN3RBX5ValueIbLZNS1_10sBoolValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5ValueIdLZNS1_12sDoubleValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6aa60c — __ZN5boost10shared_ptrIN3RBX5ValueIdLZNS1_12sDoubleValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_0x6aa60c() -> ! {
    todo!("0x6aa60c __ZN5boost10shared_ptrIN3RBX5ValueIdLZNS1_12sDoubleValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5ValueIiLZNS1_9sIntValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x6aba68 — __ZN5boost10shared_ptrIN3RBX5ValueIiLZNS1_9sIntValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_0x6aba68() -> ! {
    todo!("0x6aba68 __ZN5boost10shared_ptrIN3RBX5ValueIiLZNS1_9sIntValueEEEEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "RBX::PlayerChatLine::PlayerChatLine(RBX::ChatLine::ChatType,rbx_core::SharedPtr<RBX::Network::Player>,std::string const&,float,bool)")]
// 0x79d5a8 — __ZN3RBX14PlayerChatLineC2ENS_8ChatLine8ChatTypeEN5boost10shared_ptrINS_7Network6PlayerEEERKSsfb
// was: RBX::PlayerChatLine::PlayerChatLine(RBX::ChatLine::ChatType,rbx_core::SharedPtr<RBX::Network::Player>,std::string const&,float,bool)
pub fn stub_0x79d5a8() -> ! {
    todo!("0x79d5a8 __ZN3RBX14PlayerChatLineC2ENS_8ChatLine8ChatTypeEN5boost10shared_ptrINS_7Network6PlayerEEERKSsfb")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>> const&)")]
// 0x7a3bbc — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi
pub fn stub_0x7a3bbc() -> ! {
    todo!("0x7a3bbc __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot*)")]
// 0x7a8d40 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEEaSEPSB_
// was: rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot*)
pub fn stub_0x7a8d40() -> ! {
    todo!("0x7a8d40 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEEaSEPSB_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")]
// 0x7a8e5c — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED1Ev
// was: rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_0x7a8e5c() -> ! {
    todo!("0x7a8e5c __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")]
// 0x7a8e88 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED0Ev
// was: rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_0x7a8e88() -> ! {
    todo!("0x7a8e88 __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")]
// 0x7a9078 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call
pub fn stub_0x7a9078() -> ! {
    todo!("0x7a9078 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")]
// 0x7a9080 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMe
pub fn stub_0x7a9080() -> ! {
    todo!("0x7a9080 __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>::operator()<RBX::Network::ChatMessage>(RBX::Network::ChatMessage const&)")]
// 0x7a9088 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_7Network11ChatMessageEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRKT_
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>::operator()<RBX::Network::ChatMessage>(RBX::Network::ChatMessage const&)
pub fn stub_0x7a9088() -> ! {
    todo!("0x7a9088 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_7Network11ChatMessageEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRKT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")]
// 0x7a9384 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~cal
pub fn stub_0x7a9384() -> ! {
    todo!("0x7a9384 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")]
// 0x7a93b0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~cal
pub fn stub_0x7a93b0() -> ! {
    todo!("0x7a93b0 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev")
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX8Humanoid6StatusEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS3_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// 0x7c59d0 — __ZN5boost8functionIFvN3RBX8Humanoid6StatusEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS3_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
pub fn stub_0x7c59d0() -> ! {
    todo!("0x7c59d0 __ZN5boost8functionIFvN3RBX8Humanoid6StatusEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS3_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvN3RBX8Humanoid6StatusEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// 0x7c5ab4 — __ZN5boost9function1IvN3RBX8Humanoid6StatusEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_0x7c5ab4() -> ! {
    todo!("0x7c5ab4 __ZN5boost9function1IvN3RBX8Humanoid6StatusEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::callable_slot<boost::function<void ()(RBX::Humanoid::Status)>>::~callable_slot()")]
// 0x7c64f4 — __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE13callable_slotIN5boost8functionIS5_EEED1Ev
// was: rbx::signals::signal<void ()(RBX::Humanoid::Status)>::callable_slot<boost::function<void ()(RBX::Humanoid::Status)>>::~callable_slot()
pub fn stub_0x7c64f4() -> ! {
    todo!("0x7c64f4 __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE13callable_slotIN5boost8functionIS5_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::callable_slot<boost::function<void ()(RBX::Humanoid::Status)>>::~callable_slot()")]
// 0x7c6604 — __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE13callable_slotIN5boost8functionIS5_EEED0Ev
// was: rbx::signals::signal<void ()(RBX::Humanoid::Status)>::callable_slot<boost::function<void ()(RBX::Humanoid::Status)>>::~callable_slot()
pub fn stub_0x7c6604() -> ! {
    todo!("0x7c6604 __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE13callable_slotIN5boost8functionIS5_EEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::call(RBX::Humanoid::Status)")]
// 0x7c6850 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::call(RBX::Humanoid::Status)
pub fn stub_0x7c6850() -> ! {
    todo!("0x7c6850 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::call(RBX::Humanoid::Status)")]
// 0x7c6858 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::call(RBX::Humanoid::Status)
pub fn stub_0x7c6858() -> ! {
    todo!("0x7c6858 __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_")
}

#[doc(alias = "boost::function1<void,RBX::Humanoid::Status>::operator()(RBX::Humanoid::Status)const")]
// 0x7c6860 — __ZNK5boost9function1IvN3RBX8Humanoid6StatusEEclES3_
// was: boost::function1<void,RBX::Humanoid::Status>::operator()(RBX::Humanoid::Status)const
pub fn stub_0x7c6860() -> ! {
    todo!("0x7c6860 __ZNK5boost9function1IvN3RBX8Humanoid6StatusEEclES3_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::~callable()")]
// 0x7c6b08 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::~callable()
pub fn stub_0x7c6b08() -> ! {
    todo!("0x7c6b08 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::~callable()")]
// 0x7c6c18 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::~callable()
pub fn stub_0x7c6c18() -> ! {
    todo!("0x7c6c18 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev")
}

#[doc(alias = "boost::function1<void,RBX::Humanoid::Status>::assign_to_own(boost::function1<void,RBX::Humanoid::Status> const&)")]
// 0x7c6e48 — __ZN5boost9function1IvN3RBX8Humanoid6StatusEE13assign_to_ownERKS4_
// was: boost::function1<void,RBX::Humanoid::Status>::assign_to_own(boost::function1<void,RBX::Humanoid::Status> const&)
pub fn stub_0x7c6e48() -> ! {
    todo!("0x7c6e48 __ZN5boost9function1IvN3RBX8Humanoid6StatusEE13assign_to_ownERKS4_")
}

#[doc(alias = "__ZN5boost8functionIFvfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// 0x7c8ccc — __ZN5boost8functionIFvfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
pub fn stub_0x7c8ccc() -> ! {
    todo!("0x7c8ccc __ZN5boost8functionIFvfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvfEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x7c8db0 — __ZN5boost9function1IvfEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_0x7c8db0() -> ! {
    todo!("0x7c8db0 __ZN5boost9function1IvfEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::simulate(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,float)")]
// 0x7cf624 — __ZN3RBX5HUMAN13HumanoidState8simulateERN5boost10shared_ptrIS1_EEf
// was: RBX::HUMAN::HumanoidState::simulate(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,float)
pub fn stub_0x7cf624() -> ! {
    todo!("0x7cf624 __ZN3RBX5HUMAN13HumanoidState8simulateERN5boost10shared_ptrIS1_EEf")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::doSimulatorStateTable(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,float)")]
// 0x7cf76c — __ZN3RBX5HUMAN13HumanoidState21doSimulatorStateTableERN5boost10shared_ptrIS1_EEf
// was: RBX::HUMAN::HumanoidState::doSimulatorStateTable(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,float)
pub fn stub_0x7cf76c() -> ! {
    todo!("0x7cf76c __ZN3RBX5HUMAN13HumanoidState21doSimulatorStateTableERN5boost10shared_ptrIS1_EEf")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::noSimulate(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &)")]
// 0x7cf838 — __ZN3RBX5HUMAN13HumanoidState10noSimulateERN5boost10shared_ptrIS1_EE
// was: RBX::HUMAN::HumanoidState::noSimulate(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &)
pub fn stub_0x7cf838() -> ! {
    todo!("0x7cf838 __ZN3RBX5HUMAN13HumanoidState10noSimulateERN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::doSlaveStateTable(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,RBX::HUMAN::StateType)")]
// 0x7cf8cc — __ZN3RBX5HUMAN13HumanoidState17doSlaveStateTableERN5boost10shared_ptrIS1_EENS0_9StateTypeE
// was: RBX::HUMAN::HumanoidState::doSlaveStateTable(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,RBX::HUMAN::StateType)
pub fn stub_0x7cf8cc() -> ! {
    todo!("0x7cf8cc __ZN3RBX5HUMAN13HumanoidState17doSlaveStateTableERN5boost10shared_ptrIS1_EENS0_9StateTypeE")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::changeState(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,RBX::HUMAN::StateType)")]
// 0x7cf96c — __ZN3RBX5HUMAN13HumanoidState11changeStateERN5boost10shared_ptrIS1_EENS0_9StateTypeE
// was: RBX::HUMAN::HumanoidState::changeState(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,RBX::HUMAN::StateType)
pub fn stub_0x7cf96c() -> ! {
    todo!("0x7cf96c __ZN3RBX5HUMAN13HumanoidState11changeStateERN5boost10shared_ptrIS1_EENS0_9StateTypeE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>> const&)")]
// 0x7d14d4 — __ZN3rbx7signals6signalIFvbEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState
pub fn stub_0x7d14d4() -> ! {
    todo!("0x7d14d4 __ZN3rbx7signals6signalIFvbEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>::~callable_slot()")]
// 0x7d1800 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEED1Ev
// was: rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_0x7d1800() -> ! {
    todo!("0x7d1800 __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>::~callable_slot()")]
// 0x7d182c — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEED0Ev
// was: rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_0x7d182c() -> ! {
    todo!("0x7d182c __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
// 0x7d1900 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
// was: rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)
pub fn stub_0x7d1900() -> ! {
    todo!("0x7d1900 __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
// 0x7d1924 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)
pub fn stub_0x7d1924() -> ! {
    todo!("0x7d1924 __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list1<bool &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool> &,boost::_bi::list1<bool &> &,int)")]
// 0x7d1948 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX5HUMAN13HumanoidStateEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_bEENS0_5list1IRbEEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list1<bool &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool> &,boost::_bi::list1<bool &> &,int)
pub fn stub_0x7d1948() -> ! {
    todo!("0x7d1948 __ZN5boost3_bi5list2INS0_5valueIPN3RBX5HUMAN13HumanoidStateEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_bEENS0_5list1IRbEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::~callable()")]
// 0x7d1968 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::~callable()
pub fn stub_0x7d1968() -> ! {
    todo!("0x7d1968 __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::~callable()")]
// 0x7d1994 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::~callable()
pub fn stub_0x7d1994() -> ! {
    todo!("0x7d1994 __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev")
}

#[doc(alias = "RBX::TextService::registerTypesetter(RBX::TextService::Font,rbx_core::SharedPtr<RBX::Typesetter>)")]
// 0x7d8e48 — __ZN3RBX11TextService18registerTypesetterENS0_4FontEN5boost10shared_ptrINS_10TypesetterEEE
// was: RBX::TextService::registerTypesetter(RBX::TextService::Font,rbx_core::SharedPtr<RBX::Typesetter>)
pub fn stub_0x7d8e48() -> ! {
    todo!("0x7d8e48 __ZN3RBX11TextService18registerTypesetterENS0_4FontEN5boost10shared_ptrINS_10TypesetterEEE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Typesetter>::operator=(rbx_core::SharedPtr<RBX::Typesetter> const&)")]
// 0x7d9cb8 — __ZN5boost10shared_ptrIN3RBX10TypesetterEEaSERKS3_
// was: rbx_core::SharedPtr<RBX::Typesetter>::operator=(rbx_core::SharedPtr<RBX::Typesetter> const&)
pub fn stub_0x7d9cb8() -> ! {
    todo!("0x7d9cb8 __ZN5boost10shared_ptrIN3RBX10TypesetterEEaSERKS3_")
}

#[doc(alias = "void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::push_impl<RBX::Cocoa::String_sink>(RBX::Cocoa::String_sink const&,int,int)")]
// 0x7df948 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implIN3RBX5Cocoa11String_sinkEEEvRKT_ii
// was: void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::push_impl<RBX::Cocoa::String_sink>(RBX::Cocoa::String_sink const&,int,int)
pub fn stub_0x7df948() -> ! {
    todo!("0x7df948 __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implIN3RBX5Cocoa11String_sinkEEEvRKT_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::stream_buffer(RBX::Cocoa::String_sink const&,int,int)")]
// 0x7dfb44 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEEC2ERKS4_ii
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::stream_buffer(RBX::Cocoa::String_sink const&,int,int)
pub fn stub_0x7dfb44() -> ! {
    todo!("0x7dfb44 __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEEC2ERKS4_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(RBX::Cocoa::String_sink const&,int,int)")]
// 0x7dfc84 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9open_implERKS4_ii
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(RBX::Cocoa::String_sink const&,int,int)
pub fn stub_0x7dfc84() -> ! {
    todo!("0x7dfc84 __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9open_implERKS4_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7dfda4 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED1Ev
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_0x7dfda4() -> ! {
    todo!("0x7dfda4 __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED1Ev")
}

#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7dfda8 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED0Ev
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_0x7dfda8() -> ! {
    todo!("0x7dfda8 __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED0Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)")]
// 0x7dfe48 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE5imbueERKSt6locale
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)
pub fn stub_0x7dfe48() -> ! {
    todo!("0x7dfe48 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE5imbueERKSt6locale")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7dfe74 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_0x7dfe74() -> ! {
    todo!("0x7dfe74 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
// 0x7dfe8c — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)
pub fn stub_0x7dfe8c() -> ! {
    todo!("0x7dfe8c __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)")]
// 0x7dfec0 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4syncEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)
pub fn stub_0x7dfec0() -> ! {
    todo!("0x7dfec0 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4syncEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)")]
// 0x7dff70 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9underflowEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)
pub fn stub_0x7dff70() -> ! {
    todo!("0x7dff70 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9underflowEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)")]
// 0x7dffc8 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9pbackfailEi
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)
pub fn stub_0x7dffc8() -> ! {
    todo!("0x7dffc8 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9pbackfailEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)")]
// 0x7e00e0 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE8overflowEi
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)
pub fn stub_0x7e00e0() -> ! {
    todo!("0x7e00e0 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE8overflowEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x7e0150 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE8set_nextEPNS1_16linked_streambufIcS7_EE
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_0x7e0150() -> ! {
    todo!("0x7e0150 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE8set_nextEPNS1_16linked_streambufIcS7_EE")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)")]
// 0x7e0154 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE10close_implESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)
pub fn stub_0x7e0154() -> ! {
    todo!("0x7e0154 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE10close_implESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const")]
// 0x7e0178 — __ZNK5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE10auto_closeEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const
pub fn stub_0x7e0178() -> ! {
    todo!("0x7e0178 __ZNK5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE10auto_closeEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)")]
// 0x7e0184 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14set_auto_closeEb
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)
pub fn stub_0x7e0184() -> ! {
    todo!("0x7e0184 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14set_auto_closeEb")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)")]
// 0x7e0198 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE11strict_syncEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)
pub fn stub_0x7e0198() -> ! {
    todo!("0x7e0198 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE11strict_syncEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const")]
// 0x7e0254 — __ZNK5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14component_typeEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const
pub fn stub_0x7e0254() -> ! {
    todo!("0x7e0254 __ZNK5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14component_typeEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)")]
// 0x7e0264 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14component_implEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)
pub fn stub_0x7e0264() -> ! {
    todo!("0x7e0264 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14component_implEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)")]
// 0x7e0268 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE13init_get_areaEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)
pub fn stub_0x7e0268() -> ! {
    todo!("0x7e0268 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE13init_get_areaEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)")]
// 0x7e0274 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE13init_put_areaEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)
pub fn stub_0x7e0274() -> ! {
    todo!("0x7e0274 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE13init_put_areaEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync_impl(void)")]
// 0x7e0298 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9sync_implEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync_impl(void)
pub fn stub_0x7e0298() -> ! {
    todo!("0x7e0298 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9sync_implEv")
}

#[doc(alias = "int boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char *,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x7e02c8 — __ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4readINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPciPT_
// was: int boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char *,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_0x7e02c8() -> ! {
    todo!("0x7e02c8 __ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4readINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPciPT_")
}

#[doc(alias = "int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<RBX::Cocoa::String_sink>::type *,int)")]
// 0x7e02d0 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISD_E4typeEi
// was: int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::cha
pub fn stub_0x7e02d0() -> ! {
    todo!("0x7e02d0 __ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISD_E4typeEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7e03a8 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_0x7e03a8() -> ! {
    todo!("0x7e03a8 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode")
}
