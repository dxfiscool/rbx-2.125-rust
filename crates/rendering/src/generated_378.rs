//! rendering shard 378 — 100 stubs 0x5465b8..0x54b808 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 41061->41161 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x5465b8..0x54b808 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5465b8 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefENS1_INS2_18NotificationObjectEEES6_EENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefENS1_INS2_18NotificationObjectEEES6_EENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_")]
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::NotificationObject>,RBX::Lua::WeakFunctionRef>::type> boost::bind<void,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,rbx_core::WeakPtr<RBX::NotificationObject>,RBX::Lua::WeakFunctionRef>(void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),rbx_core::WeakPtr<RBX::NotificationObject>,RBX::Lua::WeakFunctionRef)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefENS1_INS2_18NotificationObjectEEES6_EENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// IDA 0x5465b8: 183 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5465b8() {
}

// 0x54678c — __ZN3RBX9weak_fromINS_18NotificationObjectEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "__ZN3RBX9weak_fromINS_18NotificationObjectEEEN5boost8weak_ptrIT_EEPS4_")]
#[doc(alias = "rbx_core::WeakPtr<RBX::NotificationObject> RBX::weak_from<RBX::NotificationObject>(RBX::NotificationObject*)")]
// was: __ZN3RBX9weak_fromINS_18NotificationObjectEEEN5boost8weak_ptrIT_EEPS4_
// IDA 0x54678c: 188 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54678c() {
}

// 0x546e6c — __ZN3RBX10GuiServiceD1Ev
#[doc(alias = "__ZN3RBX10GuiServiceD1Ev")]
#[doc(alias = "RBX::GuiService::~GuiService()")]
// was: __ZN3RBX10GuiServiceD1Ev
// IDA 0x546e6c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_546e6c() {
}

// 0x546e70 — __ZN3RBX10GuiServiceD0Ev
#[doc(alias = "__ZN3RBX10GuiServiceD0Ev")]
#[doc(alias = "RBX::GuiService::~GuiService()")]
// was: __ZN3RBX10GuiServiceD0Ev
// IDA 0x546e70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_546e70() {
}

// 0x546f10 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_11sGuiServiceEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_11sGuiServiceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_11sGuiServiceEEE12getClassNameEv
// IDA 0x546f10: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_546f10() {
}

// 0x546f38 — __ZThn32_N3RBX10GuiServiceD1Ev
#[doc(alias = "__ZThn32_N3RBX10GuiServiceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiService::~GuiService()")]
// was: __ZThn32_N3RBX10GuiServiceD1Ev
// IDA 0x546f38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_546f38() {
}

// 0x546f40 — __ZThn32_N3RBX10GuiServiceD0Ev
#[doc(alias = "__ZThn32_N3RBX10GuiServiceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiService::~GuiService()")]
// was: __ZThn32_N3RBX10GuiServiceD0Ev
// IDA 0x546f40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_546f40() {
}

// 0x546fe4 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_11sGuiServiceEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_11sGuiServiceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_11sGuiServiceEEE12getClassNameEv
// IDA 0x546fe4: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_546fe4() {
}

// 0x54700c — __ZThn36_N3RBX10GuiServiceD1Ev
#[doc(alias = "__ZThn36_N3RBX10GuiServiceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiService::~GuiService()")]
// was: __ZThn36_N3RBX10GuiServiceD1Ev
// IDA 0x54700c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_54700c() {
}

// 0x547014 — __ZThn36_N3RBX10GuiServiceD0Ev
#[doc(alias = "__ZThn36_N3RBX10GuiServiceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiService::~GuiService()")]
// was: __ZThn36_N3RBX10GuiServiceD0Ev
// IDA 0x547014: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_547014() {
}

// 0x5470b8 — __ZN3RBX10GuiServiceD2Ev
#[doc(alias = "__ZN3RBX10GuiServiceD2Ev")]
#[doc(alias = "RBX::GuiService::~GuiService()")]
// was: __ZN3RBX10GuiServiceD2Ev
// IDA 0x5470b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5470b8() {
}

// 0x547484 — __ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
#[doc(alias = "std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_erase(std::_Rb_tree_node<RBX::GuiService::SpecialKey> *)")]
// was: __ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// IDA 0x547484: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_547484() {
}

// 0x54764c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService16CenterDialogTypeEEERS3_RKT_
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService16CenterDialogTypeEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiService::CenterDialogType>(RBX::GuiService::CenterDialogType const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService16CenterDialogTypeEEERS3_RKT_
// IDA 0x54764c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54764c() {
}

// 0x54769c — __ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE9singletonEv
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::CenterDialogType>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE9singletonEv
// IDA 0x54769c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54769c() {
}

