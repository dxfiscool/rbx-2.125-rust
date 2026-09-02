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
pub fn stub_77c7ac() -> ! {
    todo!("0x77c7ac boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>(std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>)")
}
// 0x77c8b8 — __ZN5boost6detail12shared_countC2IPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEN3rbx6detail13sp_ms_deleterISD_EEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>(std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEN3rbx6detail13sp_ms_deleterISD_EEEET_T0_")]
pub fn stub_77c8b8() -> ! {
    todo!("0x77c8b8 boost::detail::shared_count::shared_count<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>(std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>)")
}
// 0x77c9c0 — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEED1Ev
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEED1Ev")]
pub fn stub_77c9c0() -> ! {
    todo!("0x77c9c0 boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()")
}
// 0x77c9ec — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEED0Ev
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEED0Ev")]
pub fn stub_77c9ec() -> ! {
    todo!("0x77c9ec boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()")
}
// 0x77caa8 — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE7disposeEv
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE7disposeEv")]
pub fn stub_77caa8() -> ! {
    todo!("0x77caa8 boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::dispose(void)")
}
// 0x77cac8 — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE11get_deleterERKSt9type_info
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE11get_deleterERKSt9type_info")]
pub fn stub_77cac8() -> ! {
    todo!("0x77cac8 boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_deleter(std::type_info const&)")
}
// 0x77cae0 — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE19get_untyped_deleterEv
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE19get_untyped_deleterEv")]
pub fn stub_77cae0() -> ! {
    todo!("0x77cae0 boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_untyped_deleter(void)")
}
// 0x77cae4 — __ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSB_5list2INSB_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: unknown
#[doc(alias = "__ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSB_5list2INSB_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSB_5list2INSB_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_77cae4() -> ! {
    todo!("0x77cae4 __ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSB_5list2INSB_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}
// 0x77cc0c — __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: unknown
#[doc(alias = "__ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_77cc0c() -> ! {
    todo!("0x77cc0c __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}
// 0x77cd38 — __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEEvT_
// type: unknown
#[doc(alias = "void boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_77cd38() -> ! {
    todo!("0x77cd38 void boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>)")
}
// 0x77ce74 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// type: unknown
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
pub fn stub_77ce74() -> ! {
    todo!("0x77ce74 boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}
// 0x77cef4 — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEES7_S9_P9lua_DebugE6invokeERNS1_15function_bufferES9_SK_
// type: unknown
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,RBX::Reflection::Variant,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEES7_S9_P9lua_DebugE6invokeERNS1_15function_bufferES9_SK_")]
pub fn stub_77cef4() -> ! {
    todo!("0x77cef4 boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,RBX::Reflection::Variant,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")
}
// 0x77cf18 — __ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: unknown
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_77cf18() -> ! {
    todo!("0x77cf18 bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}
// 0x77d044 — __ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: unknown
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_77d044() -> ! {
    todo!("0x77d044 bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}
// 0x77d17c — __ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEclIN3RBX10Reflection7VariantEPFSA_SsP9lua_StateENS1_IRSC_RP9lua_DebugEEEET_NS0_4typeISK_EERT0_RT1_l
// type: unknown
#[doc(alias = "RBX::Reflection::Variant boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>::operator()<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<RBX::Reflection::Variant>,RBX::Reflection::Variant (*)(std::string,lua_State *) &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,long)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEclIN3RBX10Reflection7VariantEPFSA_SsP9lua_StateENS1_IRSC_RP9lua_DebugEEEET_NS0_4typeISK_EERT0_RT1_l")]
pub fn stub_77d17c() -> ! {
    todo!("0x77d17c RBX::Reflection::Variant boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>::operator()<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<RBX::Reflection::Variant>,RBX::Reflection::Variant (*)(std::string,lua_State *) &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,long)")
}
// 0x77d2a0 — __ZNK5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEclES5_S7_
// type: unknown
#[doc(alias = "boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
#[doc(alias = "__ZNK5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEclES5_S7_")]
pub fn stub_77d2a0() -> ! {
    todo!("0x77d2a0 boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")
}
// 0x77d36c — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES12_
// type: unknown
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES12_")]
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES12_")]
pub fn stub_77d36c() -> ! {
    todo!("0x77d36c __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES12_")
}
// 0x77d484 — __ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS1_10Reflection7VariantES5_S7_EEERSA_RNS_10shared_ptrISsEEPS3_NS_3argILi1EEENSI_ILi2EEESC_NS_17reference_wrapperISA_EENSL_ISF_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISQ_T0_T1_T2_T3_T4_T5_EENSO_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMST_FSQ_SU_SV_SW_SX_SY_ES11_S12_S13_S14_S15_S16_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS1_10Reflection7VariantES5_S7_EEERSA_RNS_10shared_ptrISsEEPS3_NS_3argILi1EEENSI_ILi2EEESC_NS_17reference_wrapperISA_EENSL_ISF_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISQ_T0_T1_T2_T3_T4_T5_EENSO_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMST_FSQ_SU_SV_SW_SX_SY_ES11_S12_S13_S14_S15_S16_")]
pub fn stub_77d484() -> ! {
    todo!("0x77d484 boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")
}
// 0x77d5a8 — __ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookINS_10Reflection7VariantEEEvP9lua_StateP9lua_DebugN5boost8functionIFT_S6_S8_EEERSB_RNS9_10shared_ptrISsEE
// type: unknown
#[doc(alias = "void RBX::Scripting::ScriptDebugger::withPausedThreadHook<RBX::Reflection::Variant>(lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookINS_10Reflection7VariantEEEvP9lua_StateP9lua_DebugN5boost8functionIFT_S6_S8_EEERSB_RNS9_10shared_ptrISsEE")]
pub fn stub_77d5a8() -> ! {
    todo!("0x77d5a8 void RBX::Scripting::ScriptDebugger::withPausedThreadHook<RBX::Reflection::Variant>(lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &)")
}
// 0x77d978 — __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE13assign_to_ownERKS8_
// type: unknown
#[doc(alias = "boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to_own(boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *> const&)")]
#[doc(alias = "__ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE13assign_to_ownERKS8_")]
pub fn stub_77d978() -> ! {
    todo!("0x77d978 boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to_own(boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *> const&)")
}
// 0x77d9a8 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_
// type: unknown
#[doc(alias = "boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_")]
pub fn stub_77d9a8() -> ! {
    todo!("0x77d9a8 boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")
}
// 0x77da7c — __ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_
// type: unknown
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_")]
pub fn stub_77da7c() -> ! {
    todo!("0x77da7c boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")
}
// 0x77db54 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EEEC2ES7_S9_SA_SK_SM_
// type: unknown
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EEEC2ES7_S9_SA_SK_SM_")]
pub fn stub_77db54() -> ! {
    todo!("0x77db54 boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>)")
}
// 0x77dc30 — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE
// type: unknown
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
pub fn stub_77dc30() -> ! {
    todo!("0x77dc30 __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")
}
// 0x77dd1c — __ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE
// type: unknown
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
pub fn stub_77dd1c() -> ! {
    todo!("0x77dd1c __ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")
}
// 0x77de08 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEEvT_
// type: unknown
#[doc(alias = "void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEEvT_")]
pub fn stub_77de08() -> ! {
    todo!("0x77de08 void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>)")
}
// 0x77df04 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE6manageERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeE
// type: unknown
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE6manageERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeE")]
pub fn stub_77df04() -> ! {
    todo!("0x77df04 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}
// 0x77df20 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_")]
pub fn stub_77df20() -> ! {
    todo!("0x77df20 boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")
}
// 0x77df44 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferE
// type: unknown
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_77df44() -> ! {
    todo!("0x77df44 bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &)const")
}
// 0x77e034 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: unknown
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_77e034() -> ! {
    todo!("0x77e034 bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}
// 0x77e11c — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: unknown
#[doc(alias = "void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_77e11c() -> ! {
    todo!("0x77e11c void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}
// 0x77e1f8 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEclINS_4_mfi3mf5IvS5_SF_SH_SJ_RSD_RSO_EENS0_5list2IRSF_RSH_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEclINS_4_mfi3mf5IvS5_SF_SH_SJ_RSD_RSO_EENS0_5list2IRSF_RSH_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_77e1f8() -> ! {
    todo!("0x77e1f8 void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")
}
// 0x77e2dc — __ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS2_10Reflection7VariantES6_S8_EEERSB_RNS_10shared_ptrISsEEEclEPS4_S6_S8_SD_SE_SH_
// type: unknown
#[doc(alias = "boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS2_10Reflection7VariantES6_S8_EEERSB_RNS_10shared_ptrISsEEEclEPS4_S6_S8_SD_SE_SH_")]
pub fn stub_77e2dc() -> ! {
    todo!("0x77e2dc boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &)const")
}
// 0x77e3c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE7managerERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE7managerERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_77e3c8() -> ! {
    todo!("0x77e3c8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}
// 0x77e528 — __ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_
// type: unknown
#[doc(alias = "boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>::list2(boost::_bi::value<std::string>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_")]
pub fn stub_77e528() -> ! {
    todo!("0x77e528 boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>::list2(boost::_bi::value<std::string>,boost::arg<1>)")
}
// 0x77e644 — __ZN5boost3_bi8storage2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_
// type: unknown
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::arg<1>>::storage2(boost::_bi::value<std::string>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_")]
pub fn stub_77e644() -> ! {
    todo!("0x77e644 boost::_bi::storage2<boost::_bi::value<std::string>,boost::arg<1>>::storage2(boost::_bi::value<std::string>,boost::arg<1>)")
}
// 0x77e760 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEEixERS9_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::operator[](RBX::Script const* const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEEixERS9_")]
pub fn stub_77e760() -> ! {
    todo!("0x77e760 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::operator[](RBX::Script const* const&)")
}
// 0x77e8e0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm")]
pub fn stub_77e8e0() -> ! {
    todo!("0x77e8e0 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::reserve_for_insert(unsigned long)")
}
// 0x77e930 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm")]
pub fn stub_77e930() -> ! {
    todo!("0x77e930 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::create_buckets(unsigned long)")
}
// 0x77ea58 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE20min_buckets_for_sizeEm
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE20min_buckets_for_sizeEm")]
pub fn stub_77ea58() -> ! {
    todo!("0x77ea58 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::min_buckets_for_size(unsigned long)const")
}
// 0x77eae8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE11rehash_implEm
// type: unknown
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::rehash_impl(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE11rehash_implEm")]
pub fn stub_77eae8() -> ! {
    todo!("0x77eae8 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::rehash_impl(unsigned long)")
}
// 0x77eb14 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
// type: unknown
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE")]
pub fn stub_77eb14() -> ! {
    todo!("0x77eb14 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>> &,boost::unordered::detail::ptr_bucket *)")
}
// 0x77eb6c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEEEEE9constructEv
// type: unknown
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>>>::construct(void)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEEEEE9constructEv")]
pub fn stub_77eb6c() -> ! {
    todo!("0x77eb6c boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>>>::construct(void)")
}
// 0x77eba4 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14find_node_implIS8_SI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
// type: unknown
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::find_node_impl<RBX::Script const*,std::equal_to<RBX::Script const*>>(unsigned long,RBX::Script const* const&,std::equal_to<RBX::Script const*> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14find_node_implIS8_SI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_")]
pub fn stub_77eba4() -> ! {
    todo!("0x77eba4 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::find_node_impl<RBX::Script const*,std::equal_to<RBX::Script const*>>(unsigned long,RBX::Script const* const&,std::equal_to<RBX::Script const*> const&)const")
}
// 0x77ec10 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE9erase_keyERS9_
// type: unknown
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::erase_key(RBX::Script const* const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE9erase_keyERS9_")]
pub fn stub_77ec10() -> ! {
    todo!("0x77ec10 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::erase_key(RBX::Script const* const&)")
}
// 0x77ec98 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE12delete_nodesEPNS1_10ptr_bucketESM_
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE12delete_nodesEPNS1_10ptr_bucketESM_")]
pub fn stub_77ec98() -> ! {
    todo!("0x77ec98 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")
}
// 0x77ecd4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE10fix_bucketEmPNS1_10ptr_bucketE
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE10fix_bucketEmPNS1_10ptr_bucketE")]
pub fn stub_77ecd4() -> ! {
    todo!("0x77ecd4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")
}
// 0x77ed18 — __ZN5boost10shared_ptrIN3RBX10BaseScriptEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: unknown
#[doc(alias = "boost::shared_ptr<RBX::BaseScript>::shared_ptr<RBX::BaseScript>(boost::weak_ptr<RBX::BaseScript> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10BaseScriptEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_77ed18() -> ! {
    todo!("0x77ed18 boost::shared_ptr<RBX::BaseScript>::shared_ptr<RBX::BaseScript>(boost::weak_ptr<RBX::BaseScript> const&,boost::detail::sp_nothrow_tag)")
}
// 0x77ed94 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS8_EEEESA_EEPT_RKNS5_IT0_EE
// type: unknown
#[doc(alias = "rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> const&)")]
#[doc(alias = "__ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS8_EEEESA_EEPT_RKNS5_IT0_EE")]
pub fn stub_77ed94() -> ! {
    todo!("0x77ed94 rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> const&)")
}
// 0x77edf0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS4_RKS5_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::vector(unsigned long,boost::shared_ptr<RBX::Instance> const&,std::allocator<boost::shared_ptr<RBX::Instance>> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS4_RKS5_")]
pub fn stub_77edf0() -> ! {
    todo!("0x77edf0 std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::vector(unsigned long,boost::shared_ptr<RBX::Instance> const&,std::allocator<boost::shared_ptr<RBX::Instance>> const&)")
}
// 0x77eeb8 — __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS5_
// type: unknown
#[doc(alias = "std::_Vector_base<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_Vector_base(unsigned long,std::allocator<boost::shared_ptr<RBX::Instance>> const&)")]
#[doc(alias = "__ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS5_")]
pub fn stub_77eeb8() -> ! {
    todo!("0x77eeb8 std::_Vector_base<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_Vector_base(unsigned long,std::allocator<boost::shared_ptr<RBX::Instance>> const&)")
}
// 0x77eeec — __ZN5boost10shared_ptrISt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEC2IS6_N3rbx6detail13sp_ms_deleterIS6_EEEEPT_T0_
// type: int __fastcall(int)
#[doc(alias = "boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)")]
#[doc(alias = "__ZN5boost10shared_ptrISt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEC2IS6_N3rbx6detail13sp_ms_deleterIS6_EEEEPT_T0_")]
pub fn stub_77eeec() -> ! {
    todo!("0x77eeec boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)")
}
// 0x77eff4 — __ZN5boost6detail12shared_countC2IPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEN3rbx6detail13sp_ms_deleterIS9_EEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEN3rbx6detail13sp_ms_deleterIS9_EEEET_T0_")]
pub fn stub_77eff4() -> ! {
    todo!("0x77eff4 boost::detail::shared_count::shared_count<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)")
}
// 0x77f0f8 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED1Ev
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED1Ev")]
pub fn stub_77f0f8() -> ! {
    todo!("0x77f0f8 boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::~sp_counted_impl_pd()")
}
// 0x77f124 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED0Ev
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED0Ev")]
pub fn stub_77f124() -> ! {
    todo!("0x77f124 boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::~sp_counted_impl_pd()")
}
// 0x77f1dc — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE7disposeEv
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE7disposeEv")]
pub fn stub_77f1dc() -> ! {
    todo!("0x77f1dc boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::dispose(void)")
}
// 0x77f1f8 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE11get_deleterERKSt9type_info
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE11get_deleterERKSt9type_info")]
pub fn stub_77f1f8() -> ! {
    todo!("0x77f1f8 boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::get_deleter(std::type_info const&)")
}
// 0x77f210 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE19get_untyped_deleterEv
// type: unknown
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE19get_untyped_deleterEv")]
pub fn stub_77f210() -> ! {
    todo!("0x77f210 boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::get_untyped_deleter(void)")
}
// 0x77f214 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14delete_bucketsEv
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::delete_buckets(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14delete_bucketsEv")]
pub fn stub_77f214() -> ! {
    todo!("0x77f214 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::delete_buckets(void)")
}
// 0x77f244 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
// type: unknown
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::table(unsigned long,boost::hash<RBX::Script const*> const&,std::equal_to<RBX::Script const*> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE")]
pub fn stub_77f244() -> ! {
    todo!("0x77f244 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::table(unsigned long,boost::hash<RBX::Script const*> const&,std::equal_to<RBX::Script const*> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>> const&)")
}
// 0x77f2b0 — __ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// type: unknown
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_77f2b0() -> ! {
    todo!("0x77f2b0 __ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")
}
// 0x77f2b4 — __ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// type: unknown
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_77f2b4() -> ! {
    todo!("0x77f2b4 __ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")
}
// 0x77f354 — __ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// type: unknown
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_77f354() -> ! {
    todo!("0x77f354 __ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")
}
// 0x77f35c — __ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// type: unknown
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_77f35c() -> ! {
    todo!("0x77f35c __ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")
}
// 0x77f400 — __ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_77f400() -> ! {
    todo!("0x77f400 __ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")
}
// 0x77f408 — __ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// type: unknown
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_77f408() -> ! {
    todo!("0x77f408 __ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")
}
// 0x77f4ac — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::BoundFuncDesc(void (RBX::Scripting::DebuggerWatch::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_77f4ac() -> ! {
    todo!("0x77f4ac RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::BoundFuncDesc(void (RBX::Scripting::DebuggerWatch::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}
// 0x77f5b0 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EED0Ev")]
pub fn stub_77f5b0() -> ! {
    todo!("0x77f5b0 RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::~BoundFuncDesc()")
}
// 0x77f664 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_77f664() -> ! {
    todo!("0x77f664 RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}
// 0x77f688 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting13DebuggerWatchEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerWatch>(char const*,char const*,std::string  RBX::Scripting::DebuggerWatch::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting13DebuggerWatchEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_77f688() -> ! {
    todo!("0x77f688 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerWatch>(char const*,char const*,std::string  RBX::Scripting::DebuggerWatch::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}
// 0x77f81c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE10isReadOnlyEv")]
pub fn stub_77f81c() -> ! {
    todo!("0x77f81c RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isReadOnly(void)const")
}
// 0x77f820 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE11isWriteOnlyEv")]
pub fn stub_77f820() -> ! {
    todo!("0x77f820 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isWriteOnly(void)const")
}
// 0x77f824 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_77f824() -> ! {
    todo!("0x77f824 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::getValue(RBX::Reflection::DescribedBase const*)const")
}
// 0x77f83c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8setValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8setValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_77f83c() -> ! {
    todo!("0x77f83c RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}
// 0x77f8a4 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,std::string  RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_77f8a4() -> ! {
    todo!("0x77f8a4 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,std::string  RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}
// 0x77fa34 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv")]
pub fn stub_77fa34() -> ! {
    todo!("0x77fa34 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")
}
// 0x77fa38 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv")]
pub fn stub_77fa38() -> ! {
    todo!("0x77fa38 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")
}
// 0x77fa3c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_77fa3c() -> ! {
    todo!("0x77fa3c RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")
}
// 0x77fa54 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_77fa54() -> ! {
    todo!("0x77fa54 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}
// 0x77fabc — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,bool RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_77fabc() -> ! {
    todo!("0x77fabc RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,bool RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}
// 0x77fc50 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv")]
pub fn stub_77fc50() -> ! {
    todo!("0x77fc50 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")
}
// 0x77fc54 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv")]
pub fn stub_77fc54() -> ! {
    todo!("0x77fc54 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")
}
// 0x77fc58 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_77fc58() -> ! {
    todo!("0x77fc58 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")
}
// 0x77fc64 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKb
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_77fc64() -> ! {
    todo!("0x77fc64 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}
// 0x77fcb4 — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,int RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_77fcb4() -> ! {
    todo!("0x77fcb4 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,int RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}
// 0x77fe48 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv")]
pub fn stub_77fe48() -> ! {
    todo!("0x77fe48 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")
}
// 0x77fe4c — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv")]
pub fn stub_77fe4c() -> ! {
    todo!("0x77fe4c RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")
}
// 0x77fe50 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_77fe50() -> ! {
    todo!("0x77fe50 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")
}
// 0x77fe5c — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKi
// type: unknown
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_77fe5c() -> ! {
    todo!("0x77fe5c RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}
// 0x77feac — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::PropDescriptor<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int>(char const*,char const*,int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_77feac() -> ! {
    todo!("0x77feac RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::PropDescriptor<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int>(char const*,char const*,int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}
// 0x77ffb8 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiED0Ev")]
pub fn stub_77ffb8() -> ! {
    todo!("0x77ffb8 RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::~PropDescriptor()")
}
// 0x77ffe4 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE10isReadOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE10isReadOnlyEv")]
pub fn stub_77ffe4() -> ! {
    todo!("0x77ffe4 RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::isReadOnly(void)const")
}
// 0x77ffe8 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE11isWriteOnlyEv
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE11isWriteOnlyEv")]
pub fn stub_77ffe8() -> ! {
    todo!("0x77ffe8 RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::isWriteOnly(void)const")
}
// 0x77ffec — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE8getValueEPKNS0_13DescribedBaseE
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_77ffec() -> ! {
    todo!("0x77ffec RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}
// 0x78000c — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE8setValueEPNS0_13DescribedBaseERKi
// type: unknown
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_78000c() -> ! {
    todo!("0x78000c RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}
// 0x78012c — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_78012c() -> ! {
    todo!("0x78012c RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}
// 0x7802b0 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev")]
pub fn stub_7802b0() -> ! {
    todo!("0x7802b0 RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")
}
// 0x780364 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE")]
pub fn stub_780364() -> ! {
    todo!("0x780364 RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}
// 0x7804b8 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE")]
pub fn stub_7804b8() -> ! {
    todo!("0x7804b8 RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}
// 0x780618 — __ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_780618() -> ! {
    todo!("0x780618 RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}
// 0x780630 — __ZNK5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEclES4_
// type: unknown
#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::Instance>>::operator()(boost::shared_ptr<RBX::Instance>)const")]
#[doc(alias = "__ZNK5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEclES4_")]
pub fn stub_780630() -> ! {
    todo!("0x780630 boost::function1<void,boost::shared_ptr<RBX::Instance>>::operator()(boost::shared_ptr<RBX::Instance>)const")
}
// 0x780744 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_ED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_ED0Ev")]
pub fn stub_780744() -> ! {
    todo!("0x780744 RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")
}
// 0x7807f8 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_7807f8() -> ! {
    todo!("0x7807f8 RBX::Reflection::EventDescImpl<0,RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}
// 0x7809fc — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
pub fn stub_7809fc() -> ! {
    todo!("0x7809fc RBX::Reflection::EventDescImpl<0,RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}
// 0x780a70 — __ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_780a70() -> ! {
    todo!("0x780a70 RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}
// 0x780a84 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::EventDesc(rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_780a84() -> ! {
    todo!("0x780a84 RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::EventDesc(rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}
// 0x780c08 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_ED0Ev
// type: unknown
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_ED0Ev")]
pub fn stub_780c08() -> ! {
    todo!("0x780c08 RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")
}
// 0x780cbc — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_780cbc() -> ! {
    todo!("0x780cbc RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}
// 0x780e10 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
pub fn stub_780e10() -> ! {
    todo!("0x780e10 RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}
// 0x780e9c — __ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// type: unknown
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_780e9c() -> ! {
    todo!("0x780e9c RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}
// 0x780eb0 — __ZN3rbx7signals6signalIFviEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(int)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE13disconnectAllEv")]
pub fn stub_780eb0() -> ! {
    todo!("0x780eb0 rbx::signals::signal<void ()(int)>::disconnectAll(void)")
}
// 0x781028 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKiNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,int const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(int const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKiNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_")]
pub fn stub_781028() -> ! {
    todo!("0x781028 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,int const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(int const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")
}
// 0x781148 — __ZN5boost9function1IviE5clearEv
// type: unknown
#[doc(alias = "boost::function1<void,int>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IviE5clearEv")]
pub fn stub_781148() -> ! {
    todo!("0x781148 boost::function1<void,int>::clear(void)")
}
// 0x781178 — __ZN5boost9function1IviEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IviEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function1IviEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_781178() -> ! {
    todo!("0x781178 __ZN5boost9function1IviEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}
// 0x781260 — __ZN5boost9function1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_781260() -> ! {
    todo!("0x781260 void boost::function1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")
}
// 0x781358 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// type: unknown
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
pub fn stub_781358() -> ! {
    todo!("0x781358 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}
// 0x781378 — __ZNK5boost6detail8function13basic_vtable1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_781378() -> ! {
    todo!("0x781378 bool boost::detail::function::basic_vtable1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}
// 0x781460 — __ZNK5boost6detail8function13basic_vtable1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_781460() -> ! {
    todo!("0x781460 bool boost::detail::function::basic_vtable1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}
// 0x781548 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIiEEvRT_
// type: unknown
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<int>(int &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIiEEvRT_")]
pub fn stub_781548() -> ! {
    todo!("0x781548 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<int>(int &)")
}
// 0x781560 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_781560() -> ! {
    todo!("0x781560 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}
