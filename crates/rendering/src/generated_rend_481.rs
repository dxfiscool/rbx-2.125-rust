//! rendering — generated_rend_481 — 120 stubs EA-sorted asc gap filler not yet in rbx-rendering
//! Source: ida/export.json (85545 funcs) EA-sorted asc not in crates/rendering/src — next 120 uncovered sorted asc after 0x775d3c (0x775d80..0x77c750)
//! Filter: Ogre 9839/9839 + G3D 3882/3882 complete, fallback EA-sorted asc gap filler distinct not yet in rbx-rendering (ranged)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x775d80 — __ZNSt16ostream_iteratorIicSt11char_traitsIcEEaSERKi
#[doc(alias = "std::ostream_iterator<int,char,std::char_traits<char>>::operator=(int const&)")]
#[doc(alias = "__ZNSt16ostream_iteratorIicSt11char_traitsIcEEaSERKi")]
// IDA 0x775d80: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_775d80() {
}

// 0x775da8 — __ZNSt6vectorIiSaIiEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPiS1_EERKi
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<int,std::allocator<int>>::_M_insert_aux(__gnu_cxx::__normal_iterator<int *,std::vector<int,std::allocator<int>>>,int const&)")]
#[doc(alias = "__ZNSt6vectorIiSaIiEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPiS1_EERKi")]
// IDA 0x775da8: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_775da8() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x775e84 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEE9singletonEv")]
// was: rbx::implementation::typed_holder<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::singleton(void)
// IDA 0x775e84: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_775e84() {
}

// 0x775ef0 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEE14construct_funcEPKcPc")]
// was: rbx::implementation::typed_holder<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::construct_func(char const*,char *)
// IDA 0x775ef0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_775ef0() {
}

// 0x775f14 — __ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS8_5list3INS8_5valueISsEENS_17reference_wrapperISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS8_5list3INS8_5valueISsEENS_17reference_wrapperISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS8_5list3INS8_5valueISsEENS_17reference_wrapperISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// IDA 0x775f14: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_775f14() {
}

// 0x776040 — __ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS7_5list3INS7_5valueISsEENS_17reference_wrapperISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS7_5list3INS7_5valueISsEENS_17reference_wrapperISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS7_5list3INS7_5valueISsEENS_17reference_wrapperISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// IDA 0x776040: 105 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776040() {
}

// 0x776170 — __ZN5boost9function2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS7_5list3INS7_5valueISsEENS_17reference_wrapperISC_EENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS7_5list3INS7_5valueISsEENS_17reference_wrapperISC_EENS_3argILi1EEEEEEEEEvT_")]
// was: void boost::function2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>)
// IDA 0x776170: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776170() {
}

// 0x7762b0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEP9lua_StateENS3_5list3INS3_5valueISsEENS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEP9lua_StateENS3_5list3INS3_5valueISsEENS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x7762b0: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7762b0() {
}

// 0x776330 — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEP9lua_StateENS3_5list3INS3_5valueISsEENS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEbSB_P9lua_DebugE6invokeERNS1_15function_bufferESB_SO_
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,bool,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEP9lua_StateENS3_5list3INS3_5valueISsEENS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEbSB_P9lua_DebugE6invokeERNS1_15function_bufferESB_SO_")]
// was: boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,bool,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)
// IDA 0x776330: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776330() {
}

// 0x776354 — __ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES4_ENS9_5list3INS9_5valueISsEENS_17reference_wrapperISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES4_ENS9_5list3INS9_5valueISsEENS_17reference_wrapperISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
// was: bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
// IDA 0x776354: 104 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776354() {
}

// 0x776488 — __ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES4_ENS9_5list3INS9_5valueISsEENS_17reference_wrapperISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int(void)
#[doc(alias = "bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES4_ENS9_5list3INS9_5valueISsEENS_17reference_wrapperISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0x776488: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776488() {
}

// 0x7765b4 — __ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES4_ENS9_5list3INS9_5valueISsEENS_17reference_wrapperISE_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb1EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<true>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES4_ENS9_5list3INS9_5valueISsEENS_17reference_wrapperISE_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb1EEE")]
// was: void boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<true>)const
// IDA 0x7765b4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7765b4() {
}

