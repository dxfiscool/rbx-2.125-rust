//! rendering shard rend_c_01 — 120 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 34901->35021 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in any crate (filler after 0x4946dc, lowest remaining 0x4946dc..0x53d4a4)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 120 after 0x4946dc (distinct not in any 83061 -> 83181)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4946dc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12DialogChoiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DialogChoice,RBX::DialogChoice>(boost::shared_ptr<RBX::DialogChoice> const*,RBX::DialogChoice *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12DialogChoiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12DialogChoiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x4946dc: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4946dc() {
}

// 0x497348 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10DialogRootES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DialogRoot,RBX::DialogRoot>(boost::shared_ptr<RBX::DialogRoot> const*,RBX::DialogRoot *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10DialogRootES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10DialogRootES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x497348: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_497348() {
}

// 0x4a11c4 — __ZN3RBX11shared_fromINS_9ExplosionEEEN5boost10shared_ptrIT_EEPS4_
// demangled: boost::shared_ptr<RBX::Explosion> RBX::shared_from<RBX::Explosion>(RBX::Explosion*)
// type: int(void)
#[doc(alias = "__ZN3RBX11shared_fromINS_9ExplosionEEEN5boost10shared_ptrIT_EEPS4_")]
// was: __ZN3RBX11shared_fromINS_9ExplosionEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x4a11c4: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a11c4() {
}

// 0x4a2114 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// IDA 0x4a2114: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2114() {
}

// 0x4a2ae8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TimerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TimerService,RBX::TimerService>(boost::shared_ptr<RBX::TimerService> const*,RBX::TimerService *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TimerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TimerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x4a2ae8: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2ae8() {
}

// 0x4ab510 — __ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_
// type: unknown
#[doc(alias = "__ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_")]
// was: __ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_
// IDA 0x4ab510: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ab510() {
}

// 0x4abb28 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE22safe_static_init_mutexEv
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::safe_static_init_mutex(void)
// type: unknown
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE22safe_static_init_mutexEv")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE22safe_static_init_mutexEv
// IDA 0x4abb28: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4abb28() {
}

// 0x4abb2c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::safe_static_do_get_mutex(void)
// type: unknown
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE24safe_static_do_get_mutexEv")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE24safe_static_do_get_mutexEv
// IDA 0x4abb2c: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4abb2c() {
}

// 0x4ac008 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13BindableEventES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BindableEvent,RBX::BindableEvent>(boost::shared_ptr<RBX::BindableEvent> const*,RBX::BindableEvent *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13BindableEventES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13BindableEventES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x4ac008: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ac008() {
}

// 0x4ad850 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16BindableFunctionES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BindableFunction,RBX::BindableFunction>(boost::shared_ptr<RBX::BindableFunction> const*,RBX::BindableFunction *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16BindableFunctionES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16BindableFunctionES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x4ad850: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ad850() {
}

// 0x4ae19c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9AnimationES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Animation,RBX::Animation>(boost::shared_ptr<RBX::Animation> const*,RBX::Animation *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9AnimationES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9AnimationES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x4ae19c: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ae19c() {
}

// 0x4af228 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8SparklesES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Sparkles,RBX::Sparkles>(boost::shared_ptr<RBX::Sparkles> const*,RBX::Sparkles *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8SparklesES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8SparklesES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x4af228: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4af228() {
}

// 0x4b0264 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ForceFieldES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ForceField,RBX::ForceField>(boost::shared_ptr<RBX::ForceField> const*,RBX::ForceField *)const
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ForceFieldES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ForceFieldES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x4b0264: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b0264() {
}

// 0x4b1a4c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11CustomEventES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CustomEvent,RBX::CustomEvent>(boost::shared_ptr<RBX::CustomEvent> const*,RBX::CustomEvent *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11CustomEventES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11CustomEventES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x4b1a4c: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b1a4c() {
}

// 0x4b3730 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CustomEventReceiverES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CustomEventReceiver,RBX::CustomEventReceiver>(boost::shared_ptr<RBX::CustomEventReceiver> const*,RBX::CustomEventReceiver *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CustomEventReceiverES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CustomEventReceiverES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x4b3730: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b3730() {
}

