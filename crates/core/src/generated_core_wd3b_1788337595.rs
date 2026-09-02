//! core wd3b shard B — 120 core stubs EA-sorted asc gap filler distinct not overlapping shard A (watchdog_coreA 0x54c9a8..0x5765ec).
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 120 not yet in crates/core/src, not overlapping shard A/B.
//! Range: 0x2aa690..0x2b8c9c | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalScript,RBX::LocalScript>(rbx_core::SharedPtr<RBX::LocalScript> const*,RBX::LocalScript *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11LocalScriptES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0x2aa690 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11LocalScriptES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalScript,RBX::LocalScript>(boost::shared_ptr<RBX::LocalScript> const*,RBX::LocalScript *)const
pub fn stub_0x2aa690() -> ! {
    todo!("0x2aa690 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11LocalScriptES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LocalScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// 0x2aa780 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LocalScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_0x2aa780() -> ! {
    todo!("0x2aa780 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LocalScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::Instance>*)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE15_M_erase_at_endEPS4_")]
// 0x2aab78 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE15_M_erase_at_endEPS4_
// was: std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_erase_at_end(boost::shared_ptr<RBX::Instance>*)
pub fn stub_0x2aab78() -> ! {
    todo!("0x2aab78 __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE15_M_erase_at_endEPS4_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance>*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,unsigned long,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_")]
// 0x2aaba8 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
// was: std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance>*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,unsigned long,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0x2aaba8() -> ! {
    todo!("0x2aaba8 __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<RBX::Instance> *,unsigned long,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> *,unsigned long,rbx_core::SharedPtr<RBX::Instance> const&,std::__false_type)")]
#[doc(alias = "__ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX8InstanceEEEmS4_EvT_T0_RKT1_St12__false_type")]
// 0x2ab1a8 — __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX8InstanceEEEmS4_EvT_T0_RKT1_St12__false_type
// was: void std::__uninitialized_fill_n_aux<boost::shared_ptr<RBX::Instance> *,unsigned long,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> *,unsigned long,boost::shared_ptr<RBX::Instance> const&,std::__false_type)
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0x2ab1a8() -> ! {
    todo!("0x2ab1a8 __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX8InstanceEEEmS4_EvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptContext> RBX::Creatable<RBX::Instance>::create<RBX::ScriptContext>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_13ScriptContextEEEN5boost10shared_ptrIT_EEv")]
