// Auto-generated skeletons for rbx-script — Lua/Script filtered
// Filter: Lua|Script (4456 filtered, 1133 remaining not yet in any crate) -> next 120 EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x817bb4..0x90ebd4 | script 14091->14211 distinct (filtered)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; " and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x817bb4 — __ZN3RBX3Lua13LibraryBridge4pushEP9lua_StateRKNS0_7LibraryE
#[doc(alias = "RBX::Lua::LibraryBridge::push(lua_State *,RBX::Lua::Library const&)")]
pub fn stub_0x817bb4() -> ! {
    todo!("0x817bb4 RBX::Lua::LibraryBridge::push(lua_State *,RBX::Lua::Library const&)")
}

// 0x817dd4 — __ZN3RBX3Lua13LibraryBridge4findEP9lua_StateRKSs
#[doc(alias = "RBX::Lua::LibraryBridge::find(lua_State *,std::string const&)")]
pub fn stub_0x817dd4() -> ! {
    todo!("0x817dd4 RBX::Lua::LibraryBridge::find(lua_State *,std::string const&)")
}

// 0x817ebc — __ZN3RBX3Lua13LibraryBridge20registerClassLibraryEP9lua_State
#[doc(alias = "RBX::Lua::LibraryBridge::registerClassLibrary(lua_State *)")]
pub fn stub_0x817ebc() -> ! {
    todo!("0x817ebc RBX::Lua::LibraryBridge::registerClassLibrary(lua_State *)")
}

// 0x817ef4 — __ZN3RBX14LibraryServiceC1EPNS_13ScriptContextE
// type: _DWORD __fastcall(RBX::LibraryService *__hidden this, RBX::ScriptContext *)
#[doc(alias = "RBX::LibraryService::LibraryService(RBX::ScriptContext *)")]
pub fn stub_0x817ef4() -> ! {
    todo!("0x817ef4 RBX::LibraryService::LibraryService(RBX::ScriptContext *)")
}

// 0x818730 — __ZN3RBX14LibraryService18ContentReadyHelperEN5boost8weak_ptrINS_13ScriptContextEEESsSsNS_14AsyncHttpQueue13RequestResultEPSiNS1_10shared_ptrIKSsEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::LibraryService::ContentReadyHelper(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x818730() -> ! {
    todo!("0x818730 RBX::LibraryService::ContentReadyHelper(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)")
}

// 0x81957c — __ZN3RBX14LibraryService26registerDevelopmentLibraryERKSsN5boost10shared_ptrINS_6ScriptEEE
#[doc(alias = "RBX::LibraryService::registerDevelopmentLibrary(std::string const&,rbx_core::SharedPtr<RBX::Script>)")]
pub fn stub_0x81957c() -> ! {
    todo!("0x81957c RBX::LibraryService::registerDevelopmentLibrary(std::string const&,boost::shared_ptr<RBX::Script>)")
}

// 0x81aac0 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
#[doc(alias = "RBX::Lua::Library* RBX::Lua::Bridge<RBX::Lua::Library,true>::pushNewObject<RBX::Lua::Library>(lua_State *,RBX::Lua::Library)")]
pub fn stub_0x81aac0() -> ! {
    todo!("0x81aac0 RBX::Lua::Library* RBX::Lua::Bridge<RBX::Lua::Library,true>::pushNewObject<RBX::Lua::Library>(lua_State *,RBX::Lua::Library)")
}

// 0x81b960 — __ZNSt3mapISsN5boost10shared_ptrIN3RBX6ScriptEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_
#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::Script>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::operator[](std::string const&)")]
pub fn stub_0x81b960() -> ! {
    todo!("0x81b960 std::map<std::string,boost::shared_ptr<RBX::Script>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::operator[](std::string const&)")
}

// 0x81bb7c — __ZN5boost10shared_ptrIN3RBX6ScriptEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::Script>::operator=(rbx_core::SharedPtr<RBX::Script> const&)")]
pub fn stub_0x81bb7c() -> ! {
    todo!("0x81bb7c boost::shared_ptr<RBX::Script>::operator=(boost::shared_ptr<RBX::Script> const&)")
}

// 0x81c20c — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS2_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEES4_SsSsNS_3argILi1EEENSB_ILi2EEENSB_ILi3EEEEENS_3_bi6bind_tIT_PFSH_T0_T1_T2_T3_T4_T5_ENSF_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEESP_SR_SS_ST_SU_SV_SW_
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_6<rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0x81c20c() -> ! {
    todo!("0x81c20c boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list_av_6<boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0x81c550 — __ZN3RBX9weak_fromINS_13ScriptContextEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptContext> RBX::weak_from<RBX::ScriptContext>(RBX::ScriptContext*)")]