// 0x4fc774 — __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEED1Ev
// demangled: boost::shared_ptr<RBX::ProfanityFilter>::~shared_ptr()
// type: unknown
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX15ProfanityFilterEED1Ev")]
// was: __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEED1Ev
// IDA 0x4fc774: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4fc774() {
}

// 0x4fc788 — __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEaSERKS3_
// demangled: boost::shared_ptr<RBX::ProfanityFilter>::operator=(boost::shared_ptr<RBX::ProfanityFilter> const&)
// type: unknown
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEaSERKS3_")]
// was: __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEaSERKS3_
// IDA 0x4fc788: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fc788() {
}

// 0x4fcd04 — __ZN5boost10shared_ptrIN3RBX11CommonVerbsEE5resetIS2_EEvPT_
// demangled: void boost::shared_ptr<RBX::CommonVerbs>::reset<RBX::CommonVerbs>(RBX::CommonVerbs *)
// type: unknown
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11CommonVerbsEE5resetIS2_EEvPT_")]
// was: __ZN5boost10shared_ptrIN3RBX11CommonVerbsEE5resetIS2_EEvPT_
// IDA 0x4fcd04: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fcd04() {
}

// 0x4fcdcc — __ZN5boost4bindIvN3RBX4GameERKSsPS2_SsEENS_3_bi6bind_tIT_NS_4_mfi3mf1IS8_T0_T1_EENS6_9list_av_2IT2_T3_E4typeEEEMSB_FS8_SC_ESF_SG_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list_av_2<RBX::Game*,std::string>::type> boost::bind<void,RBX::Game,std::string const&,RBX::Game*,std::string>(void (RBX::Game::*)(std::string const&),RBX::Game*,std::string)
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "__ZN5boost4bindIvN3RBX4GameERKSsPS2_SsEENS_3_bi6bind_tIT_NS_4_mfi3mf1IS8_T0_T1_EENS6_9list_av_2IT2_T3_E4typeEEEMSB_FS8_SC_ESF_SG_")]
// was: __ZN5boost4bindIvN3RBX4GameERKSsPS2_SsEENS_3_bi6bind_tIT_NS_4_mfi3mf1IS8_T0_T1_EENS6_9list_av_2IT2_T3_E4typeEEEMSB_FS8_SC_ESF_SG_
// IDA 0x4fcdcc: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fcdcc() {
}

// 0x4fd5d4 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS7_5list2INS7_5valueIPSB_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: unknown
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS7_5list2INS7_5valueIPSB_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS7_5list2INS7_5valueIPSB_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// IDA 0x4fd5d4: 106 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fd5d4() {
}

// 0x4fd70c — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS6_5list2INS6_5valueIPSA_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: unknown
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS6_5list2INS6_5valueIPSA_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS6_5list2INS6_5valueIPSA_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// IDA 0x4fd70c: 108 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fd70c() {
}

// 0x4fd994 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS3_5list2INS3_5valueIPS8_EENSD_ISsEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: unknown
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS3_5list2INS3_5valueIPS8_EENSD_ISsEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS3_5list2INS3_5valueIPS8_EENSD_ISsEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// IDA 0x4fd994: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fd994() {
}

// 0x4fdd20 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS3_5list2INS3_5valueIPS8_EENSD_ISsEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: unknown
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS3_5list2INS3_5valueIPS8_EENSD_ISsEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS3_5list2INS3_5valueIPS8_EENSD_ISsEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x4fdd20: 112 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fdd20() {
}

// 0x4fde5c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4GameEEENS2_ISsEEEC2ES6_S7_
// demangled: boost::_bi::list2<boost::_bi::value<RBX::Game *>,boost::_bi::value<std::string>>::list2(boost::_bi::value<RBX::Game *>,boost::_bi::value<std::string>)
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX4GameEEENS2_ISsEEEC2ES6_S7_")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX4GameEEENS2_ISsEEEC2ES6_S7_
// IDA 0x4fde5c: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fde5c() {
}