// 0x7765d8 — __ZN5boost3_bi5list3INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS_3argILi1EEEEclIbPFbSsRS8_P9lua_StateENS0_5list2IRSG_RP9lua_DebugEEEET_NS0_4typeISP_EERT0_RT1_l
// type: int __fastcall(std::string *)
#[doc(alias = "bool boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>::operator()<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<bool>,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *) &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,long)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS_3argILi1EEEEclIbPFbSsRS8_P9lua_StateENS0_5list2IRSG_RP9lua_DebugEEEET_NS0_4typeISP_EERT0_RT1_l")]
// was: bool boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>::operator()<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<bool>,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *) &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,long)
// IDA 0x7765d8: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7765d8() {
}

// 0x776704 — __ZN5boost3_bi5list3INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS_3argILi1EEEEC2ES3_S9_SB_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>::list3(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS_3argILi1EEEEC2ES3_S9_SB_")]
// was: boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>::list3(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>)
// IDA 0x776704: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776704() {
}

// 0x776824 — __ZN5boost3_bi8storage3INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS_3argILi1EEEEC2ES3_S9_SB_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>::storage3(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS_3argILi1EEEEC2ES3_S9_SB_")]
// was: boost::_bi::storage3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>::storage3(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>)
// IDA 0x776824: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776824() {
}

// 0x776944 — __ZN5boost3_bi8storage2INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEEEC2ES3_S9_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>>::storage2(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEEEC2ES3_S9_")]
// was: boost::_bi::storage2<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>>::storage2(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>)
// IDA 0x776944: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776944() {
}

// 0x776a68 — __ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS8_5list4INS8_5valueISsEENS_17reference_wrapperISD_EENSI_IiEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS8_5list4INS8_5valueISsEENS_17reference_wrapperISD_EENSI_IiEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS8_5list4INS8_5valueISsEENS_17reference_wrapperISD_EENSI_IiEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
// IDA 0x776a68: 105 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776a68() {
}

// 0x776b98 — __ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS7_5list4INS7_5valueISsEENS_17reference_wrapperISC_EENSH_IiEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS7_5list4INS7_5valueISsEENS_17reference_wrapperISC_EENSH_IiEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS7_5list4INS7_5valueISsEENS_17reference_wrapperISC_EENSH_IiEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// IDA 0x776b98: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776b98() {
}

// 0x776ccc — __ZN5boost9function2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS7_5list4INS7_5valueISsEENS_17reference_wrapperISC_EENSH_IiEENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS7_5list4INS7_5valueISsEENS_17reference_wrapperISC_EENSH_IiEENS_3argILi1EEEEEEEEEvT_")]
// was: void boost::function2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>)
// IDA 0x776ccc: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776ccc() {
}

// 0x776e10 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x776e10: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776e10() {
}

// 0x776e2c — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEbSB_P9lua_DebugE6invokeERNS1_15function_bufferESB_SP_
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,bool,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEbSB_P9lua_DebugE6invokeERNS1_15function_bufferESB_SP_")]
// was: boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,bool,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)
// IDA 0x776e2c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776e2c() {
}

// 0x776e4c — __ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS4_ENS9_5list4INS9_5valueISsEENS_17reference_wrapperISE_EENSJ_IiEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS4_ENS9_5list4INS9_5valueISsEENS_17reference_wrapperISE_EENSJ_IiEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
// was: bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
// IDA 0x776e4c: 106 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776e4c() {
}

// 0x776f84 — __ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS4_ENS9_5list4INS9_5valueISsEENS_17reference_wrapperISE_EENSJ_IiEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS4_ENS9_5list4INS9_5valueISsEENS_17reference_wrapperISE_EENSJ_IiEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0x776f84: 104 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_776f84() {
}

// 0x7770b4 — __ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS4_ENS9_5list4INS9_5valueISsEENS_17reference_wrapperISE_EENSJ_IiEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IbP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS4_ENS9_5list4INS9_5valueISsEENS_17reference_wrapperISE_EENSJ_IiEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: void boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// IDA 0x7770b4: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7770b4() {
}