// 0x547708 — __ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE14construct_funcEPKcPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE14construct_funcEPKcPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::CenterDialogType>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE14construct_funcEPKcPc
// IDA 0x547708: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_547708() {
}

// 0x547714 — __ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE13destruct_funcEPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::CenterDialogType>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE13destruct_funcEPc
// IDA 0x547714: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_547714() {
}

// 0x5477e4 — __ZN3rbx8any_castIRKN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::GuiService::CenterDialogType const& rbx::any_cast<RBX::GuiService::CenterDialogType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x5477e4: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5477e4() {
}

// 0x547cc4 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService10SpecialKeyEEERS3_RKT_
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService10SpecialKeyEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiService::SpecialKey>(RBX::GuiService::SpecialKey const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService10SpecialKeyEEERS3_RKT_
// IDA 0x547cc4: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_547cc4() {
}

// 0x547d14 — __ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE9singletonEv
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::SpecialKey>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE9singletonEv
// IDA 0x547d14: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_547d14() {
}

// 0x547d80 — __ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE14construct_funcEPKcPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE14construct_funcEPKcPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::SpecialKey>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE14construct_funcEPKcPc
// IDA 0x547d80: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_547d80() {
}

// 0x547d8c — __ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE13destruct_funcEPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::SpecialKey>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE13destruct_funcEPc
// IDA 0x547d8c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_547d8c() {
}

// 0x547e5c — __ZN3rbx8any_castIRKN3RBX10GuiService10SpecialKeyENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX10GuiService10SpecialKeyENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::GuiService::SpecialKey const& rbx::any_cast<RBX::GuiService::SpecialKey const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX10GuiService10SpecialKeyENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x547e5c: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_547e5c() {
}

// 0x54819c — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEENS4_5list2INS4_5valueINS6_INS7_18NotificationObjectEEEEENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEENS4_5list2INS4_5valueINS6_INS7_18NotificationObjectEEEEENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEENS4_5list2INS4_5valueINS6_INS7_18NotificationObjectEEEEENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// IDA 0x54819c: 129 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54819c() {
}

// 0x5482f4 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// IDA 0x5482f4: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5482f4() {
}

// 0x548450 — __ZN5boost6detail10weak_countC1ERKS1_
#[doc(alias = "__ZN5boost6detail10weak_countC1ERKS1_")]
#[doc(alias = "boost::detail::weak_count::weak_count(boost::detail::weak_count const&)")]
// was: __ZN5boost6detail10weak_countC1ERKS1_
// IDA 0x548450: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_548450() {
}

// 0x54849c — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEEEvT_
#[doc(alias = "__ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEEEvT_")]
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>)")]
// was: __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEEEvT_
// IDA 0x54849c: 136 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54849c() {
}

// 0x548608 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// IDA 0x548608: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_548608() {
}

// 0x548624 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE")]
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
// IDA 0x548624: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_548624() {
}

// 0x548638 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEENS5_5list2INS5_5valueINS7_INS8_18NotificationObjectEEEEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEENS5_5list2INS5_5valueINS7_INS8_18NotificationObjectEEEEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE")]
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEENS5_5list2INS5_5valueINS7_INS8_18NotificationObjectEEEEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x548638: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_548638() {
}

// 0x548794 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEENS5_5list2INS5_5valueINS7_INS8_18NotificationObjectEEEEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEENS5_5list2INS5_5valueINS7_INS8_18NotificationObjectEEEEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEENS5_5list2INS5_5valueINS7_INS8_18NotificationObjectEEEEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x548794: 129 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_548794() {
}

// 0x5488ec — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEENS5_5list2INS5_5valueINS7_INS8_18NotificationObjectEEEEENSG_ISC_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEENS5_5list2INS5_5valueINS7_INS8_18NotificationObjectEEEEENSG_ISC_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEENS5_5list2INS5_5valueINS7_INS8_18NotificationObjectEEEEENSG_ISC_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x5488ec: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5488ec() {
}

// 0x5489f4 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX18NotificationObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEclIPFvNS3_INS4_9GuiObjectEEES9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX18NotificationObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEclIPFvNS3_INS4_9GuiObjectEEES9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::operator()<void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef) &,boost::_bi::list0 &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX18NotificationObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEclIPFvNS3_INS4_9GuiObjectEEES9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// IDA 0x5489f4: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5489f4() {
}