// 0x4fe078 — __ZN5boost10shared_ptrIN3RBX11CommonVerbsEEC2IS2_EEPT_
// demangled: boost::shared_ptr<RBX::CommonVerbs>::shared_ptr<RBX::CommonVerbs>(RBX::CommonVerbs *)
// type: unknown
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11CommonVerbsEEC2IS2_EEPT_")]
// was: __ZN5boost10shared_ptrIN3RBX11CommonVerbsEEC2IS2_EEPT_
// IDA 0x4fe078: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fe078() {
}

// 0x4fe14c — __ZN5boost6detail12shared_countC2IN3RBX11CommonVerbsEEEPT_
// demangled: boost::detail::shared_count::shared_count<RBX::CommonVerbs>(RBX::CommonVerbs *)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX11CommonVerbsEEEPT_")]
// was: __ZN5boost6detail12shared_countC2IN3RBX11CommonVerbsEEEPT_
// IDA 0x4fe14c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fe14c() {
}

// 0x4fec88 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEED1Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::~sp_counted_impl_p()
// type: unknown
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEED1Ev")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEED1Ev
// IDA 0x4fec88: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4fec88() {
}

// 0x4fec8c — __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEED0Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::~sp_counted_impl_p()
// type: unknown
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEED0Ev")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEED0Ev
// IDA 0x4fec8c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4fec8c() {
}

// 0x4fec90 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEE7disposeEv
// demangled: boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::dispose(void)
// type: unknown
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEE7disposeEv")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEE7disposeEv
// IDA 0x4fec90: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fec90() {
}

// 0x4fed34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::get_deleter(std::type_info const&)
// type: unknown
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEE11get_deleterERKSt9type_info")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEE11get_deleterERKSt9type_info
// IDA 0x4fed34: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fed34() {
}

// 0x4fed38 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::get_untyped_deleter(void)
// type: unknown
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEE19get_untyped_deleterEv")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEE19get_untyped_deleterEv
// IDA 0x4fed38: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fed38() {
}

// 0x4ff388 — __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEC2IS2_EEPT_
// demangled: boost::shared_ptr<RBX::ProfanityFilter>::shared_ptr<RBX::ProfanityFilter>(RBX::ProfanityFilter *)
// type: unknown
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEC2IS2_EEPT_")]
// was: __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEC2IS2_EEPT_
// IDA 0x4ff388: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ff388() {
}

// 0x4ff45c — __ZN5boost6detail12shared_countC2IN3RBX15ProfanityFilterEEEPT_
// demangled: boost::detail::shared_count::shared_count<RBX::ProfanityFilter>(RBX::ProfanityFilter *)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX15ProfanityFilterEEEPT_")]
// was: __ZN5boost6detail12shared_countC2IN3RBX15ProfanityFilterEEEPT_
// IDA 0x4ff45c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ff45c() {
}

// 0x4ff568 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEED1Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::~sp_counted_impl_p()
// type: unknown
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEED1Ev")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEED1Ev
// IDA 0x4ff568: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4ff568() {
}

// 0x4ff56c — __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEED0Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::~sp_counted_impl_p()
// type: unknown
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEED0Ev")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEED0Ev
// IDA 0x4ff56c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ff56c() {
}

// 0x4ff570 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::dispose(void)
// type: unknown
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE7disposeEv")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE7disposeEv
// IDA 0x4ff570: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ff570() {
}

// 0x4ff614 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::get_deleter(std::type_info const&)
// type: unknown
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE11get_deleterERKSt9type_info")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE11get_deleterERKSt9type_info
// IDA 0x4ff614: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ff614() {
}

// 0x4ff618 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::get_untyped_deleter(void)
// type: unknown
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE19get_untyped_deleterEv")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE19get_untyped_deleterEv
// IDA 0x4ff618: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ff618() {
}

// 0x4ff61c — __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// demangled: boost::shared_ptr<RBX::ProfanityFilter>::shared_ptr<RBX::ProfanityFilter>(boost::weak_ptr<RBX::ProfanityFilter> const&,boost::detail::sp_nothrow_tag)
// type: unknown
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
// was: __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// IDA 0x4ff61c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ff61c() {
}

