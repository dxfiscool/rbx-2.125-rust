//! rendering shard A — 240 stubs (120 Ogre|G3D|Gfx|Render|Adorn + 120 Ogre|G3D strict EA-sorted after last rendering stub 0xd18074)
//! Filter: Ogre|G3D strict (13333 strict total / 13663 substr, 120 this batch 0xd18118..0xd219d0, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x3adf50 — __ZNK5boost9function3IvN3G3D7Vector34AxisEffEclES3_ff
#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::operator()(G3D::Vector3::Axis,float,float)const")]
// was: boost::function3<void,G3D::Vector3::Axis,float,float>::operator()(G3D::Vector3::Axis,float,float)const
// IDA 0x3adf50: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3adf50() {
}

// 0x3ae028 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()
// IDA 0x3ae028: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ae028() {
}

// 0x3ae138 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()
// IDA 0x3ae138: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ae138() {
}

// 0x3ae268 — __ZN5boost9function3IvN3G3D7Vector34AxisEffE13assign_to_ownERKS4_
#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::assign_to_own(boost::function3<void,G3D::Vector3::Axis,float,float> const&)")]
// was: boost::function3<void,G3D::Vector3::Axis,float,float>::assign_to_own(boost::function3<void,G3D::Vector3::Axis,float,float> const&)
// IDA 0x3ae268: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ae268() {
}

// 0x3ae298 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_EC2ESA_PKcSD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::EventDesc(rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::EventDesc(rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0x3ae298: 236 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ae298() {
}

// 0x3ae4f4 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::~EventDesc()
// IDA 0x3ae4f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ae4f4() {
}

// 0x3ae518 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::~EventDesc()
// IDA 0x3ae518: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ae518() {
}

// 0x3ae5cc — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::~RemoteEventDesc()")]
// was: RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::~RemoteEventDesc()
// IDA 0x3ae5cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ae5cc() {
}

// 0x3ae680 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// IDA 0x3ae680: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ae680() {
}

// 0x3ae7e4 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::isScriptable(void)const")]
// was: RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::isScriptable(void)const
// IDA 0x3ae7e4: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ae7e4() {
}

// 0x3ae7ec — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::isBroadcast(void)const")]
// was: RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::isBroadcast(void)const
// IDA 0x3ae7ec: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ae7ec() {
}

// 0x3ae7f4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
// IDA 0x3ae7f4: 45 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ae7f4() {
}

// 0x3ae880 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
// IDA 0x3ae880: 7 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ae880() {
}

// 0x3ae890 — __ZNK3RBX10Reflection13EventDescBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::disconnectAll(RBX::Reflection::EventSource *)const
// IDA 0x3ae890: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ae890() {
}

// 0x3ae8a4 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector34AxisENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISF_T0_T1_EENSD_9list_av_2IT2_T3_E4typeEEEMSI_FSF_SJ_ESM_SN_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector3::Axis const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector3::Axis const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
// IDA 0x3ae8a4: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ae8a4() {
}

// 0x3ae9c0 — __ZN3RBX10Reflection18GenericSlotWrapper8execute1IN3G3D7Vector34AxisEEEvRKT_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<G3D::Vector3::Axis>(G3D::Vector3::Axis const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute1<G3D::Vector3::Axis>(G3D::Vector3::Axis const&)
// IDA 0x3ae9c0: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ae9c0() {
}

// 0x3aeb04 — __ZN5boost9function1IvN3G3D7Vector34AxisEE5clearEv
#[doc(alias = "boost::function1<void,G3D::Vector3::Axis>::clear(void)")]
// was: boost::function1<void,G3D::Vector3::Axis>::clear(void)
// IDA 0x3aeb04: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3aeb04() {
}

// 0x3aeb30 — __ZN5boost8functionIFvN3G3D7Vector34AxisEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvN3G3D7Vector34AxisEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3G3D7Vector34AxisEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// IDA 0x3aeb30: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3aeb30() {
}

// 0x3aec14 — __ZN5boost9function1IvN3G3D7Vector34AxisEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvN3G3D7Vector34AxisEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvN3G3D7Vector34AxisEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// IDA 0x3aec14: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3aec14() {
}

// 0x3aecfc — __ZN5boost9function1IvN3G3D7Vector34AxisEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,G3D::Vector3::Axis>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// was: void boost::function1<void,G3D::Vector3::Axis>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
// IDA 0x3aecfc: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3aecfc() {
}

// 0x3aedf4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector34AxisEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x3aedf4: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3aedf4() {
}

// 0x3aee10 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector34AxisEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,G3D::Vector3::Axis>::invoke(boost::detail::function::function_buffer &,G3D::Vector3::Axis)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,G3D::Vector3::Axis>::invoke(boost::detail::function::function_buffer &,G3D::Vector3::Axis)
// IDA 0x3aee10: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3aee10() {
}

// 0x3aee24 — __ZNK5boost6detail8function13basic_vtable1IvN3G3D7Vector34AxisEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISE_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,G3D::Vector3::Axis>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,G3D::Vector3::Axis>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
// IDA 0x3aee24: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3aee24() {
}

// 0x3aef0c — __ZNK5boost6detail8function13basic_vtable1IvN3G3D7Vector34AxisEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISE_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,G3D::Vector3::Axis>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,G3D::Vector3::Axis>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0x3aef0c: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3aef0c() {
}

// 0x3aeff0 — __ZNK5boost6detail8function13basic_vtable1IvN3G3D7Vector34AxisEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISE_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,G3D::Vector3::Axis>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,G3D::Vector3::Axis>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// IDA 0x3aeff0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3aeff0() {
}

// 0x3af0c4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector34AxisEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS9_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)
// IDA 0x3af0c4: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3af0c4() {
}

// 0x3af0dc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector34AxisEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// IDA 0x3af0dc: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3af0dc() {
}

// 0x3af234 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::function<void ()(G3D::Vector3::Axis)>>(boost::function<void ()(G3D::Vector3::Axis)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::function<void ()(G3D::Vector3::Axis)>>(boost::function<void ()(G3D::Vector3::Axis)> const&)
// IDA 0x3af234: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3af234() {
}

// 0x3af328 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>*>(boost::function<void ()(G3D::Vector3::Axis)> const&,rbx::signals::signal<void ()(G3D::Vector3::Axis)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>*>(boost::function<void ()(G3D::Vector3::Axis)> const&,rbx::signals::signal<void ()(G3D::Vector3::Axis)>*)
// IDA 0x3af328: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3af328() {
}

// 0x3af424 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost8functionIS5_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis)>>::~callable_slot()
// IDA 0x3af424: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3af424() {
}

// 0x3af534 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost8functionIS5_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis)>>::~callable_slot()
// IDA 0x3af534: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3af534() {
}

// 0x3af664 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)
// IDA 0x3af664: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3af664() {
}

// 0x3af66c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)
// IDA 0x3af66c: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3af66c() {
}

// 0x3af674 — __ZNK5boost9function1IvN3G3D7Vector34AxisEEclES3_
#[doc(alias = "boost::function1<void,G3D::Vector3::Axis>::operator()(G3D::Vector3::Axis)const")]
// was: boost::function1<void,G3D::Vector3::Axis>::operator()(G3D::Vector3::Axis)const
// IDA 0x3af674: 67 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3af674() {
}

// 0x3af738 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::~callable()
// IDA 0x3af738: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3af738() {
}

// 0x3af848 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::~callable()
// IDA 0x3af848: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3af848() {
}

// 0x3af978 — __ZN5boost9function1IvN3G3D7Vector34AxisEE13assign_to_ownERKS4_
#[doc(alias = "boost::function1<void,G3D::Vector3::Axis>::assign_to_own(boost::function1<void,G3D::Vector3::Axis> const&)")]
// was: boost::function1<void,G3D::Vector3::Axis>::assign_to_own(boost::function1<void,G3D::Vector3::Axis> const&)
// IDA 0x3af978: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3af978() {
}

// 0x3af9a8 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::EventDesc(rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::EventDesc(rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0x3af9a8: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3af9a8() {
}

// 0x3afb2c — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::~EventDesc()
// IDA 0x3afb2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3afb2c() {
}

// 0x3afb50 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::~EventDesc()
// IDA 0x3afb50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3afb50() {
}

// 0x3b0324 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEED2Ev
#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::~remote_signal()")]
// was: rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::~remote_signal()
// IDA 0x3b0324: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b0324() {
}

// 0x3b0470 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEED2Ev
#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis)>::~remote_signal()")]
// was: rbx::remote_signal<void ()(G3D::Vector3::Axis)>::~remote_signal()
// IDA 0x3b0470: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b0470() {
}

// 0x3b05bc — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::~EventReplicatorBase()")]
// was: RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::~EventReplicatorBase()
// IDA 0x3b05bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b05bc() {
}