// 0x777180 — __ZN5boost3_bi5list4INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS2_IiEENS_3argILi1EEEEclIbPFbSsRS8_iP9lua_StateENS0_5list2IRSH_RP9lua_DebugEEEET_NS0_4typeISQ_EERT0_RT1_l
// type: int __fastcall(std::string *)
#[doc(alias = "bool boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>::operator()<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<bool>,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *) &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,long)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS2_IiEENS_3argILi1EEEEclIbPFbSsRS8_iP9lua_StateENS0_5list2IRSH_RP9lua_DebugEEEET_NS0_4typeISQ_EERT0_RT1_l")]
// was: bool boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>::operator()<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<bool>,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *) &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,long)
// IDA 0x777180: 105 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_777180() {
}

// 0x7772b0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// IDA 0x7772b0: 127 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7772b0() {
}

// 0x777400 — __ZNK5boost9function2IbP9lua_StateP9lua_DebugEclES2_S4_
#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
#[doc(alias = "__ZNK5boost9function2IbP9lua_StateP9lua_DebugEclES2_S4_")]
// was: boost::function2<bool,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const
// IDA 0x777400: 71 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_777400() {
}

// 0x7774cc — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES10_
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES10_")]
// was: __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES10_
// IDA 0x7774cc: 100 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7774cc() {
}

// 0x7775e4 — __ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbS5_S7_EEERbRNS_10shared_ptrISsEEPS3_NS_3argILi1EEENSG_ILi2EEESA_NS_17reference_wrapperIbEENSJ_ISD_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISO_T0_T1_T2_T3_T4_T5_EENSM_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMSR_FSO_SS_ST_SU_SV_SW_ESZ_S10_S11_S12_S13_S14_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbS5_S7_EEERbRNS_10shared_ptrISsEEPS3_NS_3argILi1EEENSG_ILi2EEESA_NS_17reference_wrapperIbEENSJ_ISD_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISO_T0_T1_T2_T3_T4_T5_EENSM_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMSR_FSO_SS_ST_SU_SV_SW_ESZ_S10_S11_S12_S13_S14_")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>)
// IDA 0x7775e4: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7775e4() {
}

// 0x777708 — __ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookIbEEvP9lua_StateP9lua_DebugN5boost8functionIFT_S4_S6_EEERS9_RNS7_10shared_ptrISsEE
// type: int __fastcall(int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, void *, int)
#[doc(alias = "void RBX::Scripting::ScriptDebugger::withPausedThreadHook<bool>(lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool&,rbx_core::SharedPtr<std::string> &)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookIbEEvP9lua_StateP9lua_DebugN5boost8functionIFT_S4_S6_EEERS9_RNS7_10shared_ptrISsEE")]
// was: void RBX::Scripting::ScriptDebugger::withPausedThreadHook<bool>(lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool&,boost::shared_ptr<std::string> &)
// IDA 0x777708: 306 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_777708() {
}

// 0x777a6c — __ZN5boost9function2IbP9lua_StateP9lua_DebugE13assign_to_ownERKS5_
#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::assign_to_own(boost::function2<bool,lua_State *,lua_Debug *> const&)")]
#[doc(alias = "__ZN5boost9function2IbP9lua_StateP9lua_DebugE13assign_to_ownERKS5_")]
// was: boost::function2<bool,lua_State *,lua_Debug *>::assign_to_own(boost::function2<bool,lua_State *,lua_Debug *> const&)
// IDA 0x777a6c: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_777a6c() {
}

// 0x778034 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SI_SK_SN_
#[doc(alias = "boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SI_SK_SN_")]
// was: boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>)
// IDA 0x778034: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_778034() {
}

// 0x778108 — __ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SI_SK_SN_
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SI_SK_SN_")]
// was: boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>)
// IDA 0x778108: 77 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_778108() {
}

// 0x7781e0 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SA_SI_SK_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SA_SI_SK_")]
// was: boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>)
// IDA 0x7781e0: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7781e0() {
}

// 0x7782bc — __ZN5boost9function2IvP9lua_StateP9lua_DebugE4swapERS5_
#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::swap(boost::function2<void,lua_State *,lua_Debug *>&)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE4swapERS5_")]
// was: boost::function2<void,lua_State *,lua_Debug *>::swap(boost::function2<void,lua_State *,lua_Debug *>&)
// IDA 0x7782bc: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7782bc() {
}

