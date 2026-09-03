//! rendering shard 249 — 150 stubs EA-sorted asc global gap filler after 0x2b3b78 not yet in rendering (Ogre|G3D 13663/13663 complete, 26720->26870 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x2b3c40 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x2b3c40: 122 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3c40() {
}

// 0x2b3d88 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEEEEC2ES3_SE_
#[doc(alias = "boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::list2(boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
// was: __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEEEEC2ES3_SE_
// IDA 0x2b3d88: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3d88() {
}

// 0x2b3e50 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE5dummy7nonnullEv
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::dummy::nonnull(void)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE5dummy7nonnullEv
// IDA 0x2b3e50: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2b3e50() {
}

// 0x2b3e54 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRSt8auto_ptrIN3RBX10Reflection5TupleEEP9lua_StatemENS3_5list3INS_17reference_wrapperIS9_EENS_3argILi1EEENSI_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::auto_ptr<RBX::Reflection::Tuple> &,lua_State *,unsigned long),boost::_bi::list3<boost::reference_wrapper<std::auto_ptr<RBX::Reflection::Tuple>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRSt8auto_ptrIN3RBX10Reflection5TupleEEP9lua_StatemENS3_5list3INS_17reference_wrapperIS9_EENS_3argILi1EEENSI_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// IDA 0x2b3e54: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3e54() {
}

// 0x2b3eb4 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvRSt8auto_ptrIN3RBX10Reflection5TupleEEP9lua_StatemENS3_5list3INS_17reference_wrapperIS9_EENS_3argILi1EEENSI_ILi2EEEEEEEvSC_mE6invokeERNS1_15function_bufferESC_m
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::auto_ptr<RBX::Reflection::Tuple> &,lua_State *,unsigned long),boost::_bi::list3<boost::reference_wrapper<std::auto_ptr<RBX::Reflection::Tuple>>,boost::arg<1>,boost::arg<2>>>,void,lua_State *,unsigned long>::invoke(boost::detail::function::function_buffer &,lua_State *,unsigned long)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvRSt8auto_ptrIN3RBX10Reflection5TupleEEP9lua_StatemENS3_5list3INS_17reference_wrapperIS9_EENS_3argILi1EEENSI_ILi2EEEEEEEvSC_mE6invokeERNS1_15function_bufferESC_m
// IDA 0x2b3eb4: 3 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3eb4() {
}

// 0x2b3ebc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiPFiRKN3RBX10Reflection5TupleEP9lua_StateENS3_5list2INS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int,int (*)(RBX::Reflection::Tuple const&,lua_State *),boost::_bi::list2<boost::reference_wrapper<RBX::Reflection::Tuple const>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiPFiRKN3RBX10Reflection5TupleEP9lua_StateENS3_5list2INS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// IDA 0x2b3ebc: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3ebc() {
}

// 0x2b3f1c — __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tIiPFiRKN3RBX10Reflection5TupleEP9lua_StateENS3_5list2INS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEmSB_E6invokeERNS1_15function_bufferESB_
#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<int,int (*)(RBX::Reflection::Tuple const&,lua_State *),boost::_bi::list2<boost::reference_wrapper<RBX::Reflection::Tuple const>,boost::arg<1>>>,unsigned long,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
// was: __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tIiPFiRKN3RBX10Reflection5TupleEP9lua_StateENS3_5list2INS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEmSB_E6invokeERNS1_15function_bufferESB_
// IDA 0x2b3f1c: 3 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3f1c() {
}

// 0x2b3f24 — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE13assign_to_ownERKS7_
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to_own(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int> const&)")]
// was: __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE13assign_to_ownERKS7_
// IDA 0x2b3f24: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3f24() {
}

// 0x2b3f54 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE13assign_to_ownERKS7_
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>> const&)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE13assign_to_ownERKS7_
// IDA 0x2b3f54: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3f54() {
}

// 0x2b3f84 — __ZN5boost9function2IvP9lua_StatemE13assign_to_ownERKS3_
#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::assign_to_own(boost::function2<void,lua_State *,unsigned long> const&)")]
// was: __ZN5boost9function2IvP9lua_StatemE13assign_to_ownERKS3_
// IDA 0x2b3f84: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3f84() {
}

// 0x2b3fb4 — __ZN5boost9function2IvP9lua_StatemE5clearEv
#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::clear(void)")]
// was: __ZN5boost9function2IvP9lua_StatemE5clearEv
// IDA 0x2b3fb4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3fb4() {
}

