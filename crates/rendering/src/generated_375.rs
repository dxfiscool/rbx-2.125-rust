//! rendering shard 375 — 100 stubs 0x53332c..0x537cbc EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 40760->40860 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x53332c..0x537cbc (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x53332c — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// IDA 0x53332c: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53332c() {
}

// 0x533334 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::GuiButton::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x533334: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_533334() {
}

// 0x5333a8 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// IDA 0x5333a8: 7 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5333a8() {
}

// 0x5333b8 — __ZNK3RBX10Reflection13EventDescBaseINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::GuiButton::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x5333b8: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5333b8() {
}

// 0x5333cc — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiButton,void ()(std::string),1>::BoundFuncDesc(void (RBX::GuiButton::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x5333cc: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5333cc() {
}

// 0x533548 — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiButton,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x533548: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_533548() {
}

// 0x533578 — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EED0Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiButton,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EED0Ev
// IDA 0x533578: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_533578() {
}

// 0x533680 — __ZNK3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiButton,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x533680: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_533680() {
}

// 0x5337bc — __ZN3RBX10Reflection11Call1HelperINS_9GuiButtonEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_9GuiButtonEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs")]
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::GuiButton,void (RBX::GuiButton::*)(std::string),std::string,void>::call(RBX::GuiButton*,void (RBX::GuiButton::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_9GuiButtonEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// IDA 0x5337bc: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5337bc() {
}

// 0x5338ec — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// IDA 0x5338ec: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5338ec() {
}

// 0x533a4c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE8on_errorERSt9exception
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE8on_errorERSt9exception")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE8on_errorERSt9exception
// IDA 0x533a4c: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_533a4c() {
}

// 0x533a74 — __ZN3rbx7signals6signalIFviiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFviiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// IDA 0x533a74: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_533a74() {
}

// 0x533bd4 — __ZN3rbx7signals6signalIFviiEE8on_errorERSt9exception
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE8on_errorERSt9exception")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFviiEE8on_errorERSt9exception
// IDA 0x533bd4: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_533bd4() {
}

// 0x533bfc — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>> const&)")]
// was: __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// IDA 0x533bfc: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_533bfc() {
}

// 0x533c70 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE23listenerConnectionAddedEv
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE23listenerConnectionAddedEv")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::listenerConnectionAdded(void)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE23listenerConnectionAddedEv
// IDA 0x533c70: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_533c70() {
}

// 0x533cbc — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
// IDA 0x533cbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_533cbc() {
}

// 0x533ce8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
// IDA 0x533ce8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_533ce8() {
}

// 0x533dbc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
// IDA 0x533dbc: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_533dbc() {
}

// 0x533dc4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
// IDA 0x533dc4: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_533dc4() {
}

// 0x533dcc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// IDA 0x533dcc: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_533dcc() {
}

// 0x533de4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
// IDA 0x533de4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_533de4() {
}

// 0x533e10 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
// IDA 0x533e10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_533e10() {
}

// 0x533ee4 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>> const&)")]
// was: __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
// IDA 0x533ee4: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_533ee4() {
}

// 0x533f58 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE23listenerConnectionAddedEv
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE23listenerConnectionAddedEv")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::listenerConnectionAdded(void)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE23listenerConnectionAddedEv
// IDA 0x533f58: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_533f58() {
}

// 0x533fa4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev
// IDA 0x533fa4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_533fa4() {
}

// 0x533fd0 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev
// IDA 0x533fd0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_533fd0() {
}

// 0x5340a4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
// IDA 0x5340a4: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5340a4() {
}

// 0x5340ac — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
// IDA 0x5340ac: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5340ac() {
}

// 0x5340b4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
// IDA 0x5340b4: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5340b4() {
}

// 0x5340cc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev
// IDA 0x5340cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5340cc() {
}

// 0x5340f8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev
// IDA 0x5340f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5340f8() {
}

// 0x5341cc — __ZN5boost6detail8function15functor_managerIMN3RBX9GuiObjectEFvNS3_5UDim2EEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerIMN3RBX9GuiObjectEFvNS3_5UDim2EEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<void (RBX::GuiObject::*)(RBX::UDim2)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerIMN3RBX9GuiObjectEFvNS3_5UDim2EEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
// IDA 0x5341cc: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5341cc() {
}

