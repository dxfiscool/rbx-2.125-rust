//! network generated_20 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator (4797 strict, 6004 with replica/remote expansion for BG5, 120 stubs here, 5199+120=5319 total, shard BG5, EA-sorted ascending earliest gap).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Boost types mapped: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> Weak, with // was: original.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;


// 0x25f66c — __ZN3RBX10Reflection17RemoteEventCommon10Attributes10deprecatedENS1_13FunctionalityEPKNS0_16MemberDescriptorE
// type: int __fastcall(int result, int, int)
#[doc(alias = "__ZN3RBX10Reflection17RemoteEventCommon10Attributes10deprecatedENS1_13FunctionalityEPKNS0_16MemberDescriptorE")]
pub fn stub_25f66c() -> ! {
    todo!("0x25f66c RBX::Reflection::RemoteEventCommon::Attributes::deprecated(RBX::Reflection::RemoteEventCommon::Functionality,RBX::Reflection::MemberDescriptor const*)")
}

// 0x25f678 — __ZN3RBX10Reflection11EventSource18processRemoteEventERKNS0_15EventDescriptorERKSt6vectorINS0_7VariantESaIS6_EERKNS_13SystemAddressE
// type: int __fastcall(int, int)
#[doc(alias = "__ZN3RBX10Reflection11EventSource18processRemoteEventERKNS0_15EventDescriptorERKSt6vectorINS0_7VariantESaIS6_EERKNS_13SystemAddressE")]
pub fn stub_25f678() -> ! {
    todo!("0x25f678 RBX::Reflection::EventSource::processRemoteEvent(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const&)")
}

// 0x39c124 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_39c124() -> ! {
    todo!("0x39c124 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::~RemoteEventDesc()")
}

// 0x39c148 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_39c148() -> ! {
    todo!("0x39c148 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::~RemoteEventDesc()")
}

// 0x39c16c — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_39c16c() -> ! {
    todo!("0x39c16c RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::~RemoteEventDesc()")
}

// 0x39c190 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_39c190() -> ! {
    todo!("0x39c190 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")
}

// 0x39c724 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ss
// type: void __fastcall(int, int, std::string *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ss")]
pub fn stub_39c724() -> ! {
    todo!("0x39c724 RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::fireEvent(RBX::AnimationTrackState*,std::string)const")
}

// 0x39ec94 — __ZN3rbx13remote_signalIFvfffEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvfffEEC2Ev")]
pub fn stub_39ec94() -> ! {
    todo!("0x39ec94 rbx::remote_signal<void ()(float,float,float)>::remote_signal(void)")
}

// 0x39ef68 — __ZN3rbx13remote_signalIFvffEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvffEEC2Ev")]
pub fn stub_39ef68() -> ! {
    todo!("0x39ef68 rbx::remote_signal<void ()(float,float)>::remote_signal(void)")
}

// 0x39f0c4 — __ZN3rbx13remote_signalIFvffffEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvffffEEC2Ev")]
pub fn stub_39f0c4() -> ! {
    todo!("0x39f0c4 rbx::remote_signal<void ()(float,float,float,float)>::remote_signal(void)")
}

// 0x39f594 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_39f594() -> ! {
    todo!("0x39f594 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")
}

// 0x39f648 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_39f648() -> ! {
    todo!("0x39f648 RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x39f7ac — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_39f7ac() -> ! {
    todo!("0x39f7ac RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")
}

// 0x39f7b4 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_39f7b4() -> ! {
    todo!("0x39f7b4 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")
}