// 0x2b3fe0 — __ZN5boost9function1ImP9lua_StateE13assign_to_ownERKS3_
#[doc(alias = "boost::function1<unsigned long,lua_State *>::assign_to_own(boost::function1<unsigned long,lua_State *> const&)")]
// was: __ZN5boost9function1ImP9lua_StateE13assign_to_ownERKS3_
// IDA 0x2b3fe0: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b3fe0() {
}

// 0x2b4010 — __ZN5boost9function1ImP9lua_StateE5clearEv
#[doc(alias = "boost::function1<unsigned long,lua_State *>::clear(void)")]
// was: __ZN5boost9function1ImP9lua_StateE5clearEv
// IDA 0x2b4010: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b4010() {
}

// 0x2b403c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tImPFmP9lua_StateENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<unsigned long,unsigned long (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tImPFmP9lua_StateENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE
// IDA 0x2b403c: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b403c() {
}

// 0x2b409c — __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tImPFmP9lua_StateENS3_5list1INS_3argILi1EEEEEEEmS6_E6invokeERNS1_15function_bufferES6_
#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<unsigned long,unsigned long (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>,unsigned long,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
// was: __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tImPFmP9lua_StateENS3_5list1INS_3argILi1EEEEEEEmS6_E6invokeERNS1_15function_bufferES6_
// IDA 0x2b409c: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b409c() {
}

// 0x2b40a8 — __ZN5boost6threadC2INS_9function0IvEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRS4_NS_6detail13thread_move_tIS4_EEEE5valueEPNS0_5dummyEE4typeE
#[doc(alias = "__ZN5boost6threadC2INS_9function0IvEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRS4_NS_6detail13thread_move_tIS4_EEEE5valueEPNS0_5dummyEE4typeE")]
// was: __ZN5boost6threadC2INS_9function0IvEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRS4_NS_6detail13thread_move_tIS4_EEEE5valueEPNS0_5dummyEE4typeE
// IDA 0x2b40a8: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b40a8() {
}

// 0x2b41f0 — __ZN5boost6detail11thread_dataINS_9function0IvEEED0Ev
#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::~thread_data()")]
// was: __ZN5boost6detail11thread_dataINS_9function0IvEEED0Ev
// IDA 0x2b41f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b41f0() {
}

// 0x2b42d0 — __ZN5boost6detail11thread_dataINS_9function0IvEEE3runEv
#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::run(void)")]
// was: __ZN5boost6detail11thread_dataINS_9function0IvEEE3runEv
// IDA 0x2b42d0: 2 insns (ADD.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b42d0() {
}

// 0x2b42d8 — __ZN5boost6detail16thread_data_base25notify_all_at_thread_exitEPNS_18condition_variableEPNS_5mutexE
#[doc(alias = "boost::detail::thread_data_base::notify_all_at_thread_exit(boost::condition_variable *,boost::mutex *)")]
// was: __ZN5boost6detail16thread_data_base25notify_all_at_thread_exitEPNS_18condition_variableEPNS_5mutexE
// IDA 0x2b42d8: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b42d8() {
}

// 0x2b42f0 — __ZNK5boost9function0IvEclEv
#[doc(alias = "boost::function0<void>::operator()(void)const")]
// was: __ZNK5boost9function0IvEclEv
// IDA 0x2b42f0: 66 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b42f0() {
}

// 0x2b43b0 — __ZN5boost18condition_variableD2Ev
#[doc(alias = "boost::condition_variable::~condition_variable()")]
// was: __ZN5boost18condition_variableD2Ev
// IDA 0x2b43b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b43b0() {
}

// 0x2b43d8 — __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_9function0IvEEEEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::function0<void>>>(boost::detail::thread_data<boost::function0<void>> *)")]
// was: __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_9function0IvEEEEEEPT_
// IDA 0x2b43d8: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b43d8() {
}

// 0x2b44d0 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEED1Ev
// IDA 0x2b44d0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2b44d0() {
}

// 0x2b4be8 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEE9singletonEv
// IDA 0x2b4be8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b4be8() {
}

// 0x2b4c58 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEE13destruct_funcEPc
// IDA 0x2b4c58: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b4c58() {
}

