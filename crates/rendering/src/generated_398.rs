//! rendering shard 398 — 100 stubs 0x5ea7c0..0x5ef5dc EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 43010->43110 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x5ea7c0..0x5ef5dc (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5ea7c0 — __ZN3rbx8any_castIRKN5boost10shared_ptrIN3RBX8InstanceEEENS3_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN3rbx8any_castIRKN5boost10shared_ptrIN3RBX8InstanceEEENS3_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> const& rbx::any_cast<rbx_core::SharedPtr<RBX::Instance> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN5boost10shared_ptrIN3RBX8InstanceEEENS3_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x5ea7c0: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ea7c0() {
}


// 0x5ea8b0 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEENS4_IS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEENS4_IS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEENS4_IS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
// IDA 0x5ea8b0: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ea8b0() {
}


// 0x5ea9d0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE5clearEv
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE5clearEv")]
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::clear(void)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE5clearEv
// IDA 0x5ea9d0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ea9d0() {
}


// 0x5eaa00 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE9singletonEv
// type: int(void)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Instance>>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE9singletonEv
// IDA 0x5eaa00: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eaa00() {
}


// 0x5eaa70 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE13destruct_funcEPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Instance>>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE13destruct_funcEPc
// IDA 0x5eaa70: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eaa70() {
}


// 0x5eaa80 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS1_ISC_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS1_ISC_EEEENS_3argILi1EEEEEEEEEvT_")]
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS1_ISC_EEEENS_3argILi1EEEEEEEEEvT_
// IDA 0x5eaa80: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eaa80() {
}


// 0x5eab78 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINSA_IS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINSA_IS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINSA_IS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
// IDA 0x5eab78: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eab78() {
}


// 0x5eab94 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_10Reflection18GenericSlotWrapperERKS6_EENS9_5list2INS9_5valueINS3_ISE_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_10Reflection18GenericSlotWrapperERKS6_EENS9_5list2INS9_5valueINS3_ISE_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_10Reflection18GenericSlotWrapperERKS6_EENS9_5list2INS9_5valueINS3_ISE_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x5eab94: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eab94() {
}


// 0x5eac7c — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEEEN3rbx7signals10connectionET_
#[doc(alias = "__ZN3RBX12PartInstance13TouchedSignal7connectIN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEEEN3rbx7signals10connectionET_")]
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEEEN3rbx7signals10connectionET_
// IDA 0x5eac7c: 116 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eac7c() {
}


// 0x5eadc0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE13assign_to_ownERKS5_
// type: int(void)
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE13assign_to_ownERKS5_")]
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE13assign_to_ownERKS5_
// IDA 0x5eadc0: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eadc0() {
}


// 0x5eadf0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS4_12PartInstance13TouchedSignal11TouchedSlotEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS4_12PartInstance13TouchedSignal11TouchedSlotEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<RBX::PartInstance::TouchedSignal::TouchedSlot>(RBX::PartInstance::TouchedSignal::TouchedSlot const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS4_12PartInstance13TouchedSignal11TouchedSlotEEENS0_10connectionERKT_
// IDA 0x5eadf0: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eadf0() {
}


// 0x5eaee4 — __ZN3rbx7signals10connection9flogPrintEv
// type: void __fastcall(rbx::signals::connection *this)
#[doc(alias = "__ZN3rbx7signals10connection9flogPrintEv")]
#[doc(alias = "rbx::signals::connection::flogPrint(void)")]
// was: __ZN3rbx7signals10connection9flogPrintEv
// IDA 0x5eaee4: 85 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eaee4() {
}


// 0x5eafd0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE9flogPrintEv
// type: int __fastcall(int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE9flogPrintEv")]
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::flogPrint(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE9flogPrintEv
// IDA 0x5eafd0: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eafd0() {
}


// 0x5eb0bc — __ZN3RBX12PartInstance13TouchedSignal11TouchedSlotD2Ev
// type: void __fastcall(RBX::PartInstance::TouchedSignal::TouchedSlot *__hidden this)
#[doc(alias = "__ZN3RBX12PartInstance13TouchedSignal11TouchedSlotD2Ev")]
#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::~TouchedSlot()")]
// was: __ZN3RBX12PartInstance13TouchedSignal11TouchedSlotD2Ev
// IDA 0x5eb0bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5eb0bc() {
}


