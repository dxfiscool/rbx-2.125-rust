//! rendering shard 465 — 100 stubs 0x707ac8..0x70beb0 EA-sorted asc global gap filler not yet in rbx_rendering (global gap filler, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Global gap filler fallback EA asc not yet in rbx_rendering (50219->50319 distinct, fallback after 0x707ac8).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc global gap filler not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x707ac8 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotD1Ev")]
pub fn stub_707ac8() -> ! {
    todo!("0x707ac8 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot::~slot()")
}

// 0x707af8 — __ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEE13assign_to_ownERKS6_
#[doc(alias = "boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::assign_to_own(boost::function1<void,RBX::Reflection::PropertyDescriptor const*> const&)")]
#[doc(alias = "__ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEE13assign_to_ownERKS6_")]
// was: boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::assign_to_own(boost::function1<void,RBX::Reflection::PropertyDescriptor const*> const&)
pub fn stub_707af8() -> ! {
    todo!("0x707af8 boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::assign_to_own(boost::function1<void,RBX::Reflection::PropertyDescriptor const*> const&)")
}

// 0x707b28 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_EC2ESA_PKcSD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_EC2ESA_PKcSD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_707b28() -> ! {
    todo!("0x707b28 RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x707d18 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_ED0Ev")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::~EventDesc()
pub fn stub_707d18() -> ! {
    todo!("0x707d18 RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::~EventDesc()")
}

// 0x707dcc — __ZNK3RBX10Reflection13EventDescImplILi2ENS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
// was: RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_707dcc() -> ! {
    todo!("0x707dcc RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x707f20 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE")]
// was: RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_707f20() -> ! {
    todo!("0x707f20 RBX::Reflection::EventDescImpl<2,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x7080d0 — __ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::Instance::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvN5boost10shared_ptrIS2_EES5_EN3rbx6signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE")]
// was: RBX::Reflection::EventDescBase<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_7080d0() -> ! {
    todo!("0x7080d0 RBX::Reflection::EventDescBase<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::Instance::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x7080e4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13disconnectAllEv")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::disconnectAll(void)
pub fn stub_7080e4() -> ! {
    todo!("0x7080e4 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::disconnectAll(void)")
}

// 0x70825c — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEES8_NS4_IS3_EENS_3argILi1EEENSA_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEES8_NS4_IS3_EENS_3argILi1EEENSA_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
pub fn stub_70825c() -> ! {
    todo!("0x70825c boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")
}

// 0x708378 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEES6_EEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEES6_EEvRKT_RKT0_")]
// was: void RBX::Reflection::GenericSlotWrapper::execute2<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_708378() -> ! {
    todo!("0x708378 void RBX::Reflection::GenericSlotWrapper::execute2<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&)")
}

// 0x7084e0 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_E5clearEv
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::clear(void)")]
#[doc(alias = "__ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_E5clearEv")]
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::clear(void)
pub fn stub_7084e0() -> ! {
    todo!("0x7084e0 boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::clear(void)")
}