pub fn stub_0x81c550() -> ! {
    todo!("0x81c550 boost::weak_ptr<RBX::ScriptContext> RBX::weak_from<RBX::ScriptContext>(RBX::ScriptContext*)")
}

// 0x81c940 — __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua13WeakThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE
// type: int __fastcall(int32_t *__theValue)
#[doc(alias = "void rbx_core::SharedPtr_release<RBX::Lua::WeakThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef,int,0> const*)")]
pub fn stub_0x81c940() -> ! {
    todo!("0x81c940 void boost::intrusive_ptr_release<RBX::Lua::WeakThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef,int,0> const*)")
}

// 0x81ca80 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSB_5list6INSB_5valueISF_EENSJ_ISsEESL_NS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSB_5list6INSB_5valueISF_EENSJ_ISsEESL_NS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x81ca80() -> ! {
    todo!("0x81ca80 __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSB_5list6INSB_5valueISF_EENSJ_ISsEESL_NS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")
}

// 0x81cc30 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x81cc30() -> ! {
    todo!("0x81cc30 __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

// 0x81cde4 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_EC2ERKS9_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)")]
pub fn stub_0x81cde4() -> ! {
    todo!("0x81cde4 boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)")
}

// 0x81cf2c — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEEvT_
#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
pub fn stub_0x81cf2c() -> ! {
    todo!("0x81cf2c void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")
}

// 0x81d0f0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x81d0f0() -> ! {
    todo!("0x81d0f0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x81d10c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEvSA_SB_SE_E6invokeERNS1_15function_bufferESA_SB_SE_
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x81d10c() -> ! {
    todo!("0x81d10c boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)")
}

// 0x81d130 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x81d130() -> ! {
    todo!("0x81d130 bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")
}

// 0x81d2e8 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x81d2e8() -> ! {
    todo!("0x81d2e8 bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x81d498 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x81d498() -> ! {
    todo!("0x81d498 void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x81d558 — __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclIPFvS6_SsSsNS4_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS0_5list3IRSG_RSH_RSK_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&> &,int)")]
pub fn stub_0x81d558() -> ! {
    todo!("0x81d558 void boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const>&> &,int)")
}

// 0x81d7e4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x81d7e4() -> ! {
    todo!("0x81d7e4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x81d938 — __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_S8_SA_SB_SC_
#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::list6(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0x81d938() -> ! {
    todo!("0x81d938 boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::list6(boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0x81db6c — __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_S8_SA_SB_SC_
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0x81db6c() -> ! {
    todo!("0x81db6c boost::_bi::storage6<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage6(boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0x81dda0 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0x81dda0() -> ! {
    todo!("0x81dda0 boost::_bi::storage5<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>)")
}

// 0x81dfd4 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>)")]
pub fn stub_0x81dfd4() -> ! {
    todo!("0x81dfd4 boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>)")
}

// 0x81e208 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_EC2ES7_S8_S8_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn stub_0x81e208() -> ! {
    todo!("0x81e208 boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")
}

// 0x81e3e4 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEEEC2ES7_S8_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>)")]
pub fn stub_0x81e3e4() -> ! {
    todo!("0x81e3e4 boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::_bi::value<std::string>)")
}

// 0x81e558 — __ZN5boost8weak_ptrIN3RBX13ScriptContextEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptContext>::weak_ptr<RBX::ScriptContext>(rbx_core::SharedPtr<RBX::ScriptContext> const&,boost::detail::sp_enable_if_convertible<RBX::ScriptContext,RBX::ScriptContext>::type)")]
pub fn stub_0x81e558() -> ! {
    todo!("0x81e558 boost::weak_ptr<RBX::ScriptContext>::weak_ptr<RBX::ScriptContext>(boost::shared_ptr<RBX::ScriptContext> const&,boost::detail::sp_enable_if_convertible<RBX::ScriptContext,RBX::ScriptContext>::type)")
}

// 0x81eff8 — __ZN3RBX4Name7declareILZNS_12sLuaSettingsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sLuaSettingsEEEERKS0_v")]
pub fn stub_0x81eff8() -> ! {
    todo!("0x81eff8 __ZN3RBX4Name7declareILZNS_12sLuaSettingsEEEERKS0_v")
}

// 0x81f040 — __ZN3RBX4Name9doDeclareILZNS_12sLuaSettingsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sLuaSettingsEEEERKS0_v")]
pub fn stub_0x81f040() -> ! {
    todo!("0x81f040 __ZN3RBX4Name9doDeclareILZNS_12sLuaSettingsEEEERKS0_v")
}

// 0x81fc84 — __ZNSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEEC2ERS0_RKS5_
#[doc(alias = "std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>::pair(std::string const&,rbx_core::SharedPtr<RBX::Script> const&)")]
pub fn stub_0x81fc84() -> ! {
    todo!("0x81fc84 std::pair<std::string const,boost::shared_ptr<RBX::Script>>::pair(std::string const&,boost::shared_ptr<RBX::Script> const&)")
}

// 0x81fd40 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&)")]
pub fn stub_0x81fd40() -> ! {
    todo!("0x81fd40 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::pair<std::string const,boost::shared_ptr<RBX::Script>> const&)")
}

// 0x81fe2c — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&)")]
pub fn stub_0x81fe2c() -> ! {
    todo!("0x81fe2c std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,boost::shared_ptr<RBX::Script>> const&)")
}

// 0x81fe7c — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&)")]
pub fn stub_0x81fe7c() -> ! {
    todo!("0x81fe7c std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::_M_insert_unique(std::pair<std::string const,boost::shared_ptr<RBX::Script>> const&)")
}

// 0x81ff00 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_
// type: _DWORD *__fastcall(int, const shared_count *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&)")]
pub fn stub_0x81ff00() -> ! {
    todo!("0x81ff00 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::_M_create_node(std::pair<std::string const,boost::shared_ptr<RBX::Script>> const&)")
}

// 0x820008 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::lower_bound(std::string const&)")]
pub fn stub_0x820008() -> ! {
    todo!("0x820008 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::lower_bound(std::string const&)")
}

// 0x820038 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_
// type: int __fastcall(int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::find(std::string const&)")]
pub fn stub_0x820038() -> ! {
    todo!("0x820038 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::find(std::string const&)")
}

// 0x8560a8 — __ZN3RBX17ClientAppSettings44ReadValueMinNumberScriptExecutionsToGetPrizeEPKc
// type: _DWORD __fastcall(RBX::ClientAppSettings *__hidden this, const char *)
#[doc(alias = "RBX::ClientAppSettings::ReadValueMinNumberScriptExecutionsToGetPrize(char const*)")]
pub fn stub_0x8560a8() -> ! {
    todo!("0x8560a8 RBX::ClientAppSettings::ReadValueMinNumberScriptExecutionsToGetPrize(char const*)")
}

// 0x86be0c — __ZN3RBX19MegaClusterInstance13getCellScriptEiii
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, RBX::MegaClusterInstance *, int, int)
#[doc(alias = "RBX::MegaClusterInstance::getCellScript(int,int,int)")]
pub fn stub_0x86be0c() -> ! {
    todo!("0x86be0c RBX::MegaClusterInstance::getCellScript(int,int,int)")
}

// 0x86c0b4 — __ZN3RBX19MegaClusterInstance13setCellScriptEiiiNS_5Voxel12CellMaterialENS1_9CellBlockENS1_15CellOrientationE
#[doc(alias = "RBX::MegaClusterInstance::setCellScript(int,int,int,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation)")]
pub fn stub_0x86c0b4() -> ! {
    todo!("0x86c0b4 RBX::MegaClusterInstance::setCellScript(int,int,int,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation)")
}

// 0x86c178 — __ZN3RBX19MegaClusterInstance14setCellsScriptENS_12Region3int16ENS_5Voxel12CellMaterialENS2_9CellBlockENS2_15CellOrientationE
#[doc(alias = "RBX::MegaClusterInstance::setCellsScript(RBX::Region3int16,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation)")]
pub fn stub_0x86c178() -> ! {
    todo!("0x86c178 RBX::MegaClusterInstance::setCellsScript(RBX::Region3int16,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation)")
}

// 0x86c21c — __ZN3RBX19MegaClusterInstance18getWaterCellScriptEiii
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, RBX::MegaClusterInstance *, int, int)
#[doc(alias = "RBX::MegaClusterInstance::getWaterCellScript(int,int,int)")]
pub fn stub_0x86c21c() -> ! {
    todo!("0x86c21c RBX::MegaClusterInstance::getWaterCellScript(int,int,int)")
}

// 0x86c490 — __ZN3RBX19MegaClusterInstance18setWaterCellScriptEiiiNS_5Voxel14WaterCellForceENS1_18WaterCellDirectionE
#[doc(alias = "RBX::MegaClusterInstance::setWaterCellScript(int,int,int,RBX::Voxel::WaterCellForce,RBX::Voxel::WaterCellDirection)")]
pub fn stub_0x86c490() -> ! {
    todo!("0x86c490 RBX::MegaClusterInstance::setWaterCellScript(int,int,int,RBX::Voxel::WaterCellForce,RBX::Voxel::WaterCellDirection)")
}

// 0x86c528 — __ZN3RBX19MegaClusterInstance19autoWedgeCellScriptEiii
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, int, int, int)
#[doc(alias = "RBX::MegaClusterInstance::autoWedgeCellScript(int,int,int)")]
pub fn stub_0x86c528() -> ! {
    todo!("0x86c528 RBX::MegaClusterInstance::autoWedgeCellScript(int,int,int)")
}