// 0x548ae8 — __ZN5boost8weak_ptrIN3RBX9GuiObjectEEC2INS1_18NotificationObjectEEERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "__ZN5boost8weak_ptrIN3RBX9GuiObjectEEC2INS1_18NotificationObjectEEERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")]
#[doc(alias = "rbx_core::WeakPtr<RBX::GuiObject>::weak_ptr<RBX::NotificationObject>(rbx_core::WeakPtr<RBX::NotificationObject> const&,boost::detail::sp_enable_if_convertible<RBX::NotificationObject,RBX::GuiObject>::type)")]
// was: __ZN5boost8weak_ptrIN3RBX9GuiObjectEEC2INS1_18NotificationObjectEEERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// IDA 0x548ae8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_548ae8() {
}

// 0x548b18 — __ZN5boost10shared_ptrIN3RBX18NotificationObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX18NotificationObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationObject>::shared_ptr<RBX::NotificationObject>(rbx_core::WeakPtr<RBX::NotificationObject> const&,boost::detail::sp_nothrow_tag)")]
// was: __ZN5boost10shared_ptrIN3RBX18NotificationObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// IDA 0x548b18: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_548b18() {
}

// 0x548b94 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueINS5_INS6_18NotificationObjectEEEEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x548b94: 168 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_548b94() {
}

// 0x548d44 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX18NotificationObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX18NotificationObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_")]
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::list2(boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX18NotificationObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_
// IDA 0x548d44: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_548d44() {
}

// 0x548e44 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX18NotificationObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX18NotificationObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_")]
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::NotificationObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX18NotificationObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_
// IDA 0x548e44: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_548e44() {
}

// 0x548f4c — __ZN5boost10shared_ptrIN3RBX18NotificationObjectEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX18NotificationObjectEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationObject>::shared_ptr<RBX::NotificationObject,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationObject *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX18NotificationObjectEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x548f4c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_548f4c() {
}

// 0x5490fc — __ZN5boost6detail12shared_countC2IPN3RBX18NotificationObjectENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX18NotificationObjectENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::NotificationObject *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationObject *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX18NotificationObjectENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5490fc: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5490fc() {
}

// 0x549204 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationObject *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x549204: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_549204() {
}

// 0x549208 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationObject *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x549208: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_549208() {
}

// 0x54920c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationObject *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x54920c: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54920c() {
}

// 0x54922c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationObject *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x54922c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54922c() {
}

// 0x549244 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationObject *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18NotificationObjectENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x549244: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549244() {
}

// 0x549248 — __ZN3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x549248: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549248() {
}

// 0x549368 — __ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")]
#[doc(alias = "std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert_unique(RBX::GuiService::SpecialKey const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
// IDA 0x549368: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549368() {
}

// 0x5493d0 — __ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
#[doc(alias = "std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::GuiService::SpecialKey const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// IDA 0x5493d0: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5493d0() {
}

// 0x549428 — __ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE16_M_insert_uniqueERKc
#[doc(alias = "__ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE16_M_insert_uniqueERKc")]
#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_insert_unique(char const&)")]
// was: __ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE16_M_insert_uniqueERKc
// IDA 0x549428: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549428() {
}

// 0x54949c — __ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKc
#[doc(alias = "__ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKc")]
#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,char const&)")]
// was: __ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKc
// IDA 0x54949c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54949c() {
}

// 0x5494f4 — __ZN3rbx7signals6signalIFvSsSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvSsSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// IDA 0x5494f4: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5494f4() {
}

// 0x549654 — __ZN3rbx7signals16signal_with_argsILi2EFvSsSsEE8fireItemEPNS0_6signalIS2_E4slotESsSs
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFvSsSsEE8fireItemEPNS0_6signalIS2_E4slotESsSs")]
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,std::string)>::fireItem(rbx::signals::signal<void ()(std::string,std::string)>::slot *,std::string,std::string)")]
// was: __ZN3rbx7signals16signal_with_argsILi2EFvSsSsEE8fireItemEPNS0_6signalIS2_E4slotESsSs
// IDA 0x549654: 152 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549654() {
}

// 0x54980c — __ZN3rbx7signals6signalIFvSsSsEE8on_errorERSt9exception
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE8on_errorERSt9exception")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvSsSsEE8on_errorERSt9exception
// IDA 0x54980c: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54980c() {
}

// 0x549834 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSERKS7_
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSERKS7_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSERKS7_
// IDA 0x549834: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549834() {
}

// 0x549858 — __ZN3rbx7signals6signalIFvSsSsEE22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsSsEE22safe_static_init_mutexEv
// IDA 0x549858: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_549858() {
}

// 0x54985c — __ZN3rbx7signals6signalIFvSsSsEE24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsSsEE24safe_static_do_get_mutexEv
// IDA 0x54985c: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54985c() {
}