// 0x4ff700 — __ZN5boost8weak_ptrIN3RBX15ProfanityFilterEED1Ev
// demangled: boost::weak_ptr<RBX::ProfanityFilter>::~weak_ptr()
// type: unknown
#[doc(alias = "__ZN5boost8weak_ptrIN3RBX15ProfanityFilterEED1Ev")]
// was: __ZN5boost8weak_ptrIN3RBX15ProfanityFilterEED1Ev
// IDA 0x4ff700: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4ff700() {
}

// 0x4ff818 — __ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// demangled: boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
// type: unknown
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// was: __ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// IDA 0x4ff818: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ff818() {
}

// 0x502760 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12GameSettingsEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// demangled: RBX::Reflection::EventDescImpl<1,RBX::GameSettings,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::GameSettings::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_12GameSettingsEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_12GameSettingsEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x502760: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_502760() {
}

// 0x506898 — __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
// demangled: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::delete_buckets(void)
// type: unknown
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
// IDA 0x506898: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_506898() {
}

// 0x5068e8 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
// demangled: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Primitive const*>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Primitive const*>>(RBX::Primitive const* const&,boost::unordered::detail::emplace_args1<RBX::Primitive const*> const&)
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
// IDA 0x5068e8: 148 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5068e8() {
}

// 0x506a78 — __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
// demangled: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::reserve_for_insert(unsigned long)
// type: unknown
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
// IDA 0x506a78: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_506a78() {
}

// 0x506ac8 — __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
// demangled: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::create_buckets(unsigned long)
// type: unknown
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
// IDA 0x506ac8: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_506ac8() {
}

// 0x506bf0 — __ZNK5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
// demangled: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::min_buckets_for_size(unsigned long)const
// type: unknown
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm")]
// was: __ZNK5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
// IDA 0x506bf0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_506bf0() {
}

// 0x506c80 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
// demangled: boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::rehash_impl(unsigned long)
// type: unknown
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
// IDA 0x506c80: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_506c80() {
}

// 0x506cac — __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE
// demangled: boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>> &,boost::unordered::detail::ptr_bucket *)
// type: unknown
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE
// IDA 0x506cac: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_506cac() {
}

// 0x506d00 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIPKN3RBX9PrimitiveEEEEE9constructEv
// demangled: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive const*>>>::construct(void)
// type: unknown
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIPKN3RBX9PrimitiveEEEEE9constructEv")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIPKN3RBX9PrimitiveEEEEE9constructEv
// IDA 0x506d00: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_506d00() {
}

// 0x506d38 — __ZNK5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_
// demangled: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Primitive const*>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::find_node_impl<RBX::Primitive const*,std::equal_to<RBX::Primitive const*>>(unsigned long,RBX::Primitive const* const&,std::equal_to<RBX::Primitive const*> const&)const
// type: unknown
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_")]
// was: __ZNK5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_
// IDA 0x506d38: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_506d38() {
}

// 0x508e58 — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc()
// type: unknown
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED1Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED1Ev
// IDA 0x508e58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_508e58() {
}

// 0x50a0e8 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEESH_EEPT_RKNS_10shared_ptrIT0_EE
// demangled: rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> const&)
// type: unknown
#[doc(alias = "__ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEESH_EEPT_RKNS_10shared_ptrIT0_EE")]
// was: __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEESH_EEPT_RKNS_10shared_ptrIT0_EE
// IDA 0x50a0e8: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50a0e8() {
}

// 0x50a148 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// demangled: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>,std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)
// type: unknown
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// IDA 0x50a148: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50a148() {
}

// 0x50a180 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// demangled: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>,std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)
// type: unknown
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// IDA 0x50a180: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50a180() {
}

// 0x50a250 — __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()
// type: unknown
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEED1Ev")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEED1Ev
// IDA 0x50a250: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50a250() {
}

// 0x50a280 — __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_deleter(std::type_info const&)
// type: unknown
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE11get_deleterERKSt9type_info")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE11get_deleterERKSt9type_info
// IDA 0x50a280: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50a280() {
}