// 0x3b06ec — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::~EventReplicatorBase()")]
// was: RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::~EventReplicatorBase()
// IDA 0x3b06ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b06ec() {
}

// 0x3bc1ec — __ZNK3RBX17BasicPartInstance16partNeeds3dAdornEv
#[doc(alias = "RBX::BasicPartInstance::partNeeds3dAdorn(void)const")]
// was: RBX::BasicPartInstance::partNeeds3dAdorn(void)const
// IDA 0x3bc1ec: 6 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bc1ec() {
}

// 0x3bfaf4 — __ZN3RBX12BillboardGui10setAdorneeEPNS_8InstanceE
#[doc(alias = "RBX::BillboardGui::setAdornee(RBX::Instance *)")]
// was: RBX::BillboardGui::setAdornee(RBX::Instance *)
// IDA 0x3bfaf4: 233 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bfaf4() {
}

// 0x3bfd80 — __ZN3RBX12BillboardGui14setStudsOffsetERKN3G3D7Vector3E
#[doc(alias = "RBX::BillboardGui::setStudsOffset(G3D::Vector3 const&)")]
// was: RBX::BillboardGui::setStudsOffset(G3D::Vector3 const&)
// IDA 0x3bfd80: 31 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bfd80() {
}

// 0x3bfdf8 — __ZN3RBX12BillboardGui16setExtentsOffsetERKN3G3D7Vector3E
#[doc(alias = "RBX::BillboardGui::setExtentsOffset(G3D::Vector3 const&)")]
// was: RBX::BillboardGui::setExtentsOffset(G3D::Vector3 const&)
// IDA 0x3bfdf8: 31 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bfdf8() {
}

// 0x3bfe70 — __ZN3RBX12BillboardGui13setSizeOffsetERKN3G3D7Vector2E
#[doc(alias = "RBX::BillboardGui::setSizeOffset(G3D::Vector2 const&)")]
// was: RBX::BillboardGui::setSizeOffset(G3D::Vector2 const&)
// IDA 0x3bfe70: 22 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bfe70() {
}

// 0x3c0678 — __ZNK3RBX12BillboardGui13getModelAdornEv
#[doc(alias = "RBX::BillboardGui::getModelAdorn(void)const")]
// was: RBX::BillboardGui::getModelAdorn(void)const
// IDA 0x3c0678: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c0678() {
}

// 0x3c0764 — __ZNK3RBX12BillboardGui12getPartAdornEv
#[doc(alias = "RBX::BillboardGui::getPartAdorn(void)const")]
// was: RBX::BillboardGui::getPartAdorn(void)const
// IDA 0x3c0764: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c0764() {
}

// 0x3c0a34 — __ZN3RBX12BillboardGui19render3dSortedAdornEPNS_5AdornE
#[doc(alias = "RBX::BillboardGui::render3dSortedAdorn(RBX::Adorn *)")]
// was: RBX::BillboardGui::render3dSortedAdorn(RBX::Adorn *)
// IDA 0x3c0a34: 429 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c0a34() {
}

// 0x3c0e90 — __ZThn96_N3RBX12BillboardGui19render3dSortedAdornEPNS_5AdornE
#[doc(alias = "non-virtual thunk to RBX::BillboardGui::render3dSortedAdorn(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::BillboardGui::render3dSortedAdorn(RBX::Adorn *)
// IDA 0x3c0e90: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c0e90() {
}

// 0x3c0f58 — __ZNK3RBX12BillboardGui19getAdorneeDangerousEv
#[doc(alias = "RBX::BillboardGui::getAdorneeDangerous(void)const")]
// was: RBX::BillboardGui::getAdorneeDangerous(void)const
// IDA 0x3c0f58: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c0f58() {
}

// 0x3c0fa8 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::~PropDescriptor()
// IDA 0x3c0fa8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3c0fa8() {
}

// 0x3c0fcc — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::~PropDescriptor()
// IDA 0x3c0fcc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3c0fcc() {
}

// 0x3c106c — __ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEaSERKS7_
#[doc(alias = "boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)>::operator=(boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)> const&)")]
// was: boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)>::operator=(boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)> const&)
// IDA 0x3c106c: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c106c() {
}

// 0x3c1130 — __ZNK5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEclES3_S5_
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::operator()(RBX::BillboardGui *,RBX::Adorn *)const")]
// was: boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::operator()(RBX::BillboardGui *,RBX::Adorn *)const
// IDA 0x3c1130: 69 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c1130() {
}

// 0x3c12a4 — __ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv
// IDA 0x3c12a4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c12a4() {
}

// 0x3c12b8 — __ZNK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE
#[doc(alias = "RBX::BillboardGui::isVisible(G3D::Rect2D const&)const")]
// was: RBX::BillboardGui::isVisible(G3D::Rect2D const&)const
// IDA 0x3c12b8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c12b8() {
}

// 0x3c1368 — __ZThn32_NK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv
// IDA 0x3c1368: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c1368() {
}

// 0x3c1424 — __ZThn96_NK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE
#[doc(alias = "non-virtual thunk to RBX::BillboardGui::isVisible(G3D::Rect2D const&)const")]
// was: non-virtual thunk to RBX::BillboardGui::isVisible(G3D::Rect2D const&)const
// IDA 0x3c1424: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c1424() {
}

// 0x3c14d4 — __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD1Ev
// IDA 0x3c14d4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3c14d4() {
}

// 0x3c14d8 — __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD2Ev
// IDA 0x3c14d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3c14d8() {
}

// 0x3c1574 — __ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x3c1574: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c1574() {
}

// 0x3c15fc — __ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7Creator6createEv
// IDA 0x3c15fc: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c15fc() {
}

// 0x3c1740 — __ZN3RBX4Name13callDoDeclareILZNS_13sAdornmentGuiEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sAdornmentGuiEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_13sAdornmentGuiEEEEvv
// IDA 0x3c1740: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3c1740() {
}

// 0x3c1744 — __ZN3RBX4Name9doDeclareILZNS_13sAdornmentGuiEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sAdornmentGuiEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_13sAdornmentGuiEEEERKS0_v
// IDA 0x3c1744: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c1744() {
}

// 0x3c1824 — __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorC2Ev
// IDA 0x3c1824: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c1824() {
}

// 0x3c1a68 — __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE17static_getCreatorEv
// IDA 0x3c1a68: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c1a68() {
}

// 0x3c1adc — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE5dummy7nonnullEv
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::dummy::nonnull(void)")]
// was: boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::dummy::nonnull(void)
// IDA 0x3c1adc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3c1adc() {
}

// 0x3c1ae0 — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE4swapERS6_
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::swap(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>&)")]
// was: boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::swap(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>&)
// IDA 0x3c1ae0: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c1ae0() {
}

// 0x3c1bbc — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE11move_assignERS6_
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::move_assign(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>&)")]
// was: boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::move_assign(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>&)
// IDA 0x3c1bbc: 97 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c1bbc() {
}

// 0x3c1cc0 — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE13assign_to_ownERKS6_
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to_own(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *> const&)")]
// was: boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to_own(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *> const&)
// IDA 0x3c1cc0: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c1cc0() {
}

// 0x3c1cf0 — __ZN3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3c1cf0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3c1cf0() {
}

// 0x3c1cf4 — __ZN3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3c1cf4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3c1cf4() {
}

// 0x3c1d94 — __ZThn32_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3c1d94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3c1d94() {
}

// 0x3c1d9c — __ZThn32_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3c1d9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3c1d9c() {
}

// 0x3c1e40 — __ZThn36_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3c1e40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3c1e40() {
}

// 0x3c1e48 — __ZThn36_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12BillboardGuiELZNS_13sAdornmentGuiEENS_14FactoryProductIS2_NS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3c1e48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3c1e48() {
}

// 0x3c2224 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::PropDescriptor<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>(char const*,char const*,G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::PropDescriptor<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>(char const*,char const*,G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x3c2224: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c2224() {
}

// 0x3c2338 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::~PropDescriptor()
// IDA 0x3c2338: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3c2338() {
}

// 0x3c2364 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::isReadOnly(void)const
// IDA 0x3c2364: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c2364() {
}

// 0x3c2368 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::isWriteOnly(void)const
// IDA 0x3c2368: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c2368() {
}

// 0x3c236c — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x3c236c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c236c() {
}

// 0x3c239c — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const
// IDA 0x3c239c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c239c() {
}

// 0x3c23c0 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::PropDescriptor<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::PropDescriptor<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x3c23c0: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c23c0() {
}

// 0x3c24d4 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::~PropDescriptor()
// IDA 0x3c24d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3c24d4() {
}

// 0x3c2500 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::isReadOnly(void)const
// IDA 0x3c2500: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c2500() {
}

// 0x3c2504 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::isWriteOnly(void)const
// IDA 0x3c2504: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c2504() {
}

// 0x3c2508 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x3c2508: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c2508() {
}