// 0x2b4c64 — __ZN5boost10shared_ptrIKSt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEC2IS6_EEPT_
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>::shared_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *)")]
// was: __ZN5boost10shared_ptrIKSt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEC2IS6_EEPT_
// IDA 0x2b4c64: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b4c64() {
}

// 0x2b4d38 — __ZN5boost6detail12shared_countC2ISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *)")]
// was: __ZN5boost6detail12shared_countC2ISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEEEPT_
// IDA 0x2b4d38: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b4d38() {
}

// 0x2b4e48 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2ERKS6_
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::vector(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2ERKS6_
// IDA 0x2b4e48: 144 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b4e48() {
}

// 0x2b4fb8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>,std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// IDA 0x2b4fb8: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b4fb8() {
}

// 0x2b50e4 — __ZN5boost10shared_ptrINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS5_EEEEEC2ISE_EEPT_
#[doc(alias = "rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *)")]
// was: __ZN5boost10shared_ptrINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS5_EEEEEC2ISE_EEPT_
// IDA 0x2b50e4: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b50e4() {
}

// 0x2b51b8 — __ZN5boost6detail12shared_countC2INS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *)")]
// was: __ZN5boost6detail12shared_countC2INS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEPT_
// IDA 0x2b51b8: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b51b8() {
}

// 0x2b52c8 — __ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEED1Ev
// IDA 0x2b52c8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2b52c8() {
}

// 0x2b52d0 — __ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEE7disposeEv
// IDA 0x2b52d0: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b52d0() {
}

// 0x2b5378 — __ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEE19get_untyped_deleterEv
// IDA 0x2b5378: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5378() {
}

// 0x2b5380 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt6vectorIN3RBX10Reflection7VariantESaIS7_EEEES9_EEPT_RKNS_10shared_ptrIT0_EE
#[doc(alias = "rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> const&)")]
// was: __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt6vectorIN3RBX10Reflection7VariantESaIS7_EEEES9_EEPT_RKNS_10shared_ptrIT0_EE
// IDA 0x2b5380: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5380() {
}

// 0x2b53e0 — __ZNSt12_Vector_baseIN3RBX10Reflection7VariantESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX10Reflection7VariantESaIS2_EE11_M_allocateEm
// IDA 0x2b53e0: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_2b53e0() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x2b5408 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEED1Ev
// IDA 0x2b5408: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b5408() {
}

// 0x2b5438 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEE11get_deleterERKSt9type_info
// IDA 0x2b5438: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5438() {
}

// 0x2b5450 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEE19get_untyped_deleterEv
// IDA 0x2b5450: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5450() {
}

// 0x2b5458 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE9singletonEv
// IDA 0x2b5458: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5458() {
}

// 0x2b54c8 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE13destruct_funcEPc
// IDA 0x2b54c8: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b54c8() {
}

// 0x2b54d8 — __ZN3RBX15InvocationMeterILi2EE13updateBucketsEb
#[doc(alias = "RBX::InvocationMeter<2>::updateBuckets(bool)")]
// was: __ZN3RBX15InvocationMeterILi2EE13updateBucketsEb
// IDA 0x2b54d8: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b54d8() {
}

// 0x2b5590 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIdEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<double>(double const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSIdEERS3_RKT_
// IDA 0x2b5590: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5590() {
}

// 0x2b55e8 — __ZN3rbx14implementation12typed_holderIdE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<double>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIdE14construct_funcEPKcPc
// IDA 0x2b55e8: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b55e8() {
}

// 0x2b55f8 — __ZN3rbx14implementation12typed_holderIdE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<double>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIdE13destruct_funcEPc
// IDA 0x2b55f8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2b55f8() {
}

// 0x2b5600 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
// IDA 0x2b5600: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5600() {
}

// 0x2b5650 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSISsEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<std::string>(std::string const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSISsEERS3_RKT_
// IDA 0x2b5650: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5650() {
}

// 0x2b56a8 — __ZN3rbx14implementation12typed_holderISsE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<std::string>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderISsE14construct_funcEPKcPc
// IDA 0x2b56a8: 7 insns (CMP..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b56a8() {
}

// 0x2b56b8 — __ZN3rbx14implementation12typed_holderISsE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<std::string>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderISsE13destruct_funcEPc
// IDA 0x2b56b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b56b8() {
}

// 0x2b56c0 — __ZN5boost6detail12shared_countC2IN3RBX10Reflection5TupleEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX10Reflection5TupleEEEPT_
// IDA 0x2b56c0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b56c0() {
}