// 0x86c9b8 — __ZN3RBX19MegaClusterInstance20autoWedgeCellsScriptENS_12Region3int16E
#[doc(alias = "RBX::MegaClusterInstance::autoWedgeCellsScript(RBX::Region3int16)")]
pub fn stub_0x86c9b8() -> ! {
    todo!("0x86c9b8 RBX::MegaClusterInstance::autoWedgeCellsScript(RBX::Region3int16)")
}

// 0x86ca30 — __ZN3RBX19MegaClusterInstance23cellCenterToWorldScriptEiii
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, int, int, int)
#[doc(alias = "RBX::MegaClusterInstance::cellCenterToWorldScript(int,int,int)")]
pub fn stub_0x86ca30() -> ! {
    todo!("0x86ca30 RBX::MegaClusterInstance::cellCenterToWorldScript(int,int,int)")
}

// 0x86ca80 — __ZN3RBX19MegaClusterInstance28worldToCellPreferSolidScriptEN3G3D7Vector3E
#[doc(alias = "RBX::MegaClusterInstance::worldToCellPreferSolidScript(G3D::Vector3)")]
pub fn stub_0x86ca80() -> ! {
    todo!("0x86ca80 RBX::MegaClusterInstance::worldToCellPreferSolidScript(G3D::Vector3)")
}