// 0x778398 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE11move_assignERS5_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::move_assign(boost::function2<void,lua_State *,lua_Debug *>&)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE11move_assignERS5_")]
// was: boost::function2<void,lua_State *,lua_Debug *>::move_assign(boost::function2<void,lua_State *,lua_Debug *>&)
// IDA 0x778398: 97 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_778398() {
}

// 0x77849c — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE
// IDA 0x77849c: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77849c() {
}

// 0x778588 — __ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE
// IDA 0x778588: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_778588() {
}

// 0x778674 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEEvT_
#[doc(alias = "void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEEvT_")]
// was: void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>)
// IDA 0x778674: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_778674() {
}

// 0x778770 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEE6manageERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEE6manageERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x778770: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_778770() {
}

// 0x77878c — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_")]
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)
// IDA 0x77878c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77878c() {
}

// 0x7787b0 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEbT_RNS1_15function_bufferE")]
// was: bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &)const
// IDA 0x7787b0: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7787b0() {
}

// 0x7788a0 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0x7788a0: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7788a0() {
}

// 0x778988 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// IDA 0x778988: 78 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_778988() {
}

// 0x778a64 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEclINS_4_mfi3mf5IvS5_SD_SF_SH_RbRSM_EENS0_5list2IRSD_RSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEclINS_4_mfi3mf5IvS5_SD_SF_SH_RbRSM_EENS0_5list2IRSD_RSF_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)
// IDA 0x778a64: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_778a64() {
}

// 0x778b48 — __ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbS6_S8_EEERbRNS_10shared_ptrISsEEEclEPS4_S6_S8_SB_SC_SF_
#[doc(alias = "boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbS6_S8_EEERbRNS_10shared_ptrISsEEEclEPS4_S6_S8_SB_SC_SF_")]
// was: boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &)const
// IDA 0x778b48: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_778b48() {
}

// 0x778c34 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEE7managerERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEE7managerERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// IDA 0x778c34: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_778c34() {
}

// 0x778d94 — __ZN5boost3_bi5list4INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS2_IiEENS_3argILi1EEEEC2ES3_S9_SA_SC_
#[doc(alias = "boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>::list4(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS2_IiEENS_3argILi1EEEEC2ES3_S9_SA_SC_")]
// was: boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>::list4(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>)
// IDA 0x778d94: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_778d94() {
}

// 0x778ebc — __ZN5boost3_bi8storage4INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS2_IiEENS_3argILi1EEEEC2ES3_S9_SA_SC_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>::storage4(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS2_IiEENS_3argILi1EEEEC2ES3_S9_SA_SC_")]
// was: boost::_bi::storage4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>::storage4(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>)
// IDA 0x778ebc: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_778ebc() {
}

// 0x778fe4 — __ZN5boost3_bi8storage3INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS2_IiEEEC2ES3_S9_SA_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>>::storage3(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueISsEENS_17reference_wrapperIKN3RBX10Reflection7VariantEEENS2_IiEEEC2ES3_S9_SA_")]
// was: boost::_bi::storage3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>>::storage3(boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>)
// IDA 0x778fe4: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_778fe4() {
}

// 0x77910c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_P9lua_StateENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_P9lua_StateENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x77910c: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77910c() {
}

// 0x77916c — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_P9lua_StateENS3_5list1INS_3argILi1EEEEEEENS5_IKSG_EESJ_P9lua_DebugE6invokeERNS1_15function_bufferESJ_SU_
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_P9lua_StateENS3_5list1INS_3argILi1EEEEEEENS5_IKSG_EESJ_P9lua_DebugE6invokeERNS1_15function_bufferESJ_SU_")]
// was: boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)
// IDA 0x77916c: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77916c() {
}

// 0x779234 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_iP9lua_StateENS3_5list2INS3_5valueIiEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(int,lua_State *),boost::_bi::list2<boost::_bi::value<int>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_iP9lua_StateENS3_5list2INS3_5valueIiEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(int,lua_State *),boost::_bi::list2<boost::_bi::value<int>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x779234: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779234() {
}