// 0x39f7bc — __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_39f7bc() -> ! {
    todo!("0x39f7bc RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x39f960 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_39f960() -> ! {
    todo!("0x39f960 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x39f970 — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_39f970() -> ! {
    todo!("0x39f970 RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x39f984 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_39f984() -> ! {
    todo!("0x39f984 RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x39fb08 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_39fb08() -> ! {
    todo!("0x39fb08 RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x39fb2c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_39fb2c() -> ! {
    todo!("0x39fb2c RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x39fbe0 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_39fbe0() -> ! {
    todo!("0x39fbe0 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::~RemoteEventDesc()")
}

// 0x39fc94 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_39fc94() -> ! {
    todo!("0x39fc94 RBX::Reflection::EventDescImpl<3,RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x39fdf8 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_39fdf8() -> ! {
    todo!("0x39fdf8 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::isScriptable(void)const")
}

// 0x39fe00 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_39fe00() -> ! {
    todo!("0x39fe00 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::isBroadcast(void)const")
}

// 0x39fe08 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_39fe08() -> ! {
    todo!("0x39fe08 RBX::Reflection::EventDescImpl<3,RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x39feb0 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_39feb0() -> ! {
    todo!("0x39feb0 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x39fec0 — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_39fec0() -> ! {
    todo!("0x39fec0 RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3a105c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3a105c() -> ! {
    todo!("0x3a105c RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3a12b8 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_3a12b8() -> ! {
    todo!("0x3a12b8 RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x3a12dc — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_3a12dc() -> ! {
    todo!("0x3a12dc RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x3a1390 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_3a1390() -> ! {
    todo!("0x3a1390 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::~RemoteEventDesc()")
}

// 0x3a1444 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_3a1444() -> ! {
    todo!("0x3a1444 RBX::Reflection::EventDescImpl<2,RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x3a15a8 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_3a15a8() -> ! {
    todo!("0x3a15a8 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::isScriptable(void)const")
}

// 0x3a15b0 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_3a15b0() -> ! {
    todo!("0x3a15b0 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::isBroadcast(void)const")
}

// 0x3a15b8 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_3a15b8() -> ! {
    todo!("0x3a15b8 RBX::Reflection::EventDescImpl<2,RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3a1654 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_3a1654() -> ! {
    todo!("0x3a1654 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3a1664 — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3a1664() -> ! {
    todo!("0x3a1664 RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3a1678 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3a1678() -> ! {
    todo!("0x3a1678 RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3a1868 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_3a1868() -> ! {
    todo!("0x3a1868 RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x3a188c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_3a188c() -> ! {
    todo!("0x3a188c RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x3a1940 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_3a1940() -> ! {
    todo!("0x3a1940 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::~RemoteEventDesc()")
}

// 0x3a19f4 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_3a19f4() -> ! {
    todo!("0x3a19f4 RBX::Reflection::EventDescImpl<4,RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x3a1b58 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_3a1b58() -> ! {
    todo!("0x3a1b58 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::isScriptable(void)const")
}

// 0x3a1b60 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_3a1b60() -> ! {
    todo!("0x3a1b60 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::isBroadcast(void)const")
}

// 0x3a1b68 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_3a1b68() -> ! {
    todo!("0x3a1b68 RBX::Reflection::EventDescImpl<4,RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3a1c2c — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_3a1c2c() -> ! {
    todo!("0x3a1c2c RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3a1c3c — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3a1c3c() -> ! {
    todo!("0x3a1c3c RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3a2e1c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3a2e1c() -> ! {
    todo!("0x3a2e1c RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3a30e8 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_3a30e8() -> ! {
    todo!("0x3a30e8 RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x3a310c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_3a310c() -> ! {
    todo!("0x3a310c RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x3a31c0 — __ZN3rbx13remote_signalIFvffffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvffffEED2Ev")]
pub fn stub_3a31c0() -> ! {
    todo!("0x3a31c0 rbx::remote_signal<void ()(float,float,float,float)>::~remote_signal()")
}

// 0x3a330c — __ZN3rbx13remote_signalIFvfffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvfffEED2Ev")]
pub fn stub_3a330c() -> ! {
    todo!("0x3a330c rbx::remote_signal<void ()(float,float,float)>::~remote_signal()")
}

// 0x3a3458 — __ZN3rbx13remote_signalIFvffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvffEED2Ev")]
pub fn stub_3a3458() -> ! {
    todo!("0x3a3458 rbx::remote_signal<void ()(float,float)>::~remote_signal()")
}

// 0x3a7f20 — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEED1Ev")]
pub fn stub_3a7f20() -> ! {
    todo!("0x3a7f20 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::~RemoteEventDesc()")
}

// 0x3a7f44 — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEED1Ev")]
pub fn stub_3a7f44() -> ! {
    todo!("0x3a7f44 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::~RemoteEventDesc()")
}

// 0x3a9ea0 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEEC2Ev")]
pub fn stub_3a9ea0() -> ! {
    todo!("0x3a9ea0 rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::remote_signal(void)")
}

// 0x3aa174 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEEC2Ev")]
pub fn stub_3aa174() -> ! {
    todo!("0x3aa174 rbx::remote_signal<void ()(G3D::Vector3::Axis)>::remote_signal(void)")
}

// 0x3aa53c — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE12getSignalPtrEPNS0_11EventSourceE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_3aa53c() -> ! {
    todo!("0x3aa53c RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::getSignalPtr(RBX::Reflection::EventSource *)")
}

// 0x3ab198 — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE12getSignalPtrEPNS0_11EventSourceE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_3ab198() -> ! {
    todo!("0x3ab198 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::getSignalPtr(RBX::Reflection::EventSource *)")
}

// 0x3ace20 — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEED0Ev")]
pub fn stub_3ace20() -> ! {
    todo!("0x3ace20 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::~RemoteEventDesc()")
}

// 0x3aced4 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_3aced4() -> ! {
    todo!("0x3aced4 RBX::Reflection::EventDescImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x3ad038 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE12isScriptableEv")]
pub fn stub_3ad038() -> ! {
    todo!("0x3ad038 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::isScriptable(void)const")
}

// 0x3ad040 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE11isBroadcastEv")]
pub fn stub_3ad040() -> ! {
    todo!("0x3ad040 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::isBroadcast(void)const")
}

// 0x3ad048 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE")]
pub fn stub_3ad048() -> ! {
    todo!("0x3ad048 RBX::Reflection::EventDescImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3ad0f0 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE")]
pub fn stub_3ad0f0() -> ! {
    todo!("0x3ad0f0 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3ad100 — __ZNK3RBX10Reflection13EventDescBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3ad100() -> ! {
    todo!("0x3ad100 RBX::Reflection::EventDescBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3ae298 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_EC2ESA_PKcSD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_EC2ESA_PKcSD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3ae298() -> ! {
    todo!("0x3ae298 RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::EventDesc(rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3ae4f4 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_ED1Ev")]
pub fn stub_3ae4f4() -> ! {
    todo!("0x3ae4f4 RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::~EventDesc()")
}

// 0x3ae518 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_ED0Ev")]
pub fn stub_3ae518() -> ! {
    todo!("0x3ae518 RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::~EventDesc()")
}

// 0x3ae5cc — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEED0Ev")]
pub fn stub_3ae5cc() -> ! {
    todo!("0x3ae5cc RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::~RemoteEventDesc()")
}

// 0x3ae680 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_3ae680() -> ! {
    todo!("0x3ae680 RBX::Reflection::EventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x3ae7e4 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE12isScriptableEv")]
pub fn stub_3ae7e4() -> ! {
    todo!("0x3ae7e4 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::isScriptable(void)const")
}

// 0x3ae7ec — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE11isBroadcastEv")]
pub fn stub_3ae7ec() -> ! {
    todo!("0x3ae7ec RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::isBroadcast(void)const")
}

// 0x3ae7f4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE")]
pub fn stub_3ae7f4() -> ! {
    todo!("0x3ae7f4 RBX::Reflection::EventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3ae880 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE")]
pub fn stub_3ae880() -> ! {
    todo!("0x3ae880 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3ae890 — __ZNK3RBX10Reflection13EventDescBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3ae890() -> ! {
    todo!("0x3ae890 RBX::Reflection::EventDescBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3af9a8 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3af9a8() -> ! {
    todo!("0x3af9a8 RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::EventDesc(rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3afb2c — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_ED1Ev")]
pub fn stub_3afb2c() -> ! {
    todo!("0x3afb2c RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::~EventDesc()")
}

// 0x3afb50 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_ED0Ev")]
pub fn stub_3afb50() -> ! {
    todo!("0x3afb50 RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::~EventDesc()")
}

// 0x3b0324 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEED2Ev")]
pub fn stub_3b0324() -> ! {
    todo!("0x3b0324 rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::~remote_signal()")
}

// 0x3b0470 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEED2Ev")]
pub fn stub_3b0470() -> ! {
    todo!("0x3b0470 rbx::remote_signal<void ()(G3D::Vector3::Axis)>::~remote_signal()")
}

// 0x3b5318 — __ZN3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_3b5318() -> ! {
    todo!("0x3b5318 RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")
}

// 0x3b611c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ss
// type: void __fastcall(int, int, std::string *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ss")]
pub fn stub_3b611c() -> ! {
    todo!("0x3b611c RBX::Reflection::EventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::fireEvent(RBX::BadgeService*,std::string)const")
}

// 0x3b97b4 — __ZN3rbx13remote_signalIFvSsEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvSsEEC2Ev")]
pub fn stub_3b97b4() -> ! {
    todo!("0x3b97b4 rbx::remote_signal<void ()(std::string)>::remote_signal(void)")
}

// 0x3b9b0c — __ZN3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_3b9b0c() -> ! {
    todo!("0x3b9b0c RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")
}

// 0x3b9bc0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_3b9bc0() -> ! {
    todo!("0x3b9bc0 RBX::Reflection::EventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x3b9d24 — __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_3b9d24() -> ! {
    todo!("0x3b9d24 RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")
}

// 0x3b9d2c — __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_3b9d2c() -> ! {
    todo!("0x3b9d2c RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")
}

// 0x3b9d34 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_3b9d34() -> ! {
    todo!("0x3b9d34 RBX::Reflection::EventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3b9ed8 — __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_3b9ed8() -> ! {
    todo!("0x3b9ed8 RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3b9ee8 — __ZNK3RBX10Reflection13EventDescBaseINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3b9ee8() -> ! {
    todo!("0x3b9ee8 RBX::Reflection::EventDescBase<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3b9efc — __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3b9efc() -> ! {
    todo!("0x3b9efc RBX::Reflection::EventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3ba080 — __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_3ba080() -> ! {
    todo!("0x3ba080 RBX::Reflection::EventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::~EventDesc()")
}

// 0x3ba0a4 — __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_3ba0a4() -> ! {
    todo!("0x3ba0a4 RBX::Reflection::EventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::~EventDesc()")
}

// 0x3bb204 — __ZN3rbx13remote_signalIFvSsEED2Ev
// type: _DWORD *__fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvSsEED2Ev")]
pub fn stub_3bb204() -> ! {
    todo!("0x3bb204 rbx::remote_signal<void ()(std::string)>::~remote_signal()")
}

// 0x3ec400 — __ZN3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEED1Ev")]
pub fn stub_3ec400() -> ! {
    todo!("0x3ec400 RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::~RemoteEventDesc()")
}

// 0x3ed188 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPS2_S6_SsS7_
// type: void __fastcall(int, int, const shared_count *, const std::string *, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPS2_S6_SsS7_")]
pub fn stub_3ed188() -> ! {
    todo!("0x3ed188 RBX::Reflection::EventDescImpl<3,RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::fireEvent(RBX::ChatService*,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)const")
}

// 0x3ed9c8 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS3_11ChatService9ChatColorEEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS3_11ChatService9ChatColorEEEC2Ev")]
pub fn stub_3ed9c8() -> ! {
    todo!("0x3ed9c8 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::remote_signal(void)")
}

// 0x3ee4d0 — __ZN3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEED0Ev")]
pub fn stub_3ee4d0() -> ! {
    todo!("0x3ee4d0 RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::~RemoteEventDesc()")
}

// 0x3ee584 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
pub fn stub_3ee584() -> ! {
    todo!("0x3ee584 RBX::Reflection::EventDescImpl<3,RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x3ee6e8 — __ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE12isScriptableEv")]
pub fn stub_3ee6e8() -> ! {
    todo!("0x3ee6e8 RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::isScriptable(void)const")
}

// 0x3ee6f0 — __ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE11isBroadcastEv")]
pub fn stub_3ee6f0() -> ! {
    todo!("0x3ee6f0 RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::isBroadcast(void)const")
}

// 0x3ee6f8 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE")]
pub fn stub_3ee6f8() -> ! {
    todo!("0x3ee6f8 RBX::Reflection::EventDescImpl<3,RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3ee910 — __ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
pub fn stub_3ee910() -> ! {
    todo!("0x3ee910 RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3ee920 — __ZNK3RBX10Reflection13EventDescBaseINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3ee920() -> ! {
    todo!("0x3ee920 RBX::Reflection::EventDescBase<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3efce8 — __ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_EC2ESC_PKcSF_SF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_EC2ESC_PKcSF_SF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3efce8() -> ! {
    todo!("0x3efce8 RBX::Reflection::EventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3eff44 — __ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_ED1Ev")]
pub fn stub_3eff44() -> ! {
    todo!("0x3eff44 RBX::Reflection::EventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::~EventDesc()")
}

// 0x3eff68 — __ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_ED0Ev")]
pub fn stub_3eff68() -> ! {
    todo!("0x3eff68 RBX::Reflection::EventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::~EventDesc()")
}

// 0x3f0948 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS3_11ChatService9ChatColorEEED2Ev
// type: _DWORD *__fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS3_11ChatService9ChatColorEEED2Ev")]
pub fn stub_3f0948() -> ! {
    todo!("0x3f0948 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~remote_signal()")
}

// 0x3f17f8 — __ZN3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED1Ev")]
pub fn stub_3f17f8() -> ! {
    todo!("0x3f17f8 RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::~RemoteEventDesc()")
}

// 0x3f1d14 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_
// type: void __fastcall(int, int, const shared_count *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_")]
pub fn stub_3f1d14() -> ! {
    todo!("0x3f1d14 RBX::Reflection::EventDescImpl<1,RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::fireEvent(RBX::ClickDetector*,boost::shared_ptr<RBX::Instance>)const")
}

// 0x3f2330 — __ZN3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED0Ev")]
pub fn stub_3f2330() -> ! {
    todo!("0x3f2330 RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::~RemoteEventDesc()")
}

// 0x3f23e4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
pub fn stub_3f23e4() -> ! {
    todo!("0x3f23e4 RBX::Reflection::EventDescImpl<1,RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x3f2548 — __ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE12isScriptableEv")]
pub fn stub_3f2548() -> ! {
    todo!("0x3f2548 RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::isScriptable(void)const")
}

// 0x3f2550 — __ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE11isBroadcastEv")]
pub fn stub_3f2550() -> ! {
    todo!("0x3f2550 RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::isBroadcast(void)const")
}

// 0x3f2558 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
pub fn stub_3f2558() -> ! {
    todo!("0x3f2558 RBX::Reflection::EventDescImpl<1,RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3f26b8 — __ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE")]
pub fn stub_3f26b8() -> ! {
    todo!("0x3f26b8 RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3f26c8 — __ZNK3RBX10Reflection13EventDescBaseINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3f26c8() -> ! {
    todo!("0x3f26c8 RBX::Reflection::EventDescBase<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3f26dc — __ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3f26dc() -> ! {
    todo!("0x3f26dc RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3f2860 — __ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev")]
pub fn stub_3f2860() -> ! {
    todo!("0x3f2860 RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::~EventDesc()")
}
