//! rendering shard rend_wd_watchdog14 — 120 stubs 0x80c65c..0x81227c EA-sorted asc gap filler not yet in crates/rendering/src (Ogre/G3D/Render filtered exhausted -> global gap filler distinct per crate)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in crates/rendering/src — next 120 uncovered sorted asc after 0x80c4e8
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x80c65c — __ZThn4_N3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi4ES8_E4callEbSsS7_i
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi4ES8_E4callEbSsS7_i")]
pub fn stub_80c65c() -> ! {
    todo!("0x80c65c non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0x80c664 — __ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi4ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi4ES8_ED1Ev")]
pub fn stub_80c664() -> ! {
    todo!("0x80c664 rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")
}

// 0x80c774 — __ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi4ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi4ES8_ED0Ev")]
pub fn stub_80c774() -> ! {
    todo!("0x80c774 rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")
}

// 0x80c8a4 — __ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE13assign_to_ownERKS5_
#[doc(alias = "boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to_own(boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int> const&)")]
#[doc(alias = "__ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE13assign_to_ownERKS5_")]
pub fn stub_80c8a4() -> ! {
    todo!("0x80c8a4 boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to_own(boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int> const&)")
}

// 0x80c8d4 — __ZN3RBX10Reflection9EventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::EventDesc(rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_80c8d4() -> ! {
    todo!("0x80c8d4 RBX::Reflection::EventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::EventDesc(rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x80cba0 — __ZN3RBX10Reflection9EventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev")]
pub fn stub_80cba0() -> ! {
    todo!("0x80cba0 RBX::Reflection::EventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::~EventDesc()")
}

// 0x80cbc4 — __ZN3RBX10Reflection9EventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev")]
pub fn stub_80cbc4() -> ! {
    todo!("0x80cbc4 RBX::Reflection::EventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::~EventDesc()")
}

// 0x80cc78 — __ZN3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::~RemoteEventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEED0Ev")]
pub fn stub_80cc78() -> ! {
    todo!("0x80cc78 RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::~RemoteEventDesc()")
}

// 0x80cd2c — __ZNK3RBX10Reflection13EventDescImplILi3ENS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
pub fn stub_80cd2c() -> ! {
    todo!("0x80cd2c RBX::Reflection::EventDescImpl<3,RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x80ce90 — __ZNK3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::isScriptable(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE12isScriptableEv")]
pub fn stub_80ce90() -> ! {
    todo!("0x80ce90 RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::isScriptable(void)const")
}

// 0x80ce98 — __ZNK3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::isBroadcast(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE11isBroadcastEv")]
pub fn stub_80ce98() -> ! {
    todo!("0x80ce98 RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::isBroadcast(void)const")
}

// 0x80cea0 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
pub fn stub_80cea0() -> ! {
    todo!("0x80cea0 RBX::Reflection::EventDescImpl<3,RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x80d0b8 — __ZNK3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE")]
pub fn stub_80d0b8() -> ! {
    todo!("0x80d0b8 RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x80d0c8 — __ZNK3RBX10Reflection13EventDescBaseINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_80d0c8() -> ! {
    todo!("0x80d0c8 RBX::Reflection::EventDescBase<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x80d0dc — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS1_8InstanceEEERKiNS6_IS3_EENS_3argILi1EEENSE_ILi2EEENSE_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISK_T0_T1_T2_T3_EENSI_9list_av_4IT4_T5_T6_T7_E4typeEEEMSN_FSK_SO_SP_SQ_EST_SU_SV_SW_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS1_8InstanceEEERKiNS6_IS3_EENS_3argILi1EEENSE_ILi2EEENSE_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISK_T0_T1_T2_T3_EENSI_9list_av_4IT4_T5_T6_T7_E4typeEEEMSN_FSK_SO_SP_SQ_EST_SU_SV_SW_")]
pub fn stub_80d0dc() -> ! {
    todo!("0x80d0dc boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0x80d1f8 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3ISsN5boost10shared_ptrINS_8InstanceEEEiEEvRKT_RKT0_RKT1_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<std::string,rbx_core::SharedPtr<RBX::Instance>,int>(std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&)")]
#[doc(alias = "__ZN3RBX10Reflection18GenericSlotWrapper8execute3ISsN5boost10shared_ptrINS_8InstanceEEEiEEvRKT_RKT0_RKT1_")]
pub fn stub_80d1f8() -> ! {
    todo!("0x80d1f8 void RBX::Reflection::GenericSlotWrapper::execute3<std::string,rbx_core::SharedPtr<RBX::Instance>,int>(std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&)")
}

// 0x80d384 — __ZN5boost8functionIFvSsNS_10shared_ptrIN3RBX8InstanceEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsRKS4_RKiEENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvSsNS_10shared_ptrIN3RBX8InstanceEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsRKS4_RKiEENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvSsNS_10shared_ptrIN3RBX8InstanceEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsRKS4_RKiEENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE")]
pub fn stub_80d384() -> ! {
    todo!("0x80d384 __ZN5boost8functionIFvSsNS_10shared_ptrIN3RBX8InstanceEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsRKS4_RKiEENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE")
}

// 0x80d468 — __ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsRKS4_RKiEENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsRKS4_RKiEENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsRKS4_RKiEENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_80d468() -> ! {
    todo!("0x80d468 __ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsRKS4_RKiEENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")
}