// 0x779294 — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_iP9lua_StateENS3_5list2INS3_5valueIiEENS_3argILi1EEEEEEENS5_IKSG_EESJ_P9lua_DebugE6invokeERNS1_15function_bufferESJ_SW_
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(int,lua_State *),boost::_bi::list2<boost::_bi::value<int>,boost::arg<1>>>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tINS_10shared_ptrISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEEPFSH_iP9lua_StateENS3_5list2INS3_5valueIiEENS_3argILi1EEEEEEENS5_IKSG_EESJ_P9lua_DebugE6invokeERNS1_15function_bufferESJ_SW_")]
// was: boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> (*)(int,lua_State *),boost::_bi::list2<boost::_bi::value<int>,boost::arg<1>>>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)
// IDA 0x779294: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779294() {
}

// 0x779360 — __ZNK5boost9function2INS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEP9lua_StateP9lua_DebugEclESG_SI_
#[doc(alias = "boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
#[doc(alias = "__ZNK5boost9function2INS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEP9lua_StateP9lua_DebugEclESG_SI_")]
// was: boost::function2<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const
// IDA 0x779360: 71 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779360() {
}

// 0x77942c — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES1C_
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES1C_")]
// was: __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES1C_
// IDA 0x77942c: 100 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77942c() {
}

// 0x779544 — __ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIKSsSC_EEEEES5_S7_EEERSL_RNS9_ISsEEPS3_NS_3argILi1EEENSS_ILi2EEESN_NS_17reference_wrapperISL_EENSV_ISP_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5IS10_T0_T1_T2_T3_T4_T5_EENSY_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMS13_FS10_S14_S15_S16_S17_S18_ES1B_S1C_S1D_S1E_S1F_S1G_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIKSsSC_EEEEES5_S7_EEERSL_RNS9_ISsEEPS3_NS_3argILi1EEENSS_ILi2EEESN_NS_17reference_wrapperISL_EENSV_ISP_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5IS10_T0_T1_T2_T3_T4_T5_EENSY_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMS13_FS10_S14_S15_S16_S17_S18_ES1B_S1C_S1D_S1E_S1F_S1G_")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>)
// IDA 0x779544: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779544() {
}

// 0x779668 — __ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookIN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEEEvP9lua_StateP9lua_DebugNS3_8functionIFT_SI_SK_EEERSM_RNS4_ISsEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, char, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::Scripting::ScriptDebugger::withPausedThreadHook<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookIN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEEEvP9lua_StateP9lua_DebugNS3_8functionIFT_SI_SK_EEERSM_RNS4_ISsEE")]
// was: void RBX::Scripting::ScriptDebugger::withPausedThreadHook<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &)
// IDA 0x779668: 327 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779668() {
}

// 0x7799fc — __ZN5boost9function2INS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEP9lua_StateP9lua_DebugE13assign_to_ownERKSJ_
#[doc(alias = "boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::assign_to_own(boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *> const&)")]
#[doc(alias = "__ZN5boost9function2INS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEP9lua_StateP9lua_DebugE13assign_to_ownERKSJ_")]
// was: boost::function2<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::assign_to_own(boost::function2<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *> const&)
// IDA 0x7799fc: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7799fc() {
}

// 0x779a2c — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEC2ES7_S9_SA_SV_SX_SZ_
#[doc(alias = "boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEC2ES7_S9_SA_SV_SX_SZ_")]
// was: boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>)
// IDA 0x779a2c: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779a2c() {
}

// 0x779b00 — __ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEC2ES7_S9_SA_SV_SX_SZ_
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEC2ES7_S9_SA_SV_SX_SZ_")]
// was: boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>)
// IDA 0x779b00: 77 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779b00() {
}

// 0x779bd8 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EEEC2ES7_S9_SA_SV_SX_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EEEC2ES7_S9_SA_SV_SX_")]
// was: boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>)
// IDA 0x779bd8: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779bd8() {
}

// 0x779cb4 — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE
// IDA 0x779cb4: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779cb4() {
}

// 0x779da0 — __ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSB_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSB_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSB_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE
// IDA 0x779da0: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779da0() {
}

// 0x779e8c — __ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSB_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEEvT_
#[doc(alias = "void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSB_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEEvT_")]
// was: void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>)
// IDA 0x779e8c: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779e8c() {
}