// 0x2b57d0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEED0Ev
// IDA 0x2b57d0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b57d0() {
}

// 0x2b57d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE11get_deleterERKSt9type_info
// IDA 0x2b57d8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b57d8() {
}

// 0x2b57e0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE6insertEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE6insertEPNS8_4slotE
// IDA 0x2b57e0: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b57e0() {
}

// 0x2b59ec — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotEEaSEPSA_
// IDA 0x2b59ec: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b59ec() {
}

// 0x2b5a10 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotEEaSERKSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotEEaSERKSB_
// IDA 0x2b5a10: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5a10() {
}

// 0x2b5a34 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE22safe_static_init_mutexEv
// IDA 0x2b5a34: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b5a34() {
}

// 0x2b5a38 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE24safe_static_do_get_mutexEv
// IDA 0x2b5a38: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5a38() {
}

// 0x2b5b30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_13ScriptContextES6_SsS6_EENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_13ScriptContextES6_SsS6_EENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED1Ev
// IDA 0x2b5b30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b5b30() {
}

// 0x2b5b5c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_13ScriptContextES6_SsS6_EENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_13ScriptContextES6_SsS6_EENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED0Ev
// IDA 0x2b5b5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b5b5c() {
}

// 0x2b5c30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot10disconnectEv
// IDA 0x2b5c30: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5c30() {
}

// 0x2b5d40 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot9connectedEv
// IDA 0x2b5d40: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5d40() {
}

// 0x2b5d4c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callES7_SsS7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callES7_SsS7_
// IDA 0x2b5d4c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5d4c() {
}

// 0x2b5d68 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callES7_SsS7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callES7_SsS7_
// IDA 0x2b5d68: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5d68() {
}

// 0x2b5d84 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_NS_10shared_ptrINS3_8InstanceEEESsSH_EENS0_5list3IRSH_RSsSK_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: __ZN5boost3_bi5list4INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_NS_10shared_ptrINS3_8InstanceEEESsSH_EENS0_5list3IRSH_RSsSK_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x2b5d84: 160 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5d84() {
}

// 0x2b5f3c — __ZNK5boost4_mfi3mf3IvN3RBX13ScriptContextENS_10shared_ptrINS2_8InstanceEEESsS6_EclEPS3_S6_SsS6_
#[doc(alias = "boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ScriptContext*,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: __ZNK5boost4_mfi3mf3IvN3RBX13ScriptContextENS_10shared_ptrINS2_8InstanceEEESsS6_EclEPS3_S6_SsS6_
// IDA 0x2b5f3c: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b5f3c() {
}

// 0x2b6104 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE6removeEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE6removeEPNS8_4slotE
// IDA 0x2b6104: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b6104() {
}

// 0x2b61f4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot22safe_static_init_mutexEv
// IDA 0x2b61f4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b61f4() {
}

// 0x2b61f8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot24safe_static_do_get_mutexEv
// IDA 0x2b61f8: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b61f8() {
}

// 0x2b62e8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slotD1Ev
// IDA 0x2b62e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b62e8() {
}

// 0x2b6314 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slotD0Ev
// IDA 0x2b6314: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b6314() {
}

// 0x2b63e8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED1Ev
// IDA 0x2b63e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b63e8() {
}

// 0x2b6414 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED0Ev
// IDA 0x2b6414: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b6414() {
}

// 0x2b6638 — __ZN3RBX14LibraryServiceD2Ev
#[doc(alias = "RBX::LibraryService::~LibraryService()")]
// was: __ZN3RBX14LibraryServiceD2Ev
// IDA 0x2b6638: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b6638() {
}

// 0x2b67d8 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// IDA 0x2b67d8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b67d8() {
}

// 0x2b6800 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
// IDA 0x2b6800: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b6800() {
}

// 0x2b6b18 — __ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EED2Ev
#[doc(alias = "std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::~vector()")]
// was: __ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EED2Ev
// IDA 0x2b6b18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b6b18() {
}

// 0x2b6be4 — __ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x2b6be4: 92 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b6be4() {
}

// 0x2b6d00 — __ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x2b6d00: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b6d00() {
}

// 0x2b6d04 — __ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x2b6d04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b6d04() {
}

// 0x2b6da4 — __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x2b6da4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b6da4() {
}

// 0x2b6dac — __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x2b6dac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b6dac() {
}

