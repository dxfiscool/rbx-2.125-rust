//! core shard jl — 150 core stubs EA-sorted, 0x778d94..0x781e80 (EA-sorted ascending, next 150 uncovered after shard A 0x777a9c, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) EA-sorted ascending, next 150 not yet in crates/core/src after shard A (0x777a9c), global filler.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>::list4(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>)")]
// 0x778d94 — __ZN5boost3_bi5list4INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS2_IiEENS_3argILi1EEEEC2ES3_S9_SA_SC_
// was: boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>::list4(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>)
pub fn stub_0x778d94() {
    // IDA 0x778d94: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>::storage4(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>)")]
// 0x778ebc — __ZN5boost3_bi8storage4INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS2_IiEENS_3argILi1EEEEC2ES3_S9_SA_SC_
// was: boost::_bi::storage4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>::storage4(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>)
pub fn stub_0x778ebc() {
    // IDA 0x778ebc: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>>::storage3(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>)")]
// 0x778fe4 — __ZN5boost3_bi8storage3INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS2_IiEEEC2ES3_S9_SA_
// was: boost::_bi::storage3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>>::storage3(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>)
pub fn stub_0x778fe4() {
    // IDA 0x778fe4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x77910c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_P9lua_StateENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x77910c() {
    // IDA 0x77910c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
// 0x77916c — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_P9lua_StateENS3_5list1INS_3argILi1EEEEEEENS5_IKSG_EESJ_P9lua_DebugE6invokeERNS1_15function_bufferESJ_SU_
// was: boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)
pub fn stub_0x77916c() {
    // IDA 0x77916c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(int,lua_State *),boost::_bi::list2<boost::_bi::value<int>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x779234 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_iP9lua_StateENS3_5list2INS3_5valueIiEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(int,lua_State *),boost::_bi::list2<boost::_bi::value<int>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x779234() {
    // IDA 0x779234: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(int,lua_State *),boost::_bi::list2<boost::_bi::value<int>,boost::arg<1>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
// 0x779294 — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_iP9lua_StateENS3_5list2INS3_5valueIiEENS_3argILi1EEEEEEENS5_IKSG_EESJ_P9lua_DebugE6invokeERNS1_15function_bufferESJ_SW_
// was: boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(int,lua_State *),boost::_bi::list2<boost::_bi::value<int>,boost::arg<1>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)
pub fn stub_0x779294() {
    // IDA 0x779294: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
// 0x779360 — __ZNK5boost9function2INS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEP9lua_StateP9lua_DebugEclESG_SI_
// was: boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const
pub fn stub_0x779360() {
    // IDA 0x779360: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES1C_")]
// 0x77942c — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES1C_
// was: __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES1C_
pub fn stub_0x77942c() {
    // IDA 0x77942c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
// 0x779544 — __ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIKSsSC_EEEEES5_S7_EEERSL_RNS9_ISsEEPS3_NS_3argILi1EEENSS_ILi2EEESN_NS_17reference_wrapperISL_EENSV_ISP_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5IS10_T0_T1_T2_T3_T4_T5_EENSY_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMS13_FS10_S14_S15_S16_S17_S18_ES1B_S1C_S1D_S1E_S1F_S1G_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
// was: boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)
pub fn stub_0x779544() {
    // IDA 0x779544: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void RBX::Scripting::ScriptDebugger::withPausedThreadHook<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &)")]
// 0x779668 — __ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookIN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEEEvP9lua_StateP9lua_DebugNS3_8functionIFT_SI_SK_EEERSM_RNS4_ISsEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, char, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int)
// was: void RBX::Scripting::ScriptDebugger::withPausedThreadHook<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &)
pub fn stub_0x779668() {
    // IDA 0x779668: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::assign_to_own(boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *> const&)")]
// 0x7799fc — __ZN5boost9function2INS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEP9lua_StateP9lua_DebugE13assign_to_ownERKSJ_
// was: boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::assign_to_own(boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *> const&)
pub fn stub_0x7799fc() {
    // IDA 0x7799fc: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
// 0x779a2c — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEC2ES7_S9_SA_SV_SX_SZ_
// was: boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)
pub fn stub_0x779a2c() {
    // IDA 0x779a2c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
// 0x779b00 — __ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEC2ES7_S9_SA_SV_SX_SZ_
// was: boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)
pub fn stub_0x779b00() {
    // IDA 0x779b00: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>)")]
// 0x779bd8 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EEEC2ES7_S9_SA_SV_SX_
// was: boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>)
pub fn stub_0x779bd8() {
    // IDA 0x779bd8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE")]
// 0x779cb4 — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE
pub fn stub_0x779cb4() {
    // IDA 0x779cb4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSB_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE")]
// 0x779da0 — __ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSB_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSB_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE
pub fn stub_0x779da0() {
    // IDA 0x779da0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>)")]
// 0x779e8c — __ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSB_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEEvT_
// was: void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>)
pub fn stub_0x779e8c() {
    // IDA 0x779e8c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x779f88 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEE6manageERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x779f88() {
    // IDA 0x779f88: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
// 0x779fa4 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int)
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)
pub fn stub_0x779fa4() {
    // IDA 0x779fa4: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &)const")]
// 0x779fc8 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x779fc8() {
    // IDA 0x779fc8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x77a0b8 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x77a0b8() {
    // IDA 0x77a0b8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x77a1a0 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x77a1a0() {
    // IDA 0x77a1a0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")]
// 0x77a27c — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEclINS_4_mfi3mf5IvS5_SQ_SS_SU_RSO_RSY_EENS0_5list2IRSQ_RSS_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int)
// was: void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)
pub fn stub_0x77a27c() {
    // IDA 0x77a27c: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &)const")]
// 0x77a360 — __ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS2_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEES6_S8_EEERSM_RNSA_ISsEEEclEPS4_S6_S8_SO_SP_SR_
// was: boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &)const
pub fn stub_0x77a360() {
    // IDA 0x77a360: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x77a44c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEE7managerERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x77a44c() {
    // IDA 0x77a44c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&)")]
// 0x77b058 — __ZNSt6vectorIPN3RBX9Scripting13DebuggerWatchESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int, void *__src)
// was: std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&)
pub fn stub_0x77b058() {
    // IDA 0x77b058: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::_M_allocate(unsigned long)")]
// 0x77b138 — __ZNSt12_Vector_baseIPN3RBX9Scripting13DebuggerWatchESaIS3_EE11_M_allocateEm
// was: std::_Vector_base<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::_M_allocate(unsigned long)
pub fn stub_0x77b138() {
    // IDA 0x77b138: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch *>(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&,std::random_access_iterator_tag)")]
// 0x77b600 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX9Scripting13DebuggerWatchESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_St26random_access_iterator_tag
// was: __gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch *>(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&,std::random_access_iterator_tag)
pub fn stub_0x77b600() {
    // IDA 0x77b600: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting13DebuggerWatchELZNS2_14sDebuggerWatchEENS_14FactoryProductIS3_NS_8InstanceELZNS2_14sDebuggerWatchEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE2EE15classDescriptorEv")]
// 0x77b790 — __ZN3RBX10Reflection9DescribedINS_9Scripting13DebuggerWatchELZNS2_14sDebuggerWatchEENS_14FactoryProductIS3_NS_8InstanceELZNS2_14sDebuggerWatchEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE2EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
// was: __ZN3RBX10Reflection9DescribedINS_9Scripting13DebuggerWatchELZNS2_14sDebuggerWatchEENS_14FactoryProductIS3_NS_8InstanceELZNS2_14sDebuggerWatchEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE2EE15classDescriptorEv
pub fn stub_0x77b790() {
    // IDA 0x77b790: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Scripting::ScriptDebugger::FunctionInfo*,std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>>,RBX::Scripting::ScriptDebugger::FunctionInfo const&)")]
// 0x77b8ac — __ZNSt6vectorIN3RBX9Scripting14ScriptDebugger12FunctionInfoESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// was: std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Scripting::ScriptDebugger::FunctionInfo*,std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>>,RBX::Scripting::ScriptDebugger::FunctionInfo const&)
pub fn stub_0x77b8ac() {
    // IDA 0x77b8ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::FunctionInfo::operator=(RBX::Scripting::ScriptDebugger::FunctionInfo const&)")]
// 0x77c1c4 — __ZN3RBX9Scripting14ScriptDebugger12FunctionInfoaSERKS2_
// was: RBX::Scripting::ScriptDebugger::FunctionInfo::operator=(RBX::Scripting::ScriptDebugger::FunctionInfo const&)
pub fn stub_0x77c1c4() {
    // IDA 0x77c1c4: script-debugger wiring owned by the script crate — carrier no-op in core.
}

#[doc(alias = "std::_Vector_base<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::_M_allocate(unsigned long)")]
// 0x77c214 — __ZNSt12_Vector_baseIN3RBX9Scripting14ScriptDebugger12FunctionInfoESaIS3_EE11_M_allocateEm
// was: std::_Vector_base<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::_M_allocate(unsigned long)
pub fn stub_0x77c214() {
    // IDA 0x77c214: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::FunctionInfo * std::__uninitialized_copy_a<RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo>(RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>)")]
// 0x77c238 — __ZSt22__uninitialized_copy_aIPN3RBX9Scripting14ScriptDebugger12FunctionInfoES4_S3_ET0_T_S6_S5_SaIT1_E
// was: RBX::Scripting::ScriptDebugger::FunctionInfo * std::__uninitialized_copy_a<RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo>(RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>)
pub fn stub_0x77c238() {
    // IDA 0x77c238: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::FunctionInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *>(RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *)")]
// 0x77c538 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9Scripting14ScriptDebugger12FunctionInfoES7_EET0_T_S9_S8_
// was: RBX::Scripting::ScriptDebugger::FunctionInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *>(RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *)
pub fn stub_0x77c538() {
    // IDA 0x77c538: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::~vector()")]
// 0x77c594 — __ZNSt6vectorIN3RBX9Scripting14ScriptDebugger12FunctionInfoESaIS3_EED2Ev
// was: std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::~vector()
pub fn stub_0x77c594() {
    // IDA 0x77c594: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::find(std::string const&)")]
// 0x77c700 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10Reflection7VariantEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
// type: int __fastcall(int, std::string *this)
// was: std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::find(std::string const&)
pub fn stub_0x77c700() {
    // IDA 0x77c700: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> const&)")]
// 0x77c750 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEESE_EEPT_RKNS_10shared_ptrIT0_EE
// was: rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> const&)
pub fn stub_0x77c750() {
    // IDA 0x77c750: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>(std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>)")]
// 0x77c7ac — __ZN5boost10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS4_EEEEC2ISB_N3rbx6detail13sp_ms_deleterISB_EEEEPT_T0_
// was: rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>(std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>)
pub fn stub_0x77c7ac() {
    // IDA 0x77c7ac: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>(std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>)")]
// 0x77c8b8 — __ZN5boost6detail12shared_countC2IPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEN3rbx6detail13sp_ms_deleterISD_EEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>(std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>)
pub fn stub_0x77c8b8() {
    // IDA 0x77c8b8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()")]
// 0x77c9c0 — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEED1Ev
// was: boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()
pub fn stub_0x77c9c0() {
    // IDA 0x77c9c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()")]
// 0x77c9ec — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEED0Ev
// was: boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()
pub fn stub_0x77c9ec() {
    // IDA 0x77c9ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::dispose(void)")]
// 0x77caa8 — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::dispose(void)
pub fn stub_0x77caa8() {
    // IDA 0x77caa8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_deleter(std::type_info const&)")]
// 0x77cac8 — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_deleter(std::type_info const&)
pub fn stub_0x77cac8() {
    // IDA 0x77cac8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_untyped_deleter(void)")]
// 0x77cae0 — __ZN5boost6detail18sp_counted_impl_pdIPSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEN3rbx6detail13sp_ms_deleterISC_EEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_untyped_deleter(void)
pub fn stub_0x77cae0() {
    // IDA 0x77cae0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSB_5list2INSB_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x77cae4 — __ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSB_5list2INSB_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSB_5list2INSB_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_0x77cae4() {
    // IDA 0x77cae4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x77cc0c — __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
pub fn stub_0x77cc0c() {
    // IDA 0x77cc0c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>)")]
// 0x77cd38 — __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEEvT_
// was: void boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>)
pub fn stub_0x77cd38() {
    // IDA 0x77cd38: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x77ce74 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x77ce74() {
    // IDA 0x77ce74: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,RBX::Reflection::Variant,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
// 0x77cef4 — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEES7_S9_P9lua_DebugE6invokeERNS1_15function_bufferES9_SK_
// was: boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,RBX::Reflection::Variant,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)
pub fn stub_0x77cef4() {
    // IDA 0x77cef4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// 0x77cf18 — __ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x77cf18() {
    // IDA 0x77cf18: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x77d044 — __ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x77d044() {
    // IDA 0x77d044: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::Reflection::Variant boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>::operator()<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<RBX::Reflection::Variant>,RBX::Reflection::Variant (*)(std::string,lua_State *) &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,long)")]
// 0x77d17c — __ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEclIN3RBX10Reflection7VariantEPFSA_SsP9lua_StateENS1_IRSC_RP9lua_DebugEEEET_NS0_4typeISK_EERT0_RT1_l
// was: RBX::Reflection::Variant boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>::operator()<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<RBX::Reflection::Variant>,RBX::Reflection::Variant (*)(std::string,lua_State *) &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,long)
pub fn stub_0x77d17c() {
    // IDA 0x77d17c: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
// 0x77d2a0 — __ZNK5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEclES5_S7_
// was: boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const
pub fn stub_0x77d2a0() {
    // IDA 0x77d2a0: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES12_")]
// 0x77d36c — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES12_
// was: __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES12_
pub fn stub_0x77d36c() {
    // IDA 0x77d36c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
// 0x77d484 — __ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS1_10Reflection7VariantES5_S7_EEERSA_RNS_10shared_ptrISsEEPS3_NS_3argILi1EEENSI_ILi2EEESC_NS_17reference_wrapperISA_EENSL_ISF_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISQ_T0_T1_T2_T3_T4_T5_EENSO_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMST_FSQ_SU_SV_SW_SX_SY_ES11_S12_S13_S14_S15_S16_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
// was: boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)
pub fn stub_0x77d484() {
    // IDA 0x77d484: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void RBX::Scripting::ScriptDebugger::withPausedThreadHook<RBX::Reflection::Variant>(lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &)")]
// 0x77d5a8 — __ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookINS_10Reflection7VariantEEEvP9lua_StateP9lua_DebugN5boost8functionIFT_S6_S8_EEERSB_RNS9_10shared_ptrISsEE
// was: void RBX::Scripting::ScriptDebugger::withPausedThreadHook<RBX::Reflection::Variant>(lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &)
pub fn stub_0x77d5a8() {
    // IDA 0x77d5a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to_own(boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *> const&)")]
// 0x77d978 — __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE13assign_to_ownERKS8_
// was: boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to_own(boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *> const&)
pub fn stub_0x77d978() {
    // IDA 0x77d978: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
// 0x77d9a8 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_
// was: boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)
pub fn stub_0x77d9a8() {
    // IDA 0x77d9a8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
// 0x77da7c — __ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_
// was: boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)
pub fn stub_0x77da7c() {
    // IDA 0x77da7c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>)")]
// 0x77db54 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EEEC2ES7_S9_SA_SK_SM_
// was: boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>)
pub fn stub_0x77db54() {
    // IDA 0x77db54: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
// 0x77dc30 — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE
pub fn stub_0x77dc30() {
    // IDA 0x77dc30: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
// 0x77dd1c — __ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE
pub fn stub_0x77dd1c() {
    // IDA 0x77dd1c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>)")]
// 0x77de08 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEEvT_
// was: void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>)
pub fn stub_0x77de08() {
    // IDA 0x77de08: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x77df04 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE6manageERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x77df04() {
    // IDA 0x77df04: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
// 0x77df20 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int)
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)
pub fn stub_0x77df20() {
    // IDA 0x77df20: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &)const")]
// 0x77df44 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x77df44() {
    // IDA 0x77df44: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x77e034 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x77e034() {
    // IDA 0x77e034: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x77e11c — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x77e11c() {
    // IDA 0x77e11c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")]
// 0x77e1f8 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEclINS_4_mfi3mf5IvS5_SF_SH_SJ_RSD_RSO_EENS0_5list2IRSF_RSH_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int)
// was: void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)
pub fn stub_0x77e1f8() {
    // IDA 0x77e1f8: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &)const")]
// 0x77e2dc — __ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS2_10Reflection7VariantES6_S8_EEERSB_RNS_10shared_ptrISsEEEclEPS4_S6_S8_SD_SE_SH_
// was: boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &)const
pub fn stub_0x77e2dc() {
    // IDA 0x77e2dc: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x77e3c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE7managerERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x77e3c8() {
    // IDA 0x77e3c8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> const&)")]
// 0x77ed94 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS8_EEEESA_EEPT_RKNS5_IT0_EE
// was: rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> const&)
pub fn stub_0x77ed94() {
    // IDA 0x77ed94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::vector(unsigned long,rbx_core::SharedPtr<RBX::Instance> const&,std::allocator<rbx_core::SharedPtr<RBX::Instance>> const&)")]
// 0x77edf0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS4_RKS5_
// type: int __fastcall(int, int, int, int, int)
// was: std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::vector(unsigned long,rbx_core::SharedPtr<RBX::Instance> const&,std::allocator<rbx_core::SharedPtr<RBX::Instance>> const&)
pub fn stub_0x77edf0() {
    // IDA 0x77edf0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_Vector_base(unsigned long,std::allocator<rbx_core::SharedPtr<RBX::Instance>> const&)")]
// 0x77eeb8 — __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS5_
// was: std::_Vector_base<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_Vector_base(unsigned long,std::allocator<rbx_core::SharedPtr<RBX::Instance>> const&)
pub fn stub_0x77eeb8() {
    // IDA 0x77eeb8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>::shared_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
// 0x77eeec — __ZN5boost10shared_ptrISt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEC2IS6_N3rbx6detail13sp_ms_deleterIS6_EEEEPT_T0_
// type: int __fastcall(int)
// was: rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>::shared_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)
pub fn stub_0x77eeec() {
    // IDA 0x77eeec: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
// 0x77eff4 — __ZN5boost6detail12shared_countC2IPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEN3rbx6detail13sp_ms_deleterIS9_EEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)
pub fn stub_0x77eff4() {
    // IDA 0x77eff4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::~sp_counted_impl_pd()")]
// 0x77f0f8 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED1Ev
// was: boost::detail::sp_counted_impl_pd<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::~sp_counted_impl_pd()
pub fn stub_0x77f0f8() {
    // IDA 0x77f0f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::~sp_counted_impl_pd()")]
// 0x77f124 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED0Ev
// was: boost::detail::sp_counted_impl_pd<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::~sp_counted_impl_pd()
pub fn stub_0x77f124() {
    // IDA 0x77f124: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::dispose(void)")]
// 0x77f1dc — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::dispose(void)
pub fn stub_0x77f1dc() {
    // IDA 0x77f1dc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::get_deleter(std::type_info const&)")]
// 0x77f1f8 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::get_deleter(std::type_info const&)
pub fn stub_0x77f1f8() {
    // IDA 0x77f1f8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::get_untyped_deleter(void)")]
// 0x77f210 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::get_untyped_deleter(void)
pub fn stub_0x77f210() {
    // IDA 0x77f210: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
// 0x77f2b0 — __ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// was: __ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
pub fn stub_0x77f2b0() {
    // IDA 0x77f2b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
// 0x77f2b4 — __ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// was: __ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
pub fn stub_0x77f2b4() {
    // IDA 0x77f2b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
// 0x77f354 — __ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
pub fn stub_0x77f354() {
    // IDA 0x77f354: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
// 0x77f35c — __ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
pub fn stub_0x77f35c() {
    // IDA 0x77f35c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
// 0x77f400 — __ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// type: void __fastcall(int)
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
pub fn stub_0x77f400() {
    // IDA 0x77f400: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
// 0x77f408 — __ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
pub fn stub_0x77f408() {
    // IDA 0x77f408: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::BoundFuncDesc(void (RBX::Scripting::DebuggerWatch::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x77f4ac — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::BoundFuncDesc(void (RBX::Scripting::DebuggerWatch::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x77f4ac() {
    // IDA 0x77f4ac: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::~BoundFuncDesc()")]
// 0x77f5b0 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::~BoundFuncDesc()
pub fn stub_0x77f5b0() {
    // IDA 0x77f5b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x77f664 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_0x77f664() {
    // IDA 0x77f664: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerWatch>(char const*,char const*,std::string  RBX::Scripting::DebuggerWatch::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x77f688 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting13DebuggerWatchEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerWatch>(char const*,char const*,std::string  RBX::Scripting::DebuggerWatch::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
pub fn stub_0x77f688() {
    // IDA 0x77f688: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isReadOnly(void)const")]
// 0x77f81c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE10isReadOnlyEv
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isReadOnly(void)const
pub fn stub_0x77f81c() {
    // IDA 0x77f81c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isWriteOnly(void)const")]
// 0x77f820 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE11isWriteOnlyEv
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isWriteOnly(void)const
pub fn stub_0x77f820() {
    // IDA 0x77f820: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x77f824 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8getValueEPKNS0_13DescribedBaseE
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::getValue(RBX::Reflection::DescribedBase const*)const
pub fn stub_0x77f824() {
    // IDA 0x77f824: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// 0x77f83c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8setValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, int, std::string *this)
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const
pub fn stub_0x77f83c() {
    // IDA 0x77f83c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,std::string  RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x77f8a4 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,std::string  RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
pub fn stub_0x77f8a4() {
    // IDA 0x77f8a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
// 0x77fa34 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const
pub fn stub_0x77fa34() {
    // IDA 0x77fa34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
// 0x77fa38 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const
pub fn stub_0x77fa38() {
    // IDA 0x77fa38: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x77fa3c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const
pub fn stub_0x77fa3c() {
    // IDA 0x77fa3c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// 0x77fa54 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, int, std::string *this)
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const
pub fn stub_0x77fa54() {
    // IDA 0x77fa54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,bool RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x77fabc — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// was: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,bool RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
pub fn stub_0x77fabc() {
    // IDA 0x77fabc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
// 0x77fc50 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv
// was: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const
pub fn stub_0x77fc50() {
    // IDA 0x77fc50: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
// 0x77fc54 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv
// was: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const
pub fn stub_0x77fc54() {
    // IDA 0x77fc54: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x77fc58 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE
// was: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const
pub fn stub_0x77fc58() {
    // IDA 0x77fc58: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// 0x77fc64 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKb
// was: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
pub fn stub_0x77fc64() {
    // IDA 0x77fc64: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,int RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x77fcb4 — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// was: RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,int RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
pub fn stub_0x77fcb4() {
    // IDA 0x77fcb4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
// 0x77fe48 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv
// was: RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const
pub fn stub_0x77fe48() {
    // IDA 0x77fe48: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
// 0x77fe4c — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv
// was: RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const
pub fn stub_0x77fe4c() {
    // IDA 0x77fe4c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x77fe50 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE
// was: RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const
pub fn stub_0x77fe50() {
    // IDA 0x77fe50: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// 0x77fe5c — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKi
// was: RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,int const&)const
pub fn stub_0x77fe5c() {
    // IDA 0x77fe5c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::PropDescriptor<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int>(char const*,char const*,int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x77feac — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::PropDescriptor<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int>(char const*,char const*,int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
pub fn stub_0x77feac() {
    // IDA 0x77feac: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::~PropDescriptor()")]
// 0x77ffb8 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiED0Ev
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::~PropDescriptor()
pub fn stub_0x77ffb8() {
    // IDA 0x77ffb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::isReadOnly(void)const")]
// 0x77ffe4 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE10isReadOnlyEv
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::isReadOnly(void)const
pub fn stub_0x77ffe4() {
    // IDA 0x77ffe4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::isWriteOnly(void)const")]
// 0x77ffe8 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE11isWriteOnlyEv
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::isWriteOnly(void)const
pub fn stub_0x77ffe8() {
    // IDA 0x77ffe8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x77ffec — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE8getValueEPKNS0_13DescribedBaseE
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const
pub fn stub_0x77ffec() {
    // IDA 0x77ffec: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// 0x78000c — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiE7GetImplIMS3_KFivEE8setValueEPNS0_13DescribedBaseERKi
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const
pub fn stub_0x78000c() {
    // IDA 0x78000c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x78012c — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x78012c() {
    // IDA 0x78012c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
// 0x7802b0 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev
// was: RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()
pub fn stub_0x7802b0() {
    // IDA 0x7802b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x780364 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
// was: RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0x780364() {
    // IDA 0x780364: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x7804b8 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// was: RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0x7804b8() {
    // IDA 0x7804b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x780618 — __ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE
// was: RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_0x780618() {
    // IDA 0x780618: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>)const")]
// 0x780630 — __ZNK5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEclES4_
// was: boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>)const
pub fn stub_0x780630() {
    // IDA 0x780630: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
// 0x780744 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_ED0Ev
// was: RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()
pub fn stub_0x780744() {
    // IDA 0x780744: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x7807f8 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
// was: RBX::Reflection::EventDescImpl<0,RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0x7807f8() {
    // IDA 0x7807f8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x7809fc — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// was: RBX::Reflection::EventDescImpl<0,RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0x7809fc() {
    // IDA 0x7809fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x780a70 — __ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// was: RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_0x780a70() {
    // IDA 0x780a70: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::EventDesc(rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x780a84 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::EventDesc(rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x780a84() {
    // IDA 0x780a84: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
// 0x780c08 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_ED0Ev
// was: RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()
pub fn stub_0x780c08() {
    // IDA 0x780c08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x780cbc — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
// was: RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_0x780cbc() {
    // IDA 0x780cbc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x780e10 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// was: RBX::Reflection::EventDescImpl<1,RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0x780e10() {
    // IDA 0x780e10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x780e9c — __ZNK3RBX10Reflection13EventDescBaseINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// was: RBX::Reflection::EventDescBase<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_0x780e9c() {
    // IDA 0x780e9c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,int const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(int const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// 0x781028 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKiNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,int const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(int const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
pub fn stub_0x781028() {
    // IDA 0x781028: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1IviEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x781178 — __ZN5boost9function1IviEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: __ZN5boost9function1IviEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_0x781178() {
    // IDA 0x781178: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// 0x781260 — __ZN5boost9function1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void boost::function1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
pub fn stub_0x781260() {
    // IDA 0x781260: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x781358 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x781358() {
    // IDA 0x781358: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// 0x781378 — __ZNK5boost6detail8function13basic_vtable1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x781378() {
    // IDA 0x781378: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x781460 — __ZNK5boost6detail8function13basic_vtable1IviE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable1<void,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x781460() {
    // IDA 0x781460: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<int>(int &)")]
// 0x781548 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIiEEvRT_
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<int>(int &)
pub fn stub_0x781548() {
    // IDA 0x781548: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x781560 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,int const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x781560() {
    // IDA 0x781560: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::PropDescriptor<int (RBX::Scripting::ScriptDebugger::*)(void)const,int>(char const*,char const*,int (RBX::Scripting::ScriptDebugger::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x781ac4 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::PropDescriptor<int (RBX::Scripting::ScriptDebugger::*)(void)const,int>(char const*,char const*,int (RBX::Scripting::ScriptDebugger::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
pub fn stub_0x781ac4() {
    // IDA 0x781ac4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::~PropDescriptor()")]
// 0x781bd0 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiED0Ev
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::~PropDescriptor()
pub fn stub_0x781bd0() {
    // IDA 0x781bd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::isReadOnly(void)const")]
// 0x781bfc — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiE7GetImplIMS3_KFivEE10isReadOnlyEv
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::isReadOnly(void)const
pub fn stub_0x781bfc() {
    // IDA 0x781bfc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::isWriteOnly(void)const")]
// 0x781c00 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiE7GetImplIMS3_KFivEE11isWriteOnlyEv
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::isWriteOnly(void)const
pub fn stub_0x781c00() {
    // IDA 0x781c00: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x781c04 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiE7GetImplIMS3_KFivEE8getValueEPKNS0_13DescribedBaseE
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const
pub fn stub_0x781c04() {
    // IDA 0x781c04: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// 0x781c24 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiE7GetImplIMS3_KFivEE8setValueEPNS0_13DescribedBaseERKi
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::GetImpl<int (RBX::Scripting::ScriptDebugger::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const
pub fn stub_0x781c24() {
    // IDA 0x781c24: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::PropDescriptor<bool (RBX::Scripting::ScriptDebugger::*)(void)const,int>(char const*,char const*,bool (RBX::Scripting::ScriptDebugger::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x781d44 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::PropDescriptor<bool (RBX::Scripting::ScriptDebugger::*)(void)const,int>(char const*,char const*,bool (RBX::Scripting::ScriptDebugger::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
pub fn stub_0x781d44() {
    // IDA 0x781d44: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::~PropDescriptor()")]
// 0x781e50 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbED0Ev
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::~PropDescriptor()
pub fn stub_0x781e50() {
    // IDA 0x781e50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::GetImpl<bool (RBX::Scripting::ScriptDebugger::*)(void)const>::isReadOnly(void)const")]
// 0x781e7c — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbE7GetImplIMS3_KFbvEE10isReadOnlyEv
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::GetImpl<bool (RBX::Scripting::ScriptDebugger::*)(void)const>::isReadOnly(void)const
pub fn stub_0x781e7c() {
    // IDA 0x781e7c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::GetImpl<bool (RBX::Scripting::ScriptDebugger::*)(void)const>::isWriteOnly(void)const")]
// 0x781e80 — __ZNK3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbE7GetImplIMS3_KFbvEE11isWriteOnlyEv
// was: RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::GetImpl<bool (RBX::Scripting::ScriptDebugger::*)(void)const>::isWriteOnly(void)const
pub fn stub_0x781e80() {
    // IDA 0x781e80: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