// 0x86caec — __ZN3RBX19MegaClusterInstance28worldToCellPreferEmptyScriptEN3G3D7Vector3E
#[doc(alias = "RBX::MegaClusterInstance::worldToCellPreferEmptyScript(G3D::Vector3)")]
pub fn stub_0x86caec() -> ! {
    todo!("0x86caec RBX::MegaClusterInstance::worldToCellPreferEmptyScript(G3D::Vector3)")
}

// 0x86cb58 — __ZN3RBX19MegaClusterInstance17worldToCellScriptEN3G3D7Vector3E
#[doc(alias = "RBX::MegaClusterInstance::worldToCellScript(G3D::Vector3)")]
pub fn stub_0x86cb58() -> ! {
    todo!("0x86cb58 RBX::MegaClusterInstance::worldToCellScript(G3D::Vector3)")
}

// 0x86cc5c — __ZN3RBX19MegaClusterInstance16countCellsScriptEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::countCellsScript(void)")]
pub fn stub_0x86cc5c() -> ! {
    todo!("0x86cc5c RBX::MegaClusterInstance::countCellsScript(void)")
}

// 0x8714e8 — __ZN3RBX19MegaClusterInstance23cellCornerToWorldScriptEiii
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, int, int, int)
#[doc(alias = "RBX::MegaClusterInstance::cellCornerToWorldScript(int,int,int)")]
pub fn stub_0x8714e8() -> ! {
    todo!("0x8714e8 RBX::MegaClusterInstance::cellCornerToWorldScript(int,int,int)")
}

// 0x885014 — __ZN3RBX6Plugin11getMouseLuaEv
// type: _DWORD __fastcall(RBX::Plugin *__hidden this)
#[doc(alias = "RBX::Plugin::getMouseLua(void)")]
pub fn stub_0x885014() -> ! {
    todo!("0x885014 RBX::Plugin::getMouseLua(void)")
}

// 0x8d05d8 — __ZN3RBX15ServiceProvider6createINS_13LuaWebServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::LuaWebService * RBX::ServiceProvider::create<RBX::LuaWebService>(RBX::Instance const*)")]
pub fn stub_0x8d05d8() -> ! {
    todo!("0x8d05d8 RBX::LuaWebService * RBX::ServiceProvider::create<RBX::LuaWebService>(RBX::Instance const*)")
}

// 0x8d05f0 — __ZNK3RBX15ServiceProvider6createINS_13LuaWebServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::LuaWebService * RBX::ServiceProvider::create<RBX::LuaWebService>(void)const")]
pub fn stub_0x8d05f0() -> ! {
    todo!("0x8d05f0 RBX::LuaWebService * RBX::ServiceProvider::create<RBX::LuaWebService>(void)const")
}