// 0x779f88 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEE6manageERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEE6manageERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x779f88: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779f88() {
}

// 0x779fa4 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_")]
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)
// IDA 0x779fa4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779fa4() {
}

// 0x779fc8 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEbT_RNS1_15function_bufferE")]
// was: bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &)const
// IDA 0x779fc8: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_779fc8() {
}

// 0x77a0b8 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0x77a0b8: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77a0b8() {
}

// 0x77a1a0 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSD_10Reflection7VariantESt4lessISsESaISt4pairIKSsSK_EEEEES4_S6_EEERST_RNSH_ISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENS14_ILi2EEENS11_ISV_EENS_17reference_wrapperIST_EENS18_ISX_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// IDA 0x77a1a0: 78 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77a1a0() {
}

// 0x77a27c — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEclINS_4_mfi3mf5IvS5_SQ_SS_SU_RSO_RSY_EENS0_5list2IRSQ_RSS_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperISO_EENSW_INSC_ISsEEEEEclINS_4_mfi3mf5IvS5_SQ_SS_SU_RSO_RSY_EENS0_5list2IRSQ_RSS_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)
// IDA 0x77a27c: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77a27c() {
}

// 0x77a360 — __ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS2_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEES6_S8_EEERSM_RNSA_ISsEEEclEPS4_S6_S8_SO_SP_SR_
#[doc(alias = "boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS2_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEES6_S8_EEERSM_RNSA_ISsEEEclEPS4_S6_S8_SO_SP_SR_")]
// was: boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &)const
// IDA 0x77a360: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77a360() {
}

// 0x77a44c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEE7managerERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEE7managerERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// IDA 0x77a44c: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77a44c() {
}

// 0x77a5ac — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS6_5list1INS6_5valueIPSC_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS6_5list1INS6_5valueIPSC_EEEEEEED1Ev")]
// was: rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>>::~callable_slot()
// IDA 0x77a5ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77a5ac() {
}

// 0x77a5d8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS6_5list1INS6_5valueIPSC_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS6_5list1INS6_5valueIPSC_EEEEEEED0Ev")]
// was: rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>>::~callable_slot()
// IDA 0x77a5d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77a5d8() {
}

// 0x77a6b0 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv")]
// was: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::call(void)
// IDA 0x77a6b0: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77a6b0() {
}

// 0x77a6b8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::call(void)
// IDA 0x77a6b8: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77a6b8() {
}

// 0x77a6c0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS0_5list1INS0_5valueIPS6_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS0_5list1INS0_5valueIPS6_EEEEEclEv")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>::operator()(void)
// IDA 0x77a6c0: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77a6c0() {
}

// 0x77a6d8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_ED1Ev")]
// was: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::~callable()
// IDA 0x77a6d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77a6d8() {
}

// 0x77a704 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_ED0Ev")]
// was: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::~callable()
// IDA 0x77a704: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77a704() {
}

// 0x77a7dc — __ZN3rbx7signals6signalIFvP9lua_StateEE6insertEPNS5_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::insert(rbx::signals::signal<void ()(lua_State *)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE6insertEPNS5_4slotE")]
// IDA 0x77a7dc: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77a7dc() {
}

// 0x77a9e8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvP9lua_StateEE4slotEEaSEPS8_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot>::operator=(rbx::signals::signal<void ()(lua_State *)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvP9lua_StateEE4slotEEaSEPS8_")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(lua_State *)>::slot>::operator=(rbx::signals::signal<void ()(lua_State *)>::slot*)
// IDA 0x77a9e8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77a9e8() {
}

// 0x77aa10 — __ZN3rbx7signals6signalIFvP9lua_StateEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE22safe_static_init_mutexEv")]
// IDA 0x77aa10: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_77aa10() {
}

// 0x77aa14 — __ZN3rbx7signals6signalIFvP9lua_StateEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev")]
// was: rbx::signals::signal<void ()(lua_State *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>>::~callable_slot()
// IDA 0x77aa14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77aa14() {
}

// 0x77aa40 — __ZN3rbx7signals6signalIFvP9lua_StateEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev")]
// was: rbx::signals::signal<void ()(lua_State *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>>::~callable_slot()
// IDA 0x77aa40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77aa40() {
}