// 0x50a298 — __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_untyped_deleter(void)
// type: unknown
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE19get_untyped_deleterEv")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE19get_untyped_deleterEv
// IDA 0x50a298: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50a298() {
}

// 0x50b750 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9SelectionES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Selection,RBX::Selection>(boost::shared_ptr<RBX::Selection> const*,RBX::Selection *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9SelectionES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9SelectionES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x50b750: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b750() {
}

// 0x50d538 — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EEC2EMS2_FSI_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: int __fastcall(int, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EEC2EMS2_FSI_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EEC2EMS2_FSI_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x50d538: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50d538() {
}

// 0x50d63c — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED0Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc()
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED0Ev
// IDA 0x50d63c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50d63c() {
}

// 0x50d6f0 — __ZNK3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x50d6f0: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50d6f0() {
}

// 0x50d714 — __ZN3RBX10Reflection11Call0HelperINS_22GlobalAdvancedSettingsEMS2_FN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvESI_E4callEPS2_SK_RS7_
// demangled: RBX::Reflection::Call0Helper<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::GlobalAdvancedSettings*,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),RBX::Reflection::Variant&)
// type: void __fastcall(int, char *, int, int **)
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_22GlobalAdvancedSettingsEMS2_FN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvESI_E4callEPS2_SK_RS7_")]
// was: __ZN3RBX10Reflection11Call0HelperINS_22GlobalAdvancedSettingsEMS2_FN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvESI_E4callEPS2_SK_RS7_
// IDA 0x50d714: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50d714() {
}

// 0x50def4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19GlobalBasicSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GlobalBasicSettings,RBX::GlobalBasicSettings>(boost::shared_ptr<RBX::GlobalBasicSettings> const*,RBX::GlobalBasicSettings *)const
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19GlobalBasicSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19GlobalBasicSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x50def4: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50def4() {
}

// 0x50e2a4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_22GlobalAdvancedSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GlobalAdvancedSettings,RBX::GlobalAdvancedSettings>(boost::shared_ptr<RBX::GlobalAdvancedSettings> const*,RBX::GlobalAdvancedSettings *)const
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_22GlobalAdvancedSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_22GlobalAdvancedSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x50e2a4: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50e2a4() {
}

// 0x50edb8 — __ZN3RBX11shared_fromINS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPS5_
// demangled: boost::shared_ptr<RBX::Reflection::DescribedBase> RBX::shared_from<RBX::Reflection::DescribedBase>(RBX::Reflection::DescribedBase*)
// type: unknown
#[doc(alias = "__ZN3RBX11shared_fromINS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPS5_")]
// was: __ZN3RBX11shared_fromINS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPS5_
// IDA 0x50edb8: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50edb8() {
}

// 0x521964 — __ZN3RBX24shared_from_dynamic_castINS_11TextDisplayENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS4_23enable_shared_from_thisIT0_EE
// demangled: boost::shared_ptr<RBX::TextDisplay> RBX::shared_from_dynamic_cast<RBX::TextDisplay,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)
// type: unknown
#[doc(alias = "__ZN3RBX24shared_from_dynamic_castINS_11TextDisplayENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS4_23enable_shared_from_thisIT0_EE")]
// was: __ZN3RBX24shared_from_dynamic_castINS_11TextDisplayENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS4_23enable_shared_from_thisIT0_EE
// IDA 0x521964: 119 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_521964() {
}

// 0x521af0 — __ZN5boost20dynamic_pointer_castIN3RBX11TextDisplayENS1_10Reflection13DescribedBaseEEENS_10shared_ptrIT_EERKNS5_IT0_EE
// demangled: boost::shared_ptr<RBX::TextDisplay> boost::dynamic_pointer_cast<RBX::TextDisplay,RBX::Reflection::DescribedBase>(boost::shared_ptr<RBX::Reflection::DescribedBase> const&)
// type: unknown
#[doc(alias = "__ZN5boost20dynamic_pointer_castIN3RBX11TextDisplayENS1_10Reflection13DescribedBaseEEENS_10shared_ptrIT_EERKNS5_IT0_EE")]
// was: __ZN5boost20dynamic_pointer_castIN3RBX11TextDisplayENS1_10Reflection13DescribedBaseEEENS_10shared_ptrIT_EERKNS5_IT0_EE
// IDA 0x521af0: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_521af0() {
}