// 0x3c253c — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const
// IDA 0x3c253c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c253c() {
}

// 0x3c35d4 — __ZN3RBX6Camera24setCameraCoordinateFrameERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::Camera::setCameraCoordinateFrame(G3D::CoordinateFrame const&)")]
// was: RBX::Camera::setCameraCoordinateFrame(G3D::CoordinateFrame const&)
// IDA 0x3c35d4: 107 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c35d4() {
}

// 0x3c3738 — __ZN3RBX6Camera14setCameraFocusERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::Camera::setCameraFocus(G3D::CoordinateFrame const&)")]
// was: RBX::Camera::setCameraFocus(G3D::CoordinateFrame const&)
// IDA 0x3c3738: 70 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c3738() {
}

// 0x3c3fa4 — __ZN3RBX6Camera24beginCameraInterpolationEN3G3D15CoordinateFrameES2_f
#[doc(alias = "RBX::Camera::beginCameraInterpolation(G3D::CoordinateFrame,G3D::CoordinateFrame,float)")]
// was: RBX::Camera::beginCameraInterpolation(G3D::CoordinateFrame,G3D::CoordinateFrame,float)
// IDA 0x3c3fa4: 301 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c3fa4() {
}

// 0x3c4f8c — __ZNK3RBX6Camera17isPartVisibleFastERKNS_12PartInstanceERKN3G3D6Rect2DERKNS_14ContactManagerE
#[doc(alias = "RBX::Camera::isPartVisibleFast(RBX::PartInstance const&,G3D::Rect2D const&,RBX::ContactManager const&)const")]
// was: RBX::Camera::isPartVisibleFast(RBX::PartInstance const&,G3D::Rect2D const&,RBX::ContactManager const&)const
// IDA 0x3c4f8c: 126 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c4f8c() {
}

// 0x3c50fc — __ZNK3RBX6Camera15isPartInFrustumERKNS_12PartInstanceERKN3G3D6Rect2DE
#[doc(alias = "RBX::Camera::isPartInFrustum(RBX::PartInstance const&,G3D::Rect2D const&)const")]
// was: RBX::Camera::isPartInFrustum(RBX::PartInstance const&,G3D::Rect2D const&)const
// IDA 0x3c50fc: 68 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c50fc() {
}

// 0x3c51c0 — __ZNK3RBX6Camera7frustumERKN3G3D6Rect2DE
#[doc(alias = "RBX::Camera::frustum(G3D::Rect2D const&)const")]
// was: RBX::Camera::frustum(G3D::Rect2D const&)const
// IDA 0x3c51c0: 66 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c51c0() {
}

// 0x3c5284 — __ZNK3RBX6Camera22getNearViewportCornersERKN3G3D6Rect2DERNS1_7Vector3ES6_S6_S6_
#[doc(alias = "RBX::Camera::getNearViewportCorners(G3D::Rect2D const&,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &)const")]
// was: RBX::Camera::getNearViewportCorners(G3D::Rect2D const&,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &)const
// IDA 0x3c5284: 177 insns (VMOV.F32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c5284() {
}

// 0x3c6144 — __ZN3RBX6Camera35setCameraFocusWithoutPropertyChangeERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::Camera::setCameraFocusWithoutPropertyChange(G3D::CoordinateFrame const&)")]
// was: RBX::Camera::setCameraFocusWithoutPropertyChange(G3D::CoordinateFrame const&)
// IDA 0x3c6144: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c6144() {
}

// 0x3c6210 — __ZN3RBX6Camera18setCameraFocusOnlyERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::Camera::setCameraFocusOnly(G3D::CoordinateFrame const&)")]
// was: RBX::Camera::setCameraFocusOnly(G3D::CoordinateFrame const&)
// IDA 0x3c6210: 52 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c6210() {
}

// 0x3c690c — __ZN3RBX6Camera39setCameraFocusOnlyWithoutPropertyChangeERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::Camera::setCameraFocusOnlyWithoutPropertyChange(G3D::CoordinateFrame const&)")]
// was: RBX::Camera::setCameraFocusOnlyWithoutPropertyChange(G3D::CoordinateFrame const&)
// IDA 0x3c690c: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c690c() {
}

// 0x3c6d3c — __ZN3RBX6Camera21setDistanceFromTargetEfRN3G3D15CoordinateFrameERKS2_
#[doc(alias = "RBX::Camera::setDistanceFromTarget(float,G3D::CoordinateFrame &,G3D::CoordinateFrame const&)")]
// was: RBX::Camera::setDistanceFromTarget(float,G3D::CoordinateFrame &,G3D::CoordinateFrame const&)
// IDA 0x3c6d3c: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c6d3c() {
}

// 0x3c6e7c — __ZN3RBX6Camera13lerpToExtentsERKNS_7ExtentsERKN3G3D6Rect2DE
#[doc(alias = "RBX::Camera::lerpToExtents(RBX::Extents const&,G3D::Rect2D const&)")]
// was: RBX::Camera::lerpToExtents(RBX::Extents const&,G3D::Rect2D const&)
// IDA 0x3c6e7c: 373 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c6e7c() {
}

// 0x3c79f4 — __ZN3RBX6Camera11zoomExtentsERKN3G3D6Rect2DE
#[doc(alias = "RBX::Camera::zoomExtents(G3D::Rect2D const&)")]
// was: RBX::Camera::zoomExtents(G3D::Rect2D const&)
// IDA 0x3c79f4: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c79f4() {
}

// 0x3c7b34 — __ZN3RBX6Camera30setCameraFocusAndMaintainFocusERKN3G3D15CoordinateFrameEb
#[doc(alias = "RBX::Camera::setCameraFocusAndMaintainFocus(G3D::CoordinateFrame const&,bool)")]
// was: RBX::Camera::setCameraFocusAndMaintainFocus(G3D::CoordinateFrame const&,bool)
// IDA 0x3c7b34: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c7b34() {
}

// 0x3c7b48 — __ZN3RBX6Camera16legalCameraCoordERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::Camera::legalCameraCoord(G3D::CoordinateFrame const&)")]
// was: RBX::Camera::legalCameraCoord(G3D::CoordinateFrame const&)
// IDA 0x3c7b48: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c7b48() {
}

// 0x3c8734 — __ZNK3RBX6Camera18getImagePlaneDepthERKN3G3D6Rect2DE
#[doc(alias = "RBX::Camera::getImagePlaneDepth(G3D::Rect2D const&)const")]
// was: RBX::Camera::getImagePlaneDepth(G3D::Rect2D const&)const
// IDA 0x3c8734: 7 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c8734() {
}

// 0x3c8750 — __ZNK3RBX6Camera7projectERKN3G3D7Vector3ERKNS1_6Rect2DE
#[doc(alias = "RBX::Camera::project(G3D::Vector3 const&,G3D::Rect2D const&)const")]
// was: RBX::Camera::project(G3D::Vector3 const&,G3D::Rect2D const&)const
// IDA 0x3c8750: 80 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c8750() {
}

// 0x3c8888 — __ZNK3RBX6Camera8worldRayEffRKN3G3D6Rect2DE
#[doc(alias = "RBX::Camera::worldRay(float,float,G3D::Rect2D const&)const")]
// was: RBX::Camera::worldRay(float,float,G3D::Rect2D const&)const
// IDA 0x3c8888: 96 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c8888() {
}

// 0x3c89dc — __ZNK3RBX6Camera3dotERKN3G3D7Vector3E
#[doc(alias = "RBX::Camera::dot(G3D::Vector3 const&)const")]
// was: RBX::Camera::dot(G3D::Vector3 const&)const
// IDA 0x3c89dc: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c89dc() {
}

// 0x3c8a58 — __ZNK3RBX6Camera7frustumERKN3G3D6Rect2DEfRNS_7FrustumE
#[doc(alias = "RBX::Camera::frustum(G3D::Rect2D const&,float,RBX::Frustum &)const")]
// was: RBX::Camera::frustum(G3D::Rect2D const&,float,RBX::Frustum &)const
// IDA 0x3c8a58: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c8a58() {
}

// 0x3c8c14 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::~PropDescriptor()
// IDA 0x3c8c14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3c8c14() {
}

// 0x3c8e24 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::~BoundFuncDesc()
// IDA 0x3c8e24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3c8e24() {
}

// 0x3c9acc — __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE2atEm
#[doc(alias = "std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::at(unsigned long)")]
// was: std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::at(unsigned long)
// IDA 0x3c9acc: 15 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c9acc() {
}

// 0x3c9b00 — __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
#[doc(alias = "std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::insert(__gnu_cxx::__normal_iterator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>*,std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>>,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)")]
// was: std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::insert(__gnu_cxx::__normal_iterator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>*,std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>>,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)
// IDA 0x3c9b00: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c9b00() {
}

