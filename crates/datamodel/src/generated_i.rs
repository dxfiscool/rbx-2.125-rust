// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x85ba3c..0x8e8e5c | total filtered 10215, remaining 4357 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; `'` stripped from alias

#![allow(
    non_snake_case,
    dead_code,
    unused_variables,
    unused_imports,
    clippy::all
)]

use rbx_core::SharedPtr;

// 0x85ba3c — __ZN3RBX19CustomEventReceiver9setSourceEPNS_8InstanceE
#[doc(alias = "RBX::CustomEventReceiver::setSource(RBX::Instance *)")]
pub fn stub_85ba3c() -> ! {
    todo!("0x85ba3c RBX::CustomEventReceiver::setSource(RBX::Instance *)")
}

// 0x897564 — __ZN3RBX6CellIDC1EbPfN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::CellID::CellID(bool,float *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::CellID::CellID(bool,float *,boost::shared_ptr<RBX::Instance>)
pub fn stub_897564() -> ! {
    todo!("0x897564 RBX::CellID::CellID(bool,float *,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x897568 — __ZN3RBX6CellIDC2EbPfN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::CellID::CellID(bool,float *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::CellID::CellID(bool,float *,boost::shared_ptr<RBX::Instance>)
pub fn stub_897568() -> ! {
    todo!("0x897568 RBX::CellID::CellID(bool,float *,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x89f838 — __ZNK3RBX16BindableFunction12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::BindableFunction::askSetParent(RBX::Instance const*)const")]
pub fn stub_89f838() -> ! {
    todo!("0x89f838 RBX::BindableFunction::askSetParent(RBX::Instance const*)const")
}

// 0x89f83c — __ZNK3RBX13BindableEvent12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::BindableEvent::askSetParent(RBX::Instance const*)const")]
pub fn stub_89f83c() -> ! {
    todo!("0x89f83c RBX::BindableEvent::askSetParent(RBX::Instance const*)const")
}

// 0x8ac950 — __ZN3RBX15GamePassService13playerHasPassEN5boost10shared_ptrINS_8InstanceEEEiNS1_8functionIFvbEEENS5_IFvSsEEE
#[doc(alias = "RBX::GamePassService::playerHasPass(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// was: RBX::GamePassService::playerHasPass(boost::shared_ptr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
pub fn stub_8ac950() -> ! {
    todo!("0x8ac950 RBX::GamePassService::playerHasPass(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")
}

// 0x8d8a54 — __ZNK3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEELi4EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_8d8a54() -> ! {
    todo!("0x8d8a54 RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x8d8b88 — __ZN3RBX10Reflection11Call4HelperINS_18MarketplaceServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEES6_ibS7_vE4callEPS2_S9_RNS0_7VariantERKS6_RKiRKbRKS7_
#[doc(alias = "RBX::Reflection::Call4Helper<RBX::MarketplaceService,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType,void>::call(RBX::MarketplaceService*,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&)")]
// was: RBX::Reflection::Call4Helper<RBX::MarketplaceService,void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType,void>::call(RBX::MarketplaceService*,void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&)
pub fn stub_8d8b88() -> ! {
    todo!("0x8d8b88 RBX::Reflection::Call4Helper<RBX::MarketplaceService,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType,void>::call(RBX::MarketplaceService*,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&)")
}

// 0x8d8e70 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EEC2EMS2_FvS6_iNS3_8functionIFvbEEENS9_IFvSsEEEEPKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_8d8e70() -> ! {
    todo!("0x8d8e70 RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8d903c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
pub fn stub_8d903c() -> ! {
    todo!("0x8d903c RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x8d9088 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()
pub fn stub_8d9088() -> ! {
    todo!("0x8d9088 RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()")
}

// 0x8d919c — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
pub fn stub_8d919c() -> ! {
    todo!("0x8d919c RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x8dd22c — __ZN3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~RemoteEventDesc()")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~RemoteEventDesc()
pub fn stub_8dd22c() -> ! {
    todo!("0x8dd22c RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~RemoteEventDesc()")
}

// 0x8dd2e0 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_8dd2e0() -> ! {
    todo!("0x8dd2e0 RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x8dd444 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isScriptable(void)const")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isScriptable(void)const
pub fn stub_8dd444() -> ! {
    todo!("0x8dd444 RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isScriptable(void)const")
}

// 0x8dd44c — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isBroadcast(void)const")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isBroadcast(void)const
pub fn stub_8dd44c() -> ! {
    todo!("0x8dd44c RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isBroadcast(void)const")
}

// 0x8dd454 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_8dd454() -> ! {
    todo!("0x8dd454 RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8dd600 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_8dd600() -> ! {
    todo!("0x8dd600 RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8dd610 — __ZNK3RBX10Reflection13EventDescBaseINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_8dd610() -> ! {
    todo!("0x8dd610 RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x8dd624 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKiRKbRKNS1_18MarketplaceService12CurrencyTypeENS4_IS3_EENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISP_T0_T1_T2_T3_T4_EENSN_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSS_FSP_ST_SU_SV_SW_ESZ_S10_S11_S12_S13_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list_av_5<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)
pub fn stub_8dd624() -> ! {
    todo!("0x8dd624 boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")
}

// 0x8dd740 — __ZN3RBX10Reflection18GenericSlotWrapper8execute4IN5boost10shared_ptrINS_8InstanceEEEibNS_18MarketplaceService12CurrencyTypeEEEvRKT_RKT0_RKT1_RKT2_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute4<rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>(rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute4<boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>(boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&)
pub fn stub_8dd740() -> ! {
    todo!("0x8dd740 void RBX::Reflection::GenericSlotWrapper::execute4<rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>(rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&)")
}

// 0x8dd8ec — __ZN5boost9function4IvNS_10shared_ptrIN3RBX8InstanceEEEibNS2_18MarketplaceService12CurrencyTypeEE5clearEv
#[doc(alias = "boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::clear(void)")]
// was: boost::function4<void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::clear(void)
pub fn stub_8dd8ec() -> ! {
    todo!("0x8dd8ec boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::clear(void)")
}

// 0x8ddc84 — __ZN5boost9function4IvNS_10shared_ptrIN3RBX8InstanceEEEibNS2_18MarketplaceService12CurrencyTypeEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKS4_RKiRKbRKS6_EENS9_5list5INS9_5valueINS1_ISE_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEEvT_
#[doc(alias = "void boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")]
// was: void boost::function4<void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)
pub fn stub_8ddc84() -> ! {
    todo!("0x8ddc84 void boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")
}

// 0x8ddd7c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKiRKbRKNS7_18MarketplaceService12CurrencyTypeEEENS3_5list5INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEE6manageERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_8ddd7c() -> ! {
    todo!("0x8ddd7c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x8ddd98 — __ZN5boost6detail8function26void_function_obj_invoker4INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKiRKbRKNS7_18MarketplaceService12CurrencyTypeEEENS3_5list5INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEvSC_ibSK_E6invokeERNS1_15function_bufferESC_ibSK_
#[doc(alias = "boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)")]
// was: boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)
pub fn stub_8ddd98() -> ! {
    todo!("0x8ddd98 boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)")
}

// 0x8dddcc — __ZNK5boost6detail8function13basic_vtable4IvNS_10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKS6_RKiRKbRKS8_EENSB_5list5INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable4<void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const
pub fn stub_8dddcc() -> ! {
    todo!("0x8dddcc bool boost::detail::function::basic_vtable4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")
}

// 0x8ddeb4 — __ZNK5boost6detail8function13basic_vtable4IvNS_10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKS6_RKiRKbRKS8_EENSB_5list5INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable4<void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_8ddeb4() -> ! {
    todo!("0x8ddeb4 bool boost::detail::function::basic_vtable4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x8ddf98 — __ZNK5boost6detail8function13basic_vtable4IvNS_10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKS6_RKiRKbRKS8_EENSB_5list5INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable4<void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_8ddf98() -> ! {
    todo!("0x8ddf98 void boost::detail::function::basic_vtable4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x8de06c — __ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEclINS_4_mfi3mf4IvS6_RKNS3_INS4_8InstanceEEERKiRKbRKNS4_18MarketplaceService12CurrencyTypeEEENS0_5list4IRSJ_RiRbRSR_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list4<rbx_core::SharedPtr<RBX::Instance>&,int &,bool &,RBX::MarketplaceService::CurrencyType&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&> &,boost::_bi::list4<rbx_core::SharedPtr<RBX::Instance>&,int &,bool &,RBX::MarketplaceService::CurrencyType&> &,int)")]
// was: void boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list4<boost::shared_ptr<RBX::Instance>&,int &,bool &,RBX::MarketplaceService::CurrencyType&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&> &,boost::_bi::list4<boost::shared_ptr<RBX::Instance>&,int &,bool &,RBX::MarketplaceService::CurrencyType&> &,int)
pub fn stub_8de06c() -> ! {
    todo!("0x8de06c void boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list4<rbx_core::SharedPtr<RBX::Instance>&,int &,bool &,RBX::MarketplaceService::CurrencyType&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&> &,boost::_bi::list4<rbx_core::SharedPtr<RBX::Instance>&,int &,bool &,RBX::MarketplaceService::CurrencyType&> &,int)")
}

// 0x8de09c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKiRKbRKNS7_18MarketplaceService12CurrencyTypeEEENS3_5list5INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEE7managerERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_8de09c() -> ! {
    todo!("0x8de09c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&,RBX::MarketplaceService::CurrencyType const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x8de1f4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEEE7connectINS2_8functionIS9_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> const&)
pub fn stub_8de1f4() -> ! {
    todo!("0x8de1f4 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> const&)")
}

// 0x8de2e8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEEE6insertEPNSA_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot *)
pub fn stub_8de2e8() -> ! {
    todo!("0x8de2e8 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot *)")
}

// 0x8de4f4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEibNS5_18MarketplaceService12CurrencyTypeEEE4slotEEaSEPSC_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot*)
pub fn stub_8de4f4() -> ! {
    todo!("0x8de4f4 boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot*)")
}

// 0x8de518 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS5_18MarketplaceService12CurrencyTypeEEE4slotENS3_8functionISA_EELi4ESA_EC2IPSB_EERKSE_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>*>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>*)
pub fn stub_8de518() -> ! {
    todo!("0x8de518 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>*)")
}

// 0x8de614 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEEE13callable_slotINS2_8functionIS9_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~callable_slot()
pub fn stub_8de614() -> ! {
    todo!("0x8de614 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~callable_slot()")
}

// 0x8de724 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEEE13callable_slotINS2_8functionIS9_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~callable_slot()
pub fn stub_8de724() -> ! {
    todo!("0x8de724 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~callable_slot()")
}

// 0x8de854 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::disconnect(void)
pub fn stub_8de854() -> ! {
    todo!("0x8de854 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::disconnect(void)")
}

// 0x8de964 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::connected(void)const
pub fn stub_8de964() -> ! {
    todo!("0x8de964 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::connected(void)const")
}

// 0x8de970 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS5_18MarketplaceService12CurrencyTypeEEE4slotENS3_8functionISA_EELi4ESA_E4callES7_ibS9_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::call(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::call(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)
pub fn stub_8de970() -> ! {
    todo!("0x8de970 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::call(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)")
}

// 0x8dea50 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS5_18MarketplaceService12CurrencyTypeEEE4slotENS3_8functionISA_EELi4ESA_E4callES7_ibS9_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::call(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)")]
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::call(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)
pub fn stub_8dea50() -> ! {
    todo!("0x8dea50 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::call(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)")
}

// 0x8dea58 — __ZNK5boost9function4IvNS_10shared_ptrIN3RBX8InstanceEEEibNS2_18MarketplaceService12CurrencyTypeEEclES4_ibS6_
#[doc(alias = "boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::operator()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)const")]
// was: boost::function4<void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::operator()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)const
pub fn stub_8dea58() -> ! {
    todo!("0x8dea58 boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::operator()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)const")
}

// 0x8deb70 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEEE6removeEPNSA_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot *)
pub fn stub_8deb70() -> ! {
    todo!("0x8deb70 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot *)")
}

// 0x8dec60 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::safe_static_init_mutex(void)
pub fn stub_8dec60() -> ! {
    todo!("0x8dec60 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::safe_static_init_mutex(void)")
}

// 0x8dec64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::safe_static_do_get_mutex(void)
pub fn stub_8dec64() -> ! {
    todo!("0x8dec64 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::safe_static_do_get_mutex(void)")
}

// 0x8ded54 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS5_18MarketplaceService12CurrencyTypeEEE4slotENS3_8functionISA_EELi4ESA_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::~callable()
pub fn stub_8ded54() -> ! {
    todo!("0x8ded54 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::~callable()")
}

// 0x8dee64 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS5_18MarketplaceService12CurrencyTypeEEE4slotENS3_8functionISA_EELi4ESA_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::~callable()
pub fn stub_8dee64() -> ! {
    todo!("0x8dee64 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::~callable()")
}

// 0x8def94 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::~slot()
pub fn stub_8def94() -> ! {
    todo!("0x8def94 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::~slot()")
}

// 0x8defc0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS4_18MarketplaceService12CurrencyTypeEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::~slot()
pub fn stub_8defc0() -> ! {
    todo!("0x8defc0 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::slot::~slot()")
}

// 0x8df094 — __ZN5boost9function4IvNS_10shared_ptrIN3RBX8InstanceEEEibNS2_18MarketplaceService12CurrencyTypeEE13assign_to_ownERKS7_
#[doc(alias = "boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to_own(boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType> const&)")]
// was: boost::function4<void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to_own(boost::function4<void,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType> const&)
pub fn stub_8df094() -> ! {
    todo!("0x8df094 boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType>::assign_to_own(boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType> const&)")
}

// 0x8df0c4 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_EC2ESC_PKcSF_SF_SF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_8df0c4() -> ! {
    todo!("0x8df0c4 RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8df390 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()
pub fn stub_8df390() -> ! {
    todo!("0x8df390 RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()")
}

// 0x8df3b4 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()
pub fn stub_8df3b4() -> ! {
    todo!("0x8df3b4 RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()")
}

// 0x8df468 — __ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibELi3EEC2EMS2_FvS6_ibEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::BoundFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),3>::BoundFuncDesc(void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,bool),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_8df468() -> ! {
    todo!("0x8df468 RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::BoundFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8df684 — __ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
pub fn stub_8df684() -> ! {
    todo!("0x8df684 RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x8df6ec — __ZN3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibELi3EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),3>::~BoundFuncDesc()
pub fn stub_8df6ec() -> ! {
    todo!("0x8df6ec RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::~BoundFuncDesc()")
}

// 0x8df80c — __ZNK3RBX10Reflection13BoundFuncDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_8df80c() -> ! {
    todo!("0x8df80c RBX::Reflection::BoundFuncDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x8df928 — __ZN3RBX10Reflection11Call3HelperINS_18MarketplaceServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEibES6_ibvE4callEPS2_S8_RNS0_7VariantERKS6_RKiRKb
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::MarketplaceService,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx_core::SharedPtr<RBX::Instance>,int,bool,void>::call(RBX::MarketplaceService*,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&)")]
// was: RBX::Reflection::Call3Helper<RBX::MarketplaceService,void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,bool),boost::shared_ptr<RBX::Instance>,int,bool,void>::call(RBX::MarketplaceService*,void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,bool),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,int const&,bool const&)
pub fn stub_8df928() -> ! {
    todo!("0x8df928 RBX::Reflection::Call3Helper<RBX::MarketplaceService,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx_core::SharedPtr<RBX::Instance>,int,bool,void>::call(RBX::MarketplaceService*,void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,bool),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,int const&,bool const&)")
}

// 0x8e0420 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibEED2Ev
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>::~remote_signal()")]
// was: rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>::~remote_signal()
pub fn stub_8e0420() -> ! {
    todo!("0x8e0420 rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>::~remote_signal()")
}

// 0x8e056c — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS3_18MarketplaceService12CurrencyTypeEEED2Ev
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::~remote_signal()")]
// was: rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::~remote_signal()
pub fn stub_8e056c() -> ! {
    todo!("0x8e056c rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::~remote_signal()")
}

// 0x8e1440 — __ZN3RBX9GuiBase2d23RecursiveRenderChildrenEN5boost10shared_ptrINS_8InstanceEEEPNS_5AdornE
#[doc(alias = "RBX::GuiBase2d::RecursiveRenderChildren(rbx_core::SharedPtr<RBX::Instance>,RBX::Adorn *)")]
// was: RBX::GuiBase2d::RecursiveRenderChildren(boost::shared_ptr<RBX::Instance>,RBX::Adorn *)
pub fn stub_8e1440() -> ! {
    todo!("0x8e1440 RBX::GuiBase2d::RecursiveRenderChildren(rbx_core::SharedPtr<RBX::Instance>,RBX::Adorn *)")
}

// 0x8e16d4 — __ZN3RBXL14ResizeChildrenEN5boost10shared_ptrINS_8InstanceEEERKN3G3D6Rect2DEb
#[doc(alias = "RBX::ResizeChildren(rbx_core::SharedPtr<RBX::Instance>,G3D::Rect2D const&,bool)")]
// was: RBX::ResizeChildren(boost::shared_ptr<RBX::Instance>,G3D::Rect2D const&,bool)
pub fn stub_8e16d4() -> ! {
    todo!("0x8e16d4 RBX::ResizeChildren(rbx_core::SharedPtr<RBX::Instance>,G3D::Rect2D const&,bool)")
}

// 0x8e1728 — __ZNK3RBX9GuiBase2d11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::GuiBase2d::askAddChild(RBX::Instance const*)const")]
pub fn stub_8e1728() -> ! {
    todo!("0x8e1728 RBX::GuiBase2d::askAddChild(RBX::Instance const*)const")
}

// 0x8e1de8 — __ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIN3G3D6Rect2DEEENS4_IbEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEERKS6_bENS0_5list1IRKSE_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::Rect2D>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Rect2D const&,bool),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Rect2D const&,bool) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::Rect2D>,boost::_bi::value<bool>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,G3D::Rect2D const&,bool),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,G3D::Rect2D const&,bool) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_8e1de8() -> ! {
    todo!("0x8e1de8 void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::Rect2D>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Rect2D const&,bool),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Rect2D const&,bool) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0x8e1ec0 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPN3RBX5AdornEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEES7_ENS0_5list1IRKSD_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Adorn *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Adorn *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Adorn *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Adorn *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::Adorn *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Adorn *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_8e1ec0() -> ! {
    todo!("0x8e1ec0 void boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Adorn *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Adorn *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Adorn *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0x8e2e30 — __ZN3RBX17GuiLayerCollector5LoadZEN5boost10shared_ptrINS_8InstanceEEEPSt6vectorIS5_INS2_INS_7GuiBaseEEESaIS7_EESaIS9_EE
#[doc(alias = "RBX::GuiLayerCollector::LoadZ(rbx_core::SharedPtr<RBX::Instance>,std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>> *)")]
// was: RBX::GuiLayerCollector::LoadZ(boost::shared_ptr<RBX::Instance>,std::vector<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>,std::allocator<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>> *)
pub fn stub_8e2e30() -> ! {
    todo!("0x8e2e30 RBX::GuiLayerCollector::LoadZ(rbx_core::SharedPtr<RBX::Instance>,std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>> *)")
}

// 0x8e32e0 — __ZN3RBX17GuiLayerCollector15render2dContextEPNS_5AdornEPKNS_8InstanceE
#[doc(alias = "RBX::GuiLayerCollector::render2dContext(RBX::Adorn *,RBX::Instance const*)")]
pub fn stub_8e32e0() -> ! {
    todo!("0x8e32e0 RBX::GuiLayerCollector::render2dContext(RBX::Adorn *,RBX::Instance const*)")
}

// 0x8e339c — __ZN3RBX17GuiLayerCollector27render2dStandardGuiElementsEPNS_5AdornEPKNS_8InstanceERSt6vectorIN5boost10shared_ptrINS_7GuiBaseEEESaISA_EERKN3G3D6Rect2DE
#[doc(alias = "RBX::GuiLayerCollector::render2dStandardGuiElements(RBX::Adorn *,RBX::Instance const*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> &,G3D::Rect2D const&)")]
// was: RBX::GuiLayerCollector::render2dStandardGuiElements(RBX::Adorn *,RBX::Instance const*,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> &,G3D::Rect2D const&)
pub fn stub_8e339c() -> ! {
    todo!("0x8e339c RBX::GuiLayerCollector::render2dStandardGuiElements(RBX::Adorn *,RBX::Instance const*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> &,G3D::Rect2D const&)")
}

// 0x8e342c — __ZN3RBX17GuiLayerCollector23render2dTextGuiElementsEPNS_5AdornEPKNS_8InstanceERSt6vectorIN5boost10shared_ptrINS_7GuiBaseEEESaISA_EERKN3G3D6Rect2DE
#[doc(alias = "RBX::GuiLayerCollector::render2dTextGuiElements(RBX::Adorn *,RBX::Instance const*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> &,G3D::Rect2D const&)")]
// was: RBX::GuiLayerCollector::render2dTextGuiElements(RBX::Adorn *,RBX::Instance const*,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> &,G3D::Rect2D const&)
pub fn stub_8e342c() -> ! {
    todo!("0x8e342c RBX::GuiLayerCollector::render2dTextGuiElements(RBX::Adorn *,RBX::Instance const*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> &,G3D::Rect2D const&)")
}

// 0x8e3484 — __ZThn96_N3RBX17GuiLayerCollector15render2dContextEPNS_5AdornEPKNS_8InstanceE
#[doc(alias = "non-virtual thunk toRBX::GuiLayerCollector::render2dContext(RBX::Adorn *,RBX::Instance const*)")]
// was: `non-virtual thunk to'RBX::GuiLayerCollector::render2dContext(RBX::Adorn *,RBX::Instance const*)
pub fn stub_8e3484() -> ! {
    todo!("0x8e3484 non-virtual thunk toRBX::GuiLayerCollector::render2dContext(RBX::Adorn *,RBX::Instance const*)")
}

// 0x8e4060 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPSt6vectorIS5_INS_10shared_ptrIN3RBX7GuiBaseEEESaIS9_EESaISB_EEEEEclIPFvNS6_INS7_8InstanceEEESE_ENS0_5list1IRKSJ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>> *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>> *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>> *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>,std::allocator<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>> *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,std::vector<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>,std::allocator<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>> *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,std::vector<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>,std::allocator<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>> *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_8e4060() -> ! {
    todo!("0x8e4060 void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>> *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>> *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>> *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0x8e5cb8 — __ZN3RBX20ContextActionService6isToolEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::ContextActionService::isTool(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::ContextActionService::isTool(boost::shared_ptr<RBX::Instance>)
pub fn stub_8e5cb8() -> ! {
    todo!("0x8e5cb8 RBX::ContextActionService::isTool(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x8e5cec — __ZN3RBX20ContextActionService15checkForNewToolEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::ContextActionService::checkForNewTool(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::ContextActionService::checkForNewTool(boost::shared_ptr<RBX::Instance>)
pub fn stub_8e5cec() -> ! {
    todo!("0x8e5cec RBX::ContextActionService::checkForNewTool(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x8e5e38 — __ZN3RBX20ContextActionService19checkForToolRemovalEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::ContextActionService::checkForToolRemoval(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::ContextActionService::checkForToolRemoval(boost::shared_ptr<RBX::Instance>)
pub fn stub_8e5e38() -> ! {
    todo!("0x8e5e38 RBX::ContextActionService::checkForToolRemoval(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x8e6108 — __ZN3RBX20ContextActionService19localCharacterAddedEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::ContextActionService::localCharacterAdded(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::ContextActionService::localCharacterAdded(boost::shared_ptr<RBX::Instance>)
pub fn stub_8e6108() -> ! {
    todo!("0x8e6108 RBX::ContextActionService::localCharacterAdded(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x8e615c — __ZN3RBX20ContextActionService19checkForLocalPlayerEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::ContextActionService::checkForLocalPlayer(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::ContextActionService::checkForLocalPlayer(boost::shared_ptr<RBX::Instance>)
pub fn stub_8e615c() -> ! {
    todo!("0x8e615c RBX::ContextActionService::checkForLocalPlayer(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x8e6498 — __ZN3RBX10Reflection9EventDescINS_20ContextActionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*>::~EventDesc()
pub fn stub_8e6498() -> ! {
    todo!("0x8e6498 RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::~EventDesc()")
}

// 0x8e64bc — __ZN3RBX15ServiceProvider4findINS_10GuiServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::GuiService * RBX::ServiceProvider::find<RBX::GuiService>(RBX::Instance const*)")]
pub fn stub_8e64bc() -> ! {
    todo!("0x8e64bc RBX::GuiService * RBX::ServiceProvider::find<RBX::GuiService>(RBX::Instance const*)")
}

// 0x8e6644 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ContextActionServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>> const&)
pub fn stub_8e6644() -> ! {
    todo!("0x8e6644 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>> const&)")
}

// 0x8e6904 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ContextActionServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_8e6904() -> ! {
    todo!("0x8e6904 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x8e6930 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ContextActionServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_8e6930() -> ! {
    todo!("0x8e6930 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x8e6a04 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ContextActionServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_8e6a04() -> ! {
    todo!("0x8e6a04 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x8e6a20 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ContextActionServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_8e6a20() -> ! {
    todo!("0x8e6a20 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x8e6a3c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ContextActionServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ContextActionService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::ContextActionService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ContextActionService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ContextActionService,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_8e6a3c() -> ! {
    todo!("0x8e6a3c void boost::_bi::list2<boost::_bi::value<RBX::ContextActionService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")
}

// 0x8e6b14 — __ZNK5boost4_mfi3mf1IvN3RBX20ContextActionServiceENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
#[doc(alias = "boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ContextActionService*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::ContextActionService,boost::shared_ptr<RBX::Instance>>::operator()(RBX::ContextActionService*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_8e6b14() -> ! {
    todo!("0x8e6b14 boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ContextActionService*,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0x8e6bfc — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ContextActionServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_8e6bfc() -> ! {
    todo!("0x8e6bfc rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

// 0x8e6c28 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ContextActionServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_8e6c28() -> ! {
    todo!("0x8e6c28 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContextActionService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ContextActionService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

// 0x8e6cfc — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_4ToolEEEPKT_v
#[doc(alias = "RBX::Tool const* RBX::Instance::findConstFirstChildOfType<RBX::Tool>(void)const")]
pub fn stub_8e6cfc() -> ! {
    todo!("0x8e6cfc RBX::Tool const* RBX::Instance::findConstFirstChildOfType<RBX::Tool>(void)const")
}

// 0x8e6f60 — __ZN3RBX10Reflection9EventDescINS_20ContextActionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_8e6f60() -> ! {
    todo!("0x8e6f60 RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8e70e4 — __ZN3RBX10Reflection9EventDescINS_20ContextActionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*>::~EventDesc()
pub fn stub_8e70e4() -> ! {
    todo!("0x8e70e4 RBX::Reflection::EventDesc<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::~EventDesc()")
}

// 0x8e7198 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_20ContextActionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::ContextActionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_8e7198() -> ! {
    todo!("0x8e7198 RBX::Reflection::EventDescImpl<1,RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x8e72ec — __ZNK3RBX10Reflection13EventDescImplILi1ENS_20ContextActionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::ContextActionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_8e72ec() -> ! {
    todo!("0x8e72ec RBX::Reflection::EventDescImpl<1,RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8e744c — __ZNK3RBX10Reflection13EventDescBaseINS_20ContextActionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::ContextActionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ContextActionService::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_8e744c() -> ! {
    todo!("0x8e744c RBX::Reflection::EventDescBase<RBX::ContextActionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ContextActionService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x8e81e0 — __ZN3RBX13ScriptService12waitForChildEN5boost8weak_ptrINS_8InstanceEEESsNS1_8functionIFvNS1_10shared_ptrIS3_EEEEENS5_IFvSsEEE
#[doc(alias = "RBX::ScriptService::waitForChild(rbx_core::Weak<RBX::Instance>,std::string,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")]
// was: RBX::ScriptService::waitForChild(boost::weak_ptr<RBX::Instance>,std::string,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::function<void ()(std::string)>)
pub fn stub_8e81e0() -> ! {
    todo!("0x8e81e0 RBX::ScriptService::waitForChild(rbx_core::Weak<RBX::Instance>,std::string,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")
}

// 0x8e83c4 — __ZN3RBX13ScriptService12onChildAddedEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::ScriptService::onChildAdded(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::ScriptService::onChildAdded(boost::shared_ptr<RBX::Instance>)
pub fn stub_8e83c4() -> ! {
    todo!("0x8e83c4 RBX::ScriptService::onChildAdded(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x8e8690 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_13ScriptServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>> const&)
pub fn stub_8e8690() -> ! {
    todo!("0x8e8690 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>> const&)")
}

// 0x8e8d24 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_13ScriptServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_8e8d24() -> ! {
    todo!("0x8e8d24 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x8e8d50 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_13ScriptServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_8e8d50() -> ! {
    todo!("0x8e8d50 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x8e8e24 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_13ScriptServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_8e8e24() -> ! {
    todo!("0x8e8e24 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x8e8e40 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_13ScriptServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_8e8e40() -> ! {
    todo!("0x8e8e40 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x8e8e5c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX13ScriptServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::ScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_8e8e5c() -> ! {
    todo!("0x8e8e5c void boost::_bi::list2<boost::_bi::value<RBX::ScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")
}