// 0x70850c — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEES4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SF_EENS8_5list3INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEES4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SF_EENS8_5list3INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEES4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SF_EENS8_5list3INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_70850c() -> ! {
    todo!("0x70850c __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEES4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SF_EENS8_5list3INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0x7085f0 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SE_EENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SE_EENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SE_EENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_7085f0() -> ! {
    todo!("0x7085f0 __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SE_EENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

// 0x7086d8 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SE_EENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
#[doc(alias = "__ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SE_EENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_")]
// was: void boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_7086d8() -> ! {
    todo!("0x7086d8 void boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0x7087d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_7087d0() -> ! {
    todo!("0x7087d0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x7087ec — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEvSC_SC_E6invokeERNS1_15function_bufferESC_SC_
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEvSC_SC_E6invokeERNS1_15function_bufferESC_SC_")]
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_7087ec() -> ! {
    todo!("0x7087ec boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)")
}

// 0x7087f4 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEES6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_SG_EENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEES6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_SG_EENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")]
// was: bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_7087f4() -> ! {
    todo!("0x7087f4 bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")
}

// 0x7088dc — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEES6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_SG_EENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEES6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_SG_EENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_7088dc() -> ! {
    todo!("0x7088dc bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x7089c0 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEES6_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_SG_EENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEES6_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_SG_EENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: void boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_7089c0() -> ! {
    todo!("0x7089c0 void boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x708a94 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEESB_EENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSH_ILi2EEEEEEclIS9_S9_EEvRT_RT0_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> &,rbx_core::SharedPtr<RBX::Instance> &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEESB_EENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSH_ILi2EEEEEEclIS9_S9_EEvRT_RT0_")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> &,boost::shared_ptr<RBX::Instance> &)
pub fn stub_708a94() -> ! {
    todo!("0x708a94 void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> &,boost::shared_ptr<RBX::Instance> &)")
}

// 0x708ab0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_708ab0() -> ! {
    todo!("0x708ab0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x708c08 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> const&)
pub fn stub_708c08() -> ! {
    todo!("0x708c08 rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> const&)")
}

// 0x708d00 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEES7_EE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEES7_EE4slotEEaSEPSA_")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot*)
pub fn stub_708d00() -> ! {
    todo!("0x708d00 boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot*)")
}

// 0x708d24 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_EC2IPS9_EERKSC_T_")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>*>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>*)
pub fn stub_708d24() -> ! {
    todo!("0x708d24 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>*>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>*)")
}

// 0x708e20 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_8functionIS7_EEED1Ev")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub fn stub_708e20() -> ! {
    todo!("0x708e20 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::~callable_slot()")
}

// 0x708f30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_8functionIS7_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_8functionIS7_EEED0Ev")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub fn stub_708f30() -> ! {
    todo!("0x708f30 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::~callable_slot()")
}

// 0x709060 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot10disconnectEv")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::disconnect(void)
pub fn stub_709060() -> ! {
    todo!("0x709060 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::disconnect(void)")
}

// 0x709170 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot9connectedEv")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::connected(void)const
pub fn stub_709170() -> ! {
    todo!("0x709170 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::connected(void)const")
}

// 0x70917c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_E4callES7_S7_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_E4callES7_S7_")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_70917c() -> ! {
    todo!("0x70917c rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)")
}

// 0x709294 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_E4callES7_S7_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_E4callES7_S7_")]
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_709294() -> ! {
    todo!("0x709294 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)")
}

// 0x70929c — __ZNK5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_EclES4_S4_
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")]
#[doc(alias = "__ZNK5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_EclES4_S4_")]
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::operator()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)const
pub fn stub_70929c() -> ! {
    todo!("0x70929c boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::operator()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)const")
}

// 0x7093f0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE6removeEPNS8_4slotE")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_7093f0() -> ! {
    todo!("0x7093f0 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot *)")
}

// 0x7094e0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot22safe_static_init_mutexEv")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_init_mutex(void)
pub fn stub_7094e0() -> ! {
    todo!("0x7094e0 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")
}

// 0x7094e4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slot24safe_static_do_get_mutexEv")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)
pub fn stub_7094e4() -> ! {
    todo!("0x7094e4 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")
}

// 0x7095d4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_ED1Ev")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_7095d4() -> ! {
    todo!("0x7095d4 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()")
}

// 0x7096e4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_8functionIS8_EELi2ES8_ED0Ev")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_7096e4() -> ! {
    todo!("0x7096e4 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()")
}

// 0x709814 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slotD1Ev")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_709814() -> ! {
    todo!("0x709814 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::~slot()")
}

// 0x709840 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4slotD0Ev")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_709840() -> ! {
    todo!("0x709840 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot::~slot()")
}