// 0x3c9b48 — __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE9push_backERKS3_
#[doc(alias = "std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::push_back(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)")]
// was: std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::push_back(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)
// IDA 0x3c9b48: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_3c9b48() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x3c9b80 — __ZNK3RBX7Extents8containsERKN3G3D7Vector3E
#[doc(alias = "RBX::Extents::contains(G3D::Vector3 const&)const")]
// was: RBX::Extents::contains(G3D::Vector3 const&)const
// IDA 0x3c9b80: 35 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3c9b80() {
}

// 0x3cba0c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPSt4pairIN3G3D15CoordinateFrameES5_ES7_EET0_T_S9_S8_
#[doc(alias = "std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> * std::__copy<false,std::random_access_iterator_tag>::copy<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *>(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *)")]
// was: std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> * std::__copy<false,std::random_access_iterator_tag>::copy<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *>(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *)
// IDA 0x3cba0c: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_3cba0c() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x3cbab4 — __ZN9__gnu_cxx13new_allocatorISt4pairIN3G3D15CoordinateFrameES3_EE9constructEPS4_RKS4_
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>::construct(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>*,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)")]
// was: __gnu_cxx::new_allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>::construct(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>*,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)
// IDA 0x3cbab4: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cbab4() {
}

// — next 120 Ogre|G3D strict EA-sorted after last rendering stub 0xd18074 (0xd18118..0xd219d0) —

// 0xd18118 — __ZN4Ogre18MeshSerializerImpl24calcSubMeshOperationSizeEPKNS_7SubMeshE
#[doc(alias = "Ogre::MeshSerializerImpl::calcSubMeshOperationSize(Ogre::SubMesh const*)")]
// was: Ogre::MeshSerializerImpl::calcSubMeshOperationSize(Ogre::SubMesh const*)
// IDA 0xd18118: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d18118() {
}

// 0xd1811c — __ZN4Ogre18MeshSerializerImpl29calcSubMeshTextureAliasesSizeEPKNS_7SubMeshE
#[doc(alias = "Ogre::MeshSerializerImpl::calcSubMeshTextureAliasesSize(Ogre::SubMesh const*)")]
// was: Ogre::MeshSerializerImpl::calcSubMeshTextureAliasesSize(Ogre::SubMesh const*)
// IDA 0xd1811c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1811c() {
}

// 0xd1814c — __ZN4Ogre18MeshSerializerImpl16calcGeometrySizeEPKNS_10VertexDataE
#[doc(alias = "Ogre::MeshSerializerImpl::calcGeometrySize(Ogre::VertexData const*)")]
// was: Ogre::MeshSerializerImpl::calcGeometrySize(Ogre::VertexData const*)
// IDA 0xd1814c: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1814c() {
}

// 0xd18188 — __ZN4Ogre18MeshSerializerImpl12readGeometryERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_10VertexDataE
#[doc(alias = "Ogre::MeshSerializerImpl::readGeometry(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)")]
// was: Ogre::MeshSerializerImpl::readGeometry(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)
// IDA 0xd18188: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d18188() {
}

// 0xd18258 — __ZN4Ogre18MeshSerializerImpl29readGeometryVertexDeclarationERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_10VertexDataE
#[doc(alias = "Ogre::MeshSerializerImpl::readGeometryVertexDeclaration(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)")]
// was: Ogre::MeshSerializerImpl::readGeometryVertexDeclaration(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)
// IDA 0xd18258: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d18258() {
}

// 0xd182d4 — __ZN4Ogre18MeshSerializerImpl25readGeometryVertexElementERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_10VertexDataE
#[doc(alias = "Ogre::MeshSerializerImpl::readGeometryVertexElement(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)")]
// was: Ogre::MeshSerializerImpl::readGeometryVertexElement(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)
// IDA 0xd182d4: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d182d4() {
}

// 0xd1846c — __ZN4Ogre18MeshSerializerImpl24readGeometryVertexBufferERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_10VertexDataE
#[doc(alias = "Ogre::MeshSerializerImpl::readGeometryVertexBuffer(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)")]
// was: Ogre::MeshSerializerImpl::readGeometryVertexBuffer(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)
// IDA 0xd1846c: 613 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1846c() {
}

// 0xd18adc — __ZN4Ogre18MeshSerializerImpl20readSubMeshNameTableERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readSubMeshNameTable(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)")]
// was: Ogre::MeshSerializerImpl::readSubMeshNameTable(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)
// IDA 0xd18adc: 234 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d18adc() {
}

// 0xd18d44 — __ZN4Ogre18MeshSerializerImpl8readMeshERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_22MeshSerializerListenerE
#[doc(alias = "Ogre::MeshSerializerImpl::readMesh(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::MeshSerializerListener *)")]
// was: Ogre::MeshSerializerImpl::readMesh(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::MeshSerializerListener *)
// IDA 0xd18d44: 353 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d18d44() {
}

// 0xd190d4 — __ZN4Ogre18MeshSerializerImpl11readSubMeshERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_22MeshSerializerListenerE
#[doc(alias = "Ogre::MeshSerializerImpl::readSubMesh(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::MeshSerializerListener *)")]
// was: Ogre::MeshSerializerImpl::readSubMesh(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::MeshSerializerListener *)
// IDA 0xd190d4: 810 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d190d4() {
}

// 0xd1990c — __ZN4Ogre18MeshSerializerImpl20readSubMeshOperationERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_7SubMeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readSubMeshOperation(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::SubMesh *)")]
// was: Ogre::MeshSerializerImpl::readSubMeshOperation(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::SubMesh *)
// IDA 0xd1990c: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1990c() {
}

// 0xd19928 — __ZN4Ogre18MeshSerializerImpl23readSubMeshTextureAliasERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_7SubMeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readSubMeshTextureAlias(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::SubMesh *)")]
// was: Ogre::MeshSerializerImpl::readSubMeshTextureAlias(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::SubMesh *)
// IDA 0xd19928: 146 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d19928() {
}

// 0xd19ad0 — __ZN4Ogre18MeshSerializerImpl17writeSkeletonLinkERKSs
#[doc(alias = "Ogre::MeshSerializerImpl::writeSkeletonLink(std::string const&)")]
// was: Ogre::MeshSerializerImpl::writeSkeletonLink(std::string const&)
// IDA 0xd19ad0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d19ad0() {
}

// 0xd19af8 — __ZN4Ogre18MeshSerializerImpl16readSkeletonLinkERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_22MeshSerializerListenerE
#[doc(alias = "Ogre::MeshSerializerImpl::readSkeletonLink(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::MeshSerializerListener *)")]
// was: Ogre::MeshSerializerImpl::readSkeletonLink(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::MeshSerializerListener *)
// IDA 0xd19af8: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d19af8() {
}

// 0xd19c2c — __ZN4Ogre18MeshSerializerImpl16readTextureLayerERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshERNS_11MaterialPtrE
#[doc(alias = "Ogre::MeshSerializerImpl::readTextureLayer(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::MaterialPtr &)")]
// was: Ogre::MeshSerializerImpl::readTextureLayer(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::MaterialPtr &)
// IDA 0xd19c2c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d19c2c() {
}

// 0xd19c30 — __ZN4Ogre18MeshSerializerImpl20calcSkeletonLinkSizeERKSs
#[doc(alias = "Ogre::MeshSerializerImpl::calcSkeletonLinkSize(std::string const&)")]
// was: Ogre::MeshSerializerImpl::calcSkeletonLinkSize(std::string const&)
// IDA 0xd19c30: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d19c30() {
}

// 0xd19c3c — __ZN4Ogre18MeshSerializerImpl23writeMeshBoneAssignmentERKNS_22VertexBoneAssignment_sE
#[doc(alias = "Ogre::MeshSerializerImpl::writeMeshBoneAssignment(Ogre::VertexBoneAssignment_s const&)")]
// was: Ogre::MeshSerializerImpl::writeMeshBoneAssignment(Ogre::VertexBoneAssignment_s const&)
// IDA 0xd19c3c: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d19c3c() {
}

// 0xd19c7c — __ZN4Ogre18MeshSerializerImpl26writeSubMeshBoneAssignmentERKNS_22VertexBoneAssignment_sE
#[doc(alias = "Ogre::MeshSerializerImpl::writeSubMeshBoneAssignment(Ogre::VertexBoneAssignment_s const&)")]
// was: Ogre::MeshSerializerImpl::writeSubMeshBoneAssignment(Ogre::VertexBoneAssignment_s const&)
// IDA 0xd19c7c: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d19c7c() {
}

// 0xd19cbc — __ZN4Ogre18MeshSerializerImpl22readMeshBoneAssignmentERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readMeshBoneAssignment(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)")]
// was: Ogre::MeshSerializerImpl::readMeshBoneAssignment(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)
// IDA 0xd19cbc: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d19cbc() {
}

