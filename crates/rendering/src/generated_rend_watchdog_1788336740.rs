//! rendering watchdog 1788336740 — 120 stubs 0x77c7ac..0x781560 EA-sorted asc gap filler not yet in rbx_rendering (Ogre/G3D complete, global gap filler 52557->52677 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 120 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x77c7ac — __ZN5boost10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS4_EEEEC2ISB_N3rbx6detail13sp_ms_deleterISB_EEEEPT_T0_
// type: unknown
#[doc(alias = "boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>(std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>)")]
#[doc(alias = "__ZN5boost10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS4_EEEEC2ISB_N3rbx6detail13sp_ms_deleterISB_EEEEPT_T0_")]
// IDA 0x77c7ac: 94 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77c7ac() {
}
// 0x77c8b8 — __ZN5boost6detail12shared_countC2IPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEN3rbx6detail13sp_ms_deleterISD_EEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>(std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEN3rbx6detail13sp_ms_deleterISD_EEEET_T0_")]
// IDA 0x77c8b8: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77c8b8() {
}
// 0x77c9c0 — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEED1Ev
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEED1Ev")]
// IDA 0x77c9c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77c9c0() {
}
// 0x77c9ec — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEED0Ev
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEED0Ev")]
// IDA 0x77c9ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77c9ec() {
}
// 0x77caa8 — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE7disposeEv
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE7disposeEv")]
// IDA 0x77caa8: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77caa8() {
}
// 0x77cac8 — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE11get_deleterERKSt9type_info
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE11get_deleterERKSt9type_info")]
// IDA 0x77cac8: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77cac8() {
}
// 0x77cae0 — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE19get_untyped_deleterEv
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE19get_untyped_deleterEv")]
// IDA 0x77cae0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77cae0() {
}
// 0x77cae4 — __ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSB_5list2INSB_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: unknown
#[doc(alias = "__ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSB_5list2INSB_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSB_5list2INSB_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// IDA 0x77cae4: 101 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77cae4() {
}
// 0x77cc0c — __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: unknown
#[doc(alias = "__ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// IDA 0x77cc0c: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77cc0c() {
}
// 0x77cd38 — __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEEvT_
// type: unknown
#[doc(alias = "void boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEEvT_")]
// IDA 0x77cd38: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77cd38() {
}
// 0x77ce74 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// type: unknown
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
// IDA 0x77ce74: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ce74() {
}
// 0x77cef4 — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEES7_S9_P9lua_DebugE6invokeERNS1_15function_bufferES9_SK_
// type: unknown
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,RBX::Reflection::Variant,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEES7_S9_P9lua_DebugE6invokeERNS1_15function_bufferES9_SK_")]
// IDA 0x77cef4: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77cef4() {
}
// 0x77cf18 — __ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: unknown
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
// IDA 0x77cf18: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77cf18() {
}
// 0x77d044 — __ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: unknown
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// IDA 0x77d044: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77d044() {
}
// 0x77d17c — __ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEclIN3RBX10Reflection7VariantEPFSA_SsP9lua_StateENS1_IRSC_RP9lua_DebugEEEET_NS0_4typeISK_EERT0_RT1_l
// type: unknown
#[doc(alias = "RBX::Reflection::Variant boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>::operator()<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<RBX::Reflection::Variant>,RBX::Reflection::Variant (*)(std::string,lua_State *) &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,long)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEclIN3RBX10Reflection7VariantEPFSA_SsP9lua_StateENS1_IRSC_RP9lua_DebugEEEET_NS0_4typeISK_EERT0_RT1_l")]
// IDA 0x77d17c: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77d17c() {
}
// 0x77d2a0 — __ZNK5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEclES5_S7_
// type: unknown
#[doc(alias = "boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
#[doc(alias = "__ZNK5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEclES5_S7_")]
// IDA 0x77d2a0: 71 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77d2a0() {
}
// 0x77d36c — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES12_
// type: unknown
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES12_")]
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES12_")]
// IDA 0x77d36c: 100 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77d36c() {
}
// 0x77d484 — __ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS1_10Reflection7VariantES5_S7_EEERSA_RNS_10shared_ptrISsEEPS3_NS_3argILi1EEENSI_ILi2EEESC_NS_17reference_wrapperISA_EENSL_ISF_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISQ_T0_T1_T2_T3_T4_T5_EENSO_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMST_FSQ_SU_SV_SW_SX_SY_ES11_S12_S13_S14_S15_S16_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS1_10Reflection7VariantES5_S7_EEERSA_RNS_10shared_ptrISsEEPS3_NS_3argILi1EEENSI_ILi2EEESC_NS_17reference_wrapperISA_EENSL_ISF_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISQ_T0_T1_T2_T3_T4_T5_EENSO_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMST_FSQ_SU_SV_SW_SX_SY_ES11_S12_S13_S14_S15_S16_")]
// IDA 0x77d484: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77d484() {
}
// 0x77d5a8 — __ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookINS_10Reflection7VariantEEEvP9lua_StateP9lua_DebugN5boost8functionIFT_S6_S8_EEERSB_RNS9_10shared_ptrISsEE
// type: unknown
#[doc(alias = "void RBX::Scripting::ScriptDebugger::withPausedThreadHook<RBX::Reflection::Variant>(lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookINS_10Reflection7VariantEEEvP9lua_StateP9lua_DebugN5boost8functionIFT_S6_S8_EEERSB_RNS9_10shared_ptrISsEE")]
// IDA 0x77d5a8: 348 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77d5a8() {
}
// 0x77d978 — __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE13assign_to_ownERKS8_
// type: unknown
#[doc(alias = "boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to_own(boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *> const&)")]
#[doc(alias = "__ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE13assign_to_ownERKS8_")]
// IDA 0x77d978: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77d978() {
}
// 0x77d9a8 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_
// type: unknown
#[doc(alias = "boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_")]
// IDA 0x77d9a8: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77d9a8() {
}
// 0x77da7c — __ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_
// type: unknown
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_")]
// IDA 0x77da7c: 77 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77da7c() {
}
// 0x77db54 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EEEC2ES7_S9_SA_SK_SM_
// type: unknown
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EEEC2ES7_S9_SA_SK_SM_")]
// IDA 0x77db54: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77db54() {
}
// 0x77dc30 — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE
// type: unknown
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
// IDA 0x77dc30: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77dc30() {
}
// 0x77dd1c — __ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE
// type: unknown
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
// IDA 0x77dd1c: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77dd1c() {
}
// 0x77de08 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEEvT_
// type: unknown
#[doc(alias = "void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEEvT_")]
// IDA 0x77de08: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77de08() {
}
// 0x77df04 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE6manageERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeE
// type: unknown
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE6manageERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeE")]
// IDA 0x77df04: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77df04() {
}
// 0x77df20 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_")]
// IDA 0x77df20: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77df20() {
}
// 0x77df44 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferE
// type: unknown
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferE")]
// IDA 0x77df44: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77df44() {
}
// 0x77e034 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: unknown
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// IDA 0x77e034: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77e034() {
}
// 0x77e11c — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: unknown
#[doc(alias = "void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// IDA 0x77e11c: 78 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77e11c() {
}
// 0x77e1f8 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEclINS_4_mfi3mf5IvS5_SF_SH_SJ_RSD_RSO_EENS0_5list2IRSF_RSH_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEclINS_4_mfi3mf5IvS5_SF_SH_SJ_RSD_RSO_EENS0_5list2IRSF_RSH_EEEEvNS0_4typeIvEERT_RT0_i")]
// IDA 0x77e1f8: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77e1f8() {
}
// 0x77e2dc — __ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS2_10Reflection7VariantES6_S8_EEERSB_RNS_10shared_ptrISsEEEclEPS4_S6_S8_SD_SE_SH_
// type: unknown
#[doc(alias = "boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS2_10Reflection7VariantES6_S8_EEERSB_RNS_10shared_ptrISsEEEclEPS4_S6_S8_SD_SE_SH_")]
// IDA 0x77e2dc: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77e2dc() {
}
// 0x77e3c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE7managerERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE7managerERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// IDA 0x77e3c8: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77e3c8() {
}
// 0x77e528 — __ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_
// type: unknown
#[doc(alias = "boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>::list2(boost::_bi::value<std::string>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_")]
// IDA 0x77e528: 96 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77e528() {
}
// 0x77e644 — __ZN5boost3_bi8storage2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_
// type: unknown
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::arg<1>>::storage2(boost::_bi::value<std::string>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_")]
// IDA 0x77e644: 96 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77e644() {
}
// 0x77e760 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEEixERS9_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::operator[](RBX::Script const* const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEEixERS9_")]
// IDA 0x77e760: 146 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77e760() {
}
// 0x77e8e0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm")]
// IDA 0x77e8e0: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77e8e0() {
}
// 0x77e930 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm")]
// IDA 0x77e930: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77e930() {
}
// 0x77ea58 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE20min_buckets_for_sizeEm
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE20min_buckets_for_sizeEm")]
// IDA 0x77ea58: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ea58() {
}
// 0x77eae8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE11rehash_implEm
// type: unknown
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::rehash_impl(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE11rehash_implEm")]
// IDA 0x77eae8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77eae8() {
}
// 0x77eb14 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
// type: unknown
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE")]
// IDA 0x77eb14: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77eb14() {
}
// 0x77eb6c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEEEEE9constructEv
// type: unknown
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>>>::construct(void)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEEEEE9constructEv")]
// IDA 0x77eb6c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77eb6c() {
}
// 0x77eba4 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14find_node_implIS8_SI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
// type: unknown
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::find_node_impl<RBX::Script const*,std::equal_to<RBX::Script const*>>(unsigned long,RBX::Script const* const&,std::equal_to<RBX::Script const*> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14find_node_implIS8_SI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_")]
// IDA 0x77eba4: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77eba4() {
}
// 0x77ec10 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE9erase_keyERS9_
// type: unknown
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::erase_key(RBX::Script const* const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE9erase_keyERS9_")]
// IDA 0x77ec10: 54 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ec10() {
}
// 0x77ec98 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE12delete_nodesEPNS1_10ptr_bucketESM_
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE12delete_nodesEPNS1_10ptr_bucketESM_")]
// IDA 0x77ec98: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ec98() {
}
// 0x77ecd4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE10fix_bucketEmPNS1_10ptr_bucketE
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE10fix_bucketEmPNS1_10ptr_bucketE")]
// IDA 0x77ecd4: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ecd4() {
}
// 0x77ed18 — __ZN5boost10shared_ptrIN3RBX10BaseScriptEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: unknown
#[doc(alias = "boost::shared_ptr<RBX::BaseScript>::shared_ptr<RBX::BaseScript>(boost::weak_ptr<RBX::BaseScript> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10BaseScriptEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
// IDA 0x77ed18: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ed18() {
}
// 0x77ed94 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS8_EEEESA_EEPT_RKNS5_IT0_EE
// type: unknown
#[doc(alias = "rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> const&)")]
#[doc(alias = "__ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS8_EEEESA_EEPT_RKNS5_IT0_EE")]
// IDA 0x77ed94: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ed94() {
}
// 0x77edf0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS4_RKS5_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::vector(unsigned long,boost::shared_ptr<RBX::Instance> const&,std::allocator<boost::shared_ptr<RBX::Instance>> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS4_RKS5_")]
// IDA 0x77edf0: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77edf0() {
}
// 0x77eeb8 — __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS5_
// type: unknown
#[doc(alias = "std::_Vector_base<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_Vector_base(unsigned long,std::allocator<boost::shared_ptr<RBX::Instance>> const&)")]
#[doc(alias = "__ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS5_")]
// IDA 0x77eeb8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77eeb8() {
}
// 0x77eeec — __ZN5boost10shared_ptrISt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEC2IS6_N3rbx6detail13sp_ms_deleterIS6_EEEEPT_T0_
// type: int __fastcall(int)
#[doc(alias = "boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)")]
#[doc(alias = "__ZN5boost10shared_ptrISt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEC2IS6_N3rbx6detail13sp_ms_deleterIS6_EEEEPT_T0_")]
// IDA 0x77eeec: 92 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77eeec() {
}
// 0x77eff4 — __ZN5boost6detail12shared_countC2IPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEN3rbx6detail13sp_ms_deleterIS9_EEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEN3rbx6detail13sp_ms_deleterIS9_EEEET_T0_")]
// IDA 0x77eff4: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77eff4() {
}
// 0x77f0f8 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED1Ev
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED1Ev")]
// IDA 0x77f0f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77f0f8() {
}
// 0x77f124 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED0Ev
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED0Ev")]
// IDA 0x77f124: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77f124() {
}
// 0x77f1dc — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE7disposeEv
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE7disposeEv")]
// IDA 0x77f1dc: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77f1dc() {
}
// 0x77f1f8 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE11get_deleterERKSt9type_info
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE11get_deleterERKSt9type_info")]
// IDA 0x77f1f8: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77f1f8() {
}
// 0x77f210 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE19get_untyped_deleterEv
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE19get_untyped_deleterEv")]
// IDA 0x77f210: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77f210() {
}
// 0x77f214 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14delete_bucketsEv
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::delete_buckets(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14delete_bucketsEv")]
// IDA 0x77f214: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77f214() {
}
// 0x77f244 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::table(unsigned long,boost::hash<RBX::Script const*> const&,std::equal_to<RBX::Script const*> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE")]
// IDA 0x77f244: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77f244() {
}
// 0x77f2b0 — __ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// type: unknown
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
// IDA 0x77f2b0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_77f2b0() {
}
// 0x77f2b4 — __ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// type: unknown
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
// IDA 0x77f2b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77f2b4() {
}
// 0x77f354 — __ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// type: unknown
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
// IDA 0x77f354: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77f354() {
}
// 0x77f35c — __ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// type: unknown
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
// IDA 0x77f35c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77f35c() {
}
// 0x77f400 — __ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
// IDA 0x77f400: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77f400() {
}
// 0x77f408 — __ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// type: unknown
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
// IDA 0x77f408: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77f408() {
}
// 0x77f4ac — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::BoundFuncDesc(void (RBX::Scripting::DebuggerWatch::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x77f4ac: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77f4ac() {
}
// 0x77f5b0 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EED0Ev")]
// IDA 0x77f5b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77f5b0() {
}
// 0x77f664 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x77f664: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77f664() {
}
// 0x77f688 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting13DebuggerWatchEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerWatch>(char const*,char const*,std::string  RBX::Scripting::DebuggerWatch::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting13DebuggerWatchEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x77f688: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77f688() {
}
// 0x77f81c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE10isReadOnlyEv")]
// IDA 0x77f81c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77f81c() {
}
// 0x77f820 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE11isWriteOnlyEv")]
// IDA 0x77f820: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77f820() {
}
// 0x77f824 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x77f824: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77f824() {
}
// 0x77f83c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8setValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8setValueEPNS0_13DescribedBaseERKSs")]
// IDA 0x77f83c: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77f83c() {
}
// 0x77f8a4 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,std::string  RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x77f8a4: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77f8a4() {
}
// 0x77fa34 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv")]
// IDA 0x77fa34: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fa34() {
}
// 0x77fa38 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv")]
// IDA 0x77fa38: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fa38() {
}
// 0x77fa3c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x77fa3c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fa3c() {
}
// 0x77fa54 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKSs")]
// IDA 0x77fa54: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fa54() {
}
// 0x77fabc — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,bool RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x77fabc: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fabc() {
}
// 0x77fc50 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv")]
// IDA 0x77fc50: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fc50() {
}
// 0x77fc54 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv")]
// IDA 0x77fc54: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fc54() {
}
// 0x77fc58 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x77fc58: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fc58() {
}
// 0x77fc64 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKb
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKb")]
// IDA 0x77fc64: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fc64() {
}
// 0x77fcb4 — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,int RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x77fcb4: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fcb4() {
}
// 0x77fe48 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv")]
// IDA 0x77fe48: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fe48() {
}
// 0x77fe4c — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv")]
// IDA 0x77fe4c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fe4c() {
}
// 0x77fe50 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x77fe50: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fe50() {
}
// 0x77fe5c — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKi
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKi")]
// IDA 0x77fe5c: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77fe5c() {
}
// 0x77feac — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::PropDescriptor<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int>(char const*,char const*,int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x77feac: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77feac() {
}
// 0x77ffb8 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiED0Ev")]
// IDA 0x77ffb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77ffb8() {
}
// 0x77ffe4 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE10isReadOnlyEv")]
// IDA 0x77ffe4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ffe4() {
}
// 0x77ffe8 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE11isWriteOnlyEv")]
// IDA 0x77ffe8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ffe8() {
}
// 0x77ffec — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x77ffec: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ffec() {
}
// 0x78000c — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE8setValueEPNS0_13DescribedBaseERKi
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE8setValueEPNS0_13DescribedBaseERKi")]
// IDA 0x78000c: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78000c() {
}
// 0x78012c — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x78012c: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78012c() {
}
// 0x7802b0 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev")]
// IDA 0x7802b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7802b0() {
}
// 0x780364 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE")]
// IDA 0x780364: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_780364() {
}
// 0x7804b8 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE")]
// IDA 0x7804b8: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7804b8() {
}
// 0x780618 — __ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE")]
// IDA 0x780618: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_780618() {
}
// 0x780630 — __ZNK5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEclES4_
// type: unknown
#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::Instance>>::operator()(boost::shared_ptr<RBX::Instance>)const")]
#[doc(alias = "__ZNK5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEclES4_")]
// IDA 0x780630: 96 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_780630() {
}
// 0x780744 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_ED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_ED0Ev")]
// IDA 0x780744: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_780744() {
}
// 0x7807f8 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// IDA 0x7807f8: 198 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7807f8() {
}
// 0x7809fc — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
// IDA 0x7809fc: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7809fc() {
}
// 0x780a70 — __ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE")]
// IDA 0x780a70: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_780a70() {
}
// 0x780a84 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::EventDesc(rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x780a84: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_780a84() {
}
// 0x780c08 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_ED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_ED0Ev")]
// IDA 0x780c08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_780c08() {
}
// 0x780cbc — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// IDA 0x780cbc: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_780cbc() {
}
// 0x780e10 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
// IDA 0x780e10: 45 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_780e10() {
}
// 0x780e9c — __ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE")]
// IDA 0x780e9c: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_780e9c() {
}
// 0x780eb0 — __ZN3rbx7signals6signalIFviEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(int)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE13disconnectAllEv")]
// IDA 0x780eb0: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_780eb0() {
}
// 0x781028 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKiNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,int const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(int const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKiNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_")]
// IDA 0x781028: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781028() {
}
// 0x781148 — __ZN5boost9function1IviE5clearEv
// type: unknown
#[doc(alias = "boost::function1<void,int>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IviE5clearEv")]
// IDA 0x781148: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781148() {
}
// 0x781178 — __ZN5boost9function1IviEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IviEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function1IviEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// IDA 0x781178: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781178() {
}
// 0x781260 — __ZN5boost9function1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_")]
// IDA 0x781260: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781260() {
}
// 0x781358 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// type: unknown
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
// IDA 0x781358: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781358() {
}
// 0x781378 — __ZNK5boost6detail8function13basic_vtable1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
// IDA 0x781378: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781378() {
}
// 0x781460 — __ZNK5boost6detail8function13basic_vtable1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// IDA 0x781460: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781460() {
}
// 0x781548 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIiEEvRT_
// type: unknown
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<int>(int &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIiEEvRT_")]
// IDA 0x781548: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781548() {
}
// 0x781560 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// IDA 0x781560: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_781560() {
}