// 0x5eb1d0 — __ZN5boost10shared_ptrIN3RBX12PartInstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12PartInstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance>::shared_ptr<RBX::PartInstance>(rbx_core::WeakPtr<RBX::PartInstance> const&,boost::detail::sp_nothrow_tag)")]
// was: __ZN5boost10shared_ptrIN3RBX12PartInstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// IDA 0x5eb1d0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eb1d0() {
}


// 0x5eb24c — __ZN3RBX12PartInstance13TouchedSignal11TouchedSlotC2ERKN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEPS0_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX12PartInstance13TouchedSignal11TouchedSlotC2ERKN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEPS0_")]
#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::TouchedSlot(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&,RBX::PartInstance*)")]
// was: __ZN3RBX12PartInstance13TouchedSignal11TouchedSlotC2ERKN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEPS0_
// IDA 0x5eb24c: 102 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eb24c() {
}


// 0x5eb360 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE
// IDA 0x5eb360: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eb360() {
}


// 0x5eb570 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_EC2IPS9_EERKSD_T_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_EC2IPS9_EERKSD_T_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*>(RBX::PartInstance::TouchedSignal::TouchedSlot const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_EC2IPS9_EERKSD_T_
// IDA 0x5eb570: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eb570() {
}


// 0x5eb66c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS4_12PartInstance13TouchedSignal11TouchedSlotEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS4_12PartInstance13TouchedSignal11TouchedSlotEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<RBX::PartInstance::TouchedSignal::TouchedSlot>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS4_12PartInstance13TouchedSignal11TouchedSlotEED1Ev
// IDA 0x5eb66c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5eb66c() {
}


// 0x5eb77c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS4_12PartInstance13TouchedSignal11TouchedSlotEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS4_12PartInstance13TouchedSignal11TouchedSlotEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<RBX::PartInstance::TouchedSignal::TouchedSlot>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS4_12PartInstance13TouchedSignal11TouchedSlotEED0Ev
// IDA 0x5eb77c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5eb77c() {
}


// 0x5eb8b0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_E4callES7_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_E4callES7_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_E4callES7_
// IDA 0x5eb8b0: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eb8b0() {
}


// 0x5eb980 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_E4callES7_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_E4callES7_")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_E4callES7_
// IDA 0x5eb980: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eb980() {
}


// 0x5eb988 — __ZN3RBX12PartInstance13TouchedSignal11TouchedSlotclEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "__ZN3RBX12PartInstance13TouchedSignal11TouchedSlotclEN5boost10shared_ptrINS_8InstanceEEE")]
#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX12PartInstance13TouchedSignal11TouchedSlotclEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5eb988: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eb988() {
}


// 0x5eba58 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_ED1Ev
// IDA 0x5eba58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5eba58() {
}


// 0x5ebb68 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_ED0Ev
// IDA 0x5ebb68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ebb68() {
}


// 0x5ebc98 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD1Ev
// IDA 0x5ebc98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ebc98() {
}


// 0x5ebcc8 — __ZN3RBX12PartInstance13TouchedSignal11TouchedSlotC2ERKS2_
// type: _DWORD __fastcall(RBX::PartInstance::TouchedSignal::TouchedSlot *__hidden this, const RBX::PartInstance::TouchedSignal::TouchedSlot *)
#[doc(alias = "__ZN3RBX12PartInstance13TouchedSignal11TouchedSlotC2ERKS2_")]
#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::TouchedSlot(RBX::PartInstance::TouchedSignal::TouchedSlot const&)")]
// was: __ZN3RBX12PartInstance13TouchedSignal11TouchedSlotC2ERKS2_
// IDA 0x5ebcc8: 134 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ebcc8() {
}


// 0x5ebe34 — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x5ebe34: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ebe34() {
}


// 0x5ebfc4 — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEEC2ESD_PKcSG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEEC2ESD_PKcSG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEEC2ESD_PKcSG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x5ebfc4: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ebfc4() {
}


// 0x5ec154 — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEED0Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEED0Ev
// IDA 0x5ec154: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ec154() {
}


// 0x5ec208 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// IDA 0x5ec208: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ec208() {
}


// 0x5ec374 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISI_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISI_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISI_EE
// IDA 0x5ec374: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ec374() {
}


// 0x5ec4d0 — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE13disconnectAllEPNS0_11EventSourceE
// IDA 0x5ec4d0: 22 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ec4d0() {
}