// 0xd19d00 — __ZN4Ogre18MeshSerializerImpl25readSubMeshBoneAssignmentERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_7SubMeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readSubMeshBoneAssignment(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::SubMesh *)")]
// was: Ogre::MeshSerializerImpl::readSubMeshBoneAssignment(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::SubMesh *)
// IDA 0xd19d00: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d19d00() {
}

// 0xd19d44 — __ZN4Ogre18MeshSerializerImpl22calcBoneAssignmentSizeEv
#[doc(alias = "Ogre::MeshSerializerImpl::calcBoneAssignmentSize(void)")]
// was: Ogre::MeshSerializerImpl::calcBoneAssignmentSize(void)
// IDA 0xd19d44: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d19d44() {
}

// 0xd19d48 — __ZN4Ogre18MeshSerializerImpl12writeLodInfoEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::writeLodInfo(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl::writeLodInfo(Ogre::Mesh const*)
// IDA 0xd19d48: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d19d48() {
}

// 0xd19dc0 — __ZN4Ogre18MeshSerializerImpl15writeLodSummaryEtbPKNS_11LodStrategyE
#[doc(alias = "Ogre::MeshSerializerImpl::writeLodSummary(unsigned short,bool,Ogre::LodStrategy const*)")]
// was: Ogre::MeshSerializerImpl::writeLodSummary(unsigned short,bool,Ogre::LodStrategy const*)
// IDA 0xd19dc0: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d19dc0() {
}

// 0xd19e00 — __ZN4Ogre18MeshSerializerImpl19writeLodUsageManualERKNS_12MeshLodUsageE
#[doc(alias = "Ogre::MeshSerializerImpl::writeLodUsageManual(Ogre::MeshLodUsage const&)")]
// was: Ogre::MeshSerializerImpl::writeLodUsageManual(Ogre::MeshLodUsage const&)
// IDA 0xd19e00: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d19e00() {
}

// 0xd19e50 — __ZN4Ogre18MeshSerializerImpl22writeLodUsageGeneratedEPKNS_4MeshERKNS_12MeshLodUsageEt
#[doc(alias = "Ogre::MeshSerializerImpl::writeLodUsageGenerated(Ogre::Mesh const*,Ogre::MeshLodUsage const&,unsigned short)")]
// was: Ogre::MeshSerializerImpl::writeLodUsageGenerated(Ogre::Mesh const*,Ogre::MeshLodUsage const&,unsigned short)
// IDA 0xd19e50: 319 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d19e50() {
}

// 0xd1a154 — __ZN4Ogre18MeshSerializerImpl15writeBoundsInfoEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::writeBoundsInfo(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl::writeBoundsInfo(Ogre::Mesh const*)
// IDA 0xd1a154: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1a154() {
}

// 0xd1a1c0 — __ZN4Ogre18MeshSerializerImpl14readBoundsInfoERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readBoundsInfo(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)")]
// was: Ogre::MeshSerializerImpl::readBoundsInfo(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)
// IDA 0xd1a1c0: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1a1c0() {
}

// 0xd1a340 — __ZN4Ogre18MeshSerializerImpl15readMeshLodInfoERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readMeshLodInfo(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)")]
// was: Ogre::MeshSerializerImpl::readMeshLodInfo(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)
// IDA 0xd1a340: 585 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1a340() {
}

// 0xd1a984 — __ZN4Ogre18MeshSerializerImpl22readMeshLodUsageManualERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEtRNS_12MeshLodUsageE
#[doc(alias = "Ogre::MeshSerializerImpl::readMeshLodUsageManual(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,unsigned short,Ogre::MeshLodUsage &)")]
// was: Ogre::MeshSerializerImpl::readMeshLodUsageManual(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,unsigned short,Ogre::MeshLodUsage &)
// IDA 0xd1a984: 287 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1a984() {
}

// 0xd1acc0 — __ZN4Ogre18MeshSerializerImpl25readMeshLodUsageGeneratedERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEtRNS_12MeshLodUsageE
#[doc(alias = "Ogre::MeshSerializerImpl::readMeshLodUsageGenerated(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,unsigned short,Ogre::MeshLodUsage &)")]
// was: Ogre::MeshSerializerImpl::readMeshLodUsageGenerated(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,unsigned short,Ogre::MeshLodUsage &)
// IDA 0xd1acc0: 566 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1acc0() {
}

// 0xd1b288 — __ZN4Ogre18MeshSerializerImpl20flipFromLittleEndianEPvmmRKSt4listINS_13VertexElementENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::MeshSerializerImpl::flipFromLittleEndian(void *,unsigned long,unsigned long,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::MeshSerializerImpl::flipFromLittleEndian(void *,unsigned long,unsigned long,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xd1b288: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1b288() {
}

// 0xd1b2b0 — __ZN4Ogre18MeshSerializerImpl18flipToLittleEndianEPvmmRKSt4listINS_13VertexElementENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::MeshSerializerImpl::flipToLittleEndian(void *,unsigned long,unsigned long,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::MeshSerializerImpl::flipToLittleEndian(void *,unsigned long,unsigned long,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xd1b2b0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1b2b0() {
}

// 0xd1b2d8 — __ZN4Ogre18MeshSerializerImpl10flipEndianEPvmmRKSt4listINS_13VertexElementENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::MeshSerializerImpl::flipEndian(void *,unsigned long,unsigned long,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::MeshSerializerImpl::flipEndian(void *,unsigned long,unsigned long,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xd1b2d8: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1b2d8() {
}

// 0xd1b358 — __ZN4Ogre18MeshSerializerImpl16calcEdgeListSizeEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::calcEdgeListSize(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl::calcEdgeListSize(Ogre::Mesh const*)
// IDA 0xd1b358: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1b358() {
}

// 0xd1b3b4 — __ZN4Ogre18MeshSerializerImpl19calcEdgeListLodSizeEPKNS_8EdgeDataEb
#[doc(alias = "Ogre::MeshSerializerImpl::calcEdgeListLodSize(Ogre::EdgeData const*,bool)")]
// was: Ogre::MeshSerializerImpl::calcEdgeListLodSize(Ogre::EdgeData const*,bool)
// IDA 0xd1b3b4: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1b3b4() {
}

// 0xd1b408 — __ZN4Ogre18MeshSerializerImpl17calcEdgeGroupSizeERKNS_8EdgeData9EdgeGroupE
#[doc(alias = "Ogre::MeshSerializerImpl::calcEdgeGroupSize(Ogre::EdgeData::EdgeGroup const&)")]
// was: Ogre::MeshSerializerImpl::calcEdgeGroupSize(Ogre::EdgeData::EdgeGroup const&)
// IDA 0xd1b408: 7 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1b408() {
}

// 0xd1b420 — __ZN4Ogre18MeshSerializerImpl13writeEdgeListEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::writeEdgeList(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl::writeEdgeList(Ogre::Mesh const*)
// IDA 0xd1b420: 233 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1b420() {
}

// 0xd1b67c — __ZN4Ogre18MeshSerializerImpl12readEdgeListERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readEdgeList(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)")]
// was: Ogre::MeshSerializerImpl::readEdgeList(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)
// IDA 0xd1b67c: 170 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1b67c() {
}

// 0xd1b84c — __ZN4Ogre18MeshSerializerImpl19readEdgeListLodInfoERNS_9SharedPtrINS_10DataStreamEEEPNS_8EdgeDataE
#[doc(alias = "Ogre::MeshSerializerImpl::readEdgeListLodInfo(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::EdgeData *)")]
// was: Ogre::MeshSerializerImpl::readEdgeListLodInfo(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::EdgeData *)
// IDA 0xd1b84c: 462 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1b84c() {
}

// 0xd1be0c — __ZN4Ogre18MeshSerializerImpl18calcAnimationsSizeEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::calcAnimationsSize(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl::calcAnimationsSize(Ogre::Mesh const*)
// IDA 0xd1be0c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1be0c() {
}

// 0xd1be60 — __ZN4Ogre18MeshSerializerImpl17calcAnimationSizeEPKNS_9AnimationE
#[doc(alias = "Ogre::MeshSerializerImpl::calcAnimationSize(Ogre::Animation const*)")]
// was: Ogre::MeshSerializerImpl::calcAnimationSize(Ogre::Animation const*)
// IDA 0xd1be60: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1be60() {
}

// 0xd1beac — __ZN4Ogre18MeshSerializerImpl22calcAnimationTrackSizeEPKNS_20VertexAnimationTrackE
#[doc(alias = "Ogre::MeshSerializerImpl::calcAnimationTrackSize(Ogre::VertexAnimationTrack const*)")]
// was: Ogre::MeshSerializerImpl::calcAnimationTrackSize(Ogre::VertexAnimationTrack const*)
// IDA 0xd1beac: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1beac() {
}