// 0x521c00 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15EquationDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::EquationDisplay,RBX::EquationDisplay>(boost::shared_ptr<RBX::EquationDisplay> const*,RBX::EquationDisplay *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15EquationDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15EquationDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x521c00: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_521c00() {
}

// 0x521efc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11TextDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TextDisplay,RBX::TextDisplay>(boost::shared_ptr<RBX::TextDisplay> const*,RBX::TextDisplay *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11TextDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11TextDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x521efc: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_521efc() {
}

// 0x5230b8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatButton,RBX::ChatButton>(boost::shared_ptr<RBX::ChatButton> const*,RBX::ChatButton *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5230b8: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5230b8() {
}

// 0x5233b4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatWidgetES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatWidget,RBX::ChatWidget>(boost::shared_ptr<RBX::ChatWidget> const*,RBX::ChatWidget *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatWidgetES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatWidgetES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5233b4: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5233b4() {
}

// 0x5236b0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatOutputES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatOutput,RBX::ChatOutput>(boost::shared_ptr<RBX::ChatOutput> const*,RBX::ChatOutput *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatOutputES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatOutputES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5236b0: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5236b0() {
}

// 0x523d94 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13RelativePanelES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RelativePanel,RBX::RelativePanel>(boost::shared_ptr<RBX::RelativePanel> const*,RBX::RelativePanel *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13RelativePanelES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13RelativePanelES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x523d94: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_523d94() {
}

// 0x524588 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14GuiImageButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiImageButton,RBX::GuiImageButton>(boost::shared_ptr<RBX::GuiImageButton> const*,RBX::GuiImageButton *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14GuiImageButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14GuiImageButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x524588: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524588() {
}

// 0x524884 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15NotificationBoxES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::NotificationBox,RBX::NotificationBox>(boost::shared_ptr<RBX::NotificationBox> const*,RBX::NotificationBox *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15NotificationBoxES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15NotificationBoxES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x524884: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524884() {
}

// 0x524b80 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5FrameES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Frame,RBX::Frame>(boost::shared_ptr<RBX::Frame> const*,RBX::Frame *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5FrameES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5FrameES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x524b80: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_524b80() {
}

// 0x525178 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15PhysicsSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PhysicsSettings,RBX::PhysicsSettings>(boost::shared_ptr<RBX::PhysicsSettings> const*,RBX::PhysicsSettings *)const
// type: unknown
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15PhysicsSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15PhysicsSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x525178: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_525178() {
}

// 0x53a4d0 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_5UDim2ENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISD_T0_T1_EENSB_9list_av_2IT2_T3_E4typeEEEMSG_FSD_SH_ESK_SL_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::UDim2 const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_5UDim2ENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISD_T0_T1_EENSB_9list_av_2IT2_T3_E4typeEEEMSG_FSD_SH_ESK_SL_")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_5UDim2ENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISD_T0_T1_EENSB_9list_av_2IT2_T3_E4typeEEEMSG_FSD_SH_ESK_SL_
// IDA 0x53a4d0: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a4d0() {
}

// 0x53a75c — __ZN5boost8functionIFvN3RBX5UDim2EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvN3RBX5UDim2EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3RBX5UDim2EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// IDA 0x53a75c: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a75c() {
}

// 0x53a840 — __ZN5boost9function1IvN3RBX5UDim2EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvN3RBX5UDim2EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvN3RBX5UDim2EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// IDA 0x53a840: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a840() {
}

// 0x53a928 — __ZN5boost9function1IvN3RBX5UDim2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEEvT_
// demangled: void boost::function1<void,RBX::UDim2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvN3RBX5UDim2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEEvT_")]
// was: __ZN5boost9function1IvN3RBX5UDim2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEEvT_
// IDA 0x53a928: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53a928() {
}

