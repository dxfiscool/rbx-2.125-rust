//! network generated_21 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator|Replica|Remote (5974 funcs, 100 stubs here, shard watchdog refill 2, EA-sorted ascending earliest gap).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Boost types mapped: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> Weak, with // was: original.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;

// 0x28cc94 — __ZL17RemoteCheatHelperN5boost8weak_ptrIN3RBX9DataModelEEE // was: boost::weak_ptr
#[doc(alias = "__ZL17RemoteCheatHelperN5boost8weak_ptrIN3RBX9DataModelEEE")]
pub fn stub_28cc94() -> ! {
    todo!("0x28cc94 RemoteCheatHelper(boost::weak_ptr<RBX::DataModel>)")
}

// 0x3252f8 — __ZN3RBX18InterpolatedCFrame8setValueEPNS_12PartInstanceERKN3G3D15CoordinateFrameERKNS_10RemoteTimeE
#[doc(alias = "__ZN3RBX18InterpolatedCFrame8setValueEPNS_12PartInstanceERKN3G3D15CoordinateFrameERKNS_10RemoteTimeE")]
pub fn stub_3252f8() -> ! {
    todo!("0x3252f8 RBX::InterpolatedCFrame::setValue(RBX::PartInstance *,G3D::CoordinateFrame const&,RBX::RemoteTime const&)")
}

// 0x325b4c — __ZN3RBX18InterpolatedCFrame16setRenderedFrameERKN3G3D15CoordinateFrameERKNS_10RemoteTimeE
#[doc(alias = "__ZN3RBX18InterpolatedCFrame16setRenderedFrameERKN3G3D15CoordinateFrameERKNS_10RemoteTimeE")]
pub fn stub_325b4c() -> ! {
    todo!("0x325b4c RBX::InterpolatedCFrame::setRenderedFrame(G3D::CoordinateFrame const&,RBX::RemoteTime const&)")
}

// 0x3f2884 — __ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev // was: boost::shared_ptr
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev")]
pub fn stub_3f2884() -> ! {
    todo!("0x3f2884 RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::~EventDesc()")
}

// 0x419fd8 — __ZN3RBX9DataModel18setRemoteBuildModeEb
// type: int __fastcall(int this, bool)
#[doc(alias = "__ZN3RBX9DataModel18setRemoteBuildModeEb")]
pub fn stub_419fd8() -> ! {
    todo!("0x419fd8 RBX::DataModel::setRemoteBuildMode(bool)")
}

// 0x419fe0 — __ZN3RBX9DataModel18getRemoteBuildModeEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "__ZN3RBX9DataModel18getRemoteBuildModeEv")]
pub fn stub_419fe0() -> ! {
    todo!("0x419fe0 RBX::DataModel::getRemoteBuildMode(void)")
}

// 0x496344 — __ZN3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEED1Ev // was: boost::shared_ptr
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEED1Ev")]
pub fn stub_496344() -> ! {
    todo!("0x496344 RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::~RemoteEventDesc()")
}

// 0x497918 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_S6_ // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_S6_")]
pub fn stub_497918() -> ! {
    todo!("0x497918 RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::fireEvent(RBX::DialogRoot*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)const")
}

// 0x497ba4 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EEC2Ev // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EEC2Ev")]
pub fn stub_497ba4() -> ! {
    todo!("0x497ba4 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::remote_signal(void)")
}

// 0x4988dc — __ZN3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEED0Ev // was: boost::shared_ptr
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEED0Ev")]
pub fn stub_4988dc() -> ! {
    todo!("0x4988dc RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::~RemoteEventDesc()")
}