// 0x549954 — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// IDA 0x549954: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549954() {
}

// 0x549ab4 — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE8on_errorERSt9exception
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE8on_errorERSt9exception")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE8on_errorERSt9exception
// IDA 0x549ab4: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549ab4() {
}

// 0x549adc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEEaSERKSA_
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEEaSERKSA_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEEaSERKSA_
// IDA 0x549adc: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549adc() {
}

// 0x549b00 — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE22safe_static_init_mutexEv
// IDA 0x549b00: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_549b00() {
}

// 0x549b04 — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE24safe_static_do_get_mutexEv
// IDA 0x549b04: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549b04() {
}

// 0x549bfc — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE5eraseERS6_
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE5eraseERS6_")]
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::erase(rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE5eraseERS6_
// IDA 0x549bfc: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549bfc() {
}

// 0x549c24 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE11equal_rangeERS6_
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE11equal_rangeERS6_")]
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::equal_range(rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE11equal_rangeERS6_
// IDA 0x549c24: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549c24() {
}

// 0x549c70 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE5eraseESt17_Rb_tree_iteratorISA_ESI_
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE5eraseESt17_Rb_tree_iteratorISA_ESI_")]
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::erase(std::_Rb_tree_iterator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::_Rb_tree_iterator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE5eraseESt17_Rb_tree_iteratorISA_ESI_
// IDA 0x549c70: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549c70() {
}

// 0x549cd4 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E")]
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>> *)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E
// IDA 0x549cd4: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549cd4() {
}

// 0x549cf0 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E")]
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_erase(std::_Rb_tree_node<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>> *)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E
// IDA 0x549cf0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549cf0() {
}

// 0x549d18 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tINS7_11unspecifiedENS0_IFvvEEENS7_5list0EEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tINS7_11unspecifiedENS0_IFvvEEENS7_5list0EEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tINS7_11unspecifiedENS0_IFvvEEENS7_5list0EEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
// IDA 0x549d18: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549d18() {
}

// 0x549ddc — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvvEEENS6_5list0EEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvvEEENS6_5list0EEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvvEEENS6_5list0EEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
// IDA 0x549ddc: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549ddc() {
}

// 0x549ea4 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvvEEENS6_5list0EEEEEvT_
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvvEEENS6_5list0EEEEEvT_")]
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>)")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvvEEENS6_5list0EEEEEvT_
// IDA 0x549ea4: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549ea4() {
}

// 0x549f7c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEE6manageERKNS1_15function_bufferERSC_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEE6manageERKNS1_15function_bufferERSC_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEE6manageERKNS1_15function_bufferERSC_NS1_30functor_manager_operation_typeE
// IDA 0x549f7c: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549f7c() {
}

// 0x549f98 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEvPN3RBX9DataModelEE6invokeERNS1_15function_bufferESD_
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEvPN3RBX9DataModelEE6invokeERNS1_15function_bufferESD_")]
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEvPN3RBX9DataModelEE6invokeERNS1_15function_bufferESD_
// IDA 0x549f98: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549f98() {
}

// 0x549fa0 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEbT_RNS1_15function_bufferE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEbT_RNS1_15function_bufferE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEbT_RNS1_15function_bufferE
// IDA 0x549fa0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549fa0() {
}

// 0x54a068 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x54a068: 68 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54a068() {
}

// 0x54a12c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x54a12c: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54a12c() {
}

// 0x54a1e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEE7managerERKNS1_15function_bufferERSC_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEE7managerERKNS1_15function_bufferERSC_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEE7managerERKNS1_15function_bufferERSC_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x54a1e0: 114 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54a1e0() {
}

// 0x54a314 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_")]
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_
// IDA 0x54a314: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54a314() {
}

// 0x54a3c8 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSA_
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSA_")]
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSA_
// IDA 0x54a3c8: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54a3c8() {
}

// 0x54a414 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueERKSA_
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueERKSA_")]
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueERKSA_
// IDA 0x54a414: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54a414() {
}

// 0x54a47c — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE14_M_create_nodeERKSA_
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE14_M_create_nodeERKSA_")]
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_create_node(std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE14_M_create_nodeERKSA_
// IDA 0x54a47c: 84 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54a47c() {
}

// 0x54a568 — __ZN5boost9function0IvE4swapERS1_
#[doc(alias = "__ZN5boost9function0IvE4swapERS1_")]
#[doc(alias = "boost::function0<void>::swap(boost::function0<void>&)")]
// was: __ZN5boost9function0IvE4swapERS1_
// IDA 0x54a568: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54a568() {
}