// 0x80d550 — __ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsRKS4_RKiEENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_
#[doc(alias = "void boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
#[doc(alias = "__ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsRKS4_RKiEENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_")]
pub fn stub_80d550() -> ! {
    todo!("0x80d550 void boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")
}

// 0x80d648 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE")]
pub fn stub_80d648() -> ! {
    todo!("0x80d648 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x80d664 — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEvSsSE_iE6invokeERNS1_15function_bufferESsSE_i
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::invoke(boost::detail::function::function_buffer &,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEvSsSE_iE6invokeERNS1_15function_bufferESsSE_i")]
pub fn stub_80d664() -> ! {
    todo!("0x80d664 boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::invoke(boost::detail::function::function_buffer &,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0x80d688 — __ZNK5boost6detail8function13basic_vtable3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsRKS6_RKiEENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsRKS6_RKiEENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_80d688() -> ! {
    todo!("0x80d688 bool boost::detail::function::basic_vtable3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")
}

// 0x80d770 — __ZNK5boost6detail8function13basic_vtable3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsRKS6_RKiEENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsRKS6_RKiEENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_80d770() -> ! {
    todo!("0x80d770 bool boost::detail::function::basic_vtable3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x80d854 — __ZNK5boost6detail8function13basic_vtable3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsRKS6_RKiEENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsRKS6_RKiEENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_80d854() -> ! {
    todo!("0x80d854 void boost::detail::function::basic_vtable3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x80d928 — __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKSsRKNS3_INS4_8InstanceEEERKiEENS0_5list3IRSsRSK_RiEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&> &,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKSsRKNS3_INS4_8InstanceEEERKiEENS0_5list3IRSsRSK_RiEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_80d928() -> ! {
    todo!("0x80d928 void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&> &,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")
}

// 0x80d950 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_80d950() -> ! {
    todo!("0x80d950 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x80daa8 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>(boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_")]
pub fn stub_80daa8() -> ! {
    todo!("0x80daa8 rbx::signals::connection rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>(boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&)")
}

// 0x80db9c — __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi3ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*>(boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&,rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi3ES8_EC2IPS9_EERKSC_T_")]
pub fn stub_80db9c() -> ! {
    todo!("0x80db9c rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*>(boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&,rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*)")
}

// 0x80dc98 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable_slot<boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13callable_slotINS2_8functionIS7_EEED1Ev")]
pub fn stub_80dc98() -> ! {
    todo!("0x80dc98 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable_slot<boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::~callable_slot()")
}

// 0x80dda8 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13callable_slotINS2_8functionIS7_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable_slot<boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13callable_slotINS2_8functionIS7_EEED0Ev")]
pub fn stub_80dda8() -> ! {
    todo!("0x80dda8 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable_slot<boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::~callable_slot()")
}

// 0x80ded8 — __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi3ES8_E4callESsS7_i
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi3ES8_E4callESsS7_i")]
pub fn stub_80ded8() -> ! {
    todo!("0x80ded8 rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0x80e048 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi3ES8_E4callESsS7_i
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi3ES8_E4callESsS7_i")]
pub fn stub_80e048() -> ! {
    todo!("0x80e048 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0x80e050 — __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi3ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi3ES8_ED1Ev")]
pub fn stub_80e050() -> ! {
    todo!("0x80e050 rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")
}

// 0x80e160 — __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi3ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi3ES8_ED0Ev")]
pub fn stub_80e160() -> ! {
    todo!("0x80e160 rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")
}

// 0x80e290 — __ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE13assign_to_ownERKS5_
#[doc(alias = "boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to_own(boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int> const&)")]
#[doc(alias = "__ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE13assign_to_ownERKS5_")]
pub fn stub_80e290() -> ! {
    todo!("0x80e290 boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to_own(boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int> const&)")
}

// 0x80e2c0 — __ZN3RBX10Reflection9EventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::EventDesc(rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_80e2c0() -> ! {
    todo!("0x80e2c0 RBX::Reflection::EventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::EventDesc(rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x80e51c — __ZN3RBX10Reflection9EventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev")]
pub fn stub_80e51c() -> ! {
    todo!("0x80e51c RBX::Reflection::EventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::~EventDesc()")
}

// 0x80e540 — __ZN3RBX10Reflection9EventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev")]
pub fn stub_80e540() -> ! {
    todo!("0x80e540 RBX::Reflection::EventDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::~EventDesc()")
}

// 0x80e5f4 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFbSsELi1EEC2EMS2_FbSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,bool ()(std::string),1>::BoundFuncDesc(bool (RBX::TestService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFbSsELi1EEC2EMS2_FbSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_80e5f4() -> ! {
    todo!("0x80e5f4 RBX::Reflection::BoundFuncDesc<RBX::TestService,bool ()(std::string),1>::BoundFuncDesc(bool (RBX::TestService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x80e770 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFbSsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,bool ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFbSsELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_80e770() -> ! {
    todo!("0x80e770 RBX::Reflection::BoundFuncDesc<RBX::TestService,bool ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x80e7a0 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFbSsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,bool ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFbSsELi1EED0Ev")]
pub fn stub_80e7a0() -> ! {
    todo!("0x80e7a0 RBX::Reflection::BoundFuncDesc<RBX::TestService,bool ()(std::string),1>::~BoundFuncDesc()")
}

// 0x80e8a8 — __ZNK3RBX10Reflection13BoundFuncDescINS_11TestServiceEFbSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,bool ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_11TestServiceEFbSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_80e8a8() -> ! {
    todo!("0x80e8a8 RBX::Reflection::BoundFuncDesc<RBX::TestService,bool ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x80e9e8 — __ZN3RBX10Reflection11Call1HelperINS_11TestServiceEMS2_FbSsESsbE4callEPS2_S4_RNS0_7VariantERKSs
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::TestService,bool (RBX::TestService::*)(std::string),std::string,bool>::call(RBX::TestService*,bool (RBX::TestService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_11TestServiceEMS2_FbSsESsbE4callEPS2_S4_RNS0_7VariantERKSs")]
pub fn stub_80e9e8() -> ! {
    todo!("0x80e9e8 RBX::Reflection::Call1Helper<RBX::TestService,bool (RBX::TestService::*)(std::string),std::string,bool>::call(RBX::TestService*,bool (RBX::TestService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0x80eb3c — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string),1>::BoundFuncDesc(void (RBX::TestService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_80eb3c() -> ! {
    todo!("0x80eb3c RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string),1>::BoundFuncDesc(void (RBX::TestService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x80ecb8 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_80ecb8() -> ! {
    todo!("0x80ecb8 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x80ece8 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsELi1EED0Ev")]
pub fn stub_80ece8() -> ! {
    todo!("0x80ece8 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string),1>::~BoundFuncDesc()")
}

// 0x80edf0 — __ZNK3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_80edf0() -> ! {
    todo!("0x80edf0 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x80ef2c — __ZN3RBX10Reflection11Call1HelperINS_11TestServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::TestService,void (RBX::TestService::*)(std::string),std::string,void>::call(RBX::TestService*,void (RBX::TestService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_11TestServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs")]
pub fn stub_80ef2c() -> ! {
    todo!("0x80ef2c RBX::Reflection::Call1Helper<RBX::TestService,void (RBX::TestService::*)(std::string),std::string,void>::call(RBX::TestService*,void (RBX::TestService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0x80f05c — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EEC2EMS2_FSA_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::TestService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EEC2EMS2_FSA_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_80f05c() -> ! {
    todo!("0x80f05c RBX::Reflection::BoundFuncDesc<RBX::TestService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::TestService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x80f160 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED0Ev")]
pub fn stub_80f160() -> ! {
    todo!("0x80f160 RBX::Reflection::BoundFuncDesc<RBX::TestService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")
}

// 0x80f214 — __ZNK3RBX10Reflection13BoundFuncDescINS_11TestServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_11TestServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_80f214() -> ! {
    todo!("0x80f214 RBX::Reflection::BoundFuncDesc<RBX::TestService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x80f238 — __ZN3RBX10Reflection11Call0HelperINS_11TestServiceEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::TestService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::TestService::*)(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::TestService*,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::TestService::*)(void),RBX::Reflection::Variant&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_11TestServiceEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_")]
pub fn stub_80f238() -> ! {
    todo!("0x80f238 RBX::Reflection::Call0Helper<RBX::TestService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::TestService::*)(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::TestService*,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::TestService::*)(void),RBX::Reflection::Variant&)")
}

// 0x80f320 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(void),0>::BoundFuncDesc(void (RBX::TestService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_80f320() -> ! {
    todo!("0x80f320 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(void),0>::BoundFuncDesc(void (RBX::TestService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x80f424 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvvELi0EED0Ev")]
pub fn stub_80f424() -> ! {
    todo!("0x80f424 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(void),0>::~BoundFuncDesc()")
}

// 0x80f4d8 — __ZNK3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_80f4d8() -> ! {
    todo!("0x80f4d8 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x80f4f8 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiELi4EEC2EMS2_FvbSsS6_iEPKcSC_SC_SC_S6_SC_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),4>::BoundFuncDesc(void (RBX::TestService::*)(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),char const*,char const*,char const*,char const*,rbx_core::SharedPtr<RBX::Instance>,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiELi4EEC2EMS2_FvbSsS6_iEPKcSC_SC_SC_S6_SC_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_80f4f8() -> ! {
    todo!("0x80f4f8 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),4>::BoundFuncDesc(void (RBX::TestService::*)(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),char const*,char const*,char const*,char const*,rbx_core::SharedPtr<RBX::Instance>,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x80f820 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiELi4EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_SA_SB_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiELi4EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_SA_SB_")]
pub fn stub_80f820() -> ! {
    todo!("0x80f820 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x80f8a0 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiELi4EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),4>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiELi4EED0Ev")]
pub fn stub_80f8a0() -> ! {
    todo!("0x80f8a0 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),4>::~BoundFuncDesc()")
}

// 0x80f940 — __ZNK3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiELi4EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiELi4EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_80f940() -> ! {
    todo!("0x80f940 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x80faf8 — __ZN3RBX10Reflection11Call4HelperINS_11TestServiceEMS2_FvbSsN5boost10shared_ptrINS_8InstanceEEEiEbSsS6_ivE4callEPS2_S8_RNS0_7VariantERKbRKSsRKS6_RKi
#[doc(alias = "RBX::Reflection::Call4Helper<RBX::TestService,void (RBX::TestService::*)(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int,void>::call(RBX::TestService*,void (RBX::TestService::*)(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),RBX::Reflection::Variant &,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call4HelperINS_11TestServiceEMS2_FvbSsN5boost10shared_ptrINS_8InstanceEEEiEbSsS6_ivE4callEPS2_S8_RNS0_7VariantERKbRKSsRKS6_RKi")]
pub fn stub_80faf8() -> ! {
    todo!("0x80faf8 RBX::Reflection::Call4Helper<RBX::TestService,void (RBX::TestService::*)(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int,void>::call(RBX::TestService*,void (RBX::TestService::*)(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),RBX::Reflection::Variant &,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&)")
}

// 0x80fc94 — __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrINS_8InstanceEEELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrIS7_EEPNS3_10disable_ifINS3_7is_sameIS7_NS4_IKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<RBX::Instance>,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<RBX::Instance>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrINS_8InstanceEEELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrIS7_EEPNS3_10disable_ifINS3_7is_sameIS7_NS4_IKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_80fc94() -> ! {
    todo!("0x80fc94 rbx_core::SharedPtr<RBX::Instance> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<RBX::Instance>,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<RBX::Instance>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x80fea8 — __ZN3RBX10Reflection9ArgHelper10try_objectILi3ENS_8InstanceEEEbRNS0_18FunctionDescriptor9ArgumentsERN5boost10shared_ptrIT0_EEPNS7_9enable_ifINS7_10is_base_ofINS0_13DescribedBaseES9_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_object<3,RBX::Instance>(RBX::Reflection::FunctionDescriptor::Arguments &,rbx_core::SharedPtr<RBX::Instance> &,boost::enable_if<boost::is_base_of<RBX::Reflection::DescribedBase,RBX::Instance>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper10try_objectILi3ENS_8InstanceEEEbRNS0_18FunctionDescriptor9ArgumentsERN5boost10shared_ptrIT0_EEPNS7_9enable_ifINS7_10is_base_ofINS0_13DescribedBaseES9_EEvE4typeE")]
pub fn stub_80fea8() -> ! {
    todo!("0x80fea8 bool RBX::Reflection::ArgHelper::try_object<3,RBX::Instance>(RBX::Reflection::FunctionDescriptor::Arguments &,rbx_core::SharedPtr<RBX::Instance> &,boost::enable_if<boost::is_base_of<RBX::Reflection::DescribedBase,RBX::Instance>,void>::type *)")
}

// 0x80ffc8 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiELi3EEC2EMS2_FvSsS6_iEPKcSC_SC_S6_SC_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),3>::BoundFuncDesc(void (RBX::TestService::*)(std::string,rbx_core::SharedPtr<RBX::Instance>,int),char const*,char const*,char const*,rbx_core::SharedPtr<RBX::Instance>,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiELi3EEC2EMS2_FvSsS6_iEPKcSC_SC_S6_SC_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_80ffc8() -> ! {
    todo!("0x80ffc8 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),3>::BoundFuncDesc(void (RBX::TestService::*)(std::string,rbx_core::SharedPtr<RBX::Instance>,int),char const*,char const*,char const*,rbx_core::SharedPtr<RBX::Instance>,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x81029c — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_")]
pub fn stub_81029c() -> ! {
    todo!("0x81029c RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x810304 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiELi3EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiELi3EED0Ev")]
pub fn stub_810304() -> ! {
    todo!("0x810304 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),3>::~BoundFuncDesc()")
}

// 0x810438 — __ZNK3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_810438() -> ! {
    todo!("0x810438 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x8105dc — __ZN3RBX10Reflection11Call3HelperINS_11TestServiceEMS2_FvSsN5boost10shared_ptrINS_8InstanceEEEiESsS6_ivE4callEPS2_S8_RNS0_7VariantERKSsRKS6_RKi
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::TestService,void (RBX::TestService::*)(std::string,rbx_core::SharedPtr<RBX::Instance>,int),std::string,rbx_core::SharedPtr<RBX::Instance>,int,void>::call(RBX::TestService*,void (RBX::TestService::*)(std::string,rbx_core::SharedPtr<RBX::Instance>,int),RBX::Reflection::Variant &,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call3HelperINS_11TestServiceEMS2_FvSsN5boost10shared_ptrINS_8InstanceEEEiESsS6_ivE4callEPS2_S8_RNS0_7VariantERKSsRKS6_RKi")]
pub fn stub_8105dc() -> ! {
    todo!("0x8105dc RBX::Reflection::Call3Helper<RBX::TestService,void (RBX::TestService::*)(std::string,rbx_core::SharedPtr<RBX::Instance>,int),std::string,rbx_core::SharedPtr<RBX::Instance>,int,void>::call(RBX::TestService*,void (RBX::TestService::*)(std::string,rbx_core::SharedPtr<RBX::Instance>,int),RBX::Reflection::Variant &,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&)")
}

// 0x810768 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11TestServiceEFvvEvLi0EEC2EMS2_FvN5boost8functionIS3_EENS6_IFvSsEEEEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::TestService,void ()(void),void,0>::BoundYieldFuncDesc(void (RBX::TestService::*)(boost::function<void ()(void)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_11TestServiceEFvvEvLi0EEC2EMS2_FvN5boost8functionIS3_EENS6_IFvSsEEEEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_810768() -> ! {
    todo!("0x810768 RBX::Reflection::BoundYieldFuncDesc<RBX::TestService,void ()(void),void,0>::BoundYieldFuncDesc(void (RBX::TestService::*)(boost::function<void ()(void)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x81086c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11TestServiceEFvvEvLi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::TestService,void ()(void),void,0>::~BoundYieldFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_11TestServiceEFvvEvLi0EED0Ev")]
pub fn stub_81086c() -> ! {
    todo!("0x81086c RBX::Reflection::BoundYieldFuncDesc<RBX::TestService,void ()(void),void,0>::~BoundYieldFuncDesc()")
}

// 0x810920 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_11TestServiceEFvvEvLi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::TestService,void ()(void),void,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
#[doc(alias = "__ZNK3RBX10Reflection18BoundYieldFuncDescINS_11TestServiceEFvvEvLi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE")]
pub fn stub_810920() -> ! {
    todo!("0x810920 RBX::Reflection::BoundYieldFuncDesc<RBX::TestService,void ()(void),void,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x810b50 — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE0EEC2INS_11TestServiceEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)0>::BoundProp<RBX::TestService>(char const*,char const*,int RBX::TestService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE0EEC2INS_11TestServiceEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_810b50() -> ! {
    todo!("0x810b50 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)0>::BoundProp<RBX::TestService>(char const*,char const*,int RBX::TestService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x810ce0 — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE0EED0Ev
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)0>::~BoundProp()")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE0EED0Ev")]
pub fn stub_810ce0() -> ! {
    todo!("0x810ce0 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)0>::~BoundProp()")
}

// 0x810d0c — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE0EE15BoundPropGetSetINS_11TestServiceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)0>::BoundPropGetSet<RBX::TestService>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE0EE15BoundPropGetSetINS_11TestServiceEE10isReadOnlyEv")]
pub fn stub_810d0c() -> ! {
    todo!("0x810d0c RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)0>::BoundPropGetSet<RBX::TestService>::isReadOnly(void)const")
}

// 0x810d10 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE0EE15BoundPropGetSetINS_11TestServiceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)0>::BoundPropGetSet<RBX::TestService>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE0EE15BoundPropGetSetINS_11TestServiceEE11isWriteOnlyEv")]
pub fn stub_810d10() -> ! {
    todo!("0x810d10 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)0>::BoundPropGetSet<RBX::TestService>::isWriteOnly(void)const")
}

// 0x810d14 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE0EE15BoundPropGetSetINS_11TestServiceEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)0>::BoundPropGetSet<RBX::TestService>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE0EE15BoundPropGetSetINS_11TestServiceEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_810d14() -> ! {
    todo!("0x810d14 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)0>::BoundPropGetSet<RBX::TestService>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x810d20 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE0EE15BoundPropGetSetINS_11TestServiceEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)0>::BoundPropGetSet<RBX::TestService>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE0EE15BoundPropGetSetINS_11TestServiceEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_810d20() -> ! {
    todo!("0x810d20 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)0>::BoundPropGetSet<RBX::TestService>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x810e40 — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_11TestServiceEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::TestService>(char const*,char const*,int RBX::TestService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_11TestServiceEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_810e40() -> ! {
    todo!("0x810e40 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::TestService>(char const*,char const*,int RBX::TestService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x810fd0 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE10isReadOnlyEv")]
pub fn stub_810fd0() -> ! {
    todo!("0x810fd0 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isReadOnly(void)const")
}

// 0x810fd4 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE11isWriteOnlyEv")]
pub fn stub_810fd4() -> ! {
    todo!("0x810fd4 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isWriteOnly(void)const")
}

// 0x810fd8 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_810fd8() -> ! {
    todo!("0x810fd8 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x810fe4 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_810fe4() -> ! {
    todo!("0x810fe4 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x811034 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_11TestServiceEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::TestService>(char const*,char const*,bool RBX::TestService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_11TestServiceEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_811034() -> ! {
    todo!("0x811034 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::TestService>(char const*,char const*,bool RBX::TestService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x8111c4 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE10isReadOnlyEv")]
pub fn stub_8111c4() -> ! {
    todo!("0x8111c4 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isReadOnly(void)const")
}

// 0x8111c8 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE11isWriteOnlyEv")]
pub fn stub_8111c8() -> ! {
    todo!("0x8111c8 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isWriteOnly(void)const")
}

// 0x8111cc — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_8111cc() -> ! {
    todo!("0x8111cc RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x8111d8 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_8111d8() -> ! {
    todo!("0x8111d8 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x811228 — __ZN3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EEC2INS_11TestServiceEEEPKcS7_MT_dNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundProp<RBX::TestService>(char const*,char const*,double RBX::TestService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EEC2INS_11TestServiceEEEPKcS7_MT_dNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_811228() -> ! {
    todo!("0x811228 RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundProp<RBX::TestService>(char const*,char const*,double RBX::TestService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x8113b8 — __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE10isReadOnlyEv")]
pub fn stub_8113b8() -> ! {
    todo!("0x8113b8 RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isReadOnly(void)const")
}

// 0x8113bc — __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE11isWriteOnlyEv")]
pub fn stub_8113bc() -> ! {
    todo!("0x8113bc RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isWriteOnly(void)const")
}

// 0x8113c0 — __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_8113c0() -> ! {
    todo!("0x8113c0 RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x8113d0 — __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8setValueEPNS0_13DescribedBaseERKd
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8setValueEPNS0_13DescribedBaseERKd")]
pub fn stub_8113d0() -> ! {
    todo!("0x8113d0 RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::setValue(RBX::Reflection::DescribedBase *,double const&)const")
}

// 0x81142c — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_11TestServiceEEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::TestService>(char const*,char const*,std::string  RBX::TestService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_11TestServiceEEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_81142c() -> ! {
    todo!("0x81142c RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::TestService>(char const*,char const*,std::string  RBX::TestService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x8115bc — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE10isReadOnlyEv")]
pub fn stub_8115bc() -> ! {
    todo!("0x8115bc RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isReadOnly(void)const")
}

// 0x8115c0 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE11isWriteOnlyEv")]
pub fn stub_8115c0() -> ! {
    todo!("0x8115c0 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::isWriteOnly(void)const")
}

// 0x8115c4 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_8115c4() -> ! {
    todo!("0x8115c4 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x8115dc — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_11TestServiceEE8setValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_8115dc() -> ! {
    todo!("0x8115dc RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::TestService>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x811644 — __ZN3RBX10Reflection13BoundFuncDescINS_14FunctionalTestEFvSsELi1EEC2EMS2_FvSsEPKcS8_SsNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::FunctionalTest,void ()(std::string),1>::BoundFuncDesc(void (RBX::FunctionalTest::*)(std::string),char const*,char const*,std::string,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_14FunctionalTestEFvSsELi1EEC2EMS2_FvSsEPKcS8_SsNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_811644() -> ! {
    todo!("0x811644 RBX::Reflection::BoundFuncDesc<RBX::FunctionalTest,void ()(std::string),1>::BoundFuncDesc(void (RBX::FunctionalTest::*)(std::string),char const*,char const*,std::string,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x811814 — __ZN3RBX10Reflection13BoundFuncDescINS_14FunctionalTestEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::FunctionalTest,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_14FunctionalTestEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_811814() -> ! {
    todo!("0x811814 RBX::Reflection::BoundFuncDesc<RBX::FunctionalTest,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x811844 — __ZN3RBX10Reflection13BoundFuncDescINS_14FunctionalTestEFvSsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::FunctionalTest,void ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_14FunctionalTestEFvSsELi1EED0Ev")]
pub fn stub_811844() -> ! {
    todo!("0x811844 RBX::Reflection::BoundFuncDesc<RBX::FunctionalTest,void ()(std::string),1>::~BoundFuncDesc()")
}

// 0x81194c — __ZNK3RBX10Reflection13BoundFuncDescINS_14FunctionalTestEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::FunctionalTest,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_14FunctionalTestEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_81194c() -> ! {
    todo!("0x81194c RBX::Reflection::BoundFuncDesc<RBX::FunctionalTest,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x811a88 — __ZN3RBX10Reflection11Call1HelperINS_14FunctionalTestEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::FunctionalTest,void (RBX::FunctionalTest::*)(std::string),std::string,void>::call(RBX::FunctionalTest*,void (RBX::FunctionalTest::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_14FunctionalTestEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs")]
pub fn stub_811a88() -> ! {
    todo!("0x811a88 RBX::Reflection::Call1Helper<RBX::FunctionalTest,void (RBX::FunctionalTest::*)(std::string),std::string,void>::call(RBX::FunctionalTest*,void (RBX::FunctionalTest::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0x811bb8 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_14FunctionalTestEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::FunctionalTest>(char const*,char const*,bool RBX::FunctionalTest::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_14FunctionalTestEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_811bb8() -> ! {
    todo!("0x811bb8 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::FunctionalTest>(char const*,char const*,bool RBX::FunctionalTest::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x811d48 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE10isReadOnlyEv")]
pub fn stub_811d48() -> ! {
    todo!("0x811d48 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::isReadOnly(void)const")
}

// 0x811d4c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE11isWriteOnlyEv")]
pub fn stub_811d4c() -> ! {
    todo!("0x811d4c RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::isWriteOnly(void)const")
}

// 0x811d50 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_811d50() -> ! {
    todo!("0x811d50 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x811d5c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_811d5c() -> ! {
    todo!("0x811d5c RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x811dac — __ZN3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EEC2INS_14FunctionalTestEEEPKcS7_MT_dNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundProp<RBX::FunctionalTest>(char const*,char const*,double RBX::FunctionalTest::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EEC2INS_14FunctionalTestEEEPKcS7_MT_dNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_811dac() -> ! {
    todo!("0x811dac RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundProp<RBX::FunctionalTest>(char const*,char const*,double RBX::FunctionalTest::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x811f3c — __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE10isReadOnlyEv")]
pub fn stub_811f3c() -> ! {
    todo!("0x811f3c RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::isReadOnly(void)const")
}

// 0x811f40 — __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE11isWriteOnlyEv")]
pub fn stub_811f40() -> ! {
    todo!("0x811f40 RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::isWriteOnly(void)const")
}

// 0x811f44 — __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_811f44() -> ! {
    todo!("0x811f44 RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x811f54 — __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE8setValueEPNS0_13DescribedBaseERKd
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE8setValueEPNS0_13DescribedBaseERKd")]
pub fn stub_811f54() -> ! {
    todo!("0x811f54 RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::setValue(RBX::Reflection::DescribedBase *,double const&)const")
}

// 0x811fb0 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_14FunctionalTestEEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::FunctionalTest>(char const*,char const*,std::string  RBX::FunctionalTest::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_14FunctionalTestEEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_811fb0() -> ! {
    todo!("0x811fb0 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::FunctionalTest>(char const*,char const*,std::string  RBX::FunctionalTest::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x812140 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE10isReadOnlyEv")]
pub fn stub_812140() -> ! {
    todo!("0x812140 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::isReadOnly(void)const")
}

// 0x812144 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE11isWriteOnlyEv")]
pub fn stub_812144() -> ! {
    todo!("0x812144 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::isWriteOnly(void)const")
}

// 0x812148 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_812148() -> ! {
    todo!("0x812148 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x812160 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_14FunctionalTestEE8setValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_812160() -> ! {
    todo!("0x812160 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::FunctionalTest>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x8121c8 — __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::resize(unsigned long,RBX::FunctionalTest::Result)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE6resizeEmS2_")]
pub fn stub_8121c8() -> ! {
    todo!("0x8121c8 std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::resize(unsigned long,RBX::FunctionalTest::Result)")
}

// 0x8121fc — __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::push_back(RBX::FunctionalTest::Result const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE9push_backERKS2_")]
pub fn stub_8121fc() -> ! {
    todo!("0x8121fc std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::push_back(RBX::FunctionalTest::Result const&)")
}

// 0x812224 — __ZNSt3mapIPKN3RBX4NameENS0_14FunctionalTest6ResultESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::FunctionalTest::Result,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_14FunctionalTest6ResultESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_812224() -> ! {
    todo!("0x812224 std::map<RBX::Name const*,RBX::FunctionalTest::Result,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::operator[](RBX::Name const* const&)")
}

// 0x81227c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_81227c() -> ! {
    todo!("0x81227c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")
}