// 0x498990 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
pub fn stub_498990() -> ! {
    todo!("0x498990 RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x498af4 — __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE12isScriptableEv // was: boost::shared_ptr
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE12isScriptableEv")]
pub fn stub_498af4() -> ! {
    todo!("0x498af4 RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::isScriptable(void)const")
}

// 0x498afc — __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE11isBroadcastEv // was: boost::shared_ptr
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE11isBroadcastEv")]
pub fn stub_498afc() -> ! {
    todo!("0x498afc RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::isBroadcast(void)const")
}

// 0x498b04 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
pub fn stub_498b04() -> ! {
    todo!("0x498b04 RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x498cb4 — __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE // was: boost::shared_ptr
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE")]
pub fn stub_498cb4() -> ! {
    todo!("0x498cb4 RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x498cc4 — __ZNK3RBX10Reflection13EventDescBaseINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE // was: boost::shared_ptr
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_498cc4() -> ! {
    todo!("0x498cc4 RBX::Reflection::EventDescBase<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x498cd8 — __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE // was: boost::shared_ptr
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_498cd8() -> ! {
    todo!("0x498cd8 RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x498ec8 — __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev // was: boost::shared_ptr
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev")]
pub fn stub_498ec8() -> ! {
    todo!("0x498ec8 RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::~EventDesc()")
}

// 0x498eec — __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev // was: boost::shared_ptr
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev")]
pub fn stub_498eec() -> ! {
    todo!("0x498eec RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::~EventDesc()")
}

// 0x49ad94 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EED2Ev // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EED2Ev")]
pub fn stub_49ad94() -> ! {
    todo!("0x49ad94 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~remote_signal()")
}

// 0x4d5c78 — __ZN3RBX10Reflection9DescribedINS_11RemoteEventELZNS_12sRemoteEventEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sRemoteEventEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11RemoteEventELZNS_12sRemoteEventEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sRemoteEventEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4d5c78() -> ! {
    todo!("0x4d5c78 __ZN3RBX10Reflection9DescribedINS_11RemoteEventELZNS_12sRemoteEventEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sRemoteEventEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x4d5d98 — __ZN3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4d5d98() -> ! {
    todo!("0x4d5d98 __ZN3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x52bfa4 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_52bfa4() -> ! {
    todo!("0x52bfa4 RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::~RemoteEventDesc()")
}

// 0x52bfc8 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEED1Ev")]
pub fn stub_52bfc8() -> ! {
    todo!("0x52bfc8 RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::~RemoteEventDesc()")
}

// 0x52de40 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_52de40() -> ! {
    todo!("0x52de40 RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")
}

// 0x52de64 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_52de64() -> ! {
    todo!("0x52de64 RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::~RemoteEventDesc()")
}

// 0x52f410 — __ZN3rbx13remote_signalIFvvEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvvEEC2Ev")]
pub fn stub_52f410() -> ! {
    todo!("0x52f410 rbx::remote_signal<void ()(void)>::remote_signal(void)")
}

// 0x52f650 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_52f650() -> ! {
    todo!("0x52f650 RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::getSignalPtr(RBX::Reflection::EventSource *)")
}

// 0x530250 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_530250() -> ! {
    todo!("0x530250 RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>>::getSignalPtr(RBX::Reflection::EventSource *)")
}

// 0x531978 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEED0Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_531978() -> ! {
    todo!("0x531978 RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::~RemoteEventDesc()")
}

// 0x531a2c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_531a2c() -> ! {
    todo!("0x531a2c RBX::Reflection::EventDescImpl<2,RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiButton::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x531b90 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE12isScriptableEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_531b90() -> ! {
    todo!("0x531b90 RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::isScriptable(void)const")
}

// 0x531b98 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE11isBroadcastEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_531b98() -> ! {
    todo!("0x531b98 RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::isBroadcast(void)const")
}

// 0x531ba0 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_531ba0() -> ! {
    todo!("0x531ba0 RBX::Reflection::EventDescImpl<2,RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiButton::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x531c3c — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_531c3c() -> ! {
    todo!("0x531c3c RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x531c4c — __ZNK3RBX10Reflection13EventDescBaseINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_531c4c() -> ! {
    todo!("0x531c4c RBX::Reflection::EventDescBase<RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiButton::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x532d94 — __ZN3RBX10Reflection9EventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_532d94() -> ! {
    todo!("0x532d94 RBX::Reflection::EventDesc<RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiButton::*>::EventDesc(rbx::remote_signal<void ()(int,int)> RBX::GuiButton::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x532f84 — __ZN3RBX10Reflection9EventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_532f84() -> ! {
    todo!("0x532f84 RBX::Reflection::EventDesc<RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiButton::*>::~EventDesc()")
}

// 0x532fa8 — __ZN3RBX10Reflection9EventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_532fa8() -> ! {
    todo!("0x532fa8 RBX::Reflection::EventDesc<RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiButton::*>::~EventDesc()")
}

// 0x53305c — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEED0Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_53305c() -> ! {
    todo!("0x53305c RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")
}

// 0x533110 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_533110() -> ! {
    todo!("0x533110 RBX::Reflection::EventDescImpl<0,RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::GuiButton::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x533324 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_533324() -> ! {
    todo!("0x533324 RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")
}

// 0x53332c — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_53332c() -> ! {
    todo!("0x53332c RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")
}

// 0x533334 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_533334() -> ! {
    todo!("0x533334 RBX::Reflection::EventDescImpl<0,RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::GuiButton::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x5333a8 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_5333a8() -> ! {
    todo!("0x5333a8 RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x5333b8 — __ZNK3RBX10Reflection13EventDescBaseINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_5333b8() -> ! {
    todo!("0x5333b8 RBX::Reflection::EventDescBase<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::GuiButton::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x537488 — __ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev")]
pub fn stub_537488() -> ! {
    todo!("0x537488 rbx::remote_signal<void ()(RBX::UDim2)>::remote_signal(void)")
}

// 0x5375e4 — __ZN3rbx13remote_signalIFviiEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFviiEEC2Ev")]
pub fn stub_5375e4() -> ! {
    todo!("0x5375e4 rbx::remote_signal<void ()(int,int)>::remote_signal(void)")
}

// 0x537834 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_537834() -> ! {
    todo!("0x537834 RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::getSignalPtr(RBX::Reflection::EventSource *)")
}

// 0x538438 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_538438() -> ! {
    todo!("0x538438 RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::getSignalPtr(RBX::Reflection::EventSource *)")
}

// 0x53a100 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEED0Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEED0Ev")]
pub fn stub_53a100() -> ! {
    todo!("0x53a100 RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::~RemoteEventDesc()")
}

// 0x53a1b4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_53a1b4() -> ! {
    todo!("0x53a1b4 RBX::Reflection::EventDescImpl<1,RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>,rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x53a318 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12isScriptableEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12isScriptableEv")]
pub fn stub_53a318() -> ! {
    todo!("0x53a318 RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::isScriptable(void)const")
}

// 0x53a320 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE11isBroadcastEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE11isBroadcastEv")]
pub fn stub_53a320() -> ! {
    todo!("0x53a320 RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::isBroadcast(void)const")
}

// 0x53a328 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
pub fn stub_53a328() -> ! {
    todo!("0x53a328 RBX::Reflection::EventDescImpl<1,RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>,rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x53a3bc — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_53a3bc() -> ! {
    todo!("0x53a3bc RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x53a3cc — __ZNK3RBX10Reflection13EventDescBaseINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_53a3cc() -> ! {
    todo!("0x53a3cc RBX::Reflection::EventDescBase<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>,rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x53b608 — __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_53b608() -> ! {
    todo!("0x53b608 RBX::Reflection::EventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>,rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*>::EventDesc(rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x53b78c — __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev")]
pub fn stub_53b78c() -> ! {
    todo!("0x53b78c RBX::Reflection::EventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>,rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*>::~EventDesc()")
}

// 0x53b7b0 — __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev")]
pub fn stub_53b7b0() -> ! {
    todo!("0x53b7b0 RBX::Reflection::EventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>,rbx::remote_signal<void ()(RBX::UDim2)> RBX::GuiObject::*>::~EventDesc()")
}

// 0x53b864 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEED0Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_53b864() -> ! {
    todo!("0x53b864 RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::~RemoteEventDesc()")
}

// 0x53b918 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_53b918() -> ! {
    todo!("0x53b918 RBX::Reflection::EventDescImpl<2,RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x53ba7c — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12isScriptableEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_53ba7c() -> ! {
    todo!("0x53ba7c RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::isScriptable(void)const")
}

// 0x53ba84 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE11isBroadcastEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_53ba84() -> ! {
    todo!("0x53ba84 RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::isBroadcast(void)const")
}

// 0x53ba8c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_53ba8c() -> ! {
    todo!("0x53ba8c RBX::Reflection::EventDescImpl<2,RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x53bb28 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_53bb28() -> ! {
    todo!("0x53bb28 RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x53bb38 — __ZNK3RBX10Reflection13EventDescBaseINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_53bb38() -> ! {
    todo!("0x53bb38 RBX::Reflection::EventDescBase<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x53bb4c — __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_53bb4c() -> ! {
    todo!("0x53bb4c RBX::Reflection::EventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*>::EventDesc(rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x53bd3c — __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_53bd3c() -> ! {
    todo!("0x53bd3c RBX::Reflection::EventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*>::~EventDesc()")
}

// 0x53bd60 — __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_53bd60() -> ! {
    todo!("0x53bd60 RBX::Reflection::EventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*>::~EventDesc()")
}

// 0x53fea0 — __ZN3rbx13remote_signalIFvvEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvvEED2Ev")]
pub fn stub_53fea0() -> ! {
    todo!("0x53fea0 rbx::remote_signal<void ()(void)>::~remote_signal()")
}

// 0x5594a0 — __ZN3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_5594a0() -> ! {
    todo!("0x5594a0 RBX::Reflection::RemoteEventDesc<RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")
}

// 0x5595e0 — __ZN3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_5595e0() -> ! {
    todo!("0x5595e0 RBX::Reflection::RemoteEventDesc<RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")
}

// 0x55f3e4 — __ZN3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEED0Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_55f3e4() -> ! {
    todo!("0x55f3e4 RBX::Reflection::RemoteEventDesc<RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")
}

// 0x55f498 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_55f498() -> ! {
    todo!("0x55f498 RBX::Reflection::EventDescImpl<0,RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::BodyPosition::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x55f6ac — __ZNK3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_55f6ac() -> ! {
    todo!("0x55f6ac RBX::Reflection::RemoteEventDesc<RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")
}

// 0x55f6b4 — __ZNK3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_55f6b4() -> ! {
    todo!("0x55f6b4 RBX::Reflection::RemoteEventDesc<RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")
}

// 0x55f6bc — __ZNK3RBX10Reflection13EventDescImplILi0ENS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_55f6bc() -> ! {
    todo!("0x55f6bc RBX::Reflection::EventDescImpl<0,RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::BodyPosition::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x55f730 — __ZNK3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_55f730() -> ! {
    todo!("0x55f730 RBX::Reflection::RemoteEventDesc<RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x55f740 — __ZNK3RBX10Reflection13EventDescBaseINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_55f740() -> ! {
    todo!("0x55f740 RBX::Reflection::EventDescBase<RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::BodyPosition::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x561050 — __ZN3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEED0Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_561050() -> ! {
    todo!("0x561050 RBX::Reflection::RemoteEventDesc<RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")
}

// 0x561104 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_6RocketEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_6RocketEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_561104() -> ! {
    todo!("0x561104 RBX::Reflection::EventDescImpl<0,RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Rocket::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x561318 — __ZNK3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_561318() -> ! {
    todo!("0x561318 RBX::Reflection::RemoteEventDesc<RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")
}

// 0x561320 — __ZNK3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_561320() -> ! {
    todo!("0x561320 RBX::Reflection::RemoteEventDesc<RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")
}

// 0x561328 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_6RocketEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_6RocketEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_561328() -> ! {
    todo!("0x561328 RBX::Reflection::EventDescImpl<0,RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Rocket::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x56139c — __ZNK3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_6RocketEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_56139c() -> ! {
    todo!("0x56139c RBX::Reflection::RemoteEventDesc<RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x5613ac — __ZNK3RBX10Reflection13EventDescBaseINS_6RocketEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_6RocketEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_5613ac() -> ! {
    todo!("0x5613ac RBX::Reflection::EventDescBase<RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Rocket::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x567708 — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEED1Ev")]
pub fn stub_567708() -> ! {
    todo!("0x567708 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::~RemoteEventDesc()")
}

// 0x56772c — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED1Ev")]
pub fn stub_56772c() -> ! {
    todo!("0x56772c RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::~RemoteEventDesc()")
}

// 0x569460 — __ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEEC2Ev")]
pub fn stub_569460() -> ! {
    todo!("0x569460 rbx::remote_signal<void ()(RBX::NormalId,float)>::remote_signal(void)")
}

// 0x569734 — __ZN3rbx13remote_signalIFvN3RBX8NormalIdEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX8NormalIdEEEC2Ev")]
pub fn stub_569734() -> ! {
    todo!("0x569734 rbx::remote_signal<void ()(RBX::NormalId)>::remote_signal(void)")
}

// 0x569afc — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_569afc() -> ! {
    todo!("0x569afc RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::getSignalPtr(RBX::Reflection::EventSource *)")
}

// 0x56a700 — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_56a700() -> ! {
    todo!("0x56a700 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::getSignalPtr(RBX::Reflection::EventSource *)")
}

// 0x56b5a0 — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED0Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED0Ev")]
pub fn stub_56b5a0() -> ! {
    todo!("0x56b5a0 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::~RemoteEventDesc()")
}

// 0x56b654 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_56b654() -> ! {
    todo!("0x56b654 RBX::Reflection::EventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x56b7b8 — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12isScriptableEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12isScriptableEv")]
pub fn stub_56b7b8() -> ! {
    todo!("0x56b7b8 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::isScriptable(void)const")
}

// 0x56b7c0 — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE11isBroadcastEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE11isBroadcastEv")]
pub fn stub_56b7c0() -> ! {
    todo!("0x56b7c0 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::isBroadcast(void)const")
}

// 0x56b7c8 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
pub fn stub_56b7c8() -> ! {
    todo!("0x56b7c8 RBX::Reflection::EventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x56b864 — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_56b864() -> ! {
    todo!("0x56b864 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x56b874 — __ZNK3RBX10Reflection13EventDescBaseINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_56b874() -> ! {
    todo!("0x56b874 RBX::Reflection::EventDescBase<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}