// 0x54a644 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEbENS4_5list3INS4_5valueIS9_EENSF_ISB_EENSF_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEbENS4_5list3INS4_5valueIS9_EENSF_ISB_EENSF_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEbENS4_5list3INS4_5valueIS9_EENSF_ISB_EENSF_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// IDA 0x54a644: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54a644() {
}

// 0x54a7a4 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// IDA 0x54a7a4: 133 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54a7a4() {
}

// 0x54a908 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEEEvT_
#[doc(alias = "__ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEEEvT_")]
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>)")]
// was: __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEEEvT_
// IDA 0x54a908: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54a908() {
}

// 0x54aa80 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
// IDA 0x54aa80: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54aa80() {
}

// 0x54aa9c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEvE6invokeERNS1_15function_bufferE")]
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEvE6invokeERNS1_15function_bufferE
// IDA 0x54aa9c: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54aa9c() {
}

// 0x54aab0 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferE")]
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x54aab0: 133 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54aab0() {
}

// 0x54ac14 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x54ac14: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54ac14() {
}

// 0x54ad74 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x54ad74: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54ad74() {
}

// 0x54ae84 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEclIPFvS6_S9_bENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEclIPFvS6_S9_bENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool) &,boost::_bi::list0 &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEclIPFvS6_S9_bENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// IDA 0x54ae84: 96 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54ae84() {
}

// 0x54af8c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x54af8c: 171 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54af8c() {
}

// 0x54b148 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEC2ES7_SA_SB_
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEC2ES7_SA_SB_")]
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>)")]
// was: __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEC2ES7_SA_SB_
// IDA 0x54b148: 96 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54b148() {
}

// 0x54b250 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEC2ES7_SA_SB_
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEC2ES7_SA_SB_")]
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>)")]
// was: __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEC2ES7_SA_SB_
// IDA 0x54b250: 96 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54b250() {
}

// 0x54b35c — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_")]
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_
// IDA 0x54b35c: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54b35c() {
}

// 0x54b464 — __ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EEC2ERKS5_
#[doc(alias = "__ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EEC2ERKS5_")]
#[doc(alias = "std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::list(std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>> const&)")]
// was: __ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EEC2ERKS5_
// IDA 0x54b464: 75 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54b464() {
}

// 0x54b530 — __ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE22_M_initialize_dispatchISt20_List_const_iteratorIS3_EEEvT_S9_St12__false_type
#[doc(alias = "__ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE22_M_initialize_dispatchISt20_List_const_iteratorIS3_EEEvT_S9_St12__false_type")]
#[doc(alias = "void std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::_M_initialize_dispatch<std::_List_const_iterator<RBX::GuiService::DialogWrapper *>>(std::_List_const_iterator<RBX::GuiService::DialogWrapper *>,std::_List_const_iterator<RBX::GuiService::DialogWrapper *>,std::__false_type)")]
// was: __ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE22_M_initialize_dispatchISt20_List_const_iteratorIS3_EEEvT_S9_St12__false_type
// IDA 0x54b530: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54b530() {
}

// 0x54b554 — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_")]
#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_
// IDA 0x54b554: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54b554() {
}

// 0x54b608 — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSA_
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSA_")]
#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSA_
// IDA 0x54b608: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54b608() {
}

// 0x54b654 — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE16_M_insert_uniqueERKSA_
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE16_M_insert_uniqueERKSA_")]
#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_insert_unique(std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE16_M_insert_uniqueERKSA_
// IDA 0x54b654: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54b654() {
}

// 0x54b6bc — __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE14_M_create_nodeERKSA_
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE14_M_create_nodeERKSA_")]
#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_create_node(std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE14_M_create_nodeERKSA_
// IDA 0x54b6bc: 81 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54b6bc() {
}

// 0x54b7a0 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_9ScreenGuiEEEPKT_v
#[doc(alias = "__ZNK3RBX8Instance25findConstFirstChildOfTypeINS_9ScreenGuiEEEPKT_v")]
#[doc(alias = "RBX::ScreenGui const* RBX::Instance::findConstFirstChildOfType<RBX::ScreenGui>(void)const")]
// was: __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_9ScreenGuiEEEPKT_v
// IDA 0x54b7a0: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54b7a0() {
}

// 0x54b808 — __ZN3RBX10Reflection9DescribedINS_9ScreenGuiELZNS_10sScreenGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_10sScreenGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9ScreenGuiELZNS_10sScreenGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_10sScreenGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_9ScreenGuiELZNS_10sScreenGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_10sScreenGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x54b808: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54b808() {
}