// 0x8d07b8 — __ZNK3RBX15ServiceProvider4findINS_13LuaWebServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::LuaWebService * RBX::ServiceProvider::find<RBX::LuaWebService>(void)const")]
pub fn stub_0x8d07b8() -> ! {
    todo!("0x8d07b8 RBX::LuaWebService * RBX::ServiceProvider::find<RBX::LuaWebService>(void)const")
}

// 0x8d0930 — __ZN3RBX4Name7declareILZNS_14sLuaWebServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sLuaWebServiceEEEERKS0_v")]
pub fn stub_0x8d0930() -> ! {
    todo!("0x8d0930 __ZN3RBX4Name7declareILZNS_14sLuaWebServiceEEEERKS0_v")
}

// 0x8d0978 — __ZN3RBX4Name9doDeclareILZNS_14sLuaWebServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sLuaWebServiceEEEERKS0_v")]
pub fn stub_0x8d0978() -> ! {
    todo!("0x8d0978 __ZN3RBX4Name9doDeclareILZNS_14sLuaWebServiceEEEERKS0_v")
}

// 0x8d0a60 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13LuaWebServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LuaWebService>(void)")]
pub fn stub_0x8d0a60() -> ! {
    todo!("0x8d0a60 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LuaWebService>(void)")
}

// 0x8d0b38 — __ZN5boost10shared_ptrIN3RBX13LuaWebServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x8d0b38() -> ! {
    todo!("0x8d0b38 boost::shared_ptr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x8d0ce8 — __ZN5boost6detail12shared_countC2IPN3RBX13LuaWebServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x8d0ce8() -> ! {
    todo!("0x8d0ce8 boost::detail::shared_count::shared_count<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x8d0df0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x8d0df0]")]
pub fn stub_0x8d0df0() -> ! {
    todo!("0x8d0df0 boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x8d0df8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x8d0df8() -> ! {
    todo!("0x8d0df8 boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x8d0e18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x8d0e18() -> ! {
    todo!("0x8d0e18 boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x8d0e30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x8d0e30() -> ! {
    todo!("0x8d0e30 boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x8e81e0 — __ZN3RBX13ScriptService12waitForChildEN5boost8weak_ptrINS_8InstanceEEESsNS1_8functionIFvNS1_10shared_ptrIS3_EEEEENS5_IFvSsEEE
// type: void __fastcall(int, _DWORD *, const std::string *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ScriptService::waitForChild(rbx_core::WeakPtr<RBX::Instance>,std::string,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x8e81e0() -> ! {
    todo!("0x8e81e0 RBX::ScriptService::waitForChild(boost::weak_ptr<RBX::Instance>,std::string,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::function<void ()(std::string)>)")
}

// 0x8e83c4 — __ZN3RBX13ScriptService12onChildAddedEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, RBX::Instance **)
#[doc(alias = "RBX::ScriptService::onChildAdded(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x8e83c4() -> ! {
    todo!("0x8e83c4 RBX::ScriptService::onChildAdded(boost::shared_ptr<RBX::Instance>)")
}

// 0x8e8690 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_13ScriptServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>> const&)")]
pub fn stub_0x8e8690() -> ! {
    todo!("0x8e8690 rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>> const&)")
}

// 0x8e8704 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int, const shared_count *)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::push_back(rbx_core::SharedPtr<RBX::ScriptService::Info> const&)")]
pub fn stub_0x8e8704() -> ! {
    todo!("0x8e8704 std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>::push_back(boost::shared_ptr<RBX::ScriptService::Info> const&)")
}

// 0x8e8754 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS5_S7_EESB_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::erase(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info>*,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info>*,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>)")]
pub fn stub_0x8e8754() -> ! {
    todo!("0x8e8754 std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>::erase(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info>*,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info>*,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>)")
}

// 0x8e8780 — __ZSt9remove_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESt6vectorIS7_SaIS7_EEEENS4_9IsNullPtrIS7_EEET_SF_SF_T0_
// type: int __fastcall(int, int)
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>> std::remove_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>)")]
pub fn stub_0x8e8780() -> ! {
    todo!("0x8e8780 __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>> std::remove_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<boost::shared_ptr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<boost::shared_ptr<RBX::ScriptService::Info>>)")
}

// 0x8e87a0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE15_M_erase_at_endEPS5_
// type: boost::detail::sp_counted_base *__fastcall(boost::detail::sp_counted_base *result, int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::ScriptService::Info>*)")]
pub fn stub_0x8e87a0() -> ! {
    todo!("0x8e87a0 std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>::_M_erase_at_end(boost::shared_ptr<RBX::ScriptService::Info>*)")
}

// 0x8e87d0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEES9_EET0_T_SB_SA_
// type: int __fastcall(int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *>(rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *)")]
pub fn stub_0x8e87d0() -> ! {
    todo!("0x8e87d0 boost::shared_ptr<RBX::ScriptService::Info> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::ScriptService::Info> *,boost::shared_ptr<RBX::ScriptService::Info> *>(boost::shared_ptr<RBX::ScriptService::Info> *,boost::shared_ptr<RBX::ScriptService::Info> *,boost::shared_ptr<RBX::ScriptService::Info> *)")
}

// 0x8e881c — __ZN5boost10shared_ptrIN3RBX13ScriptService4InfoEEaSERKS4_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info>::operator=(rbx_core::SharedPtr<RBX::ScriptService::Info> const&)")]
pub fn stub_0x8e881c() -> ! {
    todo!("0x8e881c boost::shared_ptr<RBX::ScriptService::Info>::operator=(boost::shared_ptr<RBX::ScriptService::Info> const&)")
}

// 0x8e8854 — __ZSt14remove_copy_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESt6vectorIS7_SaIS7_EEEESC_NS4_9IsNullPtrIS7_EEET0_T_SG_SF_T1_
// type: int __fastcall(_DWORD *, _DWORD *, int)
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>> std::remove_copy_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>)")]
pub fn stub_0x8e8854() -> ! {
    todo!("0x8e8854 __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>> std::remove_copy_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<boost::shared_ptr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<boost::shared_ptr<RBX::ScriptService::Info>>)")
}

// 0x8e887c — __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESt6vectorIS7_SaIS7_EEEENS4_9IsNullPtrIS7_EEET_SF_SF_T0_St26random_access_iterator_tag
// type: _DWORD *__fastcall(_DWORD *, int)
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>> std::__find_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>,std::random_access_iterator_tag)")]
pub fn stub_0x8e887c() -> ! {
    todo!("0x8e887c __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>> std::__find_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<boost::shared_ptr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<boost::shared_ptr<RBX::ScriptService::Info>>,std::random_access_iterator_tag)")
}

