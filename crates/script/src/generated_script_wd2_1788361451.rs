// Auto-generated skeletons for rbx-script — Lua/Script watchdog wd2
// Filter: Lua|lua (746 filtered, 100 chosen) — EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x58c874..0x77e3c8 | rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE15isNullClassNameEv")]
pub fn stub_0x58c874(handle: &crate::slot::InstanceHandle) {
// RBX::NonFactoryProduct<RBX::Instance, RBX::sLuaWebService>::isNullClassName() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x76fbfc — __ZL10doSetLocalSsRKN3RBX10Reflection7VariantEiP9lua_State
#[doc(alias = "doSetLocal(std::string,RBX::Reflection::Variant const&,int,lua_State *)")]
#[doc(alias = "__ZL10doSetLocalSsRKN3RBX10Reflection7VariantEiP9lua_State")]
pub fn stub_0x76fbfc() -> crate::slot::PortedFn {
// IDA 0x76fbfc: doSetLocal(std::string, RBX::Reflection::Variant const&, int, lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x76fbfc, "doSetLocal(std::string, RBX::Reflection::Variant const&, int, lua_State*)")
}

// 0x76fe68 — __ZL12doSetUpvalueSsRKN3RBX10Reflection7VariantEiP9lua_State
#[doc(alias = "doSetUpvalue(std::string,RBX::Reflection::Variant const&,int,lua_State *)")]
#[doc(alias = "__ZL12doSetUpvalueSsRKN3RBX10Reflection7VariantEiP9lua_State")]
pub fn stub_0x76fe68() -> crate::slot::PortedFn {
// IDA 0x76fe68: doSetUpvalue(std::string, RBX::Reflection::Variant const&, int, lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x76fe68, "doSetUpvalue(std::string, RBX::Reflection::Variant const&, int, lua_State*)")
}

// 0x77014c — __ZL11doSetGlobalSsRKN3RBX10Reflection7VariantEP9lua_State
#[doc(alias = "doSetGlobal(std::string,RBX::Reflection::Variant const&,lua_State *)")]
#[doc(alias = "__ZL11doSetGlobalSsRKN3RBX10Reflection7VariantEP9lua_State")]
pub fn stub_0x77014c() -> crate::slot::PortedFn {
// IDA 0x77014c: doSetGlobal(std::string, RBX::Reflection::Variant const&, lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x77014c, "doSetGlobal(std::string, RBX::Reflection::Variant const&, lua_State*)")
}

// 0x770dfc — __ZL12getIndexInfoP9lua_State
#[doc(alias = "getIndexInfo(lua_State *)")]
#[doc(alias = "__ZL12getIndexInfoP9lua_State")]
pub fn stub_0x770dfc() -> crate::slot::PortedFn {
// IDA 0x770dfc: getIndexInfo(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x770dfc, "getIndexInfo(lua_State*)")
}

// 0x770fe0 — __ZL12setIndexInfoP9lua_State
#[doc(alias = "setIndexInfo(lua_State *)")]
#[doc(alias = "__ZL12setIndexInfoP9lua_State")]
pub fn stub_0x770fe0() -> crate::slot::PortedFn {
// IDA 0x770fe0: setIndexInfo(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x770fe0, "setIndexInfo(lua_State*)")
}

// 0x772200 — __ZN3RBX9Scripting14ScriptDebugger16withPausedThreadINS_10Reflection7VariantEEET_N5boost8functionIFS5_P9lua_StateP9lua_DebugEEE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::Variant RBX::Scripting::ScriptDebugger::withPausedThread<RBX::Reflection::Variant>(boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger16withPausedThreadINS_10Reflection7VariantEEET_N5boost8functionIFS5_P9lua_StateP9lua_DebugEEE")]
pub fn stub_0x772200(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Variant RBX::Scripting::ScriptDebugger::withPausedThread<RBX::Reflection:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x7727e8 — __ZN5boost4bindIN3RBX10Reflection7VariantESsP9lua_StateSsNS_3argILi1EEEEENS_3_bi6bind_tIT_PFSA_T0_T1_ENS8_9list_av_2IT2_T3_E4typeEEESE_SG_SH_
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list_av_2<std::string,boost::arg<1>>::type> boost::bind<RBX::Reflection::Variant,std::string,lua_State *,std::string,boost::arg<1>>(RBX::Reflection::Variant (*)(std::string,lua_State *),std::string,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIN3RBX10Reflection7VariantESsP9lua_StateSsNS_3argILi1EEEEENS_3_bi6bind_tIT_PFSA_T0_T1_ENS8_9list_av_2IT2_T3_E4typeEEESE_SG_SH_")]
pub fn stub_0x7727e8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

// 0x773270 — __ZN3RBX9Scripting14ScriptDebugger16withPausedThreadIN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEEET_NS3_8functionIFSH_P9lua_StateP9lua_DebugEEE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int)
#[doc(alias = "boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Scripting::ScriptDebugger::withPausedThread<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger16withPausedThreadIN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEEET_NS3_8functionIFSH_P9lua_StateP9lua_DebugEEE")]
pub fn stub_0x773270() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

// 0x773d80 — __ZN5boost4bindIbSsRKN3RBX10Reflection7VariantEiP9lua_StateSsNS_17reference_wrapperIS4_EEiNS_3argILi1EEEEENS_3_bi6bind_tIT_PFSE_T0_T1_T2_T3_ENSC_9list_av_4IT4_T5_T6_T7_E4typeEEESK_SM_SN_SO_SP_
// type: int __fastcall(int, int, std::string *, int, int)
#[doc(alias = "boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list_av_4<std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,int,boost::arg<1>>::type> boost::bind<bool,std::string,RBX::Reflection::Variant const&,int,lua_State *,std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,int,boost::arg<1>>(bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,int,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIbSsRKN3RBX10Reflection7VariantEiP9lua_StateSsNS_17reference_wrapperIS4_EEiNS_3argILi1EEEEENS_3_bi6bind_tIT_PFSE_T0_T1_T2_T3_ENSC_9list_av_4IT4_T5_T6_T7_E4typeEEESK_SM_SN_SO_SP_")]
pub fn stub_0x773d80() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

// 0x773f38 — __ZN5boost4bindIbSsRKN3RBX10Reflection7VariantEP9lua_StateSsNS_17reference_wrapperIS4_EENS_3argILi1EEEEENS_3_bi6bind_tIT_PFSE_T0_T1_T2_ENSC_9list_av_3IT3_T4_T5_E4typeEEESJ_SL_SM_SN_
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list_av_3<std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>::type> boost::bind<bool,std::string,RBX::Reflection::Variant const&,lua_State *,std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>(bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIbSsRKN3RBX10Reflection7VariantEP9lua_StateSsNS_17reference_wrapperIS4_EENS_3argILi1EEEEENS_3_bi6bind_tIT_PFSE_T0_T1_T2_ENSC_9list_av_3IT3_T4_T5_E4typeEEESJ_SL_SM_SN_")]
pub fn stub_0x773f38() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

// 0x776170 — __ZN5boost9function2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS7_5list3INS7_5valueISsEENS_17reference_wrapperISC_EENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS7_5list3INS7_5valueISsEENS_17reference_wrapperISC_EENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_0x776170(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x7762b0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEP9lua_StateENS3_5list3INS3_5valueISsEENS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEP9lua_StateENS3_5list3INS3_5valueISsEENS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x7762b0(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x776330 — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEP9lua_StateENS3_5list3INS3_5valueISsEENS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEbSB_P9lua_DebugE6invokeERNS1_15function_bufferESB_SO_
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,bool,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEP9lua_StateENS3_5list3INS3_5valueISsEENS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEbSB_P9lua_DebugE6invokeERNS1_15function_bufferESB_SO_")]
pub fn stub_0x776330(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

// 0x776354 — __ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES4_ENS9_5list3INS9_5valueISsEENS_17reference_wrapperISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES4_ENS9_5list3INS9_5valueISsEENS_17reference_wrapperISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x776354(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x776488 — __ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES4_ENS9_5list3INS9_5valueISsEENS_17reference_wrapperISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int(void)
#[doc(alias = "bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES4_ENS9_5list3INS9_5valueISsEENS_17reference_wrapperISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x776488(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x7765b4 — __ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES4_ENS9_5list3INS9_5valueISsEENS_17reference_wrapperISE_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb1EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<true>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES4_ENS9_5list3INS9_5valueISsEENS_17reference_wrapperISE_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb1EEE")]
pub fn stub_0x7765b4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x7765d8 — __ZN5boost3_bi5list3INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS_3argILi1EEEEclIbPFbSsRS8_P9lua_StateENS0_5list2IRSG_RP9lua_DebugEEEET_NS0_4typeISP_EERT0_RT1_l
// type: int __fastcall(std::string *)
#[doc(alias = "bool boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>::operator()<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<bool>,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *) &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,long)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS_3argILi1EEEEclIbPFbSsRS8_P9lua_StateENS0_5list2IRSG_RP9lua_DebugEEEET_NS0_4typeISP_EERT0_RT1_l")]
pub fn stub_0x7765d8(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x7765d8: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

// 0x776ccc — __ZN5boost9function2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS7_5list4INS7_5valueISsEENS_17reference_wrapperISC_EENSH_IiEENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS7_5list4INS7_5valueISsEENS_17reference_wrapperISC_EENSH_IiEENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_0x776ccc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x776e10 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x776e10(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x776e2c — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEbSB_P9lua_DebugE6invokeERNS1_15function_bufferESB_SP_
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,bool,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEbSB_P9lua_DebugE6invokeERNS1_15function_bufferESB_SP_")]
pub fn stub_0x776e2c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

// 0x776e4c — __ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS4_ENS9_5list4INS9_5valueISsEENS_17reference_wrapperISE_EENSJ_IiEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS4_ENS9_5list4INS9_5valueISsEENS_17reference_wrapperISE_EENSJ_IiEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x776e4c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x776f84 — __ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS4_ENS9_5list4INS9_5valueISsEENS_17reference_wrapperISE_EENSJ_IiEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS4_ENS9_5list4INS9_5valueISsEENS_17reference_wrapperISE_EENSJ_IiEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x776f84(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x7770b4 — __ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS4_ENS9_5list4INS9_5valueISsEENS_17reference_wrapperISE_EENSJ_IiEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS4_ENS9_5list4INS9_5valueISsEENS_17reference_wrapperISE_EENSJ_IiEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x7770b4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x777180 — __ZN5boost3_bi5list4INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS2_IiEENS_3argILi1EEEEclIbPFbSsRS8_iP9lua_StateENS0_5list2IRSH_RP9lua_DebugEEEET_NS0_4typeISQ_EERT0_RT1_l
// type: int __fastcall(std::string *)
#[doc(alias = "bool boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>::operator()<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<bool>,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *) &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,long)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS2_IiEENS_3argILi1EEEEclIbPFbSsRS8_iP9lua_StateENS0_5list2IRSH_RP9lua_DebugEEEET_NS0_4typeISQ_EERT0_RT1_l")]
pub fn stub_0x777180(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x777180: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

// 0x7772b0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x7772b0(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x778034 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SI_SK_SN_
#[doc(alias = "boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SI_SK_SN_")]
pub fn stub_0x778034() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

// 0x778108 — __ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SI_SK_SN_
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SI_SK_SN_")]
pub fn stub_0x778108() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

// 0x7781e0 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SA_SI_SK_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SA_SI_SK_")]
pub fn stub_0x7781e0() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

// 0x7782bc — __ZN5boost9function2IvP9lua_StateP9lua_DebugE4swapERS5_
#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::swap(boost::function2<void,lua_State *,lua_Debug *>&)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE4swapERS5_")]
pub fn stub_0x7782bc() -> crate::slot::PortedFn {
// IDA 0x7782bc: boost::function2<void, lua_State*, lua_Debug*>::swap(boost::function2<void, lua_State*, lua_Debug*>&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x7782bc, "boost::function2<void, lua_State*, lua_Debug*>::swap(boost::function2<void, lua_State*, lua_Debug*>&~")
}

// 0x778398 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE11move_assignERS5_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::move_assign(boost::function2<void,lua_State *,lua_Debug *>&)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE11move_assignERS5_")]
pub fn stub_0x778398() -> crate::slot::PortedFn {
// IDA 0x778398: boost::function2<void, lua_State*, lua_Debug*>::move_assign(boost::function2<void, lua_State*, lua_Debug*>&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x778398, "boost::function2<void, lua_State*, lua_Debug*>::move_assign(boost::function2<void, lua_State*, lua_D~")
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x77849c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x778588() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

// 0x778674 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEEvT_
#[doc(alias = "void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEEvT_")]
pub fn stub_0x778674(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x778770 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEE6manageERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEE6manageERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x778770(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x77878c — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_")]
pub fn stub_0x77878c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

// 0x7787b0 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x7787b0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x7788a0 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x7788a0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x778988 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x778988(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x778a64 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEclINS_4_mfi3mf5IvS5_SD_SF_SH_RbRSM_EENS0_5list2IRSD_RSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEclINS_4_mfi3mf5IvS5_SD_SF_SH_RbRSM_EENS0_5list2IRSD_RSF_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x778a64(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

// 0x778b48 — __ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbS6_S8_EEERbRNS_10shared_ptrISsEEEclEPS4_S6_S8_SB_SC_SF_
#[doc(alias = "boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbS6_S8_EEERbRNS_10shared_ptrISsEEEclEPS4_S6_S8_SB_SC_SF_")]
pub fn stub_0x778b48(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

// 0x778c34 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEE7managerERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEE7managerERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x778c34(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x77910c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_P9lua_StateENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_P9lua_StateENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x77910c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x77916c — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_P9lua_StateENS3_5list1INS_3argILi1EEEEEEENS5_IKSG_EESJ_P9lua_DebugE6invokeERNS1_15function_bufferESJ_SU_
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_P9lua_StateENS3_5list1INS_3argILi1EEEEEEENS5_IKSG_EESJ_P9lua_DebugE6invokeERNS1_15function_bufferESJ_SU_")]
pub fn stub_0x77916c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

// 0x779234 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_iP9lua_StateENS3_5list2INS3_5valueIiEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(int,lua_State *),boost::_bi::list2<boost::_bi::value<int>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_iP9lua_StateENS3_5list2INS3_5valueIiEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x779234(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x779294 — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_iP9lua_StateENS3_5list2INS3_5valueIiEENS_3argILi1EEEEEEENS5_IKSG_EESJ_P9lua_DebugE6invokeERNS1_15function_bufferESJ_SW_
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(int,lua_State *),boost::_bi::list2<boost::_bi::value<int>,boost::arg<1>>>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_iP9lua_StateENS3_5list2INS3_5valueIiEENS_3argILi1EEEEEEENS5_IKSG_EESJ_P9lua_DebugE6invokeERNS1_15function_bufferESJ_SW_")]
pub fn stub_0x779294(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

// 0x779360 — __ZNK5boost9function2INS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEP9lua_StateP9lua_DebugEclESG_SI_
#[doc(alias = "boost::function2<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
#[doc(alias = "__ZNK5boost9function2INS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEP9lua_StateP9lua_DebugEclESG_SI_")]
pub fn stub_0x779360(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES1C_")]
pub fn stub_0x77942c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

// 0x779544 — __ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIKSsSC_EEEEES5_S7_EEERSL_RNS9_ISsEEPS3_NS_3argILi1EEENSS_ILi2EEESN_NS_17reference_wrapperISL_EENSV_ISP_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5IS10_T0_T1_T2_T3_T4_T5_EENSY_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMS13_FS10_S14_S15_S16_S17_S18_ES1B_S1C_S1D_S1E_S1F_S1G_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIKSsSC_EEEEES5_S7_EEERSL_RNS9_ISsEEPS3_NS_3argILi1EEENSS_ILi2EEESN_NS_17reference_wrapperISL_EENSV_ISP_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5IS10_T0_T1_T2_T3_T4_T5_EENSY_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMS13_FS10_S14_S15_S16_S17_S18_ES1B_S1C_S1D_S1E_S1F_S1G_")]
pub fn stub_0x779544() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

// 0x779668 — __ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookIN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEEEvP9lua_StateP9lua_DebugNS3_8functionIFT_SI_SK_EEERSM_RNS4_ISsEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, char, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::Scripting::ScriptDebugger::withPausedThreadHook<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookIN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEEEvP9lua_StateP9lua_DebugNS3_8functionIFT_SI_SK_EEERSM_RNS4_ISsEE")]
pub fn stub_0x779668() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

// 0x7799fc — __ZN5boost9function2INS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEP9lua_StateP9lua_DebugE13assign_to_ownERKSJ_
#[doc(alias = "boost::function2<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::assign_to_own(boost::function2<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *> const&)")]
#[doc(alias = "__ZN5boost9function2INS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEP9lua_StateP9lua_DebugE13assign_to_ownERKSJ_")]
pub fn stub_0x7799fc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x779a2c — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEC2ES7_S9_SA_SV_SX_SZ_
#[doc(alias = "boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEC2ES7_S9_SA_SV_SX_SZ_")]
pub fn stub_0x779a2c() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

// 0x779b00 — __ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEC2ES7_S9_SA_SV_SX_SZ_
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEC2ES7_S9_SA_SV_SX_SZ_")]
pub fn stub_0x779b00() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

// 0x779bd8 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EEEC2ES7_S9_SA_SV_SX_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EEEC2ES7_S9_SA_SV_SX_")]
pub fn stub_0x779bd8() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x779cb4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSB_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x779da0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

// 0x779e8c — __ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSB_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEEvT_
#[doc(alias = "void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSB_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEEvT_")]
pub fn stub_0x779e8c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x779f88 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEE6manageERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEE6manageERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x779f88(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x779fa4 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_")]
pub fn stub_0x779fa4(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

// 0x779fc8 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x779fc8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x77a0b8 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x77a0b8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x77a1a0 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x77a1a0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x77a27c — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEclINS_4_mfi3mf5IvS5_SQ_SS_SU_RSO_RSY_EENS0_5list2IRSQ_RSS_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEclINS_4_mfi3mf5IvS5_SQ_SS_SU_RSO_RSY_EENS0_5list2IRSQ_RSS_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x77a27c(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

// 0x77a360 — __ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS2_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEES6_S8_EEERSM_RNSA_ISsEEEclEPS4_S6_S8_SO_SP_SR_
#[doc(alias = "boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS2_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEES6_S8_EEERSM_RNSA_ISsEEEclEPS4_S6_S8_SO_SP_SR_")]
pub fn stub_0x77a360(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

// 0x77a44c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEE7managerERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEE7managerERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x77a44c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x77a9e8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvP9lua_StateEE4slotEEaSEPS8_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(lua_State *)>::slot>::operator=(rbx::signals::signal<void ()(lua_State *)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvP9lua_StateEE4slotEEaSEPS8_")]
pub fn stub_0x77a9e8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x77aa14 — __ZN3rbx7signals6signalIFvP9lua_StateEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev")]
pub fn stub_0x77aa14(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x77aa40 — __ZN3rbx7signals6signalIFvP9lua_StateEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>>::~callable_slot() [0x77aa40]")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev")]
pub fn stub_0x77aa40(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x77ac30 — __ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::call(lua_State *)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
pub fn stub_0x77ac30(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x77ac30: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x77ac44 — __ZThn4_N3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::call(lua_State *)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
pub fn stub_0x77ac44(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x77ac44: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x77ac58 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS8_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>::operator()<lua_State *>(lua_State * &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS8_EEvRT_")]
pub fn stub_0x77ac58() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

// 0x77af54 — __ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev")]
pub fn stub_0x77af54(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x77af54: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "__ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSB_5list2INSB_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x77cae4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x77cc0c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

// 0x77cd38 — __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_0x77cd38(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x77ce74 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x77ce74(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x77cef4 — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEES7_S9_P9lua_DebugE6invokeERNS1_15function_bufferES9_SK_
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,RBX::Reflection::Variant,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIN3RBX10Reflection7VariantEPFS7_SsP9lua_StateENS3_5list2INS3_5valueISsEENS_3argILi1EEEEEEES7_S9_P9lua_DebugE6invokeERNS1_15function_bufferES9_SK_")]
pub fn stub_0x77cef4(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

// 0x77cf18 — __ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x77cf18(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x77d044 — __ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIS5_PFS5_SsS7_ENSC_5list2INSC_5valueISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x77d044(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x77d17c — __ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEclIN3RBX10Reflection7VariantEPFSA_SsP9lua_StateENS1_IRSC_RP9lua_DebugEEEET_NS0_4typeISK_EERT0_RT1_l
#[doc(alias = "RBX::Reflection::Variant boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>::operator()<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<RBX::Reflection::Variant>,RBX::Reflection::Variant (*)(std::string,lua_State *) &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,long)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEclIN3RBX10Reflection7VariantEPFSA_SsP9lua_StateENS1_IRSC_RP9lua_DebugEEEET_NS0_4typeISK_EERT0_RT1_l")]
pub fn stub_0x77d17c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x77d17c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

// 0x77d2a0 — __ZNK5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEclES5_S7_
#[doc(alias = "boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
#[doc(alias = "__ZNK5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEclES5_S7_")]
pub fn stub_0x77d2a0(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES12_")]
pub fn stub_0x77d36c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

// 0x77d484 — __ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS1_10Reflection7VariantES5_S7_EEERSA_RNS_10shared_ptrISsEEPS3_NS_3argILi1EEENSI_ILi2EEESC_NS_17reference_wrapperISA_EENSL_ISF_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISQ_T0_T1_T2_T3_T4_T5_EENSO_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMST_FSQ_SU_SV_SW_SX_SY_ES11_S12_S13_S14_S15_S16_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS1_10Reflection7VariantES5_S7_EEERSA_RNS_10shared_ptrISsEEPS3_NS_3argILi1EEENSI_ILi2EEESC_NS_17reference_wrapperISA_EENSL_ISF_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISQ_T0_T1_T2_T3_T4_T5_EENSO_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMST_FSQ_SU_SV_SW_SX_SY_ES11_S12_S13_S14_S15_S16_")]
pub fn stub_0x77d484() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

// 0x77d5a8 — __ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookINS_10Reflection7VariantEEEvP9lua_StateP9lua_DebugN5boost8functionIFT_S6_S8_EEERSB_RNS9_10shared_ptrISsEE
#[doc(alias = "void RBX::Scripting::ScriptDebugger::withPausedThreadHook<RBX::Reflection::Variant>(lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookINS_10Reflection7VariantEEEvP9lua_StateP9lua_DebugN5boost8functionIFT_S6_S8_EEERSB_RNS9_10shared_ptrISsEE")]
pub fn stub_0x77d5a8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string")
}

// 0x77d978 — __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE13assign_to_ownERKS8_
#[doc(alias = "boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to_own(boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *> const&)")]
#[doc(alias = "__ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE13assign_to_ownERKS8_")]
pub fn stub_0x77d978(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x77d9a8 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_
#[doc(alias = "boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_")]
pub fn stub_0x77d9a8() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

// 0x77da7c — __ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SK_SM_SP_")]
pub fn stub_0x77da7c() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

// 0x77db54 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EEEC2ES7_S9_SA_SK_SM_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EEEC2ES7_S9_SA_SK_SM_")]
pub fn stub_0x77db54() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x77dc30() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x77dd1c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

// 0x77de08 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEEvT_
#[doc(alias = "void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEEvT_")]
pub fn stub_0x77de08(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x77df04 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE6manageERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE6manageERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x77df04(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

// 0x77df20 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_")]
pub fn stub_0x77df20(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

// 0x77df44 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x77df44(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x77e034 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x77e034(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x77e11c — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNSD_10Reflection7VariantES4_S6_EEERSI_RNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSU_ILi2EEENSR_ISK_EENS_17reference_wrapperISI_EENSY_ISN_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x77e11c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x77e1f8 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEclINS_4_mfi3mf5IvS5_SF_SH_SJ_RSD_RSO_EENS0_5list2IRSF_RSH_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS3_10Reflection7VariantEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISD_EENSL_INS_10shared_ptrISsEEEEEclINS_4_mfi3mf5IvS5_SF_SH_SJ_RSD_RSO_EENS0_5list2IRSF_RSH_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x77e1f8(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

// 0x77e2dc — __ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS2_10Reflection7VariantES6_S8_EEERSB_RNS_10shared_ptrISsEEEclEPS4_S6_S8_SD_SE_SH_
#[doc(alias = "boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS2_10Reflection7VariantES6_S8_EEERSB_RNS_10shared_ptrISsEEEclEPS4_S6_S8_SD_SE_SH_")]
pub fn stub_0x77e2dc(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

// 0x77e3c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE7managerERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE7managerERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x77e3c8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}