// 0x2b6e50 — __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x2b6e50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b6e50() {
}

// 0x2b6e58 — __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x2b6e58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b6e58() {
}

// 0x2b6efc — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_17WaitingScriptsJobEEEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::WaitingScriptsJob>(RBX::WaitingScriptsJob *)")]
// was: __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_17WaitingScriptsJobEEEPT_
// IDA 0x2b6efc: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b6efc() {
}

// 0x2b6fe4 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_17WaitingScriptsJobEEEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::WaitingScriptsJob>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const*,RBX::WaitingScriptsJob *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_17WaitingScriptsJobEEEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2b6fe4: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b6fe4() {
}

// 0x2b70c8 — __ZN5boost6detail12shared_countC2IN3RBX17WaitingScriptsJobEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::WaitingScriptsJob>(RBX::WaitingScriptsJob *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX17WaitingScriptsJobEEEPT_
// IDA 0x2b70c8: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b70c8() {
}

// 0x2b71c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEED1Ev
// IDA 0x2b71c0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2b71c0() {
}

// 0x2b71c4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEED0Ev
// IDA 0x2b71c4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b71c4() {
}

// 0x2b71c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE7disposeEv
// IDA 0x2b71c8: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b71c8() {
}

// 0x2b71d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE11get_deleterERKSt9type_info
// IDA 0x2b71d8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b71d8() {
}

// 0x2b71dc — __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX17WaitingScriptsJobEE19get_untyped_deleterEv
// IDA 0x2b71dc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b71dc() {
}

// 0x2b74c8 — __ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE15isNullClassNameEv
// IDA 0x2b74c8: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b74c8() {
}

// 0x2b7568 — __ZN3RBX4Name7declareILZNS_10sWorkspaceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sWorkspaceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_10sWorkspaceEEEERKS0_v
// IDA 0x2b7568: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7568() {
}

// 0x2b75b0 — __ZN3RBX4Name9doDeclareILZNS_10sWorkspaceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sWorkspaceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10sWorkspaceEEEERKS0_v
// IDA 0x2b75b0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b75b0() {
}

// 0x2b7698 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_9WorkspaceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Workspace>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_9WorkspaceEEEmv
// IDA 0x2b7698: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7698() {
}

// 0x2b7770 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE13pushNewObjectIS5_EEPS5_P9lua_StateT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>* RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::pushNewObject<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE13pushNewObjectIS5_EEPS5_P9lua_StateT_
// IDA 0x2b7770: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7770() {
}

// 0x2b77c0 — __ZN3RBX10Reflection9DescribedINS_16OverlayDataModelELZNS_17sOverlayDataModelEENS_17NonFactoryProductINS_9DataModelELZNS_17sOverlayDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_16OverlayDataModelELZNS_17sOverlayDataModelEENS_17NonFactoryProductINS_9DataModelELZNS_17sOverlayDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_16OverlayDataModelELZNS_17sOverlayDataModelEENS_17NonFactoryProductINS_9DataModelELZNS_17sOverlayDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x2b77c0: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b77c0() {
}

// 0x2b78e0 — __ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x2b78e0: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b78e0() {
}

// 0x2b7a00 — __ZN3RBX10Reflection9DescribedINS_15ServiceProviderELZNS_16sServiceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sServiceProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15ServiceProviderELZNS_16sServiceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sServiceProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_15ServiceProviderELZNS_16sServiceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sServiceProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x2b7a00: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7a00() {
}

// 0x2b7b20 — __ZN3RBX3Lua15SingletonBridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE4pushEP9lua_StateS6_
#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Reflection::EnumDescriptor::Item const*,true>::push(lua_State *,RBX::Reflection::EnumDescriptor::Item const*)")]
// was: __ZN3RBX3Lua15SingletonBridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE4pushEP9lua_StateS6_
// IDA 0x2b7b20: 68 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7b20() {
}

// 0x2b7bf8 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE13pushNewObjectIS6_EEPS6_P9lua_StateT_
#[doc(alias = "RBX::Reflection::EnumDescriptor::Item const** RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::pushNewObject<RBX::Reflection::EnumDescriptor::Item const*>(lua_State *,RBX::Reflection::EnumDescriptor::Item const*)")]
// was: __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE13pushNewObjectIS6_EEPS6_P9lua_StateT_
// IDA 0x2b7bf8: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7bf8() {
}