// 0x77ab14 — __ZN3rbx7signals6signalIFvP9lua_StateEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE4slot10disconnectEv")]
// IDA 0x77ab14: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ab14() {
}

// 0x77ac24 — __ZNK3rbx7signals6signalIFvP9lua_StateEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvP9lua_StateEE4slot9connectedEv")]
// IDA 0x77ac24: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ac24() {
}

// 0x77ac30 — __ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::call(lua_State *)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
// was: rbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::call(lua_State *)
// IDA 0x77ac30: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ac30() {
}

// 0x77ac44 — __ZThn4_N3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::call(lua_State *)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::call(lua_State *)
// IDA 0x77ac44: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ac44() {
}

// 0x77ac58 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS8_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>::operator()<lua_State *>(lua_State * &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS8_EEvRT_")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>::operator()<lua_State *>(lua_State * &)
// IDA 0x77ac58: 9 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ac58() {
}

// 0x77ac70 — __ZN3rbx7signals6signalIFvP9lua_StateEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::remove(rbx::signals::signal<void ()(lua_State *)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE6removeEPNS5_4slotE")]
// IDA 0x77ac70: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ac70() {
}

// 0x77ad60 — __ZN3rbx7signals6signalIFvP9lua_StateEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE4slot22safe_static_init_mutexEv")]
// IDA 0x77ad60: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_77ad60() {
}

// 0x77ad64 — __ZN3rbx7signals6signalIFvP9lua_StateEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE4slot24safe_static_do_get_mutexEv")]
// IDA 0x77ad64: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77ad64() {
}

// 0x77ae54 — __ZN3rbx7signals6signalIFvP9lua_StateEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE4slotD1Ev")]
// IDA 0x77ae54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77ae54() {
}

// 0x77ae80 — __ZN3rbx7signals6signalIFvP9lua_StateEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE4slotD0Ev")]
// IDA 0x77ae80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77ae80() {
}

// 0x77af54 — __ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev")]
// was: rbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::~callable()
// IDA 0x77af54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77af54() {
}

// 0x77af80 — __ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev")]
// was: rbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::~callable()
// IDA 0x77af80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77af80() {
}

// 0x77b058 — __ZNSt6vectorIPN3RBX9Scripting13DebuggerWatchESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX9Scripting13DebuggerWatchESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
// IDA 0x77b058: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_77b058() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x77b138 — __ZNSt12_Vector_baseIPN3RBX9Scripting13DebuggerWatchESaIS3_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX9Scripting13DebuggerWatchESaIS3_EE11_M_allocateEm")]
// IDA 0x77b138: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_77b138() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x77b150 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEEixERS5_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::operator[](int const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEEixERS5_")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::operator[](int const&)
// IDA 0x77b150: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77b150() {
}

// 0x77b2cc — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE18reserve_for_insertEm")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::reserve_for_insert(unsigned long)
// IDA 0x77b2cc: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77b2cc() {
}

// 0x77b320 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE14create_bucketsEm")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::create_buckets(unsigned long)
// IDA 0x77b320: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77b320() {
}

// 0x77b448 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE20min_buckets_for_sizeEm")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::min_buckets_for_size(unsigned long)const
// IDA 0x77b448: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77b448() {
}

// 0x77b4d8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::rehash_impl(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE11rehash_implEm")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::rehash_impl(unsigned long)
// IDA 0x77b4d8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77b4d8() {
}

// 0x77b504 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>> &,boost::unordered::detail::ptr_bucket *)
// IDA 0x77b504: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77b504() {
}

// 0x77b55c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>>>::construct(void)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEEEE9constructEv")]
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>>>::construct(void)
// IDA 0x77b55c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77b55c() {
}

// 0x77b594 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE14find_node_implIiSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::find_node_impl<int,std::equal_to<int>>(unsigned long,int const&,std::equal_to<int> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE14find_node_implIiSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_")]
// was: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::find_node_impl<int,std::equal_to<int>>(unsigned long,int const&,std::equal_to<int> const&)const
// IDA 0x77b594: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77b594() {
}