// 0x53422c — __ZN5boost6detail8function26function_void_mem_invoker2IMN3RBX9GuiObjectEFvNS3_5UDim2EEvPS4_S5_E6invokeERNS1_15function_bufferES8_S5_
#[doc(alias = "__ZN5boost6detail8function26function_void_mem_invoker2IMN3RBX9GuiObjectEFvNS3_5UDim2EEvPS4_S5_E6invokeERNS1_15function_bufferES8_S5_")]
#[doc(alias = "boost::detail::function::function_void_mem_invoker2<void (RBX::GuiObject::*)(RBX::UDim2),void,RBX::GuiObject*,RBX::UDim2>::invoke(boost::detail::function::function_buffer &,RBX::GuiObject*,RBX::UDim2)")]
// was: __ZN5boost6detail8function26function_void_mem_invoker2IMN3RBX9GuiObjectEFvNS3_5UDim2EEvPS4_S5_E6invokeERNS1_15function_bufferES8_S5_
// IDA 0x53422c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53422c() {
}

// 0x534260 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE4swapERS4_
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE4swapERS4_")]
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::swap(boost::function1<void,RBX::GuiObject::TweenStatus>&)")]
// was: __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE4swapERS4_
// IDA 0x534260: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_534260() {
}

// 0x53433c — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE11move_assignERS4_
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE11move_assignERS4_")]
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::move_assign(boost::function1<void,RBX::GuiObject::TweenStatus>&)")]
// was: __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE11move_assignERS4_
// IDA 0x53433c: 97 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53433c() {
}

// 0x534440 — __ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS7_5list2INS7_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS7_5list2INS7_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS7_5list2INS7_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// IDA 0x534440: 149 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_534440() {
}

// 0x5345d8 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS6_5list2INS6_5valueIS9_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS6_5list2INS6_5valueIS9_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS6_5list2INS6_5valueIS9_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// IDA 0x5345d8: 151 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5345d8() {
}

// 0x534774 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS6_5list2INS6_5valueIS9_EENSF_ISB_EEEEEEEEvT_
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS6_5list2INS6_5valueIS9_EENSF_ISB_EEEEEEEEvT_")]
#[doc(alias = "void boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>)")]
// was: __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS6_5list2INS6_5valueIS9_EENSF_ISB_EEEEEEEEvT_
// IDA 0x534774: 156 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_534774() {
}

// 0x534920 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// IDA 0x534920: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_534920() {
}

// 0x53493c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEvNS7_11TweenStatusEE6invokeERNS1_15function_bufferESJ_
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEvNS7_11TweenStatusEE6invokeERNS1_15function_bufferESJ_")]
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,void,RBX::GuiObject::TweenStatus>::invoke(boost::detail::function::function_buffer &,RBX::GuiObject::TweenStatus)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEvNS7_11TweenStatusEE6invokeERNS1_15function_bufferESJ_
// IDA 0x53493c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53493c() {
}

// 0x534958 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefEENS8_5list2INS8_5valueISB_EENSH_ISD_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefEENS8_5list2INS8_5valueISB_EENSH_ISD_EEEEEEEEbT_RNS1_15function_bufferE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefEENS8_5list2INS8_5valueISB_EENSH_ISD_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x534958: 151 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_534958() {
}

// 0x534af4 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefEENS8_5list2INS8_5valueISB_EENSH_ISD_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefEENS8_5list2INS8_5valueISB_EENSH_ISD_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefEENS8_5list2INS8_5valueISB_EENSH_ISD_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x534af4: 149 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_534af4() {
}

// 0x534c8c — __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefEENS8_5list2INS8_5valueISB_EENSH_ISD_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefEENS8_5list2INS8_5valueISB_EENSH_ISD_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefEENS8_5list2INS8_5valueISB_EENSH_ISD_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x534c8c: 119 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_534c8c() {
}

// 0x534dd0 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEclIPFvS6_S9_ENS0_5list1IRNS5_11TweenStatusEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEclIPFvS6_S9_ENS0_5list1IRNS5_11TweenStatusEEEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::operator()<void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list1<RBX::GuiObject::TweenStatus &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef) &,boost::_bi::list1<RBX::GuiObject::TweenStatus &> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEclIPFvS6_S9_ENS0_5list1IRNS5_11TweenStatusEEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x534dd0: 113 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_534dd0() {
}