// 0x53aa20 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_5UDim2EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: unknown
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_5UDim2EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_5UDim2EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// IDA 0x53aa20: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53aa20() {
}

// 0x53aa3c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_5UDim2EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::UDim2>::invoke(boost::detail::function::function_buffer &,RBX::UDim2)
// type: unknown
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_5UDim2EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_5UDim2EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
// IDA 0x53aa3c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53aa3c() {
}

// 0x53aa58 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX5UDim2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable1<void,RBX::UDim2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX5UDim2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
// was: __ZNK5boost6detail8function13basic_vtable1IvN3RBX5UDim2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x53aa58: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53aa58() {
}

// 0x53ab40 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX5UDim2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable1<void,RBX::UDim2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX5UDim2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: __ZNK5boost6detail8function13basic_vtable1IvN3RBX5UDim2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x53ab40: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ab40() {
}

// 0x53ac24 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX5UDim2EE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// demangled: void boost::detail::function::basic_vtable1<void,RBX::UDim2>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// type: unknown
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX5UDim2EE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: __ZNK5boost6detail8function13basic_vtable1IvN3RBX5UDim2EE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x53ac24: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ac24() {
}

// 0x53acf8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_5UDim2EEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS7_EEvRT_
// demangled: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::UDim2>(RBX::UDim2 &)
// type: unknown
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_5UDim2EEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS7_EEvRT_")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_5UDim2EEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS7_EEvRT_
// IDA 0x53acf8: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53acf8() {
}

// 0x53ad10 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_5UDim2EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::UDim2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_5UDim2EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_5UDim2EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x53ad10: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ad10() {
}

// 0x53cb4c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE12setEnumValueEPNS0_13DescribedBaseEi")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x53cb4c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cb4c() {
}

// 0x53cb98 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE11getEnumItemEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x53cb98: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cb98() {
}

// 0x53cbb8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x53cbb8: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cbb8() {
}

// 0x53cbec — __ZNK3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::GuiObject::SizeConstraint>::convertToIndex(RBX::GuiObject::SizeConstraint)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE14convertToIndexES3_")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE14convertToIndexES3_
// IDA 0x53cbec: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cbec() {
}

// 0x53cc5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE11setIntValueEPNS0_13DescribedBaseEi")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x53cc5c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cc5c() {
}

// 0x53cc9c — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::GetSetImpl<RBX::GuiObject::SizeConstraint (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::GuiObject::SizeConstraint)>::isReadOnly(void)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x53cc9c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cc9c() {
}

// 0x53cca0 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::GetSetImpl<RBX::GuiObject::SizeConstraint (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::GuiObject::SizeConstraint)>::isWriteOnly(void)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x53cca0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cca0() {
}

// 0x53cca4 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::GetSetImpl<RBX::GuiObject::SizeConstraint (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::GuiObject::SizeConstraint)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x53cca4: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cca4() {
}

// 0x53ccc4 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::GetSetImpl<RBX::GuiObject::SizeConstraint (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::GuiObject::SizeConstraint)>::setValue(RBX::Reflection::DescribedBase *,RBX::GuiObject::SizeConstraint const&)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x53ccc4: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ccc4() {
}

// 0x53cce8 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEiEC2IMNS_9GuiBase2dEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,int>::PropDescriptor<int (RBX::GuiBase2d::*)(void)const,void (RBX::GuiObject::*)(int)>(char const*,char const*,int (RBX::GuiBase2d::*)(void)const,void (RBX::GuiObject::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEiEC2IMNS_9GuiBase2dEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEiEC2IMNS_9GuiBase2dEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x53cce8: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cce8() {
}

// 0x53cdfc — __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEiED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,int>::~PropDescriptor()
// type: unknown
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEiED0Ev")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEiED0Ev
// IDA 0x53cdfc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53cdfc() {
}

// 0x53ce28 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMNS_9GuiBase2dEKFivEMS2_FviEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,int>::GetSetImpl<int (RBX::GuiBase2d::*)(void)const,void (RBX::GuiObject::*)(int)>::isReadOnly(void)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMNS_9GuiBase2dEKFivEMS2_FviEE10isReadOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMNS_9GuiBase2dEKFivEMS2_FviEE10isReadOnlyEv
// IDA 0x53ce28: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ce28() {
}

