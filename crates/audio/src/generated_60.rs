//! audio generated_60 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio exhausted (2541 distinct) — filler workspace EA-sorted asc after 0x5776fc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0x577720..0x57cd40 EA-sorted asc after 0x5776fc, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x577720 — __ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::BoundFuncDesc(void (RBX::HopperBin::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_577720() -> ! {
    todo!("0x577720 RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::BoundFuncDesc(void (RBX::HopperBin::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x577824 — __ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_577824() -> ! {
    todo!("0x577824 RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::~BoundFuncDesc()")
}

// 0x5778d8 — __ZNK3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_5778d8() -> ! {
    todo!("0x5778d8 RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x5778f8 — __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::~EventDesc()")]
pub fn stub_5778f8() -> ! {
    todo!("0x5778f8 RBX::Reflection::EventDesc<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::~EventDesc()")
}

// 0x5779ac — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE // was: boost
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_5779ac() -> ! {
    todo!("0x5779ac RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x577bb0 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_577bb0() -> ! {
    todo!("0x577bb0 RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x577c24 — __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_577c24() -> ! {
    todo!("0x577c24 RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x577c38 — __ZN3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
pub fn stub_577c38() -> ! {
    todo!("0x577c38 RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")
}

// 0x577cec — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE // was: boost
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_577cec() -> ! {
    todo!("0x577cec RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x577f00 — __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")]
pub fn stub_577f00() -> ! {
    todo!("0x577f00 RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")
}

// 0x577f08 — __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")]
pub fn stub_577f08() -> ! {
    todo!("0x577f08 RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")
}

// 0x577f10 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_577f10() -> ! {
    todo!("0x577f10 RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x577f84 — __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_577f84() -> ! {
    todo!("0x577f84 RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x577f94 — __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_577f94() -> ! {
    todo!("0x577f94 RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x577fa8 — __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE // was: boost
#[doc(alias = "RBX::Reflection::EventDesc<RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_577fa8() -> ! {
    todo!("0x577fa8 RBX::Reflection::EventDesc<RBX::HopperBin,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::HopperBin::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::HopperBin::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x57812c — __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev // was: boost
#[doc(alias = "RBX::Reflection::EventDesc<RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::~EventDesc()")]
pub fn stub_57812c() -> ! {
    todo!("0x57812c RBX::Reflection::EventDesc<RBX::HopperBin,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::HopperBin::*>::~EventDesc()")
}

// 0x5781e0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE // was: boost
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_5781e0() -> ! {
    todo!("0x5781e0 RBX::Reflection::EventDescImpl<1,RBX::HopperBin,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x578344 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE // was: boost
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_578344() -> ! {
    todo!("0x578344 RBX::Reflection::EventDescImpl<1,RBX::HopperBin,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x5784a4 — __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE // was: boost
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_5784a4() -> ! {
    todo!("0x5784a4 RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x5784b8 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9HopperBinEEEPKcS7_MT_bMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HopperBin>(char const*,char const*,bool RBX::HopperBin::*,void (RBX::HopperBin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_5784b8() -> ! {
    todo!("0x5784b8 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HopperBin>(char const*,char const*,bool RBX::HopperBin::*,void (RBX::HopperBin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x57864c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::isReadOnly(void)const")]
pub fn stub_57864c() -> ! {
    todo!("0x57864c RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::isReadOnly(void)const")
}

// 0x578650 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::isWriteOnly(void)const")]
pub fn stub_578650() -> ! {
    todo!("0x578650 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::isWriteOnly(void)const")
}

// 0x578654 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_578654() -> ! {
    todo!("0x578654 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x578660 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_578660() -> ! {
    todo!("0x578660 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x5786b0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::EnumPropDescriptor<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>(char const*,char const*,RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_5786b0() -> ! {
    todo!("0x5786b0 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::EnumPropDescriptor<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>(char const*,char const*,RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x578864 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::~EnumPropDescriptor()")]
pub fn stub_578864() -> ! {
    todo!("0x578864 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::~EnumPropDescriptor()")
}

// 0x578890 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::isReadOnly(void)const")]
pub fn stub_578890() -> ! {
    todo!("0x578890 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::isReadOnly(void)const")
}

// 0x5788a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::isWriteOnly(void)const")]
pub fn stub_5788a0() -> ! {
    todo!("0x5788a0 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::isWriteOnly(void)const")
}

// 0x5788b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_5788b0() -> ! {
    todo!("0x5788b0 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x5788d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_5788d8() -> ! {
    todo!("0x5788d8 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x5788fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_5788fc() -> ! {
    todo!("0x5788fc RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x578a48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_578a48() -> ! {
    todo!("0x578a48 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x578a6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::hasStringValue(void)const")]
pub fn stub_578a6c() -> ! {
    todo!("0x578a6c RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::hasStringValue(void)const")
}

// 0x578a70 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_578a70() -> ! {
    todo!("0x578a70 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x578a94 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_578a94() -> ! {
    todo!("0x578a94 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x578ad4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_578ad4() -> ! {
    todo!("0x578ad4 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x578af4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_578af4() -> ! {
    todo!("0x578af4 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x578d34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_578d34() -> ! {
    todo!("0x578d34 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x578d50 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_578d50() -> ! {
    todo!("0x578d50 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x578d84 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_578d84() -> ! {
    todo!("0x578d84 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x578d8c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_578d8c() -> ! {
    todo!("0x578d8c RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x578dd8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_578dd8() -> ! {
    todo!("0x578dd8 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x578df8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_578df8() -> ! {
    todo!("0x578df8 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x578e2c — __ZNK3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::convertToIndex(RBX::HopperBin::BinType)const")]
pub fn stub_578e2c() -> ! {
    todo!("0x578e2c RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::convertToIndex(RBX::HopperBin::BinType)const")
}

// 0x578e9c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_578e9c() -> ! {
    todo!("0x578e9c RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x578edc — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::isReadOnly(void)const")]
pub fn stub_578edc() -> ! {
    todo!("0x578edc RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::isReadOnly(void)const")
}

// 0x578ee0 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::isWriteOnly(void)const")]
pub fn stub_578ee0() -> ! {
    todo!("0x578ee0 RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::isWriteOnly(void)const")
}

// 0x578ee4 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_578ee4() -> ! {
    todo!("0x578ee4 RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x578f04 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::setValue(RBX::Reflection::DescribedBase *,RBX::HopperBin::BinType const&)const")]
pub fn stub_578f04() -> ! {
    todo!("0x578f04 RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::setValue(RBX::Reflection::DescribedBase *,RBX::HopperBin::BinType const&)const")
}

// 0x578f28 — __ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEEC2IMS2_KFKS3_vEMS2_FvRS6_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::PropDescriptor<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>(char const*,char const*,RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_578f28() -> ! {
    todo!("0x578f28 RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::PropDescriptor<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>(char const*,char const*,RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x57903c — __ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::~PropDescriptor()")]
pub fn stub_57903c() -> ! {
    todo!("0x57903c RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::~PropDescriptor()")
}

// 0x579068 — __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::isReadOnly(void)const")]
pub fn stub_579068() -> ! {
    todo!("0x579068 RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::isReadOnly(void)const")
}

// 0x57906c — __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::isWriteOnly(void)const")]
pub fn stub_57906c() -> ! {
    todo!("0x57906c RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::isWriteOnly(void)const")
}

// 0x579070 — __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_579070() -> ! {
    todo!("0x579070 RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x579098 — __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE8setValueEPNS0_13DescribedBaseES9_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
pub fn stub_579098() -> ! {
    todo!("0x579098 RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")
}

// 0x5790bc — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::resize(unsigned long,RBX::HopperBin::BinType)")]
pub fn stub_5790bc() -> ! {
    todo!("0x5790bc std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::resize(unsigned long,RBX::HopperBin::BinType)")
}

// 0x5790f0 — __ZNSt3mapIPKN3RBX4NameENS0_9HopperBin7BinTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::HopperBin::BinType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_5790f0() -> ! {
    todo!("0x5790f0 std::map<RBX::Name const*,RBX::HopperBin::BinType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::operator[](RBX::Name const* const&)")
}

// 0x579148 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
pub fn stub_579148() -> ! {
    todo!("0x579148 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")
}

// 0x5791fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
pub fn stub_5791fc() -> ! {
    todo!("0x5791fc std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")
}

// 0x579254 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
pub fn stub_579254() -> ! {
    todo!("0x579254 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")
}

// 0x5792bc — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,unsigned long,RBX::HopperBin::BinType const&)")]
pub fn stub_5792bc() -> ! {
    todo!("0x5792bc std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,unsigned long,RBX::HopperBin::BinType const&)")
}

// 0x57944c — __ZNSt12_Vector_baseIN3RBX9HopperBin7BinTypeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_allocate(unsigned long)")]
pub fn stub_57944c() -> ! {
    todo!("0x57944c std::_Vector_base<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_allocate(unsigned long)")
}

// 0x579464 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9HopperBin7BinTypeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::HopperBin::BinType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HopperBin::BinType *,RBX::HopperBin::BinType *>(RBX::HopperBin::BinType *,RBX::HopperBin::BinType *,RBX::HopperBin::BinType *)")]
pub fn stub_579464() -> ! {
    todo!("0x579464 RBX::HopperBin::BinType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HopperBin::BinType *,RBX::HopperBin::BinType *>(RBX::HopperBin::BinType *,RBX::HopperBin::BinType *,RBX::HopperBin::BinType *)")
}

// 0x5794a0 — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::push_back(RBX::HopperBin::BinType const&)")]
pub fn stub_5794a0() -> ! {
    todo!("0x5794a0 std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::push_back(RBX::HopperBin::BinType const&)")
}

// 0x5794c8 — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,RBX::HopperBin::BinType const&)")]
pub fn stub_5794c8() -> ! {
    todo!("0x5794c8 std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,RBX::HopperBin::BinType const&)")
}

// 0x5795ac — __ZN3RBX9HopperBinD2Ev
#[doc(alias = "RBX::HopperBin::~HopperBin()")]
pub fn stub_5795ac() -> ! {
    todo!("0x5795ac RBX::HopperBin::~HopperBin()")
}

// 0x579878 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEED2Ev // was: boost
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::~remote_signal()")]
pub fn stub_579878() -> ! {
    todo!("0x579878 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>::~remote_signal()")
}

// 0x5799c4 — __GLOBAL__I_a_212
#[doc(alias = "global constructor keyed to_a_212")]
pub fn stub_5799c4() -> ! {
    todo!("0x5799c4 `global constructor keyed to_a_212")
}

// 0x579f70 — __ZN3RBX17ICharacterSubjectC2Ev
#[doc(alias = "RBX::ICharacterSubject::ICharacterSubject(void)")]
pub fn stub_579f70() -> ! {
    todo!("0x579f70 RBX::ICharacterSubject::ICharacterSubject(void)")
}

// 0x579fcc — __ZN3RBX17ICharacterSubject10initCameraERN3G3D7Vector3ERNS1_15CoordinateFrameE
#[doc(alias = "RBX::ICharacterSubject::initCamera(G3D::Vector3 &,G3D::CoordinateFrame &)")]
pub fn stub_579fcc() -> ! {
    todo!("0x579fcc RBX::ICharacterSubject::initCamera(G3D::Vector3 &,G3D::CoordinateFrame &)")
}

// 0x57a09c — __ZNK3RBX17ICharacterSubject13isFirstPersonEv
#[doc(alias = "RBX::ICharacterSubject::isFirstPerson(void)const")]
pub fn stub_57a09c() -> ! {
    todo!("0x57a09c RBX::ICharacterSubject::isFirstPerson(void)const")
}

// 0x57a0b4 — __ZN3RBX17ICharacterSubject20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
#[doc(alias = "RBX::ICharacterSubject::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
pub fn stub_57a0b4() -> ! {
    todo!("0x57a0b4 RBX::ICharacterSubject::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")
}

// 0x57a17c — __ZN3RBX17ICharacterSubject12doCameraMoveERN3G3D7Vector3ERNS1_15CoordinateFrameEd
#[doc(alias = "RBX::ICharacterSubject::doCameraMove(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
pub fn stub_57a17c() -> ! {
    todo!("0x57a17c RBX::ICharacterSubject::doCameraMove(G3D::Vector3 &,G3D::CoordinateFrame &,double)")
}

// 0x57a6c8 — __ZN3RBX17ICharacterSubject15doCameraOccludeERN3G3D7Vector3ERKNS1_15CoordinateFrameEd
#[doc(alias = "RBX::ICharacterSubject::doCameraOcclude(G3D::Vector3 &,G3D::CoordinateFrame const&,double)")]
pub fn stub_57a6c8() -> ! {
    todo!("0x57a6c8 RBX::ICharacterSubject::doCameraOcclude(G3D::Vector3 &,G3D::CoordinateFrame const&,double)")
}

// 0x57ab88 — __ZNK3RBX17ICharacterSubject19getNearPlaneCornersERN5boost5arrayIN3G3D7Vector3ELm4EEE // was: boost
#[doc(alias = "RBX::ICharacterSubject::getNearPlaneCorners(boost::array<G3D::Vector3,4ul> &)const")]
pub fn stub_57ab88() -> ! {
    todo!("0x57ab88 RBX::ICharacterSubject::getNearPlaneCorners(boost::array<G3D::Vector3,4ul> &)const")
}

// 0x57ad58 — __ZN3RBX17ICharacterSubject16getHalfDistancesERN5boost5arrayIfLm4EEERKN3G3D7Vector3ERKNS5_15CoordinateFrameE // was: boost
#[doc(alias = "RBX::ICharacterSubject::getHalfDistances(boost::array<float,4ul> &,G3D::Vector3 const&,G3D::CoordinateFrame const&)")]
pub fn stub_57ad58() -> ! {
    todo!("0x57ad58 RBX::ICharacterSubject::getHalfDistances(boost::array<float,4ul> &,G3D::Vector3 const&,G3D::CoordinateFrame const&)")
}

// 0x57b03c — __ZN3RBX17ICharacterSubject22characterOcclusionTestERKN3G3D7Vector3ERKNS1_15CoordinateFrameE
#[doc(alias = "RBX::ICharacterSubject::characterOcclusionTest(G3D::Vector3 const&,G3D::CoordinateFrame const&)")]
pub fn stub_57b03c() -> ! {
    todo!("0x57b03c RBX::ICharacterSubject::characterOcclusionTest(G3D::Vector3 const&,G3D::CoordinateFrame const&)")
}

// 0x57b5e4 — __ZN3RBX17ICharacterSubject4zoomEfRN3G3D15CoordinateFrameES3_
#[doc(alias = "RBX::ICharacterSubject::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
pub fn stub_57b5e4() -> ! {
    todo!("0x57b5e4 RBX::ICharacterSubject::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")
}

// 0x57bbe0 — __ZN3RBX17ICharacterSubject17onCameraHeartbeatERKN3G3D7Vector3ES4_
#[doc(alias = "RBX::ICharacterSubject::onCameraHeartbeat(G3D::Vector3 const&,G3D::Vector3 const&)")]
pub fn stub_57bbe0() -> ! {
    todo!("0x57bbe0 RBX::ICharacterSubject::onCameraHeartbeat(G3D::Vector3 const&,G3D::Vector3 const&)")
}

// 0x57bd7c — __ZN3RBX17ICharacterSubject13setCameraModeENS_6Camera10CameraModeE
#[doc(alias = "RBX::ICharacterSubject::setCameraMode(RBX::Camera::CameraMode)")]
pub fn stub_57bd7c() -> ! {
    todo!("0x57bd7c RBX::ICharacterSubject::setCameraMode(RBX::Camera::CameraMode)")
}

// 0x57bd94 — __GLOBAL__I_a_213
#[doc(alias = "global constructor keyed to_a_213")]
pub fn stub_57bd94() -> ! {
    todo!("0x57bd94 `global constructor keyed to_a_213")
}

// 0x57bf9c — __ZN3RBX10IEquipableC2Ev
#[doc(alias = "RBX::IEquipable::IEquipable(void)")]
pub fn stub_57bf9c() -> ! {
    todo!("0x57bf9c RBX::IEquipable::IEquipable(void)")
}

// 0x57bfb4 — __ZN3RBX10IEquipableD0Ev
#[doc(alias = "RBX::IEquipable::~IEquipable()")]
pub fn stub_57bfb4() -> ! {
    todo!("0x57bfb4 RBX::IEquipable::~IEquipable()")
}

// 0x57c054 — __ZN3RBX10IEquipableD1Ev
#[doc(alias = "RBX::IEquipable::~IEquipable()")]
pub fn stub_57c054() -> ! {
    todo!("0x57c054 RBX::IEquipable::~IEquipable()")
}

// 0x57c058 — __ZN3RBX10IEquipableD2Ev
#[doc(alias = "RBX::IEquipable::~IEquipable()")]
pub fn stub_57c058() -> ! {
    todo!("0x57c058 RBX::IEquipable::~IEquipable()")
}

// 0x57c17c — __ZN3RBX10IEquipable9buildWeldEPNS_12PartInstanceES2_RKN3G3D15CoordinateFrameES6_RKSs
#[doc(alias = "RBX::IEquipable::buildWeld(RBX::PartInstance *,RBX::PartInstance *,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,std::string const&)")]
pub fn stub_57c17c() -> ! {
    todo!("0x57c17c RBX::IEquipable::buildWeld(RBX::PartInstance *,RBX::PartInstance *,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,std::string const&)")
}

// 0x57c39c — __ZN5boost10shared_ptrIN3RBX4WeldEEaSERKS3_ // was: boost
#[doc(alias = "rbx_core::SharedPtr<RBX::Weld>::operator=(rbx_core::SharedPtr<RBX::Weld> const&)")]
pub fn stub_57c39c() -> ! {
    todo!("0x57c39c boost::shared_ptr<RBX::Weld>::operator=(boost::shared_ptr<RBX::Weld> const&)")
}

// 0x57c3d4 — __GLOBAL__I_a_214
#[doc(alias = "global constructor keyed to_a_214")]
pub fn stub_57c3d4() -> ! {
    todo!("0x57c3d4 `global constructor keyed to_a_214")
}

// 0x57c644 — __ZN3RBX14GuiImageButtonC2Ev
#[doc(alias = "RBX::GuiImageButton::GuiImageButton(void)")]
pub fn stub_57c644() -> ! {
    todo!("0x57c644 RBX::GuiImageButton::GuiImageButton(void)")
}

// 0x57c894 — __ZN3RBX14GuiImageButtonC1EPNS_4VerbE
#[doc(alias = "RBX::GuiImageButton::GuiImageButton(RBX::Verb *)")]
pub fn stub_57c894() -> ! {
    todo!("0x57c894 RBX::GuiImageButton::GuiImageButton(RBX::Verb *)")
}

// 0x57c898 — __ZN3RBX14GuiImageButtonC2EPNS_4VerbE
#[doc(alias = "RBX::GuiImageButton::GuiImageButton(RBX::Verb *)")]
pub fn stub_57c898() -> ! {
    todo!("0x57c898 RBX::GuiImageButton::GuiImageButton(RBX::Verb *)")
}

// 0x57caf4 — __ZN3RBX14GuiImageButton8setImageENS_9TextureIdE
#[doc(alias = "RBX::GuiImageButton::setImage(RBX::TextureId)")]
pub fn stub_57caf4() -> ! {
    todo!("0x57caf4 RBX::GuiImageButton::setImage(RBX::TextureId)")
}

// 0x57cb34 — __ZThn800_N3RBX14GuiImageButton8setImageENS_9TextureIdE
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::setImage(RBX::TextureId)")]
pub fn stub_57cb34() -> ! {
    todo!("0x57cb34 `non-virtual thunk to'RBX::GuiImageButton::setImage(RBX::TextureId)")
}

// 0x57cb3c — __ZN3RBX14GuiImageButton18setImageRectOffsetEN3G3D7Vector2E
#[doc(alias = "RBX::GuiImageButton::setImageRectOffset(G3D::Vector2)")]
pub fn stub_57cb3c() -> ! {
    todo!("0x57cb3c RBX::GuiImageButton::setImageRectOffset(G3D::Vector2)")
}

// 0x57cb88 — __ZThn800_N3RBX14GuiImageButton18setImageRectOffsetEN3G3D7Vector2E
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::setImageRectOffset(G3D::Vector2)")]
pub fn stub_57cb88() -> ! {
    todo!("0x57cb88 `non-virtual thunk to'RBX::GuiImageButton::setImageRectOffset(G3D::Vector2)")
}

// 0x57cb90 — __ZN3RBX14GuiImageButton16setImageRectSizeEN3G3D7Vector2E
#[doc(alias = "RBX::GuiImageButton::setImageRectSize(G3D::Vector2)")]
pub fn stub_57cb90() -> ! {
    todo!("0x57cb90 RBX::GuiImageButton::setImageRectSize(G3D::Vector2)")
}

// 0x57cbdc — __ZThn800_N3RBX14GuiImageButton16setImageRectSizeEN3G3D7Vector2E
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::setImageRectSize(G3D::Vector2)")]
pub fn stub_57cbdc() -> ! {
    todo!("0x57cbdc `non-virtual thunk to'RBX::GuiImageButton::setImageRectSize(G3D::Vector2)")
}

// 0x57cbe4 — __ZN3RBX14GuiImageButton8render2dEPNS_5AdornE
#[doc(alias = "RBX::GuiImageButton::render2d(RBX::Adorn *)")]
pub fn stub_57cbe4() -> ! {
    todo!("0x57cbe4 RBX::GuiImageButton::render2d(RBX::Adorn *)")
}

// 0x57cd38 — __ZThn96_N3RBX14GuiImageButton8render2dEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::render2d(RBX::Adorn *)")]
pub fn stub_57cd38() -> ! {
    todo!("0x57cd38 `non-virtual thunk to'RBX::GuiImageButton::render2d(RBX::Adorn *)")
}

// 0x57cd40 — __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::~PropDescriptor()")]
pub fn stub_57cd40() -> ! {
    todo!("0x57cd40 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::~PropDescriptor()")
}