// 0x709914 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_E13assign_to_ownERKS5_
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::assign_to_own(boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>> const&)")]
#[doc(alias = "__ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_E13assign_to_ownERKS5_")]
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to_own(boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>> const&)
pub fn stub_709914() -> ! {
    todo!("0x709914 boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to_own(boost::function2<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>> const&)")
}

// 0x709944 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEEC2ESC_PKcSF_NS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEEC2ESC_PKcSF_NS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)
pub fn stub_709944() -> ! {
    todo!("0x709944 RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)")
}

// 0x709ad4 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEED0Ev")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::~EventDesc()
pub fn stub_709ad4() -> ! {
    todo!("0x709ad4 RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::~EventDesc()")
}

// 0x709b88 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_709b88() -> ! {
    todo!("0x709b88 RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x709cf4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_709cf4() -> ! {
    todo!("0x709cf4 RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x709e50 — __ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE13disconnectAllEPNS0_11EventSourceE")]
// was: RBX::Reflection::EventDescBase<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_709e50() -> ! {
    todo!("0x709e50 RBX::Reflection::EventDescBase<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x709e7c — __ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE9getSignalEPS2_
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::getSignal(RBX::Instance*)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEE9getSignalEPS2_")]
// was: RBX::Reflection::EventDescBase<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::getSignal(RBX::Instance*)const
pub fn stub_709e7c() -> ! {
    todo!("0x709e7c RBX::Reflection::EventDescBase<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::getSignal(RBX::Instance*)const")
}

// 0x709ef0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*)
pub fn stub_709ef0() -> ! {
    todo!("0x709ef0 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*)")
}

// 0x709ff0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE6removeEPNS8_4slotE")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_709ff0() -> ! {
    todo!("0x709ff0 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot *)")
}