// 0x5ec508 — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE9getSignalEPS2_
// type: int __fastcall(int, RBX::Instance *this)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE9getSignalEPS2_")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::getSignal(RBX::PartInstance*)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE9getSignalEPS2_
// IDA 0x5ec508: 38 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ec508() {
}


// 0x5ec580 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE8on_errorERSt9exception
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE8on_errorERSt9exception")]
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE8on_errorERSt9exception
// IDA 0x5ec580: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ec580() {
}


// 0x5ec5a8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// IDA 0x5ec5a8: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ec5a8() {
}


// 0x5ec6a0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
// IDA 0x5ec6a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ec6a0() {
}


// 0x5ec7b0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
// IDA 0x5ec7b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ec7b0() {
}


// 0x5ec8e0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// IDA 0x5ec8e0: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ec8e0() {
}


// 0x5ec9b0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// IDA 0x5ec9b0: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ec9b0() {
}


// 0x5ec9b8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev
// IDA 0x5ec9b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ec9b8() {
}


// 0x5ecac8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev
// IDA 0x5ecac8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ecac8() {
}


// 0x5ecbf8 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEEC2IMS2_KFKS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEEC2IMS2_KFKS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::SystemAddress>::PropDescriptor<RBX::SystemAddress const (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(RBX::SystemAddress)>(char const*,char const*,RBX::SystemAddress const (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(RBX::SystemAddress),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEEC2IMS2_KFKS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5ecbf8: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ecbf8() {
}


// 0x5ece30 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::SystemAddress>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEED0Ev
// IDA 0x5ece30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ece30() {
}


// 0x5ed068 — __ZN3rbx8any_castIRKN3RBX13SystemAddressENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX13SystemAddressENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::SystemAddress const& rbx::any_cast<RBX::SystemAddress const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX13SystemAddressENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x5ed068: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed068() {
}


// 0x5ed158 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SystemAddressEEERS3_RKT_
// type: int(void)
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SystemAddressEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SystemAddress>(RBX::SystemAddress const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SystemAddressEEERS3_RKT_
// IDA 0x5ed158: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed158() {
}


// 0x5ed1b0 — __ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE9singletonEv
// type: int(void)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::SystemAddress>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE9singletonEv
// IDA 0x5ed1b0: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed1b0() {
}


// 0x5ed21c — __ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE14construct_funcEPKcPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE14construct_funcEPKcPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::SystemAddress>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE14construct_funcEPKcPc
// IDA 0x5ed21c: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed21c() {
}


// 0x5ed22c — __ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE13destruct_funcEPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::SystemAddress>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE13destruct_funcEPc
// IDA 0x5ed22c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5ed22c() {
}


// 0x5ed280 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEE10GetSetImplIMS2_KFKS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEE10GetSetImplIMS2_KFKS3_vEMS2_FvS3_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::SystemAddress>::GetSetImpl<RBX::SystemAddress const (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(RBX::SystemAddress)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEE10GetSetImplIMS2_KFKS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x5ed280: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed280() {
}


// 0x5ed284 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEE10GetSetImplIMS2_KFKS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEE10GetSetImplIMS2_KFKS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::SystemAddress>::GetSetImpl<RBX::SystemAddress const (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(RBX::SystemAddress)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEE10GetSetImplIMS2_KFKS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x5ed284: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed284() {
}


// 0x5ed288 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEE10GetSetImplIMS2_KFKS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEE10GetSetImplIMS2_KFKS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::SystemAddress>::GetSetImpl<RBX::SystemAddress const (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(RBX::SystemAddress)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEE10GetSetImplIMS2_KFKS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5ed288: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed288() {
}


// 0x5ed2b0 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEE10GetSetImplIMS2_KFKS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERS6_
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEE10GetSetImplIMS2_KFKS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERS6_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::SystemAddress>::GetSetImpl<RBX::SystemAddress const (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(RBX::SystemAddress)>::setValue(RBX::Reflection::DescribedBase *,RBX::SystemAddress const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_13SystemAddressEE10GetSetImplIMS2_KFKS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERS6_
// IDA 0x5ed2b0: 20 insns (PUSH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed2b0() {
}


// 0x5ed2ec — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EEC2EMS2_FbS3_iEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EEC2EMS2_FbS3_iEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,bool ()(RBX::NormalId,int),2>::BoundFuncDesc(bool (RBX::PartInstance::*)(RBX::NormalId,int),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EEC2EMS2_FbS3_iEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x5ed2ec: 176 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed2ec() {
}