// 0xd1bf34 — __ZN4Ogre18MeshSerializerImpl21calcMorphKeyframeSizeEPKNS_19VertexMorphKeyFrameEm
#[doc(alias = "Ogre::MeshSerializerImpl::calcMorphKeyframeSize(Ogre::VertexMorphKeyFrame const*,unsigned long)")]
// was: Ogre::MeshSerializerImpl::calcMorphKeyframeSize(Ogre::VertexMorphKeyFrame const*,unsigned long)
// IDA 0xd1bf34: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1bf34() {
}

// 0xd1bf54 — __ZN4Ogre18MeshSerializerImpl20calcPoseKeyframeSizeEPKNS_18VertexPoseKeyFrameE
#[doc(alias = "Ogre::MeshSerializerImpl::calcPoseKeyframeSize(Ogre::VertexPoseKeyFrame const*)")]
// was: Ogre::MeshSerializerImpl::calcPoseKeyframeSize(Ogre::VertexPoseKeyFrame const*)
// IDA 0xd1bf54: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1bf54() {
}

// 0xd1bf78 — __ZN4Ogre18MeshSerializerImpl27calcPoseKeyframePoseRefSizeEv
#[doc(alias = "Ogre::MeshSerializerImpl::calcPoseKeyframePoseRefSize(void)")]
// was: Ogre::MeshSerializerImpl::calcPoseKeyframePoseRefSize(void)
// IDA 0xd1bf78: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1bf78() {
}

// 0xd1bf7c — __ZN4Ogre18MeshSerializerImpl13calcPosesSizeEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::calcPosesSize(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl::calcPosesSize(Ogre::Mesh const*)
// IDA 0xd1bf7c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1bf7c() {
}

// 0xd1bfb8 — __ZN4Ogre18MeshSerializerImpl12calcPoseSizeEPKNS_4PoseE
#[doc(alias = "Ogre::MeshSerializerImpl::calcPoseSize(Ogre::Pose const*)")]
// was: Ogre::MeshSerializerImpl::calcPoseSize(Ogre::Pose const*)
// IDA 0xd1bfb8: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1bfb8() {
}

// 0xd1bfd4 — __ZN4Ogre18MeshSerializerImpl18calcPoseVertexSizeEPKNS_4PoseE
#[doc(alias = "Ogre::MeshSerializerImpl::calcPoseVertexSize(Ogre::Pose const*)")]
// was: Ogre::MeshSerializerImpl::calcPoseVertexSize(Ogre::Pose const*)
// IDA 0xd1bfd4: 6 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1bfd4() {
}

// 0xd1bfe0 — __ZN4Ogre18MeshSerializerImpl10writePosesEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::writePoses(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl::writePoses(Ogre::Mesh const*)
// IDA 0xd1bfe0: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1bfe0() {
}

// 0xd1c02c — __ZN4Ogre18MeshSerializerImpl9writePoseEPKNS_4PoseE
#[doc(alias = "Ogre::MeshSerializerImpl::writePose(Ogre::Pose const*)")]
// was: Ogre::MeshSerializerImpl::writePose(Ogre::Pose const*)
// IDA 0xd1c02c: 96 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1c02c() {
}

// 0xd1c120 — __ZN4Ogre18MeshSerializerImpl15writeAnimationsEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::writeAnimations(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl::writeAnimations(Ogre::Mesh const*)
// IDA 0xd1c120: 256 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1c120() {
}

// 0xd1c3cc — __ZN4Ogre18MeshSerializerImpl14writeAnimationEPKNS_9AnimationE
#[doc(alias = "Ogre::MeshSerializerImpl::writeAnimation(Ogre::Animation const*)")]
// was: Ogre::MeshSerializerImpl::writeAnimation(Ogre::Animation const*)
// IDA 0xd1c3cc: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1c3cc() {
}

// 0xd1c488 — __ZN4Ogre18MeshSerializerImpl19writeAnimationTrackEPKNS_20VertexAnimationTrackE
#[doc(alias = "Ogre::MeshSerializerImpl::writeAnimationTrack(Ogre::VertexAnimationTrack const*)")]
// was: Ogre::MeshSerializerImpl::writeAnimationTrack(Ogre::VertexAnimationTrack const*)
// IDA 0xd1c488: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1c488() {
}

// 0xd1c530 — __ZN4Ogre18MeshSerializerImpl18writeMorphKeyframeEPKNS_19VertexMorphKeyFrameEm
#[doc(alias = "Ogre::MeshSerializerImpl::writeMorphKeyframe(Ogre::VertexMorphKeyFrame const*,unsigned long)")]
// was: Ogre::MeshSerializerImpl::writeMorphKeyframe(Ogre::VertexMorphKeyFrame const*,unsigned long)
// IDA 0xd1c530: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1c530() {
}

// 0xd1c5cc — __ZN4Ogre18MeshSerializerImpl17writePoseKeyframeEPKNS_18VertexPoseKeyFrameE
#[doc(alias = "Ogre::MeshSerializerImpl::writePoseKeyframe(Ogre::VertexPoseKeyFrame const*)")]
// was: Ogre::MeshSerializerImpl::writePoseKeyframe(Ogre::VertexPoseKeyFrame const*)
// IDA 0xd1c5cc: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1c5cc() {
}

// 0xd1c634 — __ZN4Ogre18MeshSerializerImpl24writePoseKeyframePoseRefERKNS_18VertexPoseKeyFrame7PoseRefE
#[doc(alias = "Ogre::MeshSerializerImpl::writePoseKeyframePoseRef(Ogre::VertexPoseKeyFrame::PoseRef const&)")]
// was: Ogre::MeshSerializerImpl::writePoseKeyframePoseRef(Ogre::VertexPoseKeyFrame::PoseRef const&)
// IDA 0xd1c634: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1c634() {
}

// 0xd1c668 — __ZN4Ogre18MeshSerializerImpl9readPosesERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readPoses(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)")]
// was: Ogre::MeshSerializerImpl::readPoses(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)
// IDA 0xd1c668: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1c668() {
}

// 0xd1c6dc — __ZN4Ogre18MeshSerializerImpl8readPoseERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readPose(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)")]
// was: Ogre::MeshSerializerImpl::readPose(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)
// IDA 0xd1c6dc: 221 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1c6dc() {
}

// 0xd1c918 — __ZN4Ogre18MeshSerializerImpl14readAnimationsERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readAnimations(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)")]
// was: Ogre::MeshSerializerImpl::readAnimations(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)
// IDA 0xd1c918: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1c918() {
}

// 0xd1c98c — __ZN4Ogre18MeshSerializerImpl13readAnimationERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readAnimation(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)")]
// was: Ogre::MeshSerializerImpl::readAnimation(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)
// IDA 0xd1c98c: 274 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1c98c() {
}

// 0xd1cc5c — __ZN4Ogre18MeshSerializerImpl18readAnimationTrackERNS_9SharedPtrINS_10DataStreamEEEPNS_9AnimationEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readAnimationTrack(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Animation *,Ogre::Mesh *)")]
// was: Ogre::MeshSerializerImpl::readAnimationTrack(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Animation *,Ogre::Mesh *)
// IDA 0xd1cc5c: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1cc5c() {
}

// 0xd1cd34 — __ZN4Ogre18MeshSerializerImpl17readMorphKeyFrameERNS_9SharedPtrINS_10DataStreamEEEPNS_20VertexAnimationTrackE
#[doc(alias = "Ogre::MeshSerializerImpl::readMorphKeyFrame(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::VertexAnimationTrack *)")]
// was: Ogre::MeshSerializerImpl::readMorphKeyFrame(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::VertexAnimationTrack *)
// IDA 0xd1cd34: 200 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1cd34() {
}

// 0xd1cf34 — __ZN4Ogre18MeshSerializerImpl16readPoseKeyFrameERNS_9SharedPtrINS_10DataStreamEEEPNS_20VertexAnimationTrackE
#[doc(alias = "Ogre::MeshSerializerImpl::readPoseKeyFrame(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::VertexAnimationTrack *)")]
// was: Ogre::MeshSerializerImpl::readPoseKeyFrame(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::VertexAnimationTrack *)
// IDA 0xd1cf34: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1cf34() {
}

// 0xd1cff0 — __ZN4Ogre18MeshSerializerImpl12readExtremesERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::readExtremes(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)")]
// was: Ogre::MeshSerializerImpl::readExtremes(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)
// IDA 0xd1cff0: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1cff0() {
}

// 0xd1d0ac — __ZN4Ogre24MeshSerializerImpl_v1_41C1Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_41::MeshSerializerImpl_v1_41(void)")]
// was: Ogre::MeshSerializerImpl_v1_41::MeshSerializerImpl_v1_41(void)
// IDA 0xd1d0ac: 81 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1d0ac() {
}

// 0xd1d198 — __ZN4Ogre24MeshSerializerImpl_v1_41D0Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_41::~MeshSerializerImpl_v1_41()")]
// was: Ogre::MeshSerializerImpl_v1_41::~MeshSerializerImpl_v1_41()
// IDA 0xd1d198: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1d198() {
}