// 0x534f14 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x534f14: 188 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_534f14() {
}

// 0x535104 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_")]
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::list2(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_
// IDA 0x535104: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535104() {
}

// 0x535248 — __ZNK3RBX15ServiceProvider6createINS_12TweenServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_12TweenServiceEEEPT_v")]
#[doc(alias = "RBX::TweenService * RBX::ServiceProvider::create<RBX::TweenService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_12TweenServiceEEEPT_v
// IDA 0x535248: 161 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535248() {
}

// 0x535410 — __ZNK3RBX15ServiceProvider4findINS_12TweenServiceEEEPT_v
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_12TweenServiceEEEPT_v")]
#[doc(alias = "RBX::TweenService * RBX::ServiceProvider::find<RBX::TweenService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_12TweenServiceEEEPT_v
// IDA 0x535410: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535410() {
}

// 0x535584 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12TweenServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_12TweenServiceEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::TweenService> RBX::Creatable<RBX::Instance>::create<RBX::TweenService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_12TweenServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x535584: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535584() {
}

// 0x535634 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12TweenServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12TweenServiceEEERS3_RKNS0_IT_EE")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::TweenService>(rbx_core::SharedPtr<RBX::TweenService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12TweenServiceEEERS3_RKNS0_IT_EE
// IDA 0x535634: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535634() {
}

// 0x535668 — __ZN3RBX4Name7declareILZNS_13sTweenServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sTweenServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_13sTweenServiceEEEERKS0_v
// IDA 0x535668: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535668() {
}

// 0x5356ac — __ZN3RBX4Name13callDoDeclareILZNS_13sTweenServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sTweenServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_13sTweenServiceEEEEvv
// IDA 0x5356ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5356ac() {
}

// 0x5356b0 — __ZN3RBX4Name9doDeclareILZNS_13sTweenServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sTweenServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_13sTweenServiceEEEERKS0_v
// IDA 0x5356b0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5356b0() {
}

// 0x535794 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12TweenServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12TweenServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::TweenService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12TweenServiceEEEvv
// IDA 0x535794: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_535794() {
}

// 0x535798 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12TweenServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_12TweenServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TweenService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_12TweenServiceEEEmv
// IDA 0x535798: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535798() {
}

// 0x535870 — __ZN5boost10shared_ptrIN3RBX12TweenServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12TweenServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::TweenService>::shared_ptr<RBX::TweenService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX12TweenServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x535870: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535870() {
}

// 0x535938 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TweenServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TweenServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TweenService,RBX::TweenService>(rbx_core::SharedPtr<RBX::TweenService> const*,RBX::TweenService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TweenServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x535938: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535938() {
}

// 0x535a20 — __ZN5boost6detail12shared_countC2IPN3RBX12TweenServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX12TweenServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX12TweenServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x535a20: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535a20() {
}

// 0x535b28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x535b28: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_535b28() {
}

// 0x535b2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x535b2c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_535b2c() {
}

// 0x535b30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x535b30: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535b30() {
}

// 0x535b50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x535b50: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535b50() {
}

// 0x535b68 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x535b68: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535b68() {
}

// 0x535b6c — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEE15isNullClassNameEv
// IDA 0x535b6c: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535b6c() {
}

// 0x535c0c — __ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS7_5list3INS7_5valueISA_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS7_5list3INS7_5valueISA_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS7_5list3INS7_5valueISA_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// IDA 0x535c0c: 149 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535c0c() {
}

// 0x535da4 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// IDA 0x535da4: 151 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535da4() {
}

// 0x535f40 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEEvT_")]
#[doc(alias = "void boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>)")]
// was: __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEEvT_
// IDA 0x535f40: 156 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_535f40() {
}

// 0x5360ec — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// IDA 0x5360ec: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5360ec() {
}

// 0x536108 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")]
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>,void,RBX::GuiObject::TweenStatus>::invoke(boost::detail::function::function_buffer &,RBX::GuiObject::TweenStatus)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// IDA 0x536108: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_536108() {
}

// 0x536124 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x536124: 151 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_536124() {
}

// 0x5362c0 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x5362c0: 149 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5362c0() {
}

// 0x536458 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x536458: 119 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_536458() {
}