// 0x77b600 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX9Scripting13DebuggerWatchESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch *>(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&,std::random_access_iterator_tag)")]
#[doc(alias = "__ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX9Scripting13DebuggerWatchESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_St26random_access_iterator_tag")]
// IDA 0x77b600: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77b600() {
}

// 0x77b690 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE9erase_keyERS5_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::erase_key(int const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE9erase_keyERS5_")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::erase_key(int const&)
// IDA 0x77b690: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77b690() {
}

// 0x77b710 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE12delete_nodesEPNS1_10ptr_bucketESJ_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE12delete_nodesEPNS1_10ptr_bucketESJ_")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)
// IDA 0x77b710: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77b710() {
}

// 0x77b74c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE10fix_bucketEmPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE10fix_bucketEmPNS1_10ptr_bucketE")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)
// IDA 0x77b74c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77b74c() {
}

// 0x77b790 — __ZN3RBX10Reflection9DescribedINS_9Scripting13DebuggerWatchELZNS2_14sDebuggerWatchEENS_14FactoryProductIS3_NS_8InstanceELZNS2_14sDebuggerWatchEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE2EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting13DebuggerWatchELZNS2_14sDebuggerWatchEENS_14FactoryProductIS3_NS_8InstanceELZNS2_14sDebuggerWatchEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE2EE15classDescriptorEv")]
// IDA 0x77b790: 92 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77b790() {
}

// 0x77b8ac — __ZNSt6vectorIN3RBX9Scripting14ScriptDebugger12FunctionInfoESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
#[doc(alias = "std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Scripting::ScriptDebugger::FunctionInfo*,std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>>,RBX::Scripting::ScriptDebugger::FunctionInfo const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9Scripting14ScriptDebugger12FunctionInfoESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
// IDA 0x77b8ac: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_77b8ac() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x77c1c4 — __ZN3RBX9Scripting14ScriptDebugger12FunctionInfoaSERKS2_
#[doc(alias = "RBX::Scripting::ScriptDebugger::FunctionInfo::operator=(RBX::Scripting::ScriptDebugger::FunctionInfo const&)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger12FunctionInfoaSERKS2_")]
// IDA 0x77c1c4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77c1c4() {
}

// 0x77c214 — __ZNSt12_Vector_baseIN3RBX9Scripting14ScriptDebugger12FunctionInfoESaIS3_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9Scripting14ScriptDebugger12FunctionInfoESaIS3_EE11_M_allocateEm")]
// IDA 0x77c214: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_77c214() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x77c238 — __ZSt22__uninitialized_copy_aIPN3RBX9Scripting14ScriptDebugger12FunctionInfoES4_S3_ET0_T_S6_S5_SaIT1_E
#[doc(alias = "RBX::Scripting::ScriptDebugger::FunctionInfo * std::__uninitialized_copy_a<RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo>(RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>)")]
#[doc(alias = "__ZSt22__uninitialized_copy_aIPN3RBX9Scripting14ScriptDebugger12FunctionInfoES4_S3_ET0_T_S6_S5_SaIT1_E")]
// IDA 0x77c238: 306 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77c238() {
}

// 0x77c538 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9Scripting14ScriptDebugger12FunctionInfoES7_EET0_T_S9_S8_
#[doc(alias = "RBX::Scripting::ScriptDebugger::FunctionInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *>(RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9Scripting14ScriptDebugger12FunctionInfoES7_EET0_T_S9_S8_")]
// IDA 0x77c538: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_77c538() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x77c594 — __ZNSt6vectorIN3RBX9Scripting14ScriptDebugger12FunctionInfoESaIS3_EED2Ev
#[doc(alias = "std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3RBX9Scripting14ScriptDebugger12FunctionInfoESaIS3_EED2Ev")]
// IDA 0x77c594: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77c594() {
}

// 0x77c700 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10Reflection7VariantEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
// type: int __fastcall(int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::find(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10Reflection7VariantEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")]
// IDA 0x77c700: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77c700() {
}

// 0x77c750 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEESE_EEPT_RKNS_10shared_ptrIT0_EE
#[doc(alias = "rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> const&)")]
#[doc(alias = "__ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEESE_EEPT_RKNS_10shared_ptrIT0_EE")]
// was: rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> const&)
// IDA 0x77c750: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77c750() {
}