// 0x2abdd8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ScriptContextEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ScriptContext> RBX::Creatable<RBX::Instance>::create<RBX::ScriptContext>(void)
pub fn stub_0x2abdd8() -> ! {
    todo!("0x2abdd8 __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ScriptContextEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX13ScriptContextENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// 0x2abe90 — __ZN5boost6detail12shared_countC2IPN3RBX13ScriptContextENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x2abe90() -> ! {
    todo!("0x2abe90 __ZN5boost6detail12shared_countC2IPN3RBX13ScriptContextENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// 0x2abf98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_0x2abf98() -> ! {
    todo!("0x2abf98 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// 0x2abfa0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_0x2abfa0() -> ! {
    todo!("0x2abfa0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// 0x2abfc0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_0x2abfc0() -> ! {
    todo!("0x2abfc0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// 0x2abfd8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_0x2abfd8() -> ! {
    todo!("0x2abfd8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::list3(boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEC2ES6_S9_SB_")]
// 0x2b24f8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEC2ES6_S9_SB_
pub fn stub_0x2b24f8() -> ! {
    todo!("0x2b24f8 __ZN5boost3_bi5list3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEC2ES6_S9_SB_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::storage3(boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEC2ES6_S9_SB_")]
// 0x2b25c0 — __ZN5boost3_bi8storage3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEC2ES6_S9_SB_
pub fn stub_0x2b25c0() -> ! {
    todo!("0x2b25c0 __ZN5boost3_bi8storage3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEC2ES6_S9_SB_")
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// 0x2b2e3c — __ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
pub fn stub_0x2b2e3c() -> ! {
    todo!("0x2b2e3c __ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// 0x2b2f10 — __ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
pub fn stub_0x2b2f10() -> ! {
    todo!("0x2b2f10 __ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>)")]
#[doc(alias = "__ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEEvT_")]
// 0x2b2fe4 — __ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEEvT_
// was: void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>>)
pub fn stub_0x2b2fe4() -> ! {
    todo!("0x2b2fe4 __ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")]
// 0x2b30c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x2b30c8() -> ! {
    todo!("0x2b30c8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,void,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEvS6_E6invokeERNS1_15function_bufferES6_")]
// 0x2b30e4 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEvS6_E6invokeERNS1_15function_bufferES6_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>>,void,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)
pub fn stub_0x2b30e4() -> ! {
    todo!("0x2b30e4 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEvS6_E6invokeERNS1_15function_bufferES6_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE")]
// 0x2b3100 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x2b3100() -> ! {
    todo!("0x2b3100 __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x2b31d8 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x2b31d8() -> ! {
    todo!("0x2b31d8 __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x2b32a8 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x2b32a8() -> ! {
    todo!("0x2b32a8 __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>::operator()<void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>) &,boost::_bi::list1<lua_State *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEclIPFvP9lua_StateSD_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0x2b336c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEclIPFvP9lua_StateSD_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>::operator()<void (*)(lua_State *,boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>),boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,void (*)(lua_State *,boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>) &,boost::_bi::list1<lua_State *&> &,int)
pub fn stub_0x2b336c() -> ! {
    todo!("0x2b336c __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEclIPFvP9lua_StateSD_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x2b3434 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0x2b3434() -> ! {
    todo!("0x2b3434 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>::list2(boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEC2ES3_SE_")]
// 0x2b357c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEC2ES3_SE_
// was: boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>>::list2(boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>>)
pub fn stub_0x2b357c() -> ! {
    todo!("0x2b357c __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEC2ES3_SE_")
}

#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::dummy::nonnull(void)")]
#[doc(alias = "__ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE5dummy7nonnullEv")]
// 0x2b3644 — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE5dummy7nonnullEv
// was: boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::dummy::nonnull(void)
pub fn stub_0x2b3644() -> ! {
    todo!("0x2b3644 __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE5dummy7nonnullEv")
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// 0x2b3648 — __ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
pub fn stub_0x2b3648() -> ! {
    todo!("0x2b3648 __ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// 0x2b371c — __ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
pub fn stub_0x2b371c() -> ! {
    todo!("0x2b371c __ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>)")]
#[doc(alias = "__ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEEvT_")]
// 0x2b37f0 — __ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEEvT_
// was: void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>)
pub fn stub_0x2b37f0() -> ! {
    todo!("0x2b37f0 __ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")]
// 0x2b38d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x2b38d4() -> ! {
    todo!("0x2b38d4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,void,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEvS6_E6invokeERNS1_15function_bufferES6_")]
// 0x2b38f0 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEvS6_E6invokeERNS1_15function_bufferES6_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>,void,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)
pub fn stub_0x2b38f0() -> ! {
    todo!("0x2b38f0 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEvS6_E6invokeERNS1_15function_bufferES6_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE")]
// 0x2b390c — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x2b390c() -> ! {
    todo!("0x2b390c __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x2b39e4 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x2b39e4() -> ! {
    todo!("0x2b39e4 __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x2b3ab4 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x2b3ab4() -> ! {
    todo!("0x2b3ab4 __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::operator()<void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>) &,boost::_bi::list1<lua_State *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEEEEclIPFvP9lua_StateSD_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0x2b3b78 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEEEEclIPFvP9lua_StateSD_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>::operator()<void (*)(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>),boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,void (*)(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>) &,boost::_bi::list1<lua_State *&> &,int)
pub fn stub_0x2b3b78() -> ! {
    todo!("0x2b3b78 __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEEEEclIPFvP9lua_StateSD_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x2b3c40 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0x2b3c40() -> ! {
    todo!("0x2b3c40 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::list2(boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEEEEC2ES3_SE_")]
// 0x2b3d88 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEEEEC2ES3_SE_
// was: boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>::list2(boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)
pub fn stub_0x2b3d88() -> ! {
    todo!("0x2b3d88 __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEEEEC2ES3_SE_")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::dummy::nonnull(void)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE5dummy7nonnullEv")]
// 0x2b3e50 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE5dummy7nonnullEv
// was: boost::function1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::dummy::nonnull(void)
pub fn stub_0x2b3e50() -> ! {
    todo!("0x2b3e50 __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE5dummy7nonnullEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::auto_ptr<RBX::Reflection::Tuple> &,lua_State *,unsigned long),boost::_bi::list3<boost::reference_wrapper<std::auto_ptr<RBX::Reflection::Tuple>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRSt8auto_ptrIN3RBX10Reflection5TupleEEP9lua_StatemENS3_5list3INS_17reference_wrapperIS9_EENS_3argILi1EEENSI_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
// 0x2b3e54 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRSt8auto_ptrIN3RBX10Reflection5TupleEEP9lua_StatemENS3_5list3INS_17reference_wrapperIS9_EENS_3argILi1EEENSI_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
pub fn stub_0x2b3e54() -> ! {
    todo!("0x2b3e54 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRSt8auto_ptrIN3RBX10Reflection5TupleEEP9lua_StatemENS3_5list3INS_17reference_wrapperIS9_EENS_3argILi1EEENSI_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::auto_ptr<RBX::Reflection::Tuple> &,lua_State *,unsigned long),boost::_bi::list3<boost::reference_wrapper<std::auto_ptr<RBX::Reflection::Tuple>>,boost::arg<1>,boost::arg<2>>>,void,lua_State *,unsigned long>::invoke(boost::detail::function::function_buffer &,lua_State *,unsigned long)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvRSt8auto_ptrIN3RBX10Reflection5TupleEEP9lua_StatemENS3_5list3INS_17reference_wrapperIS9_EENS_3argILi1EEENSI_ILi2EEEEEEEvSC_mE6invokeERNS1_15function_bufferESC_m")]
// 0x2b3eb4 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvRSt8auto_ptrIN3RBX10Reflection5TupleEEP9lua_StatemENS3_5list3INS_17reference_wrapperIS9_EENS_3argILi1EEENSI_ILi2EEEEEEEvSC_mE6invokeERNS1_15function_bufferESC_m
pub fn stub_0x2b3eb4() -> ! {
    todo!("0x2b3eb4 __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvRSt8auto_ptrIN3RBX10Reflection5TupleEEP9lua_StatemENS3_5list3INS_17reference_wrapperIS9_EENS_3argILi1EEENSI_ILi2EEEEEEEvSC_mE6invokeERNS1_15function_bufferESC_m")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int,int (*)(RBX::Reflection::Tuple const&,lua_State *),boost::_bi::list2<boost::reference_wrapper<RBX::Reflection::Tuple const>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiPFiRKN3RBX10Reflection5TupleEP9lua_StateENS3_5list2INS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")]
// 0x2b3ebc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiPFiRKN3RBX10Reflection5TupleEP9lua_StateENS3_5list2INS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
pub fn stub_0x2b3ebc() -> ! {
    todo!("0x2b3ebc __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiPFiRKN3RBX10Reflection5TupleEP9lua_StateENS3_5list2INS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<int,int (*)(RBX::Reflection::Tuple const&,lua_State *),boost::_bi::list2<boost::reference_wrapper<RBX::Reflection::Tuple const>,boost::arg<1>>>,unsigned long,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tIiPFiRKN3RBX10Reflection5TupleEP9lua_StateENS3_5list2INS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEmSB_E6invokeERNS1_15function_bufferESB_")]
// 0x2b3f1c — __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tIiPFiRKN3RBX10Reflection5TupleEP9lua_StateENS3_5list2INS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEmSB_E6invokeERNS1_15function_bufferESB_
pub fn stub_0x2b3f1c() -> ! {
    todo!("0x2b3f1c __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tIiPFiRKN3RBX10Reflection5TupleEP9lua_StateENS3_5list2INS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEmSB_E6invokeERNS1_15function_bufferESB_")
}

#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to_own(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int> const&)")]
#[doc(alias = "__ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE13assign_to_ownERKS7_")]
// 0x2b3f24 — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE13assign_to_ownERKS7_
// was: boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::assign_to_own(boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int> const&)
pub fn stub_0x2b3f24() -> ! {
    todo!("0x2b3f24 __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE13assign_to_ownERKS7_")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>> const&)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE13assign_to_ownERKS7_")]
// 0x2b3f54 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE13assign_to_ownERKS7_
// was: boost::function1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::Reflection::Tuple const>> const&)
pub fn stub_0x2b3f54() -> ! {
    todo!("0x2b3f54 __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE13assign_to_ownERKS7_")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
// 0x2b44f0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2b44f0() -> ! {
    todo!("0x2b44f0 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>)")]
#[doc(alias = "__ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEEvT_")]
// 0x2b45d8 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2b45d8() -> ! {
    todo!("0x2b45d8 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")]
// 0x2b46d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x2b46d0() -> ! {
    todo!("0x2b46d0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEvE6invokeERNS1_15function_bufferE")]
// 0x2b46ec — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_0x2b46ec() -> ! {
    todo!("0x2b46ec __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferE")]
// 0x2b46f4 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2b46f4() -> ! {
    todo!("0x2b46f4 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x2b47dc — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x2b47dc() -> ! {
    todo!("0x2b47dc __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x2b48c0 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x2b48c0() -> ! {
    todo!("0x2b48c0 __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS0_5list1INS0_5valueINS_10shared_ptrIS5_EEEEEEEclEv")]
// 0x2b4994 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS0_5list1INS0_5valueINS_10shared_ptrIS5_EEEEEEEclEv
// was: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>::operator()(void)
pub fn stub_0x2b4994() -> ! {
    todo!("0x2b4994 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS0_5list1INS0_5valueINS_10shared_ptrIS5_EEEEEEEclEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x2b49ac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0x2b49ac() -> ! {
    todo!("0x2b49ac __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>)")]
#[doc(alias = "__ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13ScriptContextEEEEEEC2ES7_")]
// 0x2b4b04 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13ScriptContextEEEEEEC2ES7_
// was: boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::ScriptContext>>)
pub fn stub_0x2b4b04() -> ! {
    todo!("0x2b4b04 __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13ScriptContextEEEEEEC2ES7_")
}

#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEE9singletonEv")]
// 0x2b4be8 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEE9singletonEv
// was: rbx::implementation::typed_holder<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::singleton(void)
pub fn stub_0x2b4be8() -> ! {
    todo!("0x2b4be8 __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEE13destruct_funcEPc")]
// 0x2b4c58 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEE13destruct_funcEPc
// was: rbx::implementation::typed_holder<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::destruct_func(char *)
pub fn stub_0x2b4c58() -> ! {
    todo!("0x2b4c58 __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEE13destruct_funcEPc")
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>::shared_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *)")]
#[doc(alias = "__ZN5boost10shared_ptrIKSt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEC2IS6_EEPT_")]
// 0x2b4c64 — __ZN5boost10shared_ptrIKSt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEC2IS6_EEPT_
// was: boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *)
pub fn stub_0x2b4c64() -> ! {
    todo!("0x2b4c64 __ZN5boost10shared_ptrIKSt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEC2IS6_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2ISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEEEPT_")]
// 0x2b4d38 — __ZN5boost6detail12shared_countC2ISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEEEPT_
// was: boost::detail::shared_count::shared_count<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *)
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x2b4d38() -> ! {
    todo!("0x2b4d38 __ZN5boost6detail12shared_countC2ISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEEEPT_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::vector(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2ERKS6_")]
// 0x2b4e48 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2ERKS6_
// was: std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::vector(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&)
pub fn stub_0x2b4e48() -> ! {
    todo!("0x2b4e48 __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2ERKS6_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>,std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")]
// 0x2b4fb8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_0x2b4fb8() -> ! {
    todo!("0x2b4fb8 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *)")]
#[doc(alias = "__ZN5boost10shared_ptrINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS5_EEEEEC2ISE_EEPT_")]
// 0x2b50e4 — __ZN5boost10shared_ptrINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS5_EEEEEC2ISE_EEPT_
// was: boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *)
pub fn stub_0x2b50e4() -> ! {
    todo!("0x2b50e4 __ZN5boost10shared_ptrINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS5_EEEEEC2ISE_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2INS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEPT_")]
// 0x2b51b8 — __ZN5boost6detail12shared_countC2INS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x2b51b8() -> ! {
    todo!("0x2b51b8 __ZN5boost6detail12shared_countC2INS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEED1Ev")]
// 0x2b52c8 — __ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEED1Ev
pub fn stub_0x2b52c8() -> ! {
    todo!("0x2b52c8 __ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEE7disposeEv")]
// 0x2b52d0 — __ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEE7disposeEv
pub fn stub_0x2b52d0() -> ! {
    todo!("0x2b52d0 __ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEE19get_untyped_deleterEv")]
// 0x2b5378 — __ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEE19get_untyped_deleterEv
pub fn stub_0x2b5378() -> ! {
    todo!("0x2b5378 __ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> const&)")]
#[doc(alias = "__ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt6vectorIN3RBX10Reflection7VariantESaIS7_EEEES9_EEPT_RKNS_10shared_ptrIT0_EE")]
// 0x2b5380 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt6vectorIN3RBX10Reflection7VariantESaIS7_EEEES9_EEPT_RKNS_10shared_ptrIT0_EE
// was: rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> const&)
pub fn stub_0x2b5380() -> ! {
    todo!("0x2b5380 __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt6vectorIN3RBX10Reflection7VariantESaIS7_EEEES9_EEPT_RKNS_10shared_ptrIT0_EE")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEED1Ev")]
// 0x2b5408 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEED1Ev
pub fn stub_0x2b5408() -> ! {
    todo!("0x2b5408 __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEE11get_deleterERKSt9type_info")]
// 0x2b5438 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEE11get_deleterERKSt9type_info
pub fn stub_0x2b5438() -> ! {
    todo!("0x2b5438 __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEE19get_untyped_deleterEv")]
// 0x2b5450 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEE19get_untyped_deleterEv
pub fn stub_0x2b5450() -> ! {
    todo!("0x2b5450 __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE9singletonEv")]
// 0x2b5458 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE9singletonEv
// was: rbx::implementation::typed_holder<boost::shared_ptr<RBX::Reflection::Tuple const>>::singleton(void)
pub fn stub_0x2b5458() -> ! {
    todo!("0x2b5458 __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE13destruct_funcEPc")]
// 0x2b54c8 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE13destruct_funcEPc
// was: rbx::implementation::typed_holder<boost::shared_ptr<RBX::Reflection::Tuple const>>::destruct_func(char *)
pub fn stub_0x2b54c8() -> ! {
    todo!("0x2b54c8 __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE13destruct_funcEPc")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX10Reflection5TupleEEEPT_")]
// 0x2b56c0 — __ZN5boost6detail12shared_countC2IN3RBX10Reflection5TupleEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x2b56c0() -> ! {
    todo!("0x2b56c0 __ZN5boost6detail12shared_countC2IN3RBX10Reflection5TupleEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEED0Ev")]
// 0x2b57d0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEED0Ev
pub fn stub_0x2b57d0() -> ! {
    todo!("0x2b57d0 __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE11get_deleterERKSt9type_info")]
// 0x2b57d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE11get_deleterERKSt9type_info
pub fn stub_0x2b57d8() -> ! {
    todo!("0x2b57d8 __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE11get_deleterERKSt9type_info")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE6insertEPNS8_4slotE")]
// 0x2b57e0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE6insertEPNS8_4slotE
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot *)
// type: void __fastcall(int *, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x2b57e0() -> ! {
    todo!("0x2b57e0 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE6insertEPNS8_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotEEaSEPSA_")]
// 0x2b59ec — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot*)
pub fn stub_0x2b59ec() -> ! {
    todo!("0x2b59ec __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotEEaSEPSA_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotEEaSERKSB_")]
// 0x2b5a10 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotEEaSERKSB_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot> const&)
pub fn stub_0x2b5a10() -> ! {
    todo!("0x2b5a10 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotEEaSERKSB_")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE22safe_static_init_mutexEv")]
// 0x2b5a34 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::safe_static_init_mutex(void)
pub fn stub_0x2b5a34() -> ! {
    todo!("0x2b5a34 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE24safe_static_do_get_mutexEv")]
// 0x2b5a38 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::safe_static_do_get_mutex(void)
pub fn stub_0x2b5a38() -> ! {
    todo!("0x2b5a38 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_13ScriptContextES6_SsS6_EENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED1Ev")]
// 0x2b5b30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_13ScriptContextES6_SsS6_EENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()
pub fn stub_0x2b5b30() -> ! {
    todo!("0x2b5b30 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_13ScriptContextES6_SsS6_EENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_13ScriptContextES6_SsS6_EENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED0Ev")]
// 0x2b5b5c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_13ScriptContextES6_SsS6_EENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()
pub fn stub_0x2b5b5c() -> ! {
    todo!("0x2b5b5c __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_13ScriptContextES6_SsS6_EENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot10disconnectEv")]
// 0x2b5c30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot10disconnectEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::disconnect(void)
pub fn stub_0x2b5c30() -> ! {
    todo!("0x2b5c30 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot9connectedEv")]
// 0x2b5d40 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot9connectedEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::connected(void)const
pub fn stub_0x2b5d40() -> ! {
    todo!("0x2b5d40 __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callES7_SsS7_")]
// 0x2b5d4c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callES7_SsS7_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2b5d4c() -> ! {
    todo!("0x2b5d4c __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callES7_SsS7_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callES7_SsS7_")]
// 0x2b5d68 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callES7_SsS7_
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2b5d68() -> ! {
    todo!("0x2b5d68 __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callES7_SsS7_")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_NS_10shared_ptrINS3_8InstanceEEESsSH_EENS0_5list3IRSH_RSsSK_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0x2b5d84 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_NS_10shared_ptrINS3_8InstanceEEESsSH_EENS0_5list3IRSH_RSsSK_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list4<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_0x2b5d84() -> ! {
    todo!("0x2b5d84 __ZN5boost3_bi5list4INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_NS_10shared_ptrINS3_8InstanceEEESsSH_EENS0_5list3IRSH_RSsSK_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ScriptContext*,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf3IvN3RBX13ScriptContextENS_10shared_ptrINS2_8InstanceEEESsS6_EclEPS3_S6_SsS6_")]
// 0x2b5f3c — __ZNK5boost4_mfi3mf3IvN3RBX13ScriptContextENS_10shared_ptrINS2_8InstanceEEESsS6_EclEPS3_S6_SsS6_
// was: boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>::operator()(RBX::ScriptContext*,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)const
pub fn stub_0x2b5f3c() -> ! {
    todo!("0x2b5f3c __ZNK5boost4_mfi3mf3IvN3RBX13ScriptContextENS_10shared_ptrINS2_8InstanceEEESsS6_EclEPS3_S6_SsS6_")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE6removeEPNS8_4slotE")]
// 0x2b6104 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE6removeEPNS8_4slotE
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot *)
// type: int __fastcall(int, char *)
pub fn stub_0x2b6104() -> ! {
    todo!("0x2b6104 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE6removeEPNS8_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot22safe_static_init_mutexEv")]
// 0x2b61f4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_init_mutex(void)
pub fn stub_0x2b61f4() -> ! {
    todo!("0x2b61f4 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot24safe_static_do_get_mutexEv")]
// 0x2b61f8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)
pub fn stub_0x2b61f8() -> ! {
    todo!("0x2b61f8 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slotD1Ev")]
// 0x2b62e8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slotD1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_0x2b62e8() -> ! {
    todo!("0x2b62e8 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slotD0Ev")]
// 0x2b6314 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slotD0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_0x2b6314() -> ! {
    todo!("0x2b6314 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slotD0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED1Ev")]
// 0x2b63e8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_0x2b63e8() -> ! {
    todo!("0x2b63e8 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED0Ev")]
// 0x2b6414 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_0x2b6414() -> ! {
    todo!("0x2b6414 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED0Ev")
}

#[doc(alias = "boost::scoped_ptr<RBX::LuaAllocator>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX12LuaAllocatorEED2Ev")]
// 0x2b64e8 — __ZN5boost10scoped_ptrIN3RBX12LuaAllocatorEED2Ev
pub fn stub_0x2b64e8() -> ! {
    todo!("0x2b64e8 __ZN5boost10scoped_ptrIN3RBX12LuaAllocatorEED2Ev")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// 0x2b6828 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Script>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::Script>>> *)
pub fn stub_0x2b6828() -> ! {
    todo!("0x2b6828 __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEEE7destroyEPS8_")]
// 0x2b6858 — __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEEE7destroyEPS8_
// was: __gnu_cxx::new_allocator<std::pair<std::string const,boost::shared_ptr<RBX::Script>>>::destroy(std::pair<std::string const,boost::shared_ptr<RBX::Script>>*)
pub fn stub_0x2b6858() -> ! {
    todo!("0x2b6858 __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEEE7destroyEPS8_")
}

#[doc(alias = "boost::scoped_ptr<RBX::Lua::YieldingThreads>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX3Lua15YieldingThreadsEED2Ev")]
// 0x2b6a68 — __ZN5boost10scoped_ptrIN3RBX3Lua15YieldingThreadsEED2Ev
pub fn stub_0x2b6a68() -> ! {
    todo!("0x2b6a68 __ZN5boost10scoped_ptrIN3RBX3Lua15YieldingThreadsEED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::WaitingScriptsJob>(RBX::WaitingScriptsJob *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_17WaitingScriptsJobEEEPT_")]
// 0x2b6efc — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_17WaitingScriptsJobEEEPT_
// was: boost::shared_ptr<RBX::TaskScheduler::Job>::shared_ptr<RBX::WaitingScriptsJob>(RBX::WaitingScriptsJob *)
// type: int __fastcall(int, void *, int, int, int, int)
pub fn stub_0x2b6efc() -> ! {
    todo!("0x2b6efc __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_17WaitingScriptsJobEEEPT_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::WaitingScriptsJob>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const*,RBX::WaitingScriptsJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_17WaitingScriptsJobEEEvPKNS_10shared_ptrIT_EEPT0_")]
// 0x2b6fe4 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_17WaitingScriptsJobEEEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::WaitingScriptsJob>(boost::shared_ptr<RBX::TaskScheduler::Job> const*,RBX::WaitingScriptsJob *)const
pub fn stub_0x2b6fe4() -> ! {
    todo!("0x2b6fe4 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_17WaitingScriptsJobEEEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::WaitingScriptsJob>(RBX::WaitingScriptsJob *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX17WaitingScriptsJobEEEPT_")]
// 0x2b70c8 — __ZN5boost6detail12shared_countC2IN3RBX17WaitingScriptsJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x2b70c8() -> ! {
    todo!("0x2b70c8 __ZN5boost6detail12shared_countC2IN3RBX17WaitingScriptsJobEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEED1Ev")]
// 0x2b71c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEED1Ev
pub fn stub_0x2b71c0() -> ! {
    todo!("0x2b71c0 __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEED0Ev")]
// 0x2b71c4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEED0Ev
pub fn stub_0x2b71c4() -> ! {
    todo!("0x2b71c4 __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE7disposeEv")]
// 0x2b71c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE7disposeEv
pub fn stub_0x2b71c8() -> ! {
    todo!("0x2b71c8 __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE11get_deleterERKSt9type_info")]
// 0x2b71d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE11get_deleterERKSt9type_info
pub fn stub_0x2b71d8() -> ! {
    todo!("0x2b71d8 __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE19get_untyped_deleterEv")]
// 0x2b71dc — __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE19get_untyped_deleterEv
pub fn stub_0x2b71dc() -> ! {
    todo!("0x2b71dc __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>* RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::pushNewObject<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE13pushNewObjectIS5_EEPS5_P9lua_StateT_")]
// 0x2b7770 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE13pushNewObjectIS5_EEPS5_P9lua_StateT_
// was: boost::shared_ptr<RBX::Instance>* RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::pushNewObject<boost::shared_ptr<RBX::Instance>>(lua_State *,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2b7770() -> ! {
    todo!("0x2b7770 __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE13pushNewObjectIS5_EEPS5_P9lua_StateT_")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_index(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8on_indexEP9lua_State")]
// 0x2b7e68 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8on_indexEP9lua_State
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_index(lua_State *)
pub fn stub_0x2b7e68() -> ! {
    todo!("0x2b7e68 __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8on_indexEP9lua_State")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_newindex(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_newindexEP9lua_State")]
// 0x2b7e9c — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_newindexEP9lua_State
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_newindex(lua_State *)
pub fn stub_0x2b7e9c() -> ! {
    todo!("0x2b7e9c __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_newindexEP9lua_State")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaSettings> RBX::Creatable<RBX::Instance>::create<RBX::LuaSettings>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_11LuaSettingsEEEN5boost10shared_ptrIT_EEv")]
// 0x2b8254 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11LuaSettingsEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::LuaSettings> RBX::Creatable<RBX::Instance>::create<RBX::LuaSettings>(void)
pub fn stub_0x2b8254() -> ! {
    todo!("0x2b8254 __ZN3RBX9CreatableINS_8InstanceEE6createINS_11LuaSettingsEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaSettings>::shared_ptr<RBX::LuaSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11LuaSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x2b8304 — __ZN5boost10shared_ptrIN3RBX11LuaSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::LuaSettings>::shared_ptr<RBX::LuaSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x2b8304() -> ! {
    todo!("0x2b8304 __ZN5boost10shared_ptrIN3RBX11LuaSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaSettings,RBX::LuaSettings>(rbx_core::SharedPtr<RBX::LuaSettings> const*,RBX::LuaSettings *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11LuaSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0x2b83cc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11LuaSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaSettings,RBX::LuaSettings>(boost::shared_ptr<RBX::LuaSettings> const*,RBX::LuaSettings *)const
pub fn stub_0x2b83cc() -> ! {
    todo!("0x2b83cc __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11LuaSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX11LuaSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// 0x2b84b8 — __ZN5boost6detail12shared_countC2IPN3RBX11LuaSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x2b84b8() -> ! {
    todo!("0x2b84b8 __ZN5boost6detail12shared_countC2IPN3RBX11LuaSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// 0x2b85c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_0x2b85c0() -> ! {
    todo!("0x2b85c0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// 0x2b85c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_0x2b85c8() -> ! {
    todo!("0x2b85c8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// 0x2b85e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_0x2b85e8() -> ! {
    todo!("0x2b85e8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// 0x2b8600 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_0x2b8600() -> ! {
    todo!("0x2b8600 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::EventDesc(rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// 0x2b8838 — __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::EventDesc(rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x2b8838() -> ! {
    todo!("0x2b8838 __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev")]
// 0x2b8a94 — __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
// was: RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()
pub fn stub_0x2b8a94() -> ! {
    todo!("0x2b8a94 __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
// 0x2b8b48 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
pub fn stub_0x2b8b48() -> ! {
    todo!("0x2b8b48 __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
// 0x2b8c9c — __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// was: RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_0x2b8c9c() -> ! {
    todo!("0x2b8c9c __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")
}