// 0xd1d228 — __ZN4Ogre24MeshSerializerImpl_v1_41D1Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_41::~MeshSerializerImpl_v1_41()")]
// was: Ogre::MeshSerializerImpl_v1_41::~MeshSerializerImpl_v1_41()
// IDA 0xd1d228: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1d228() {
}

// 0xd1d234 — __ZN4Ogre24MeshSerializerImpl_v1_4118writeMorphKeyframeEPKNS_19VertexMorphKeyFrameEm
#[doc(alias = "Ogre::MeshSerializerImpl_v1_41::writeMorphKeyframe(Ogre::VertexMorphKeyFrame const*,unsigned long)")]
// was: Ogre::MeshSerializerImpl_v1_41::writeMorphKeyframe(Ogre::VertexMorphKeyFrame const*,unsigned long)
// IDA 0xd1d234: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1d234() {
}

// 0xd1d2a0 — __ZN4Ogre24MeshSerializerImpl_v1_4117readMorphKeyFrameERNS_9SharedPtrINS_10DataStreamEEEPNS_20VertexAnimationTrackE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_41::readMorphKeyFrame(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::VertexAnimationTrack *)")]
// was: Ogre::MeshSerializerImpl_v1_41::readMorphKeyFrame(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::VertexAnimationTrack *)
// IDA 0xd1d2a0: 187 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1d2a0() {
}

// 0xd1d480 — __ZN4Ogre24MeshSerializerImpl_v1_419writePoseEPKNS_4PoseE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_41::writePose(Ogre::Pose const*)")]
// was: Ogre::MeshSerializerImpl_v1_41::writePose(Ogre::Pose const*)
// IDA 0xd1d480: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1d480() {
}

// 0xd1d520 — __ZN4Ogre24MeshSerializerImpl_v1_418readPoseERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_41::readPose(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)")]
// was: Ogre::MeshSerializerImpl_v1_41::readPose(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)
// IDA 0xd1d520: 196 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1d520() {
}

// 0xd1d720 — __ZN4Ogre24MeshSerializerImpl_v1_4112calcPoseSizeEPKNS_4PoseE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_41::calcPoseSize(Ogre::Pose const*)")]
// was: Ogre::MeshSerializerImpl_v1_41::calcPoseSize(Ogre::Pose const*)
// IDA 0xd1d720: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1d720() {
}

// 0xd1d734 — __ZN4Ogre24MeshSerializerImpl_v1_4121calcMorphKeyframeSizeEPKNS_19VertexMorphKeyFrameEm
#[doc(alias = "Ogre::MeshSerializerImpl_v1_41::calcMorphKeyframeSize(Ogre::VertexMorphKeyFrame const*,unsigned long)")]
// was: Ogre::MeshSerializerImpl_v1_41::calcMorphKeyframeSize(Ogre::VertexMorphKeyFrame const*,unsigned long)
// IDA 0xd1d734: 4 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1d734() {
}

// 0xd1d740 — __ZN4Ogre23MeshSerializerImpl_v1_4C1Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_4::MeshSerializerImpl_v1_4(void)")]
// was: Ogre::MeshSerializerImpl_v1_4::MeshSerializerImpl_v1_4(void)
// IDA 0xd1d740: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1d740() {
}

// 0xd1d858 — __ZN4Ogre23MeshSerializerImpl_v1_4D0Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_4::~MeshSerializerImpl_v1_4()")]
// was: Ogre::MeshSerializerImpl_v1_4::~MeshSerializerImpl_v1_4()
// IDA 0xd1d858: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1d858() {
}

// 0xd1d8e8 — __ZN4Ogre23MeshSerializerImpl_v1_4D1Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_4::~MeshSerializerImpl_v1_4()")]
// was: Ogre::MeshSerializerImpl_v1_4::~MeshSerializerImpl_v1_4()
// IDA 0xd1d8e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1d8e8() {
}

// 0xd1d8f4 — __ZN4Ogre23MeshSerializerImpl_v1_415writeLodSummaryEtbPKNS_11LodStrategyE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_4::writeLodSummary(unsigned short,bool,Ogre::LodStrategy const*)")]
// was: Ogre::MeshSerializerImpl_v1_4::writeLodSummary(unsigned short,bool,Ogre::LodStrategy const*)
// IDA 0xd1d8f4: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1d8f4() {
}

// 0xd1d92c — __ZN4Ogre23MeshSerializerImpl_v1_419writeLodUsageManualERKNS_12MeshLodUsageE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_4::writeLodUsageManual(Ogre::MeshLodUsage const&)")]
// was: Ogre::MeshSerializerImpl_v1_4::writeLodUsageManual(Ogre::MeshLodUsage const&)
// IDA 0xd1d92c: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1d92c() {
}

// 0xd1d97c — __ZN4Ogre23MeshSerializerImpl_v1_422writeLodUsageGeneratedEPKNS_4MeshERKNS_12MeshLodUsageEt
#[doc(alias = "Ogre::MeshSerializerImpl_v1_4::writeLodUsageGenerated(Ogre::Mesh const*,Ogre::MeshLodUsage const&,unsigned short)")]
// was: Ogre::MeshSerializerImpl_v1_4::writeLodUsageGenerated(Ogre::Mesh const*,Ogre::MeshLodUsage const&,unsigned short)
// IDA 0xd1d97c: 320 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1d97c() {
}

// 0xd1dc80 — __ZN4Ogre23MeshSerializerImpl_v1_415readMeshLodInfoERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_4::readMeshLodInfo(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)")]
// was: Ogre::MeshSerializerImpl_v1_4::readMeshLodInfo(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)
// IDA 0xd1dc80: 519 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1dc80() {
}

// 0xd1e1e4 — __ZN4Ogre23MeshSerializerImpl_v1_3C1Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_3::MeshSerializerImpl_v1_3(void)")]
// was: Ogre::MeshSerializerImpl_v1_3::MeshSerializerImpl_v1_3(void)
// IDA 0xd1e1e4: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1e1e4() {
}

// 0xd1e1f0 — __ZN4Ogre23MeshSerializerImpl_v1_3C2Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_3::MeshSerializerImpl_v1_3(void)")]
// was: Ogre::MeshSerializerImpl_v1_3::MeshSerializerImpl_v1_3(void)
// IDA 0xd1e1f0: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1e1f0() {
}

// 0xd1e330 — __ZN4Ogre23MeshSerializerImpl_v1_3D0Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_3::~MeshSerializerImpl_v1_3()")]
// was: Ogre::MeshSerializerImpl_v1_3::~MeshSerializerImpl_v1_3()
// IDA 0xd1e330: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1e330() {
}

// 0xd1e3c0 — __ZN4Ogre23MeshSerializerImpl_v1_3D1Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_3::~MeshSerializerImpl_v1_3()")]
// was: Ogre::MeshSerializerImpl_v1_3::~MeshSerializerImpl_v1_3()
// IDA 0xd1e3c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1e3c0() {
}

// 0xd1e3cc — __ZN4Ogre23MeshSerializerImpl_v1_319readEdgeListLodInfoERNS_9SharedPtrINS_10DataStreamEEEPNS_8EdgeDataE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_3::readEdgeListLodInfo(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::EdgeData *)")]
// was: Ogre::MeshSerializerImpl_v1_3::readEdgeListLodInfo(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::EdgeData *)
// IDA 0xd1e3cc: 448 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1e3cc() {
}

// 0xd1e970 — __ZN4Ogre23MeshSerializerImpl_v1_319reorganiseTrianglesEPNS_8EdgeDataE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_3::reorganiseTriangles(Ogre::EdgeData *)")]
// was: Ogre::MeshSerializerImpl_v1_3::reorganiseTriangles(Ogre::EdgeData *)
// IDA 0xd1e970: 351 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1e970() {
}

// 0xd1ecfc — __ZN4Ogre23MeshSerializerImpl_v1_313writeEdgeListEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_3::writeEdgeList(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl_v1_3::writeEdgeList(Ogre::Mesh const*)
// IDA 0xd1ecfc: 214 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1ecfc() {
}

// 0xd1ef1c — __ZN4Ogre23MeshSerializerImpl_v1_2C1Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_2::MeshSerializerImpl_v1_2(void)")]
// was: Ogre::MeshSerializerImpl_v1_2::MeshSerializerImpl_v1_2(void)
// IDA 0xd1ef1c: 65 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1ef1c() {
}

// 0xd1efe0 — __ZN4Ogre23MeshSerializerImpl_v1_2D0Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_2::~MeshSerializerImpl_v1_2()")]
// was: Ogre::MeshSerializerImpl_v1_2::~MeshSerializerImpl_v1_2()
// IDA 0xd1efe0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1efe0() {
}

// 0xd1f070 — __ZN4Ogre23MeshSerializerImpl_v1_2D1Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_2::~MeshSerializerImpl_v1_2()")]
// was: Ogre::MeshSerializerImpl_v1_2::~MeshSerializerImpl_v1_2()
// IDA 0xd1f070: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1f070() {
}