// 0x8e88f0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: void __fastcall(int *, char *, const shared_count *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info>*,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,rbx_core::SharedPtr<RBX::ScriptService::Info> const&)")]
pub fn stub_0x8e88f0() -> ! {
    todo!("0x8e88f0 std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info>*,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,boost::shared_ptr<RBX::ScriptService::Info> const&)")
}

// 0x8e8cbc — __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::_M_allocate(unsigned long)")]
pub fn stub_0x8e8cbc() -> ! {
    todo!("0x8e8cbc std::_Vector_base<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>::_M_allocate(unsigned long)")
}

// 0x8e8cd4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEES9_EET0_T_SB_SA_
// type: int __fastcall(int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *>(rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *)")]
pub fn stub_0x8e8cd4() -> ! {
    todo!("0x8e8cd4 boost::shared_ptr<RBX::ScriptService::Info> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::ScriptService::Info> *,boost::shared_ptr<RBX::ScriptService::Info> *>(boost::shared_ptr<RBX::ScriptService::Info> *,boost::shared_ptr<RBX::ScriptService::Info> *,boost::shared_ptr<RBX::ScriptService::Info> *)")
}

// 0x8e8d24 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_13ScriptServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x8e8d24() -> ! {
    todo!("0x8e8d24 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x8e8d50 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_13ScriptServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot() [0x8e8d50]")]
pub fn stub_0x8e8d50() -> ! {
    todo!("0x8e8d50 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x8e8e24 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_13ScriptServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x8e8e24() -> ! {
    todo!("0x8e8e24 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")
}

// 0x8e8e40 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_13ScriptServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x8e8e40() -> ! {
    todo!("0x8e8e40 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")
}

// 0x8e8e5c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX13ScriptServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, const shared_count **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
pub fn stub_0x8e8e5c() -> ! {
    todo!("0x8e8e5c void boost::_bi::list2<boost::_bi::value<RBX::ScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)")
}

// 0x8e911c — __ZN5boost10shared_ptrIN3RBX13ScriptService4InfoEEC2IS3_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info>::shared_ptr<RBX::ScriptService::Info>(RBX::ScriptService::Info *)")]
pub fn stub_0x8e911c() -> ! {
    todo!("0x8e911c boost::shared_ptr<RBX::ScriptService::Info>::shared_ptr<RBX::ScriptService::Info>(RBX::ScriptService::Info *)")
}

// 0x8e91f0 — __ZN5boost6detail12shared_countC2IN3RBX13ScriptService4InfoEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptService::Info>(RBX::ScriptService::Info *)")]
pub fn stub_0x8e91f0() -> ! {
    todo!("0x8e91f0 boost::detail::shared_count::shared_count<RBX::ScriptService::Info>(RBX::ScriptService::Info *)")
}

