//! rendering shard 274 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15112/15112 complete, 29770->29870 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 29770 before -> 29870 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x399ae4 — __ZThn36_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_399ae4() -> ! {
    todo!("0x399ae4 __ZThn36_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x399b88 — __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
pub fn stub_399b88() -> ! {
    todo!("0x399b88 RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::~EventDesc()")
}


// 0x399c3c — __ZNK3RBX10Reflection13EventDescImplILi0ENS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_399c3c() -> ! {
    todo!("0x399c3c RBX::Reflection::EventDescImpl<0,RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}


// 0x399e40 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
pub fn stub_399e40() -> ! {
    todo!("0x399e40 RBX::Reflection::EventDescImpl<0,RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}


// 0x399eb4 — __ZNK3RBX10Reflection13EventDescBaseINS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_399eb4() -> ! {
    todo!("0x399eb4 RBX::Reflection::EventDescBase<RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}


// 0x399ec8 — __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::AnimationTrack::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_399ec8() -> ! {
    todo!("0x399ec8 RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::AnimationTrack::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}


// 0x39a04c — __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_ED0Ev
pub fn stub_39a04c() -> ! {
    todo!("0x39a04c RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::~EventDesc()")
}


// 0x39a100 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_39a100() -> ! {
    todo!("0x39a100 RBX::Reflection::EventDescImpl<1,RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}


// 0x39a254 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
pub fn stub_39a254() -> ! {
    todo!("0x39a254 RBX::Reflection::EventDescImpl<1,RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}


// 0x39a3f8 — __ZNK3RBX10Reflection13EventDescBaseINS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_39a3f8() -> ! {
    todo!("0x39a3f8 RBX::Reflection::EventDescBase<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}


// 0x39a40c — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EEC2EMS2_FvffEPKcS8_fS8_fNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, unsigned int, int, int, float, int, float, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::BoundFuncDesc(void (RBX::AnimationTrack::*)(float,float),char const*,char const*,float,char const*,float,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EEC2EMS2_FvffEPKcS8_fS8_fNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_39a40c() -> ! {
    todo!("0x39a40c RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::BoundFuncDesc(void (RBX::AnimationTrack::*)(float,float),char const*,char const*,float,char const*,float,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}


// 0x39a648 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// type: int __fastcall(int, int, int *, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
pub fn stub_39a648() -> ! {
    todo!("0x39a648 RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}


// 0x39a694 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EED0Ev
pub fn stub_39a694() -> ! {
    todo!("0x39a694 RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::~BoundFuncDesc()")
}


// 0x39a774 — __ZNK3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_39a774() -> ! {
    todo!("0x39a774 RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}


// 0x39a7d4 — __ZN3RBX10Reflection9ArgHelper6getArgIfLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(int, _DWORD **)
#[doc(alias = "float RBX::Reflection::ArgHelper::getArg<float,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<float> const&,boost::disable_if<boost::is_same<float,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIfLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_39a7d4() -> ! {
    todo!("0x39a7d4 float RBX::Reflection::ArgHelper::getArg<float,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<float> const&,boost::disable_if<boost::is_same<float,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}


// 0x39a978 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EEC2EMS2_FvfEPKcS8_fNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, unsigned int, int, int, float, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::BoundFuncDesc(void (RBX::AnimationTrack::*)(float),char const*,char const*,float,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EEC2EMS2_FvfEPKcS8_fNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_39a978() -> ! {
    todo!("0x39a978 RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::BoundFuncDesc(void (RBX::AnimationTrack::*)(float),char const*,char const*,float,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}


// 0x39ab30 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_39ab30() -> ! {
    todo!("0x39ab30 RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}


// 0x39ab60 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EED0Ev
pub fn stub_39ab60() -> ! {
    todo!("0x39ab60 RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::~BoundFuncDesc()")
}


// 0x39ac34 — __ZNK3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_39ac34() -> ! {
    todo!("0x39ac34 RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}


// 0x39ac70 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EEC2EMS2_FvfffEPKcS8_fS8_fS8_fNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, unsigned int, int, int, float, int, float, int, float, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::BoundFuncDesc(void (RBX::AnimationTrack::*)(float,float,float),char const*,char const*,float,char const*,float,char const*,float,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EEC2EMS2_FvfffEPKcS8_fS8_fS8_fNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_39ac70() -> ! {
    todo!("0x39ac70 RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::BoundFuncDesc(void (RBX::AnimationTrack::*)(float,float,float),char const*,char const*,float,char const*,float,char const*,float,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}


// 0x39af34 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_
// type: int __fastcall(int, int, int *, int, int *, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_
pub fn stub_39af34() -> ! {
    todo!("0x39af34 RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}


// 0x39af9c — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EED0Ev
pub fn stub_39af9c() -> ! {
    todo!("0x39af9c RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::~BoundFuncDesc()")
}


// 0x39b088 — __ZNK3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_39b088() -> ! {
    todo!("0x39b088 RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}


// 0x39b100 — __GLOBAL__I_a_156
// type: 
#[doc(alias = "global constructor keyed to_a_156")]
// was: __GLOBAL__I_a_156
pub fn stub_39b100() -> ! {
    todo!("0x39b100 global constructor keyed to_a_156")
}


// 0x39b490 — __ZN3RBX19AnimationTrackStateC1EN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS1_8weak_ptrIKNS_8AnimatorEEE
// type: int()
#[doc(alias = "RBX::AnimationTrackState::AnimationTrackState(boost::shared_ptr<RBX::KeyframeSequence const>,boost::weak_ptr<RBX::Animator const>)")]
// was: __ZN3RBX19AnimationTrackStateC1EN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS1_8weak_ptrIKNS_8AnimatorEEE
pub fn stub_39b490() -> ! {
    todo!("0x39b490 RBX::AnimationTrackState::AnimationTrackState(boost::shared_ptr<RBX::KeyframeSequence const>,boost::weak_ptr<RBX::Animator const>)")
}


// 0x39b494 — __ZN3RBX19AnimationTrackStateC2EN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS1_8weak_ptrIKNS_8AnimatorEEE
// type: RBX::Instance *__fastcall(RBX::Instance *, int, int)
#[doc(alias = "RBX::AnimationTrackState::AnimationTrackState(boost::shared_ptr<RBX::KeyframeSequence const>,boost::weak_ptr<RBX::Animator const>)")]
// was: __ZN3RBX19AnimationTrackStateC2EN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS1_8weak_ptrIKNS_8AnimatorEEE
pub fn stub_39b494() -> ! {
    todo!("0x39b494 RBX::AnimationTrackState::AnimationTrackState(boost::shared_ptr<RBX::KeyframeSequence const>,boost::weak_ptr<RBX::Animator const>)")
}


// 0x39b950 — __ZN3RBX19AnimationTrackState6onPlayEffff
// type: int __fastcall(int this, float, float, float, float)
#[doc(alias = "RBX::AnimationTrackState::onPlay(float,float,float,float)")]
// was: __ZN3RBX19AnimationTrackState6onPlayEffff
pub fn stub_39b950() -> ! {
    todo!("0x39b950 RBX::AnimationTrackState::onPlay(float,float,float,float)")
}


// 0x39b9ac — __ZN3RBX19AnimationTrackState6onStopEff
// type: int __fastcall(RBX::AnimationTrackState *this, float32_t, float32_t)
#[doc(alias = "RBX::AnimationTrackState::onStop(float,float)")]
// was: __ZN3RBX19AnimationTrackState6onStopEff
pub fn stub_39b9ac() -> ! {
    todo!("0x39b9ac RBX::AnimationTrackState::onStop(float,float)")
}


// 0x39b9f4 — __ZN3RBX19AnimationTrackState14onAdjustWeightEfff
// type: int __fastcall(RBX::AnimationTrackState *this, float32_t, float, float32_t)
#[doc(alias = "RBX::AnimationTrackState::onAdjustWeight(float,float,float)")]
// was: __ZN3RBX19AnimationTrackState14onAdjustWeightEfff
pub fn stub_39b9f4() -> ! {
    todo!("0x39b9f4 RBX::AnimationTrackState::onAdjustWeight(float,float,float)")
}


// 0x39ba40 — __ZN3RBX19AnimationTrackState13onAdjustSpeedEff
// type: int __fastcall(int this, float, float)
#[doc(alias = "RBX::AnimationTrackState::onAdjustSpeed(float,float)")]
// was: __ZN3RBX19AnimationTrackState13onAdjustSpeedEff
pub fn stub_39ba40() -> ! {
    todo!("0x39ba40 RBX::AnimationTrackState::onAdjustSpeed(float,float)")
}


// 0x39ba88 — __ZN3RBX19AnimationTrackState9isStoppedEd
// type: bool __fastcall(RBX::AnimationTrackState *this, double)
#[doc(alias = "RBX::AnimationTrackState::isStopped(double)")]
// was: __ZN3RBX19AnimationTrackState9isStoppedEd
pub fn stub_39ba88() -> ! {
    todo!("0x39ba88 RBX::AnimationTrackState::isStopped(double)")
}


// 0x39bb00 — __ZN3RBX19AnimationTrackState11getGameTimeEv
// type: __int64 __fastcall(RBX::AnimationTrackState *this)
#[doc(alias = "RBX::AnimationTrackState::getGameTime(void)")]
// was: __ZN3RBX19AnimationTrackState11getGameTimeEv
pub fn stub_39bb00() -> ! {
    todo!("0x39bb00 RBX::AnimationTrackState::getGameTime(void)")
}


// 0x39bc00 — __ZN3RBX19AnimationTrackState15getWeightAtTimeEd
// type: __int64 __fastcall(RBX::AnimationTrackState *this, double)
#[doc(alias = "RBX::AnimationTrackState::getWeightAtTime(double)")]
// was: __ZN3RBX19AnimationTrackState15getWeightAtTimeEd
pub fn stub_39bc00() -> ! {
    todo!("0x39bc00 RBX::AnimationTrackState::getWeightAtTime(double)")
}


// 0x39bc5c — __ZN3RBX19AnimationTrackState4playEfff
// type: int __fastcall(RBX::AnimationTrackState *this, int, int, float)
#[doc(alias = "RBX::AnimationTrackState::play(float,float,float)")]
// was: __ZN3RBX19AnimationTrackState4playEfff
pub fn stub_39bc5c() -> ! {
    todo!("0x39bc5c RBX::AnimationTrackState::play(float,float,float)")
}


// 0x39bcbc — __ZN3RBX19AnimationTrackState4stopEf
// type: int __fastcall(RBX::AnimationTrackState *this, int)
#[doc(alias = "RBX::AnimationTrackState::stop(float)")]
// was: __ZN3RBX19AnimationTrackState4stopEf
pub fn stub_39bcbc() -> ! {
    todo!("0x39bcbc RBX::AnimationTrackState::stop(float)")
}


// 0x39bd0c — __ZN3RBX19AnimationTrackState12adjustWeightEff
// type: int __fastcall(RBX::AnimationTrackState *this, int, int)
#[doc(alias = "RBX::AnimationTrackState::adjustWeight(float,float)")]
// was: __ZN3RBX19AnimationTrackState12adjustWeightEff
pub fn stub_39bd0c() -> ! {
    todo!("0x39bd0c RBX::AnimationTrackState::adjustWeight(float,float)")
}


// 0x39bd64 — __ZN3RBX19AnimationTrackState11adjustSpeedEf
// type: int __fastcall(RBX::AnimationTrackState *this, int)
#[doc(alias = "RBX::AnimationTrackState::adjustSpeed(float)")]
// was: __ZN3RBX19AnimationTrackState11adjustSpeedEf
pub fn stub_39bd64() -> ! {
    todo!("0x39bd64 RBX::AnimationTrackState::adjustSpeed(float)")
}


// 0x39bdb4 — __ZN3RBX19AnimationTrackState28triggerKeyframeReachedSignalERKN5boost10shared_ptrINS_8InstanceEEEdd
// type: void __fastcall(int, int, unsigned int, unsigned int, double)
#[doc(alias = "RBX::AnimationTrackState::triggerKeyframeReachedSignal(boost::shared_ptr<RBX::Instance> const&,double,double)")]
// was: __ZN3RBX19AnimationTrackState28triggerKeyframeReachedSignalERKN5boost10shared_ptrINS_8InstanceEEEdd
pub fn stub_39bdb4() -> ! {
    todo!("0x39bdb4 RBX::AnimationTrackState::triggerKeyframeReachedSignal(boost::shared_ptr<RBX::Instance> const&,double,double)")
}


// 0x39bf44 — __ZN3RBX19AnimationTrackState4stepERSt6vectorINS_15PoseAccumulatorESaIS2_EEd
// type: void __fastcall(int, int, double, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::AnimationTrackState::step(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> &,double)")]
// was: __ZN3RBX19AnimationTrackState4stepERSt6vectorINS_15PoseAccumulatorESaIS2_EEd
pub fn stub_39bf44() -> ! {
    todo!("0x39bf44 RBX::AnimationTrackState::step(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> &,double)")
}


// 0x39c124 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED1Ev
pub fn stub_39c124() -> ! {
    todo!("0x39c124 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::~RemoteEventDesc()")
}


// 0x39c148 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED1Ev
pub fn stub_39c148() -> ! {
    todo!("0x39c148 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::~RemoteEventDesc()")
}


// 0x39c16c — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED1Ev
pub fn stub_39c16c() -> ! {
    todo!("0x39c16c RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::~RemoteEventDesc()")
}


// 0x39c190 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED1Ev
pub fn stub_39c190() -> ! {
    todo!("0x39c190 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")
}


// 0x39c1b4 — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE21fireAndReplicateEventEPS2_Ss
// type: void __fastcall(int, int, std::string *)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::fireAndReplicateEvent(RBX::AnimationTrackState*,std::string)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE21fireAndReplicateEventEPS2_Ss
pub fn stub_39c1b4() -> ! {
    todo!("0x39c1b4 RBX::Reflection::RemoteEventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::fireAndReplicateEvent(RBX::AnimationTrackState*,std::string)")
}


// 0x39c370 — __ZN3RBX19AnimationTrackStateD1Ev
// type: void __fastcall(RBX::AnimationTrackState *__hidden this)
#[doc(alias = "RBX::AnimationTrackState::~AnimationTrackState()")]
// was: __ZN3RBX19AnimationTrackStateD1Ev
pub fn stub_39c370() -> ! {
    todo!("0x39c370 RBX::AnimationTrackState::~AnimationTrackState()")
}


// 0x39c374 — __ZN3RBX19AnimationTrackStateD0Ev
// type: void __fastcall(RBX::AnimationTrackState *__hidden this)
#[doc(alias = "RBX::AnimationTrackState::~AnimationTrackState()")]
// was: __ZN3RBX19AnimationTrackStateD0Ev
pub fn stub_39c374() -> ! {
    todo!("0x39c374 RBX::AnimationTrackState::~AnimationTrackState()")
}


// 0x39c414 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv
pub fn stub_39c414() -> ! {
    todo!("0x39c414 __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv")
}


// 0x39c43c — __ZThn32_N3RBX19AnimationTrackStateD1Ev
// type: void __fastcall(RBX::AnimationTrackState *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState()")]
// was: __ZThn32_N3RBX19AnimationTrackStateD1Ev
pub fn stub_39c43c() -> ! {
    todo!("0x39c43c non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState()")
}


// 0x39c444 — __ZThn32_N3RBX19AnimationTrackStateD0Ev
// type: void __fastcall(RBX::AnimationTrackState *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState()")]
// was: __ZThn32_N3RBX19AnimationTrackStateD0Ev
pub fn stub_39c444() -> ! {
    todo!("0x39c444 non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState()")
}


// 0x39c44c — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv
// type: 
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv
pub fn stub_39c44c() -> ! {
    todo!("0x39c44c __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv")
}


// 0x39c474 — __ZThn36_N3RBX19AnimationTrackStateD1Ev
// type: void __fastcall(RBX::AnimationTrackState *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState()")]
// was: __ZThn36_N3RBX19AnimationTrackStateD1Ev
pub fn stub_39c474() -> ! {
    todo!("0x39c474 non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState()")
}


// 0x39c47c — __ZThn36_N3RBX19AnimationTrackStateD0Ev
// type: void __fastcall(RBX::AnimationTrackState *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState()")]
// was: __ZThn36_N3RBX19AnimationTrackStateD0Ev
pub fn stub_39c47c() -> ! {
    todo!("0x39c47c non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState()")
}


// 0x39c484 — __ZN3RBX19AnimationTrackStateD2Ev
// type: void __fastcall(RBX::AnimationTrackState *this, int, int, int)
#[doc(alias = "RBX::AnimationTrackState::~AnimationTrackState()")]
// was: __ZN3RBX19AnimationTrackStateD2Ev
pub fn stub_39c484() -> ! {
    todo!("0x39c484 RBX::AnimationTrackState::~AnimationTrackState()")
}


// 0x39c640 — __ZN3RBX4Name13callDoDeclareILZNS_20sAnimationTrackStateEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sAnimationTrackStateEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_20sAnimationTrackStateEEEEvv
pub fn stub_39c640() -> ! {
    todo!("0x39c640 __ZN3RBX4Name13callDoDeclareILZNS_20sAnimationTrackStateEEEEvv")
}


// 0x39c644 — __ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v
pub fn stub_39c644() -> ! {
    todo!("0x39c644 __ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v")
}


// 0x39c724 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ss
// type: void __fastcall(int, int, std::string *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::fireEvent(RBX::AnimationTrackState*,std::string)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ss
pub fn stub_39c724() -> ! {
    todo!("0x39c724 RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::fireEvent(RBX::AnimationTrackState*,std::string)const")
}


// 0x39c840 — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceESs
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::replicateEvent(RBX::Reflection::EventSource *,std::string)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceESs
pub fn stub_39c840() -> ! {
    todo!("0x39c840 RBX::Reflection::RemoteEventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::replicateEvent(RBX::Reflection::EventSource *,std::string)")
}


// 0x39c98c — __ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEfff
// type: int __fastcall(int, int, int, int, float)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<3,RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::replicateEvent(RBX::Reflection::EventSource *,float,float,float)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEfff
pub fn stub_39c98c() -> ! {
    todo!("0x39c98c RBX::Reflection::RemoteEventDescImpl<3,RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::replicateEvent(RBX::Reflection::EventSource *,float,float,float)")
}


// 0x39cb28 — __ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(float,float,float)>::operator()(float,float,float)")]
// was: __ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff
pub fn stub_39cb28() -> ! {
    todo!("0x39cb28 rbx::signals::signal_with_args<3,void ()(float,float,float)>::operator()(float,float,float)")
}


// 0x39cc88 — __ZN3rbx7signals6signalIFvfffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvfffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_39cc88() -> ! {
    todo!("0x39cc88 rbx::signals::signal<void ()(float,float,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot> &)")
}


// 0x39cde8 — __ZN3rbx7signals6signalIFvfffEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvfffEE8on_errorERSt9exception
pub fn stub_39cde8() -> ! {
    todo!("0x39cde8 rbx::signals::signal<void ()(float,float,float)>::on_error(std::exception &)")
}


// 0x39ce10 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSERKS7_
// type: int *__fastcall(int *, int *)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSERKS7_
pub fn stub_39ce10() -> ! {
    todo!("0x39ce10 boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot> const&)")
}


// 0x39ce34 — __ZN3rbx7signals6signalIFvfffEE22safe_static_init_mutexEv
// type: 
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvfffEE22safe_static_init_mutexEv
pub fn stub_39ce34() -> ! {
    todo!("0x39ce34 rbx::signals::signal<void ()(float,float,float)>::safe_static_init_mutex(void)")
}


// 0x39ce38 — __ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv
pub fn stub_39ce38() -> ! {
    todo!("0x39ce38 rbx::signals::signal<void ()(float,float,float)>::safe_static_do_get_mutex(void)")
}


// 0x39cf30 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEff
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::replicateEvent(RBX::Reflection::EventSource *,float,float)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEff
pub fn stub_39cf30() -> ! {
    todo!("0x39cf30 RBX::Reflection::RemoteEventDescImpl<2,RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::replicateEvent(RBX::Reflection::EventSource *,float,float)")
}


// 0x39d09c — __ZN3RBX10Reflection19RemoteEventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEffff
// type: int __fastcall(int, int, int, int, float, float)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<4,RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::replicateEvent(RBX::Reflection::EventSource *,float,float,float,float)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEffff
pub fn stub_39d09c() -> ! {
    todo!("0x39d09c RBX::Reflection::RemoteEventDescImpl<4,RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::replicateEvent(RBX::Reflection::EventSource *,float,float,float,float)")
}


// 0x39d260 — __ZN3rbx7signals16signal_with_argsILi4EFvffffEEclEffff
// type: void __fastcall(_DWORD *, int, int, const void *, float)
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(float,float,float,float)>::operator()(float,float,float,float)")]
// was: __ZN3rbx7signals16signal_with_argsILi4EFvffffEEclEffff
pub fn stub_39d260() -> ! {
    todo!("0x39d260 rbx::signals::signal_with_args<4,void ()(float,float,float,float)>::operator()(float,float,float,float)")
}


// 0x39d3dc — __ZN3rbx7signals6signalIFvffffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvffffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_39d3dc() -> ! {
    todo!("0x39d3dc rbx::signals::signal<void ()(float,float,float,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot> &)")
}


// 0x39d53c — __ZN3rbx7signals6signalIFvffffEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvffffEE8on_errorERSt9exception
pub fn stub_39d53c() -> ! {
    todo!("0x39d53c rbx::signals::signal<void ()(float,float,float,float)>::on_error(std::exception &)")
}


// 0x39d564 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSERKS7_
// type: int *__fastcall(int *, int *)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSERKS7_
pub fn stub_39d564() -> ! {
    todo!("0x39d564 boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot> const&)")
}


// 0x39d588 — __ZN3rbx7signals6signalIFvffffEE22safe_static_init_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvffffEE22safe_static_init_mutexEv
pub fn stub_39d588() -> ! {
    todo!("0x39d588 rbx::signals::signal<void ()(float,float,float,float)>::safe_static_init_mutex(void)")
}


// 0x39d58c — __ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv
pub fn stub_39d58c() -> ! {
    todo!("0x39d58c rbx::signals::signal<void ()(float,float,float,float)>::safe_static_do_get_mutex(void)")
}


// 0x39d684 — __ZN5boost10shared_ptrIKN3RBX8AnimatorEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::shared_ptr<RBX::Animator const>::shared_ptr<RBX::Animator const>(boost::weak_ptr<RBX::Animator const> const&,boost::detail::sp_nothrow_tag)")]
// was: __ZN5boost10shared_ptrIKN3RBX8AnimatorEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
pub fn stub_39d684() -> ! {
    todo!("0x39d684 boost::shared_ptr<RBX::Animator const>::shared_ptr<RBX::Animator const>(boost::weak_ptr<RBX::Animator const> const&,boost::detail::sp_nothrow_tag)")
}


// 0x39d700 — __ZN3rbx7signals6signalIFvfffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
// was: __ZN3rbx7signals6signalIFvfffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_
pub fn stub_39d700() -> ! {
    todo!("0x39d700 rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")
}


// 0x39d774 — __ZN3rbx7signals6signalIFvfffEE6insertEPNS3_4slotE
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvfffEE6insertEPNS3_4slotE
pub fn stub_39d774() -> ! {
    todo!("0x39d774 rbx::signals::signal<void ()(float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float)>::slot *)")
}


// 0x39d980 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSEPS6_
// type: int *__fastcall(int *, int)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSEPS6_
pub fn stub_39d980() -> ! {
    todo!("0x39d980 boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float)>::slot*)")
}


// 0x39d9a4 — __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED1Ev
pub fn stub_39d9a4() -> ! {
    todo!("0x39d9a4 rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}


// 0x39d9d0 — __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED0Ev
pub fn stub_39d9d0() -> ! {
    todo!("0x39d9d0 rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}


// 0x39daa4 — __ZN3rbx7signals6signalIFvfffEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvfffEE4slot10disconnectEv
pub fn stub_39daa4() -> ! {
    todo!("0x39daa4 rbx::signals::signal<void ()(float,float,float)>::slot::disconnect(void)")
}


// 0x39dbb4 — __ZNK3rbx7signals6signalIFvfffEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvfffEE4slot9connectedEv
pub fn stub_39dbb4() -> ! {
    todo!("0x39dbb4 rbx::signals::signal<void ()(float,float,float)>::slot::connected(void)const")
}


// 0x39dbc0 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callEfff
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::call(float,float,float)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callEfff
pub fn stub_39dbc0() -> ! {
    todo!("0x39dbc0 rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::call(float,float,float)")
}


// 0x39dbec — __ZThn4_N3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callEfff
// type: int __fastcall(int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::call(float,float,float)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callEfff
pub fn stub_39dbec() -> ! {
    todo!("0x39dbec non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::call(float,float,float)")
}


// 0x39dc18 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_fffEENS0_5list3IRfSH_SH_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, _DWORD **)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list3<float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float> &,boost::_bi::list3<float &,float &,float &> &,int)")]
// was: __ZN5boost3_bi5list4INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_fffEENS0_5list3IRfSH_SH_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_39dc18() -> ! {
    todo!("0x39dc18 void boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list3<float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float> &,boost::_bi::list3<float &,float &,float &> &,int)")
}


// 0x39dc54 — __ZN3rbx7signals6signalIFvfffEE6removeEPNS3_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvfffEE6removeEPNS3_4slotE
pub fn stub_39dc54() -> ! {
    todo!("0x39dc54 rbx::signals::signal<void ()(float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float)>::slot *)")
}


// 0x39dd44 — __ZN3rbx7signals6signalIFvfffEE4slot22safe_static_init_mutexEv
// type: 
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvfffEE4slot22safe_static_init_mutexEv
pub fn stub_39dd44() -> ! {
    todo!("0x39dd44 rbx::signals::signal<void ()(float,float,float)>::slot::safe_static_init_mutex(void)")
}


// 0x39dd48 — __ZN3rbx7signals6signalIFvfffEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvfffEE4slot24safe_static_do_get_mutexEv
pub fn stub_39dd48() -> ! {
    todo!("0x39dd48 rbx::signals::signal<void ()(float,float,float)>::slot::safe_static_do_get_mutex(void)")
}


// 0x39de38 — __ZN3rbx7signals6signalIFvfffEE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvfffEE4slotD1Ev
pub fn stub_39de38() -> ! {
    todo!("0x39de38 rbx::signals::signal<void ()(float,float,float)>::slot::~slot()")
}


// 0x39de64 — __ZN3rbx7signals6signalIFvfffEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvfffEE4slotD0Ev
pub fn stub_39de64() -> ! {
    todo!("0x39de64 rbx::signals::signal<void ()(float,float,float)>::slot::~slot()")
}


// 0x39df38 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED1Ev
pub fn stub_39df38() -> ! {
    todo!("0x39df38 rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::~callable()")
}


// 0x39df64 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED0Ev
pub fn stub_39df64() -> ! {
    todo!("0x39df64 rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::~callable()")
}


// 0x39e038 — __ZN3rbx7signals6signalIFvffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>> const&)")]
// was: __ZN3rbx7signals6signalIFvffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_39e038() -> ! {
    todo!("0x39e038 rbx::signals::connection rbx::signals::signal<void ()(float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>> const&)")
}


// 0x39e0ac — __ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev
pub fn stub_39e0ac() -> ! {
    todo!("0x39e0ac rbx::signals::signal<void ()(float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}


// 0x39e0d8 — __ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev
pub fn stub_39e0d8() -> ! {
    todo!("0x39e0d8 rbx::signals::signal<void ()(float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}


// 0x39e1ac — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEff
// type: int __fastcall(int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::call(float,float)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEff
pub fn stub_39e1ac() -> ! {
    todo!("0x39e1ac rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::call(float,float)")
}


// 0x39e1d4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEff
// type: int __fastcall(int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::call(float,float)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEff
pub fn stub_39e1d4() -> ! {
    todo!("0x39e1d4 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::call(float,float)")
}


// 0x39e1fc — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_ffEENS0_5list2IRfSG_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, int **)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list2<float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float> &,boost::_bi::list2<float &,float &> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_ffEENS0_5list2IRfSG_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_39e1fc() -> ! {
    todo!("0x39e1fc void boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list2<float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float> &,boost::_bi::list2<float &,float &> &,int)")
}


// 0x39e228 — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev
pub fn stub_39e228() -> ! {
    todo!("0x39e228 rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::~callable()")
}


// 0x39e254 — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev
pub fn stub_39e254() -> ! {
    todo!("0x39e254 rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::~callable()")
}


// 0x39e328 — __ZN3rbx7signals6signalIFvffffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>> const&)")]
// was: __ZN3rbx7signals6signalIFvffffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEEENS0_10connectionERKT_
pub fn stub_39e328() -> ! {
    todo!("0x39e328 rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>> const&)")
}


// 0x39e39c — __ZN3rbx7signals6signalIFvffffEE6insertEPNS3_4slotE
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvffffEE6insertEPNS3_4slotE
pub fn stub_39e39c() -> ! {
    todo!("0x39e39c rbx::signals::signal<void ()(float,float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")
}