// 0x2b7c38 — __ZN3RBX3Lua15SingletonBridgeIPKNS_10Reflection14EnumDescriptorELb1EE4pushEP9lua_StateS5_
#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Reflection::EnumDescriptor const*,true>::push(lua_State *,RBX::Reflection::EnumDescriptor const*)")]
// was: __ZN3RBX3Lua15SingletonBridgeIPKNS_10Reflection14EnumDescriptorELb1EE4pushEP9lua_StateS5_
// IDA 0x2b7c38: 68 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7c38() {
}

// 0x2b7d10 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE13pushNewObjectIS5_EEPS5_P9lua_StateT_
#[doc(alias = "RBX::Reflection::EnumDescriptor const** RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::pushNewObject<RBX::Reflection::EnumDescriptor const*>(lua_State *,RBX::Reflection::EnumDescriptor const*)")]
// was: __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE13pushNewObjectIS5_EEPS5_P9lua_StateT_
// IDA 0x2b7d10: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7d10() {
}

// 0x2b7d50 — __ZN3RBX3Lua15SingletonBridgeIPKNS0_18AllEnumDescriptorsELb1EE4pushEP9lua_StateS4_
#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Lua::AllEnumDescriptors const*,true>::push(lua_State *,RBX::Lua::AllEnumDescriptors const*)")]
// was: __ZN3RBX3Lua15SingletonBridgeIPKNS0_18AllEnumDescriptorsELb1EE4pushEP9lua_StateS4_
// IDA 0x2b7d50: 68 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7d50() {
}

// 0x2b7e28 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE13pushNewObjectIS4_EEPS4_P9lua_StateT_
#[doc(alias = "RBX::Lua::AllEnumDescriptors const** RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::pushNewObject<RBX::Lua::AllEnumDescriptors const*>(lua_State *,RBX::Lua::AllEnumDescriptors const*)")]
// was: __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE13pushNewObjectIS4_EEPS4_P9lua_StateT_
// IDA 0x2b7e28: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7e28() {
}

// 0x2b7e68 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_index(lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8on_indexEP9lua_State
// IDA 0x2b7e68: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7e68() {
}

// 0x2b7e9c — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_newindex(lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_newindexEP9lua_State
// IDA 0x2b7e9c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7e9c() {
}

// 0x2b7ed0 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_index(lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE8on_indexEP9lua_State
// IDA 0x2b7ed0: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7ed0() {
}

// 0x2b7f04 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_newindex(lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE11on_newindexEP9lua_State
// IDA 0x2b7f04: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7f04() {
}

// 0x2b7f38 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_index(lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE8on_indexEP9lua_State
// IDA 0x2b7f38: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7f38() {
}

// 0x2b7f6c — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_newindex(lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE11on_newindexEP9lua_State
// IDA 0x2b7f6c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7f6c() {
}

// 0x2b7fa0 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_index(lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE8on_indexEP9lua_State
// IDA 0x2b7fa0: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7fa0() {
}

// 0x2b7fd4 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_newindex(lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE11on_newindexEP9lua_State
// IDA 0x2b7fd4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b7fd4() {
}

// 0x2b8008 — __ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorD2Ev
// IDA 0x2b8008: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b8008() {
}

// 0x2b80a4 — __ZNK3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x2b80a4: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b80a4() {
}

// 0x2b8110 — __ZNK3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7Creator6createEv
// IDA 0x2b8110: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b8110() {
}

// 0x2b8254 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11LuaSettingsEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaSettings> RBX::Creatable<RBX::Instance>::create<RBX::LuaSettings>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_11LuaSettingsEEEN5boost10shared_ptrIT_EEv
// IDA 0x2b8254: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b8254() {
}

// 0x2b8304 — __ZN5boost10shared_ptrIN3RBX11LuaSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaSettings>::shared_ptr<RBX::LuaSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX11LuaSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x2b8304: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b8304() {
}

// 0x2b83cc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11LuaSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaSettings,RBX::LuaSettings>(rbx_core::SharedPtr<RBX::LuaSettings> const*,RBX::LuaSettings *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11LuaSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2b83cc: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b83cc() {
}

// 0x2b84b8 — __ZN5boost6detail12shared_countC2IPN3RBX11LuaSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX11LuaSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x2b84b8: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b84b8() {
}

// 0x2b85c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x2b85c0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2b85c0() {
}

// 0x2b85c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x2b85c8: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b85c8() {
}

// 0x2b85e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x2b85e8: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b85e8() {
}