// 0x53ce2c — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMNS_9GuiBase2dEKFivEMS2_FviEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,int>::GetSetImpl<int (RBX::GuiBase2d::*)(void)const,void (RBX::GuiObject::*)(int)>::isWriteOnly(void)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMNS_9GuiBase2dEKFivEMS2_FviEE11isWriteOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMNS_9GuiBase2dEKFivEMS2_FviEE11isWriteOnlyEv
// IDA 0x53ce2c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ce2c() {
}

// 0x53ce30 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMNS_9GuiBase2dEKFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,int>::GetSetImpl<int (RBX::GuiBase2d::*)(void)const,void (RBX::GuiObject::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMNS_9GuiBase2dEKFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMNS_9GuiBase2dEKFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x53ce30: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ce30() {
}

// 0x53ce50 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMNS_9GuiBase2dEKFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,int>::GetSetImpl<int (RBX::GuiBase2d::*)(void)const,void (RBX::GuiObject::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMNS_9GuiBase2dEKFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMNS_9GuiBase2dEKFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x53ce50: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ce50() {
}

// 0x53ce74 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,int>::PropDescriptor<int (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(int)>(char const*,char const*,int (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x53ce74: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ce74() {
}

// 0x53cf88 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,int>::GetSetImpl<int (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(int)>::isReadOnly(void)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
// IDA 0x53cf88: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cf88() {
}

// 0x53cf8c — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,int>::GetSetImpl<int (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(int)>::isWriteOnly(void)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// IDA 0x53cf8c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cf8c() {
}

// 0x53cf90 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,int>::GetSetImpl<int (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x53cf90: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cf90() {
}

// 0x53cfb0 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,int>::GetSetImpl<int (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x53cfb0: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cfb0() {
}

// 0x53cfd4 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::UDim2>::PropDescriptor<RBX::UDim2 (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::UDim2)>(char const*,char const*,RBX::UDim2 (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::UDim2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x53cfd4: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cfd4() {
}

// 0x53d0e8 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: unknown
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x53d0e8: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53d0e8() {
}

// 0x53d20c — __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::UDim2>::~PropDescriptor()
// type: unknown
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EED0Ev")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EED0Ev
// IDA 0x53d20c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53d20c() {
}

// 0x53d238 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE10isReadOnlyEv
// demangled: RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::isReadOnly(void)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE10isReadOnlyEv")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE10isReadOnlyEv
// IDA 0x53d238: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53d238() {
}

// 0x53d248 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE11isWriteOnlyEv
// demangled: RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::isWriteOnly(void)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE11isWriteOnlyEv")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE11isWriteOnlyEv
// IDA 0x53d248: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53d248() {
}

// 0x53d258 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE11equalValuesEPKNS0_13DescribedBaseES6_
// demangled: RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE11equalValuesEPKNS0_13DescribedBaseES6_")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE11equalValuesEPKNS0_13DescribedBaseES6_
// IDA 0x53d258: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53d258() {
}

// 0x53d2c4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x53d2c4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53d2c4() {
}

// 0x53d2f0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x53d2f0: 132 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53d2f0() {
}

// 0x53d458 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE9copyValueEPKNS0_13DescribedBaseEPS4_
// demangled: RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: unknown
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE9copyValueEPKNS0_13DescribedBaseEPS4_")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EE9copyValueEPKNS0_13DescribedBaseEPS4_
// IDA 0x53d458: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53d458() {
}

// 0x53d480 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EED1Ev
// demangled: RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::~TypedPropertyDescriptor()
// type: unknown
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EED1Ev")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EED1Ev
// IDA 0x53d480: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53d480() {
}

// 0x53d4a4 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EED0Ev
// demangled: RBX::Reflection::TypedPropertyDescriptor<RBX::UDim2>::~TypedPropertyDescriptor()
// type: unknown
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EED0Ev")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorINS_5UDim2EED0Ev
// IDA 0x53d4a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53d4a4() {
}