// 0x5ed4b4 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,bool ()(RBX::NormalId,int),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_
// IDA 0x5ed4b4: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed4b4() {
}


// 0x5ed500 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EED0Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,bool ()(RBX::NormalId,int),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EED0Ev
// IDA 0x5ed500: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ed500() {
}


// 0x5ed5e0 — __ZNK3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,bool ()(RBX::NormalId,int),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbNS_8NormalIdEiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x5ed5e0: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed5e0() {
}


// 0x5ed630 — __ZN3RBX10Reflection11Call2HelperINS_12PartInstanceEMS2_FbNS_8NormalIdEiES3_ibE4callEPS2_S5_RNS0_7VariantERKS3_RKi
#[doc(alias = "__ZN3RBX10Reflection11Call2HelperINS_12PartInstanceEMS2_FbNS_8NormalIdEiES3_ibE4callEPS2_S5_RNS0_7VariantERKS3_RKi")]
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::PartInstance,bool (RBX::PartInstance::*)(RBX::NormalId,int),RBX::NormalId,int,bool>::call(RBX::PartInstance*,bool (RBX::PartInstance::*)(RBX::NormalId,int),RBX::Reflection::Variant &,RBX::NormalId const&,int const&)")]
// was: __ZN3RBX10Reflection11Call2HelperINS_12PartInstanceEMS2_FbNS_8NormalIdEiES3_ibE4callEPS2_S5_RNS0_7VariantERKS3_RKi
// IDA 0x5ed630: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed630() {
}


// 0x5ed670 — __ZN3RBX10Reflection9ArgHelper6getArgINS_8NormalIdELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_8NormalIdELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
#[doc(alias = "RBX::NormalId RBX::Reflection::ArgHelper::getArg<RBX::NormalId,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::NormalId> const&,boost::disable_if<boost::is_same<RBX::NormalId,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_8NormalIdELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x5ed670: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed670() {
}


// 0x5ed800 — __ZN3RBX10Reflection9ArgHelper6getArgIiLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgIiLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
#[doc(alias = "int RBX::Reflection::ArgHelper::getArg<int,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<int> const&,boost::disable_if<boost::is_same<int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIiLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x5ed800: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed800() {
}


// 0x5ed998 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_8NormalIdEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINS9_7is_enumIS7_EEvE4typeE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_8NormalIdEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINS9_7is_enumIS7_EEvE4typeE")]
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::NormalId>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::NormalId &,boost::enable_if<boost::is_enum<RBX::NormalId>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_8NormalIdEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINS9_7is_enumIS7_EEvE4typeE
// IDA 0x5ed998: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed998() {
}


// 0x5eddb0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8NormalIdEEERS3_RKT_
// type: int(void)
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8NormalIdEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::NormalId>(RBX::NormalId const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8NormalIdEEERS3_RKT_
// IDA 0x5eddb0: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eddb0() {
}


// 0x5ede00 — __ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE9singletonEv
// type: int(void)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::NormalId>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE9singletonEv
// IDA 0x5ede00: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ede00() {
}


// 0x5ede70 — __ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE13destruct_funcEPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::NormalId>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE13destruct_funcEPc
// IDA 0x5ede70: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5ede70() {
}


// 0x5ede78 — __ZN3rbx8any_castIRKN3RBX8NormalIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX8NormalIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::NormalId const& rbx::any_cast<RBX::NormalId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX8NormalIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x5ede78: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ede78() {
}


// 0x5ee114 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,int>::PropDescriptor<int (RBX::PartInstance::*)(void)const,int>(char const*,char const*,int (RBX::PartInstance::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5ee114: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee114() {
}


// 0x5ee344 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEiED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEiED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEiED0Ev
// IDA 0x5ee344: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ee344() {
}


// 0x5ee590 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEiE7GetImplIMS2_KFivEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEiE7GetImplIMS2_KFivEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,int>::GetImpl<int (RBX::PartInstance::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEiE7GetImplIMS2_KFivEE10isReadOnlyEv
// IDA 0x5ee590: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee590() {
}


// 0x5ee594 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEiE7GetImplIMS2_KFivEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,int>::GetImpl<int (RBX::PartInstance::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
// IDA 0x5ee594: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee594() {
}