// 0xd1f07c — __ZN4Ogre23MeshSerializerImpl_v1_28readMeshERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_22MeshSerializerListenerE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_2::readMesh(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::MeshSerializerListener *)")]
// was: Ogre::MeshSerializerImpl_v1_2::readMesh(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::MeshSerializerListener *)
// IDA 0xd1f07c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1f07c() {
}

// 0xd1f090 — __ZN4Ogre23MeshSerializerImpl_v1_212readGeometryERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_10VertexDataE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_2::readGeometry(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)")]
// was: Ogre::MeshSerializerImpl_v1_2::readGeometry(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)
// IDA 0xd1f090: 110 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1f090() {
}

// 0xd1f1a0 — __ZN4Ogre23MeshSerializerImpl_v1_221readGeometryPositionsEtRNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_10VertexDataE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_2::readGeometryPositions(unsigned short,Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)")]
// was: Ogre::MeshSerializerImpl_v1_2::readGeometryPositions(unsigned short,Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)
// IDA 0xd1f1a0: 311 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1f1a0() {
}

// 0xd1f4a0 — __ZN4Ogre23MeshSerializerImpl_v1_219readGeometryNormalsEtRNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_10VertexDataE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_2::readGeometryNormals(unsigned short,Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)")]
// was: Ogre::MeshSerializerImpl_v1_2::readGeometryNormals(unsigned short,Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)
// IDA 0xd1f4a0: 311 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1f4a0() {
}

// 0xd1f79c — __ZN4Ogre23MeshSerializerImpl_v1_219readGeometryColoursEtRNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_10VertexDataE
#[doc(alias = "Ogre::MeshSerializerImpl_v1_2::readGeometryColours(unsigned short,Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)")]
// was: Ogre::MeshSerializerImpl_v1_2::readGeometryColours(unsigned short,Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *)
// IDA 0xd1f79c: 310 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1f79c() {
}

// 0xd1fa94 — __ZN4Ogre23MeshSerializerImpl_v1_221readGeometryTexCoordsEtRNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_10VertexDataEt
#[doc(alias = "Ogre::MeshSerializerImpl_v1_2::readGeometryTexCoords(unsigned short,Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *,unsigned short)")]
// was: Ogre::MeshSerializerImpl_v1_2::readGeometryTexCoords(unsigned short,Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *,unsigned short)
// IDA 0xd1fa94: 330 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1fa94() {
}

// 0xd1fdc0 — __ZN4Ogre23MeshSerializerImpl_v1_1C1Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_1::MeshSerializerImpl_v1_1(void)")]
// was: Ogre::MeshSerializerImpl_v1_1::MeshSerializerImpl_v1_1(void)
// IDA 0xd1fdc0: 81 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1fdc0() {
}

// 0xd1feac — __ZN4Ogre23MeshSerializerImpl_v1_1D0Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_1::~MeshSerializerImpl_v1_1()")]
// was: Ogre::MeshSerializerImpl_v1_1::~MeshSerializerImpl_v1_1()
// IDA 0xd1feac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1feac() {
}

// 0xd1ff3c — __ZN4Ogre23MeshSerializerImpl_v1_1D1Ev
#[doc(alias = "Ogre::MeshSerializerImpl_v1_1::~MeshSerializerImpl_v1_1()")]
// was: Ogre::MeshSerializerImpl_v1_1::~MeshSerializerImpl_v1_1()
// IDA 0xd1ff3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1ff3c() {
}

// 0xd1ff48 — __ZN4Ogre23MeshSerializerImpl_v1_121readGeometryTexCoordsEtRNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_10VertexDataEt
#[doc(alias = "Ogre::MeshSerializerImpl_v1_1::readGeometryTexCoords(unsigned short,Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *,unsigned short)")]
// was: Ogre::MeshSerializerImpl_v1_1::readGeometryTexCoords(unsigned short,Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::VertexData *,unsigned short)
// IDA 0xd1ff48: 350 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1ff48() {
}

// 0xd202a8 — __ZNSt3mapItSsSt4lessItEN4Ogre12STLAllocatorISt4pairIKtSsENS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEixERS5_
#[doc(alias = "std::map<unsigned short,std::string,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](unsigned short const&)")]
// was: std::map<unsigned short,std::string,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](unsigned short const&)
// IDA 0xd202a8: 167 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d202a8() {
}

// 0xd20480 — __ZNSt6vectorIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_
#[doc(alias = "std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::MeshLodUsage const&)")]
// was: std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::MeshLodUsage const&)
// IDA 0xd20480: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_d20480() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0xd205f4 — __ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
#[doc(alias = "std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::EdgeData::Edge*,std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::EdgeData::Edge const&)")]
// was: std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::EdgeData::Edge*,std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::EdgeData::Edge const&)
// IDA 0xd205f4: 235 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d205f4() {
}

// 0xd208c8 — __ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_
#[doc(alias = "std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector4*,std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>>,unsigned long,Ogre::Vector4 const&)")]
// was: std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector4*,std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>>,unsigned long,Ogre::Vector4 const&)
// IDA 0xd208c8: 199 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d208c8() {
}

// 0xd20b04 — __ZNSt6vectorIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
#[doc(alias = "std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::EdgeData::Triangle*,std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::EdgeData::Triangle const&)")]
// was: std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::EdgeData::Triangle*,std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::EdgeData::Triangle const&)
// IDA 0xd20b04: 215 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d20b04() {
}

// 0xd20d84 — __ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,std::string>>,std::pair<unsigned short const,std::string> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,std::string>>,std::pair<unsigned short const,std::string> const&)
// IDA 0xd20d84: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d20d84() {
}

// 0xd20e38 — __ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS2_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,std::string> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,std::string> const&)
// IDA 0xd20e38: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d20e38() {
}

// 0xd20f70 — __ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::string> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::string> const&)
// IDA 0xd20f70: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d20f70() {
}

// 0xd20fdc — __ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xd20fdc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d20fdc() {
}

// 0xd20fe0 — __ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xd20fe0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d20fe0() {
}

// 0xd20fec — __ZNSt8_Rb_treeItSt4pairIKtSsESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::string>> *)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::string>,std::_Select1st<std::pair<unsigned short const,std::string>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::string>> *)
// IDA 0xd20fec: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d20fec() {
}

// 0xd21098 — __ZN4Ogre13MovableObjectC2Ev
#[doc(alias = "Ogre::MovableObject::MovableObject(void)")]
// was: Ogre::MovableObject::MovableObject(void)
// IDA 0xd21098: 258 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d21098() {
}

// 0xd21384 — __ZN4Ogre13MovableObjectC2ERKSs
#[doc(alias = "Ogre::MovableObject::MovableObject(std::string const&)")]
// was: Ogre::MovableObject::MovableObject(std::string const&)
// IDA 0xd21384: 265 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d21384() {
}

// 0xd2166c — __ZN4Ogre13MovableObjectD0Ev
#[doc(alias = "Ogre::MovableObject::~MovableObject()")]
// was: Ogre::MovableObject::~MovableObject()
// IDA 0xd2166c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d2166c() {
}

// 0xd216fc — __ZN4Ogre13MovableObjectD1Ev
#[doc(alias = "Ogre::MovableObject::~MovableObject()")]
// was: Ogre::MovableObject::~MovableObject()
// IDA 0xd216fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d216fc() {
}

// 0xd21708 — __ZThn4_N4Ogre13MovableObjectD0Ev
#[doc(alias = "non-virtual thunk toOgre::MovableObject::~MovableObject()")]
// was: non-virtual thunk to Ogre::MovableObject::~MovableObject()
// IDA 0xd21708: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d21708() {
}

// 0xd2179c — __ZN4Ogre13MovableObjectD2Ev
#[doc(alias = "Ogre::MovableObject::~MovableObject()")]
// was: Ogre::MovableObject::~MovableObject()
// IDA 0xd2179c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d2179c() {
}

// 0xd21990 — __ZThn4_N4Ogre13MovableObjectD1Ev
#[doc(alias = "non-virtual thunk toOgre::MovableObject::~MovableObject()")]
// was: non-virtual thunk to Ogre::MovableObject::~MovableObject()
// IDA 0xd21990: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d21990() {
}

// 0xd2199c — __ZN4Ogre13MovableObject15_notifyAttachedEPNS_4NodeEb
#[doc(alias = "Ogre::MovableObject::_notifyAttached(Ogre::Node *,bool)")]
// was: Ogre::MovableObject::_notifyAttached(Ogre::Node *,bool)
// IDA 0xd2199c: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2199c() {
}

// 0xd219d0 — __ZNK4Ogre13MovableObject13getParentNodeEv
#[doc(alias = "Ogre::MovableObject::getParentNode(void)const")]
// was: Ogre::MovableObject::getParentNode(void)const
// IDA 0xd219d0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d219d0() {
}