// 0x70a0e4 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEEC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void)>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::Instance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8InstanceEFvN5boost10shared_ptrIS2_EEEN3rbx6signalIS6_EEMS2_FRS9_vEEC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_70a0e4() -> ! {
    todo!("0x70a0e4 RBX::Reflection::EventDesc<RBX::Instance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void)>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::Instance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x70a274 — __ZN3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_EC2IMS2_KFPS2_vEMS2_FvS5_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::Instance::*)(void)const,void (RBX::Instance::*)(RBX::Instance*)>(char const*,char const*,RBX::Instance* (RBX::Instance::*)(void)const,void (RBX::Instance::*)(RBX::Instance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_EC2IMS2_KFPS2_vEMS2_FvS5_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_70a274() -> ! {
    todo!("0x70a274 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::Instance::*)(void)const,void (RBX::Instance::*)(RBX::Instance*)>(char const*,char const*,RBX::Instance* (RBX::Instance::*)(void)const,void (RBX::Instance::*)(RBX::Instance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x70a318 — __ZN3RBX10Reflection7RefTypeIPNS_8InstanceEE9singletonEv
#[doc(alias = "RBX::Reflection::RefType<RBX::Instance *>::singleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection7RefTypeIPNS_8InstanceEE9singletonEv")]
pub fn stub_70a318() -> ! {
    todo!("0x70a318 RBX::Reflection::RefType<RBX::Instance *>::singleton(void)")
}

// 0x70a410 — __ZN3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_ED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_ED0Ev")]
pub fn stub_70a410() -> ! {
    todo!("0x70a410 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::~RefPropDescriptor()")
}

// 0x70a440 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E10isReadOnlyEv")]
pub fn stub_70a440() -> ! {
    todo!("0x70a440 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::isReadOnly(void)const")
}

// 0x70a450 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11isWriteOnlyEv")]
pub fn stub_70a450() -> ! {
    todo!("0x70a450 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::isWriteOnly(void)const")
}

// 0x70a460 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11equalValuesEPKNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11equalValuesEPKNS0_13DescribedBaseES6_")]
pub fn stub_70a460() -> ! {
    todo!("0x70a460 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x70a488 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_70a488() -> ! {
    todo!("0x70a488 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x70a5a0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_70a5a0() -> ! {
    todo!("0x70a5a0 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x70a668 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E9copyValueEPKNS0_13DescribedBaseEPS4_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E9copyValueEPKNS0_13DescribedBaseEPS4_")]
pub fn stub_70a668() -> ! {
    todo!("0x70a668 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x70a68c — __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_70a68c() -> ! {
    todo!("0x70a68c RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x70a760 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_70a760() -> ! {
    todo!("0x70a760 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x70a784 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11getRefValueEPKNS0_13DescribedBaseE")]
pub fn stub_70a784() -> ! {
    todo!("0x70a784 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x70a798 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11setRefValueEPNS0_13DescribedBaseES5_
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11setRefValueEPNS0_13DescribedBaseES5_")]
pub fn stub_70a798() -> ! {
    todo!("0x70a798 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")
}

// 0x70a810 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E17setRefValueUnsafeEPNS0_13DescribedBaseES5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E17setRefValueUnsafeEPNS0_13DescribedBaseES5_")]
pub fn stub_70a810() -> ! {
    todo!("0x70a810 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")
}

// 0x70a830 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
pub fn stub_70a830() -> ! {
    todo!("0x70a830 RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0x70a910 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk to RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
pub fn stub_70a910() -> ! {
    todo!("0x70a910 non-virtual thunk to RBX::Reflection::RefPropDescriptor<RBX::Instance,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0x70a918 — __ZN16XmlNameValuePair8setValueEN3RBX14InstanceHandleE
#[doc(alias = "XmlNameValuePair::setValue(RBX::InstanceHandle)")]
#[doc(alias = "__ZN16XmlNameValuePair8setValueEN3RBX14InstanceHandleE")]
pub fn stub_70a918() -> ! {
    todo!("0x70a918 XmlNameValuePair::setValue(RBX::InstanceHandle)")
}

// 0x70a9e0 — __ZNK3RBX10Reflection7Variant3getIN5boost10shared_ptrINS0_13DescribedBaseEEEEET_v
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::DescribedBase> RBX::Reflection::Variant::get<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection7Variant3getIN5boost10shared_ptrINS0_13DescribedBaseEEEEET_v")]
// was: boost::shared_ptr<RBX::Reflection::DescribedBase> RBX::Reflection::Variant::get<boost::shared_ptr<RBX::Reflection::DescribedBase>>(void)const
pub fn stub_70a9e0() -> ! {
    todo!("0x70a9e0 boost::shared_ptr<RBX::Reflection::DescribedBase> RBX::Reflection::Variant::get<boost::shared_ptr<RBX::Reflection::DescribedBase>>(void)const")
}

// 0x70ab58 — __ZN3rbx8any_castIRKN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEENS3_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::DescribedBase> const& rbx::any_cast<rbx_core::SharedPtr<RBX::Reflection::DescribedBase> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEENS3_7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: boost::shared_ptr<RBX::Reflection::DescribedBase> const& rbx::any_cast<boost::shared_ptr<RBX::Reflection::DescribedBase> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_70ab58() -> ! {
    todo!("0x70ab58 boost::shared_ptr<RBX::Reflection::DescribedBase> const& rbx::any_cast<boost::shared_ptr<RBX::Reflection::DescribedBase> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x70ac48 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrINS1_10Reflection13DescribedBaseEEEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>(rbx_core::SharedPtr<RBX::Reflection::DescribedBase> const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrINS1_10Reflection13DescribedBaseEEEEERS3_RKT_")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<boost::shared_ptr<RBX::Reflection::DescribedBase>>(boost::shared_ptr<RBX::Reflection::DescribedBase> const&)
pub fn stub_70ac48() -> ! {
    todo!("0x70ac48 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<boost::shared_ptr<RBX::Reflection::DescribedBase>>(boost::shared_ptr<RBX::Reflection::DescribedBase> const&)")
}

// 0x70acb0 — __ZN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEaSERKS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::DescribedBase>::operator=(rbx_core::SharedPtr<RBX::Reflection::DescribedBase> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEaSERKS4_")]
// was: boost::shared_ptr<RBX::Reflection::DescribedBase>::operator=(boost::shared_ptr<RBX::Reflection::DescribedBase> const&)
pub fn stub_70acb0() -> ! {
    todo!("0x70acb0 boost::shared_ptr<RBX::Reflection::DescribedBase>::operator=(boost::shared_ptr<RBX::Reflection::DescribedBase> const&)")
}

// 0x70ace8 — __ZNK3RBX10Reflection14PropDescriptorINS_8InstanceEPS2_E10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,RBX::Instance*>::GetSetImpl<RBX::Instance* (RBX::Instance::*)(void)const,void (RBX::Instance::*)(RBX::Instance*)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8InstanceEPS2_E10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
pub fn stub_70ace8() -> ! {
    todo!("0x70ace8 RBX::Reflection::PropDescriptor<RBX::Instance,RBX::Instance*>::GetSetImpl<RBX::Instance* (RBX::Instance::*)(void)const,void (RBX::Instance::*)(RBX::Instance*)>::isReadOnly(void)const")
}

// 0x70acec — __ZNK3RBX10Reflection14PropDescriptorINS_8InstanceEPS2_E10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,RBX::Instance*>::GetSetImpl<RBX::Instance* (RBX::Instance::*)(void)const,void (RBX::Instance::*)(RBX::Instance*)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8InstanceEPS2_E10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_70acec() -> ! {
    todo!("0x70acec RBX::Reflection::PropDescriptor<RBX::Instance,RBX::Instance*>::GetSetImpl<RBX::Instance* (RBX::Instance::*)(void)const,void (RBX::Instance::*)(RBX::Instance*)>::isWriteOnly(void)const")
}

// 0x70acf0 — __ZNK3RBX10Reflection14PropDescriptorINS_8InstanceEPS2_E10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,RBX::Instance*>::GetSetImpl<RBX::Instance* (RBX::Instance::*)(void)const,void (RBX::Instance::*)(RBX::Instance*)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8InstanceEPS2_E10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_70acf0() -> ! {
    todo!("0x70acf0 RBX::Reflection::PropDescriptor<RBX::Instance,RBX::Instance*>::GetSetImpl<RBX::Instance* (RBX::Instance::*)(void)const,void (RBX::Instance::*)(RBX::Instance*)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x70ad10 — __ZNK3RBX10Reflection14PropDescriptorINS_8InstanceEPS2_E10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,RBX::Instance*>::GetSetImpl<RBX::Instance* (RBX::Instance::*)(void)const,void (RBX::Instance::*)(RBX::Instance*)>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance* const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8InstanceEPS2_E10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_70ad10() -> ! {
    todo!("0x70ad10 RBX::Reflection::PropDescriptor<RBX::Instance,RBX::Instance*>::GetSetImpl<RBX::Instance* (RBX::Instance::*)(void)const,void (RBX::Instance::*)(RBX::Instance*)>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance* const&)const")
}

// 0x70ad34 — __ZN3RBX10Reflection14PropDescriptorINS_8InstanceESsEC2IMS2_KFRKSsvEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,std::string>::PropDescriptor<std::string const& (RBX::Instance::*)(void)const,void (RBX::Instance::*)(std::string const&)>(char const*,char const*,std::string const& (RBX::Instance::*)(void)const,void (RBX::Instance::*)(std::string const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8InstanceESsEC2IMS2_KFRKSsvEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_70ad34() -> ! {
    todo!("0x70ad34 RBX::Reflection::PropDescriptor<RBX::Instance,std::string>::PropDescriptor<std::string const& (RBX::Instance::*)(void)const,void (RBX::Instance::*)(std::string const&)>(char const*,char const*,std::string const& (RBX::Instance::*)(void)const,void (RBX::Instance::*)(std::string const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x70ae4c — __ZN3RBX10Reflection14PropDescriptorINS_8InstanceESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8InstanceESsED0Ev")]
pub fn stub_70ae4c() -> ! {
    todo!("0x70ae4c RBX::Reflection::PropDescriptor<RBX::Instance,std::string>::~PropDescriptor()")
}

// 0x70ae78 — __ZNK3RBX10Reflection14PropDescriptorINS_8InstanceESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,std::string>::GetSetImpl<std::string const& (RBX::Instance::*)(void)const,void (RBX::Instance::*)(std::string const&)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8InstanceESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE10isReadOnlyEv")]
pub fn stub_70ae78() -> ! {
    todo!("0x70ae78 RBX::Reflection::PropDescriptor<RBX::Instance,std::string>::GetSetImpl<std::string const& (RBX::Instance::*)(void)const,void (RBX::Instance::*)(std::string const&)>::isReadOnly(void)const")
}

// 0x70ae7c — __ZNK3RBX10Reflection14PropDescriptorINS_8InstanceESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,std::string>::GetSetImpl<std::string const& (RBX::Instance::*)(void)const,void (RBX::Instance::*)(std::string const&)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8InstanceESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE11isWriteOnlyEv")]
pub fn stub_70ae7c() -> ! {
    todo!("0x70ae7c RBX::Reflection::PropDescriptor<RBX::Instance,std::string>::GetSetImpl<std::string const& (RBX::Instance::*)(void)const,void (RBX::Instance::*)(std::string const&)>::isWriteOnly(void)const")
}

// 0x70ae80 — __ZNK3RBX10Reflection14PropDescriptorINS_8InstanceESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,std::string>::GetSetImpl<std::string const& (RBX::Instance::*)(void)const,void (RBX::Instance::*)(std::string const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8InstanceESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_70ae80() -> ! {
    todo!("0x70ae80 RBX::Reflection::PropDescriptor<RBX::Instance,std::string>::GetSetImpl<std::string const& (RBX::Instance::*)(void)const,void (RBX::Instance::*)(std::string const&)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x70aeb0 — __ZNK3RBX10Reflection14PropDescriptorINS_8InstanceESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE8setValueEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,std::string>::GetSetImpl<std::string const& (RBX::Instance::*)(void)const,void (RBX::Instance::*)(std::string const&)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8InstanceESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE8setValueEPNS0_13DescribedBaseES6_")]
pub fn stub_70aeb0() -> ! {
    todo!("0x70aeb0 RBX::Reflection::PropDescriptor<RBX::Instance,std::string>::GetSetImpl<std::string const& (RBX::Instance::*)(void)const,void (RBX::Instance::*)(std::string const&)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x70aed4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsES5_Li1EEC2EMS2_FvSsNS3_8functionIFvS5_EEENS8_IFvSsEEEEPKcSG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,rbx_core::SharedPtr<RBX::Instance> ()(std::string),rbx_core::SharedPtr<RBX::Instance>,1>::BoundYieldFuncDesc(void (RBX::Instance::*)(std::string,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsES5_Li1EEC2EMS2_FvSsNS3_8functionIFvS5_EEENS8_IFvSsEEEEPKcSG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,boost::shared_ptr<RBX::Instance> ()(std::string),boost::shared_ptr<RBX::Instance>,1>::BoundYieldFuncDesc(void (RBX::Instance::*)(std::string,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_70aed4() -> ! {
    todo!("0x70aed4 RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,boost::shared_ptr<RBX::Instance> ()(std::string),boost::shared_ptr<RBX::Instance>,1>::BoundYieldFuncDesc(void (RBX::Instance::*)(std::string,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x70b04c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsES5_Li1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,rbx_core::SharedPtr<RBX::Instance> ()(std::string),rbx_core::SharedPtr<RBX::Instance>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsES5_Li1EE16declareSignatureEPKcNS0_7VariantE")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,boost::shared_ptr<RBX::Instance> ()(std::string),boost::shared_ptr<RBX::Instance>,1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_70b04c() -> ! {
    todo!("0x70b04c RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,boost::shared_ptr<RBX::Instance> ()(std::string),boost::shared_ptr<RBX::Instance>,1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x70b07c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsES5_Li1EED0Ev
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,rbx_core::SharedPtr<RBX::Instance> ()(std::string),rbx_core::SharedPtr<RBX::Instance>,1>::~BoundYieldFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsES5_Li1EED0Ev")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,boost::shared_ptr<RBX::Instance> ()(std::string),boost::shared_ptr<RBX::Instance>,1>::~BoundYieldFuncDesc()
pub fn stub_70b07c() -> ! {
    todo!("0x70b07c RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,boost::shared_ptr<RBX::Instance> ()(std::string),boost::shared_ptr<RBX::Instance>,1>::~BoundYieldFuncDesc()")
}

// 0x70b148 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsES5_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSD_IFvSsEEE
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,rbx_core::SharedPtr<RBX::Instance> ()(std::string),rbx_core::SharedPtr<RBX::Instance>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
#[doc(alias = "__ZNK3RBX10Reflection18BoundYieldFuncDescINS_8InstanceEFN5boost10shared_ptrIS2_EESsES5_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSD_IFvSsEEE")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,boost::shared_ptr<RBX::Instance> ()(std::string),boost::shared_ptr<RBX::Instance>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
pub fn stub_70b148() -> ! {
    todo!("0x70b148 RBX::Reflection::BoundYieldFuncDesc<RBX::Instance,boost::shared_ptr<RBX::Instance> ()(std::string),boost::shared_ptr<RBX::Instance>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x70b370 — __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrINS2_8InstanceEEES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSE_T0_T1_ENSC_9list_av_2IT2_T3_E4typeEEESI_SK_SL_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrINS2_8InstanceEEES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSE_T0_T1_ENSC_9list_av_2IT2_T3_E4typeEEESI_SK_SL_")]
// was: boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)
pub fn stub_70b370() -> ! {
    todo!("0x70b370 boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")
}

// 0x70b470 — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEC2ES9_SB_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::list2(boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEC2ES9_SB_")]
// was: boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::list2(boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>)
pub fn stub_70b470() -> ! {
    todo!("0x70b470 boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::list2(boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>)")
}

// 0x70b534 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS2_10Reflection7VariantEEEES4_ENS8_5list2INS8_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS2_10Reflection7VariantEEEES4_ENS8_5list2INS8_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS2_10Reflection7VariantEEEES4_ENS8_5list2INS8_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_70b534() -> ! {
    todo!("0x70b534 __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS2_10Reflection7VariantEEEES4_ENS8_5list2INS8_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

// 0x70b608 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS2_10Reflection7VariantEEEES4_ENS7_5list2INS7_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS2_10Reflection7VariantEEEES4_ENS7_5list2INS7_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS2_10Reflection7VariantEEEES4_ENS7_5list2INS7_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_70b608() -> ! {
    todo!("0x70b608 __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS2_10Reflection7VariantEEEES4_ENS7_5list2INS7_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

// 0x70b6dc — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS2_10Reflection7VariantEEEES4_ENS7_5list2INS7_5valueISD_EENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS2_10Reflection7VariantEEEES4_ENS7_5list2INS7_5valueISD_EENS_3argILi1EEEEEEEEEvT_")]
// was: void boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)
pub fn stub_70b6dc() -> ! {
    todo!("0x70b6dc void boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")
}

// 0x70b7c0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrINS6_8InstanceEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrINS6_8InstanceEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_70b7c0() -> ! {
    todo!("0x70b7c0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x70b7dc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrINS6_8InstanceEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSD_E6invokeERNS1_15function_bufferESD_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrINS6_8InstanceEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSD_E6invokeERNS1_15function_bufferESD_")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
pub fn stub_70b7dc() -> ! {
    todo!("0x70b7dc boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)")
}

// 0x70b7f4 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS4_10Reflection7VariantEEEES6_ENS9_5list2INS9_5valueISF_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS4_10Reflection7VariantEEEES6_ENS9_5list2INS9_5valueISF_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_70b7f4() -> ! {
    todo!("0x70b7f4 bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0x70b8cc — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS4_10Reflection7VariantEEEES6_ENS9_5list2INS9_5valueISF_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS4_10Reflection7VariantEEEES6_ENS9_5list2INS9_5valueISF_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_70b8cc() -> ! {
    todo!("0x70b8cc bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x70b99c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS4_10Reflection7VariantEEEES6_ENS9_5list2INS9_5valueISF_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS4_10Reflection7VariantEEEES6_ENS9_5list2INS9_5valueISF_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_70b99c() -> ! {
    todo!("0x70b99c void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x70ba60 — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_NS_10shared_ptrINS4_8InstanceEEEENS0_5list1IRSG_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_NS_10shared_ptrINS4_8InstanceEEEENS0_5list1IRSG_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_70ba60() -> ! {
    todo!("0x70ba60 void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)")
}

// 0x70bb6c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrINS6_8InstanceEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrINS6_8InstanceEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_70bb6c() -> ! {
    todo!("0x70bb6c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x70bcb8 — __ZN3RBX10Reflection23YieldFunctionDescriptorD0Ev
// type: void __fastcall(RBX::Reflection::YieldFunctionDescriptor *__hidden this)
#[doc(alias = "RBX::Reflection::YieldFunctionDescriptor::~YieldFunctionDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection23YieldFunctionDescriptorD0Ev")]
pub fn stub_70bcb8() -> ! {
    todo!("0x70bcb8 RBX::Reflection::YieldFunctionDescriptor::~YieldFunctionDescriptor()")
}

// 0x70bd70 — __ZN3RBX10Reflection14PropDescriptorINS_8InstanceEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,int>::PropDescriptor<int (RBX::Instance::*)(void)const,int>(char const*,char const*,int (RBX::Instance::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8InstanceEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_70bd70() -> ! {
    todo!("0x70bd70 RBX::Reflection::PropDescriptor<RBX::Instance,int>::PropDescriptor<int (RBX::Instance::*)(void)const,int>(char const*,char const*,int (RBX::Instance::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x70be80 — __ZN3RBX10Reflection14PropDescriptorINS_8InstanceEiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8InstanceEiED0Ev")]
pub fn stub_70be80() -> ! {
    todo!("0x70be80 RBX::Reflection::PropDescriptor<RBX::Instance,int>::~PropDescriptor()")
}

// 0x70beac — __ZNK3RBX10Reflection14PropDescriptorINS_8InstanceEiE7GetImplIMS2_KFivEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,int>::GetImpl<int (RBX::Instance::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8InstanceEiE7GetImplIMS2_KFivEE10isReadOnlyEv")]
pub fn stub_70beac() -> ! {
    todo!("0x70beac RBX::Reflection::PropDescriptor<RBX::Instance,int>::GetImpl<int (RBX::Instance::*)(void)const>::isReadOnly(void)const")
}

// 0x70beb0 — __ZNK3RBX10Reflection14PropDescriptorINS_8InstanceEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Instance,int>::GetImpl<int (RBX::Instance::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8InstanceEiE7GetImplIMS2_KFivEE11isWriteOnlyEv")]
pub fn stub_70beb0() -> ! {
    todo!("0x70beb0 RBX::Reflection::PropDescriptor<RBX::Instance,int>::GetImpl<int (RBX::Instance::*)(void)const>::isWriteOnly(void)const")
}