// 0x5ee598 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,int>::GetImpl<int (RBX::PartInstance::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5ee598: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee598() {
}


// 0x5ee5b8 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,int>::GetImpl<int (RBX::PartInstance::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x5ee5b8: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee5b8() {
}


// 0x5ee6d8 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::Faces>::PropDescriptor<RBX::Faces (RBX::PartInstance::*)(void)const,int>(char const*,char const*,RBX::Faces (RBX::PartInstance::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5ee6d8: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee6d8() {
}


// 0x5ee908 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::Faces>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEED0Ev
// IDA 0x5ee908: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ee908() {
}


// 0x5eeb20 — __ZN3rbx8any_castIRKN3RBX5FacesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX5FacesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::Faces const& rbx::any_cast<RBX::Faces const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX5FacesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x5eeb20: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eeb20() {
}


// 0x5eec10 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5FacesEEERS3_RKT_
// type: int **__fastcall(int **, int **)
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5FacesEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Faces>(RBX::Faces const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5FacesEEERS3_RKT_
// IDA 0x5eec10: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eec10() {
}


// 0x5eec60 — __ZN3rbx14implementation12typed_holderIN3RBX5FacesEE9singletonEv
// type: int(void)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX5FacesEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::Faces>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX5FacesEE9singletonEv
// IDA 0x5eec60: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eec60() {
}


// 0x5eecd0 — __ZN3rbx14implementation12typed_holderIN3RBX5FacesEE13destruct_funcEPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX5FacesEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::Faces>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX5FacesEE13destruct_funcEPc
// IDA 0x5eecd0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5eecd0() {
}


// 0x5eed24 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::Faces>::GetImpl<RBX::Faces (RBX::PartInstance::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
// IDA 0x5eed24: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eed24() {
}


// 0x5eed28 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::Faces>::GetImpl<RBX::Faces (RBX::PartInstance::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
// IDA 0x5eed28: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eed28() {
}


// 0x5eed2c — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::Faces>::GetImpl<RBX::Faces (RBX::PartInstance::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5eed2c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eed2c() {
}


// 0x5eed4c — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,RBX::Faces>::GetImpl<RBX::Faces (RBX::PartInstance::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Faces const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceENS_5FacesEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x5eed4c: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eed4c() {
}


// 0x5eee6c — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,bool>::PropDescriptor<bool (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(bool)>(char const*,char const*,bool (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5eee6c: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eee6c() {
}


// 0x5ef0a4 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEbED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEbED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEbED0Ev
// IDA 0x5ef0a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ef0a4() {
}


// 0x5ef144 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,bool>::GetSetImpl<bool (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// IDA 0x5ef144: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef144() {
}


// 0x5ef148 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,bool>::GetSetImpl<bool (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// IDA 0x5ef148: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef148() {
}


// 0x5ef14c — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,bool>::GetSetImpl<bool (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5ef14c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef14c() {
}


// 0x5ef170 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,bool>::GetSetImpl<bool (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x5ef170: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef170() {
}


// 0x5ef194 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::EnumPropDescriptor<RBX::Material (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(RBX::Material)>(char const*,char const*,RBX::Material (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(RBX::Material),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5ef194: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef194() {
}


// 0x5ef348 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEED0Ev
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEED0Ev
// IDA 0x5ef348: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ef348() {
}


// 0x5ef374 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE10isReadOnlyEv
// IDA 0x5ef374: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef374() {
}


// 0x5ef384 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE11isWriteOnlyEv
// IDA 0x5ef384: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef384() {
}


// 0x5ef394 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE11equalValuesEPKNS0_13DescribedBaseES7_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x5ef394: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef394() {
}


// 0x5ef3bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x5ef3bc: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef3bc() {
}


// 0x5ef3e0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x5ef3e0: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef3e0() {
}


// 0x5ef52c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x5ef52c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef52c() {
}


// 0x5ef554 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE14hasStringValueEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE14hasStringValueEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE14hasStringValueEv
// IDA 0x5ef554: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef554() {
}


// 0x5ef558 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE14getStringValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x5ef558: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef558() {
}


// 0x5ef57c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE14setStringValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x5ef57c: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef57c() {
}


// 0x5ef5bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x5ef5bc: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef5bc() {
}


// 0x5ef5dc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PartInstance,RBX::Material>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_12PartInstanceENS_8MaterialEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x5ef5dc: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef5dc() {
}