// 0x53659c — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEclIPFvS6_S9_NS5_11TweenStatusEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEclIPFvS6_S9_NS5_11TweenStatusEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::operator()<void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list1<RBX::GuiObject::TweenStatus&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus) &,boost::_bi::list1<RBX::GuiObject::TweenStatus&> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEclIPFvS6_S9_NS5_11TweenStatusEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x53659c: 117 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53659c() {
}

// 0x5366e8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x5366e8: 188 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5366e8() {
}

// 0x5368d8 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEC2ES7_SA_SC_
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEC2ES7_SA_SC_")]
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>)")]
// was: __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEC2ES7_SA_SC_
// IDA 0x5368d8: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5368d8() {
}

// 0x536a1c — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEC2ES7_SA_SC_
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEC2ES7_SA_SC_")]
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>)")]
// was: __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEC2ES7_SA_SC_
// IDA 0x536a1c: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_536a1c() {
}

// 0x536b60 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_9GuiObject11TweenStatusEEEESA_ENS7_5list2INS7_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_9GuiObject11TweenStatusEEEESA_ENS7_5list2INS7_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_9GuiObject11TweenStatusEEEESA_ENS7_5list2INS7_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// IDA 0x536b60: 77 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_536b60() {
}

// 0x536c38 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// IDA 0x536c38: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_536c38() {
}

// 0x536d10 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEEvT_
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEEvT_")]
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>)")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEEvT_
// IDA 0x536d10: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_536d10() {
}

// 0x536df8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// IDA 0x536df8: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_536df8() {
}

// 0x536e14 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_")]
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_
// IDA 0x536e14: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_536e14() {
}

// 0x536e30 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x536e30: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_536e30() {
}

// 0x536f0c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x536f0c: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_536f0c() {
}

// 0x536fe0 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x536fe0: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_536fe0() {
}

// 0x5370ac — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEclIPFvS8_S6_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEclIPFvS8_S6_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::operator()<void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEclIPFvS8_S6_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x5370ac: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5370ac() {
}

// 0x537170 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x537170: 125 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_537170() {
}

// 0x5372bc — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE13assign_to_ownERKS4_
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE13assign_to_ownERKS4_")]
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to_own(boost::function1<void,RBX::GuiObject::TweenStatus> const&)")]
// was: __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE13assign_to_ownERKS4_
// IDA 0x5372bc: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5372bc() {
}

// 0x5372ec — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_")]
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::list2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_
// IDA 0x5372ec: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5372ec() {
}

// 0x5373b4 — __ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_")]
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::storage2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_
// IDA 0x5373b4: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5373b4() {
}

// 0x537484 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5dummy7nonnullEv
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5dummy7nonnullEv")]
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::dummy::nonnull(void)")]
// was: __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5dummy7nonnullEv
// IDA 0x537484: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_537484() {
}

// 0x537488 — __ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(RBX::UDim2)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev
// IDA 0x537488: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_537488() {
}

// 0x5375e4 — __ZN3rbx13remote_signalIFviiEEC2Ev
#[doc(alias = "__ZN3rbx13remote_signalIFviiEEC2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(int,int)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFviiEEC2Ev
// IDA 0x5375e4: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5375e4() {
}

// 0x537740 — __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv")]
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)")]
// was: __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv
// IDA 0x537740: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_537740() {
}

// 0x537834 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
// IDA 0x537834: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_537834() {
}

// 0x53789c — __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE25signalProducedIncrementedES2_
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE25signalProducedIncrementedES2_")]
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::signalProducedIncremented(RBX::UDim2)")]
// was: __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE25signalProducedIncrementedES2_
// IDA 0x53789c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53789c() {
}

// 0x5378c4 — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_")]
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::replicateEvent(RBX::Reflection::EventSource *,RBX::UDim2)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_
// IDA 0x5378c4: 129 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5378c4() {
}

// 0x537a18 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// IDA 0x537a18: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_537a18() {
}

// 0x537a8c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6insertEPNS5_4slotE
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6insertEPNS5_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::insert(rbx::signals::signal<void ()(RBX::UDim2)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6insertEPNS5_4slotE
// IDA 0x537a8c: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_537a8c() {
}

// 0x537c98 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSEPS8_
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSEPS8_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UDim2)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSEPS8_
// IDA 0x537c98: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_537c98() {
}

// 0x537cbc — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev
// IDA 0x537cbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_537cbc() {
}