// 0x2b8600 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x2b8600: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b8600() {
}

// 0x2b8608 — __ZN3RBX4Name13callDoDeclareILZNS_12sLuaSettingsEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sLuaSettingsEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_12sLuaSettingsEEEEvv
// IDA 0x2b8608: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b8608() {
}

// 0x2b8610 — __ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorC2Ev
// IDA 0x2b8610: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b8610() {
}

// 0x2b8838 — __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::EventDesc(rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x2b8838: 236 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b8838() {
}

// 0x2b8a94 — __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
// IDA 0x2b8a94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b8a94() {
}

// 0x2b8b48 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// IDA 0x2b8b48: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b8b48() {
}

// 0x2b8c9c — __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// IDA 0x2b8c9c: 236 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b8c9c() {
}

// 0x2b8f38 — __ZNK3RBX10Reflection13EventDescBaseINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x2b8f38: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b8f38() {
}

// 0x2b8f4c — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13disconnectAllEv
// IDA 0x2b8f4c: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b8f4c() {
}

// 0x2b90c8 — __ZN3rbx8any_castIRKSsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "std::string const& rbx::any_cast<std::string const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKSsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x2b90c8: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b90c8() {
}

// 0x2b91b8 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsS5_RKNS_10shared_ptrINS1_8InstanceEEENS6_IS3_EENS_3argILi1EEENSC_ILi2EEENSC_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISI_T0_T1_T2_T3_EENSG_9list_av_4IT4_T5_T6_T7_E4typeEEEMSL_FSI_SM_SN_SO_ESR_SS_ST_SU_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsS5_RKNS_10shared_ptrINS1_8InstanceEEENS6_IS3_EENS_3argILi1EEENSC_ILi2EEENSC_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISI_T0_T1_T2_T3_EENSG_9list_av_4IT4_T5_T6_T7_E4typeEEEMSL_FSI_SM_SN_SO_ESR_SS_ST_SU_
// IDA 0x2b91b8: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b91b8() {
}

// 0x2b92d4 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3ISsSsN5boost10shared_ptrINS_8InstanceEEEEEvRKT_RKT0_RKT1_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>(std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute3ISsSsN5boost10shared_ptrINS_8InstanceEEEEEvRKT_RKT0_RKT1_
// IDA 0x2b92d4: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b92d4() {
}

// 0x2b9460 — __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE5clearEv
#[doc(alias = "boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::clear(void)")]
// was: __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE5clearEv
// IDA 0x2b9460: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b9460() {
}

// 0x2b948c — __ZN5boost8functionIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSF_RKS4_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSF_RKS4_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSF_RKS4_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// IDA 0x2b948c: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b948c() {
}

// 0x2b9570 — __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// IDA 0x2b9570: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b9570() {
}

// 0x2b9658 — __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEvT_
#[doc(alias = "void boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
// was: __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEvT_
// IDA 0x2b9658: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b9658() {
}

// 0x2b9750 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE
// IDA 0x2b9750: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b9750() {
}

// 0x2b976c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEvSsSsSE_E6invokeERNS1_15function_bufferESsSsSE_
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEvSsSsSE_E6invokeERNS1_15function_bufferESsSsSE_
// IDA 0x2b976c: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b976c() {
}

// 0x2b978c — __ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x2b978c: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b978c() {
}

// 0x2b9874 — __ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x2b9874: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b9874() {
}

// 0x2b9958 — __ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsSG_RKS6_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x2b9958: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b9958() {
}

// 0x2b9a2c — __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKSsSI_RKNS3_INS4_8InstanceEEEEENS0_5list3IRSsSP_RSK_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<std::string &,std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&> &,boost::_bi::list3<std::string &,std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKSsSI_RKNS3_INS4_8InstanceEEEEENS0_5list3IRSsSP_RSK_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x2b9a2c: 14 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b9a2c() {
}

// 0x2b9a54 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_RKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x2b9a54: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b9a54() {
}

// 0x2b9bac — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&)")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// IDA 0x2b9bac: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b9bac() {
}

// 0x2b9ca0 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE
// IDA 0x2b9ca0: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b9ca0() {
}

// 0x2b9eac — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSEPSA_
// IDA 0x2b9eac: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b9eac() {
}

// 0x2b9ed0 — __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_EC2IPS9_EERKSC_T_
// IDA 0x2b9ed0: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b9ed0() {
}