// 0x8e92fc — __ZN3RBX13ScriptService4InfoD2Ev
// type: void __fastcall(int32_t **this)
#[doc(alias = "RBX::ScriptService::Info::~Info()")]
pub fn stub_0x8e92fc() -> ! {
    todo!("0x8e92fc RBX::ScriptService::Info::~Info()")
}

// 0x8e9440 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::~sp_counted_impl_p()")]
pub fn stub_0x8e9440() -> ! {
    todo!("0x8e9440 boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::~sp_counted_impl_p()")
}

// 0x8e9444 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::~sp_counted_impl_p() [0x8e9444]")]
pub fn stub_0x8e9444() -> ! {
    todo!("0x8e9444 boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::~sp_counted_impl_p()")
}

// 0x8e9448 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::dispose(void)")]
pub fn stub_0x8e9448() -> ! {
    todo!("0x8e9448 boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::dispose(void)")
}

// 0x8e94ec — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::get_deleter(std::type_info const&)")]
pub fn stub_0x8e94ec() -> ! {
    todo!("0x8e94ec boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::get_deleter(std::type_info const&)")
}

// 0x8e94f0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::get_untyped_deleter(void)")]
pub fn stub_0x8e94f0() -> ! {
    todo!("0x8e94f0 boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::get_untyped_deleter(void)")
}

// 0x8fae10 — __ZN3RBX16OverlayDataModel20unloadGameFromScriptEN5boost8functionIFvvEEENS2_IFvSsEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int, char, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, char, char, int, int, int, int, int)
#[doc(alias = "RBX::OverlayDataModel::unloadGameFromScript(boost::function<void ()(void)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x8fae10() -> ! {
    todo!("0x8fae10 RBX::OverlayDataModel::unloadGameFromScript(boost::function<void ()(void)>,boost::function<void ()(std::string)>)")
}

// 0x8fcd28 — __ZN3RBX16OverlayDataModel19processSignedScriptEPKSsPKSt9exception
// type: void __fastcall(RBX::OverlayDataModel *this, const std::string *, std::exception *)
#[doc(alias = "RBX::OverlayDataModel::processSignedScript(std::string const*,std::exception const*)")]
pub fn stub_0x8fcd28() -> ! {
    todo!("0x8fcd28 RBX::OverlayDataModel::processSignedScript(std::string const*,std::exception const*)")
}

// 0x8fd3b8 — __ZN3RBX16OverlayDataModel26executeSignedScriptFromUrlERKSs
// type: void __fastcall(RBX::OverlayDataModel *this, const char **)
#[doc(alias = "RBX::OverlayDataModel::executeSignedScriptFromUrl(std::string const&)")]
pub fn stub_0x8fd3b8() -> ! {
    todo!("0x8fd3b8 RBX::OverlayDataModel::executeSignedScriptFromUrl(std::string const&)")
}

// 0x8fd7ec — __ZN3RBX16OverlayDataModel14loadJoinScriptERKSs
// type: void __fastcall(RBX::OverlayDataModel *this, const std::string *)
#[doc(alias = "RBX::OverlayDataModel::loadJoinScript(std::string const&)")]
pub fn stub_0x8fd7ec() -> ! {
    todo!("0x8fd7ec RBX::OverlayDataModel::loadJoinScript(std::string const&)")
}

// 0x9012f0 — __ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorD1Ev")]
pub fn stub_0x9012f0() -> ! {
    todo!("0x9012f0 __ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorD1Ev")
}

// 0x908028 — __ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator6createEv")]
pub fn stub_0x908028() -> ! {
    todo!("0x908028 __ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator6createEv")
}

// 0x908238 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ScriptContextES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScriptContext,RBX::ScriptContext>(rbx_core::SharedPtr<RBX::ScriptContext> const*,RBX::ScriptContext *)const")]
pub fn stub_0x908238() -> ! {
    todo!("0x908238 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScriptContext,RBX::ScriptContext>(boost::shared_ptr<RBX::ScriptContext> const*,RBX::ScriptContext *)const")
}

// 0x908330 — __ZN3RBX4Name7declareILZNS_14sScriptContextEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sScriptContextEEEERKS0_v")]
pub fn stub_0x908330() -> ! {
    todo!("0x908330 __ZN3RBX4Name7declareILZNS_14sScriptContextEEEERKS0_v")
}

// 0x90e500 — __ZN3RBX19ServerScriptServiceC1Ev
// type: _DWORD __fastcall(RBX::ServerScriptService *__hidden this)
#[doc(alias = "RBX::ServerScriptService::ServerScriptService(void)")]
pub fn stub_0x90e500() -> ! {
    todo!("0x90e500 RBX::ServerScriptService::ServerScriptService(void)")
}

// 0x90e504 — __ZN3RBX19ServerScriptServiceC2Ev
// type: _DWORD __fastcall(RBX::ServerScriptService *__hidden this)
#[doc(alias = "RBX::ServerScriptService::ServerScriptService(void) [0x90e504]")]
pub fn stub_0x90e504() -> ! {
    todo!("0x90e504 RBX::ServerScriptService::ServerScriptService(void)")
}

// 0x90e76c — __ZN3RBX19ServerScriptService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::ServerScriptService *__hidden this, RBX::BaseScript *)
#[doc(alias = "RBX::ServerScriptService::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x90e76c() -> ! {
    todo!("0x90e76c RBX::ServerScriptService::scriptShouldRun(RBX::BaseScript *)")
}

// 0x90e830 — __ZThn96_N3RBX19ServerScriptService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::ServerScriptService *__hidden this, RBX::BaseScript *)
#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x90e830() -> ! {
    todo!("0x90e830 non-virtual thunk toRBX::ServerScriptService::scriptShouldRun(RBX::BaseScript *)")
}

// 0x90e83c — __ZN3RBX19ServerScriptServiceD1Ev
// type: void __fastcall(RBX::ServerScriptService *__hidden this)
#[doc(alias = "RBX::ServerScriptService::~ServerScriptService()")]
pub fn stub_0x90e83c() -> ! {
    todo!("0x90e83c RBX::ServerScriptService::~ServerScriptService()")
}

// 0x90e840 — __ZN3RBX19ServerScriptServiceD0Ev
// type: void __fastcall(RBX::ServerScriptService *__hidden this)
#[doc(alias = "RBX::ServerScriptService::~ServerScriptService() [0x90e840]")]
pub fn stub_0x90e840() -> ! {
    todo!("0x90e840 RBX::ServerScriptService::~ServerScriptService()")
}

// 0x90e904 — __ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E12getClassNameEv")]
pub fn stub_0x90e904() -> ! {
    todo!("0x90e904 __ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E12getClassNameEv")
}

// 0x90e914 — __ZThn32_N3RBX19ServerScriptServiceD1Ev
// type: void __fastcall(RBX::ServerScriptService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::~ServerScriptService()")]
pub fn stub_0x90e914() -> ! {
    todo!("0x90e914 non-virtual thunk toRBX::ServerScriptService::~ServerScriptService()")
}

// 0x90e91c — __ZThn32_N3RBX19ServerScriptServiceD0Ev
// type: void __fastcall(RBX::ServerScriptService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::~ServerScriptService() [0x90e91c]")]
pub fn stub_0x90e91c() -> ! {
    todo!("0x90e91c non-virtual thunk toRBX::ServerScriptService::~ServerScriptService()")
}

// 0x90e9c0 — __ZThn32_NK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E12getClassNameEv")]
pub fn stub_0x90e9c0() -> ! {
    todo!("0x90e9c0 __ZThn32_NK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E12getClassNameEv")
}

// 0x90e9d0 — __ZThn36_N3RBX19ServerScriptServiceD1Ev
// type: void __fastcall(RBX::ServerScriptService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::~ServerScriptService() [0x90e9d0]")]
pub fn stub_0x90e9d0() -> ! {
    todo!("0x90e9d0 non-virtual thunk toRBX::ServerScriptService::~ServerScriptService()")
}

// 0x90e9d8 — __ZThn36_N3RBX19ServerScriptServiceD0Ev
// type: void __fastcall(RBX::ServerScriptService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::~ServerScriptService() [0x90e9d8]")]
pub fn stub_0x90e9d8() -> ! {
    todo!("0x90e9d8 non-virtual thunk toRBX::ServerScriptService::~ServerScriptService()")
}

// 0x90ea7c — __ZN3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x90ea7c() -> ! {
    todo!("0x90ea7c __ZN3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x90ea80 — __ZN3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x90ea80() -> ! {
    todo!("0x90ea80 __ZN3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x90eb20 — __ZThn32_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x90eb20() -> ! {
    todo!("0x90eb20 __ZThn32_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x90eb28 — __ZThn32_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x90eb28() -> ! {
    todo!("0x90eb28 __ZThn32_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x90ebcc — __ZThn36_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x90ebcc() -> ! {
    todo!("0x90ebcc __ZThn36_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x90ebd4 — __ZThn36_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x90ebd4() -> ! {
    todo!("0x90ebd4 __ZThn36_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}
