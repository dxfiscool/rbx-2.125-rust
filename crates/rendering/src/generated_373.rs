//! rendering shard 373 — 100 stubs 0x52c538..0x53015c EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 40560->40660 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x52c538..0x53015c (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x52c538 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEE7addPairES3_PKc
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEE7addPairES3_PKc")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::TweenEasingStyle>::addPair(RBX::GuiObject::TweenEasingStyle,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEE7addPairES3_PKc
// IDA 0x52c538: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52c538() {
}

// 0x52c898 — __ZN3RBX10Reflection7Variant14genericConvertINS_9GuiObject16TweenEasingStyleEEERT_v
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_9GuiObject16TweenEasingStyleEEERT_v")]
#[doc(alias = "RBX::GuiObject::TweenEasingStyle & RBX::Reflection::Variant::genericConvert<RBX::GuiObject::TweenEasingStyle>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_9GuiObject16TweenEasingStyleEEERT_v
// IDA 0x52c898: 143 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52c898() {
}

// 0x52ca84 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEE7addPairES3_PKc
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEE7addPairES3_PKc")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::TweenStatus>::addPair(RBX::GuiObject::TweenStatus,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEE7addPairES3_PKc
// IDA 0x52ca84: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52ca84() {
}

// 0x52cde4 — __ZNK5boost9function2IvPN3RBX9GuiObjectENS1_5UDim2EEclES3_S4_
#[doc(alias = "__ZNK5boost9function2IvPN3RBX9GuiObjectENS1_5UDim2EEclES3_S4_")]
#[doc(alias = "boost::function2<void,RBX::GuiObject *,RBX::UDim2>::operator()(RBX::GuiObject *,RBX::UDim2)const")]
// was: __ZNK5boost9function2IvPN3RBX9GuiObjectENS1_5UDim2EEclES3_S4_
// IDA 0x52cde4: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52cde4() {
}

// 0x52ceb8 — __ZN5boost4bindIvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES4_S6_S4_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// type: void __fastcall(_DWORD *, int, int, int)
#[doc(alias = "__ZN5boost4bindIvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES4_S6_S4_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_")]
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list_av_2<boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus>::type> boost::bind<void,boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus,boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus>(void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus)")]
// was: __ZN5boost4bindIvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES4_S6_S4_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// IDA 0x52ceb8: 96 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52ceb8() {
}

// 0x52cfc0 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefENS3_11TweenStatusES4_S6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSC_T0_T1_T2_ENSA_9list_av_3IT3_T4_T5_E4typeEEESH_SJ_SK_SL_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefENS3_11TweenStatusES4_S6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSC_T0_T1_T2_ENSA_9list_av_3IT3_T4_T5_E4typeEEESH_SJ_SK_SL_")]
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>>(void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefENS3_11TweenStatusES4_S6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSC_T0_T1_T2_ENSA_9list_av_3IT3_T4_T5_E4typeEEESH_SJ_SK_SL_
// IDA 0x52cfc0: 225 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52cfc0() {
}

// 0x52d218 — __ZN3RBX15ServiceProvider6createINS_12TweenServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_12TweenServiceEEEPT_PKNS_8InstanceE")]
#[doc(alias = "RBX::TweenService * RBX::ServiceProvider::create<RBX::TweenService>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider6createINS_12TweenServiceEEEPT_PKNS_8InstanceE
// IDA 0x52d218: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52d218() {
}

// 0x52d230 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_")]
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef>::type> boost::bind<void,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef>(void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// IDA 0x52d230: 225 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52d230() {
}

// 0x52d488 — __ZN5boost10scoped_ptrIN3RBX9GuiObject6TweensEE5resetEPS3_
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX9GuiObject6TweensEE5resetEPS3_")]
#[doc(alias = "boost::scoped_ptr<RBX::GuiObject::Tweens>::reset(RBX::GuiObject::Tweens*)")]
// was: __ZN5boost10scoped_ptrIN3RBX9GuiObject6TweensEE5resetEPS3_
// IDA 0x52d488: 75 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52d488() {
}

// 0x52d55c — __ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEaSERKS5_
#[doc(alias = "__ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEaSERKS5_")]
#[doc(alias = "boost::function<void ()(RBX::GuiObject::TweenStatus)>::operator=(boost::function<void ()(RBX::GuiObject::TweenStatus)> const&)")]
// was: __ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEaSERKS5_
// IDA 0x52d55c: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52d55c() {
}

// 0x52d620 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::setListenerMode(bool)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb
// IDA 0x52d620: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52d620() {
}

// 0x52d780 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE15setListenerModeEb
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE15setListenerModeEb")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::setListenerMode(bool)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE15setListenerModeEb
// IDA 0x52d780: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52d780() {
}

// 0x52d9c4 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x52d9c4: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52d9c4() {
}

// 0x52da24 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x52da24: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52da24() {
}

// 0x52da84 — __ZN3rbx7signals16signal_with_argsILi2EFviiEEclEii
// type: void __fastcall(_DWORD *, int, int, const void *)
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFviiEEclEii")]
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(int,int)>::operator()(int,int)")]
// was: __ZN3rbx7signals16signal_with_argsILi2EFviiEEclEii
// IDA 0x52da84: 122 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52da84() {
}

// 0x52dbd0 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX5UDim2EEEclES3_
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX5UDim2EEEclES3_")]
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::UDim2)>::operator()(RBX::UDim2)")]
// was: __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX5UDim2EEEclES3_
// IDA 0x52dbd0: 95 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52dbd0() {
}

// 0x52dd4c — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EED1Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiButton,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9GuiButtonEFvSsELi1EED1Ev
// IDA 0x52dd4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52dd4c() {
}

// 0x52de40 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEED1Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEED1Ev
// IDA 0x52de40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52de40() {
}

// 0x52de64 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEED1Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEED1Ev
// IDA 0x52de64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52de64() {
}

// 0x52de88 — __ZNK3RBX9GuiButton18getAutoButtonColorEv
// type: _DWORD __fastcall(RBX::GuiButton *__hidden this)
#[doc(alias = "__ZNK3RBX9GuiButton18getAutoButtonColorEv")]
#[doc(alias = "RBX::GuiButton::getAutoButtonColor(void)const")]
// was: __ZNK3RBX9GuiButton18getAutoButtonColorEv
// IDA 0x52de88: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52de88() {
}

// 0x52de90 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiButtonEbED1Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiButtonEbED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiButton,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9GuiButtonEbED1Ev
// IDA 0x52de90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52de90() {
}

// 0x52deb4 — __ZNK3RBX9GuiButton11getSelectedEv
// type: _DWORD __fastcall(RBX::GuiButton *__hidden this)
#[doc(alias = "__ZNK3RBX9GuiButton11getSelectedEv")]
#[doc(alias = "RBX::GuiButton::getSelected(void)const")]
// was: __ZNK3RBX9GuiButton11getSelectedEv
// IDA 0x52deb4: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52deb4() {
}

// 0x52debc — __ZNK3RBX9GuiButton8getModalEv
// type: _DWORD __fastcall(RBX::GuiButton *__hidden this)
#[doc(alias = "__ZNK3RBX9GuiButton8getModalEv")]
#[doc(alias = "RBX::GuiButton::getModal(void)const")]
// was: __ZNK3RBX9GuiButton8getModalEv
// IDA 0x52debc: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52debc() {
}

// 0x52dec4 — __ZNK3RBX9GuiButton8getStyleEv
// type: _DWORD __fastcall(RBX::GuiButton *__hidden this)
#[doc(alias = "__ZNK3RBX9GuiButton8getStyleEv")]
#[doc(alias = "RBX::GuiButton::getStyle(void)const")]
// was: __ZNK3RBX9GuiButton8getStyleEv
// IDA 0x52dec4: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52dec4() {
}

// 0x52decc — __ZN3RBX10Reflection18EnumPropDescriptorINS_9GuiButtonENS2_5StyleEED1Ev
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9GuiButtonENS2_5StyleEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiButton,RBX::GuiButton::Style>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9GuiButtonENS2_5StyleEED1Ev
// IDA 0x52decc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52decc() {
}

// 0x52def0 — __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE7addPairES3_PKc
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE7addPairES3_PKc")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::addPair(RBX::GuiButton::Style,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE7addPairES3_PKc
// IDA 0x52def0: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52def0() {
}

// 0x52e250 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE15setListenerModeEb
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE15setListenerModeEb")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::setListenerMode(bool)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE15setListenerModeEb
// IDA 0x52e250: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52e250() {
}

// 0x52e3b0 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::setListenerMode(bool)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb
// IDA 0x52e3b0: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52e3b0() {
}

// 0x52e510 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x52e510: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52e510() {
}

// 0x52e570 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x52e570: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52e570() {
}

// 0x52e5d0 — __ZN3RBX9TextureIdD1Ev
// type: void __fastcall(RBX::TextureId *__hidden this)
#[doc(alias = "__ZN3RBX9TextureIdD1Ev")]
#[doc(alias = "RBX::TextureId::~TextureId()")]
// was: __ZN3RBX9TextureIdD1Ev
// IDA 0x52e5d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52e5d0() {
}

// 0x52e5e0 — __ZN3RBX9GuiButtonD1Ev
// type: void __fastcall(RBX::GuiButton *__hidden this)
#[doc(alias = "__ZN3RBX9GuiButtonD1Ev")]
#[doc(alias = "RBX::GuiButton::~GuiButton()")]
// was: __ZN3RBX9GuiButtonD1Ev
// IDA 0x52e5e0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_52e5e0() {
}

// 0x52e5e4 — __ZN3RBX9GuiButtonD0Ev
// type: void __fastcall(RBX::GuiButton *__hidden this)
#[doc(alias = "__ZN3RBX9GuiButtonD0Ev")]
#[doc(alias = "RBX::GuiButton::~GuiButton()")]
// was: __ZN3RBX9GuiButtonD0Ev
// IDA 0x52e5e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52e5e4() {
}

// 0x52e684 — __ZNK3RBX17NonFactoryProductINS_9GuiObjectELZNS_10sGuiButtonEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiObjectELZNS_10sGuiButtonEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_9GuiObjectELZNS_10sGuiButtonEEE12getClassNameEv
// IDA 0x52e684: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52e684() {
}

// 0x52e6ac — __ZNK3RBX9GuiButton9isGuiLeafEv
// type: _DWORD __fastcall(RBX::GuiButton *__hidden this)
#[doc(alias = "__ZNK3RBX9GuiButton9isGuiLeafEv")]
#[doc(alias = "RBX::GuiButton::isGuiLeaf(void)const")]
// was: __ZNK3RBX9GuiButton9isGuiLeafEv
// IDA 0x52e6ac: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52e6ac() {
}

// 0x52e6b0 — __ZThn32_N3RBX9GuiButtonD1Ev
// type: void __fastcall(RBX::GuiButton *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9GuiButtonD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiButton::~GuiButton()")]
// was: __ZThn32_N3RBX9GuiButtonD1Ev
// IDA 0x52e6b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52e6b0() {
}

// 0x52e6b8 — __ZThn32_N3RBX9GuiButtonD0Ev
// type: void __fastcall(RBX::GuiButton *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9GuiButtonD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiButton::~GuiButton()")]
// was: __ZThn32_N3RBX9GuiButtonD0Ev
// IDA 0x52e6b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52e6b8() {
}

// 0x52e75c — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiObjectELZNS_10sGuiButtonEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiObjectELZNS_10sGuiButtonEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_9GuiObjectELZNS_10sGuiButtonEEE12getClassNameEv
// IDA 0x52e75c: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52e75c() {
}

// 0x52e784 — __ZThn36_N3RBX9GuiButtonD1Ev
// type: void __fastcall(RBX::GuiButton *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9GuiButtonD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiButton::~GuiButton()")]
// was: __ZThn36_N3RBX9GuiButtonD1Ev
// IDA 0x52e784: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52e784() {
}

// 0x52e78c — __ZThn36_N3RBX9GuiButtonD0Ev
// type: void __fastcall(RBX::GuiButton *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9GuiButtonD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiButton::~GuiButton()")]
// was: __ZThn36_N3RBX9GuiButtonD0Ev
// IDA 0x52e78c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52e78c() {
}

// 0x52e830 — __ZN3RBX8GuiLabelD1Ev
// type: void __fastcall(RBX::GuiLabel *__hidden this)
#[doc(alias = "__ZN3RBX8GuiLabelD1Ev")]
#[doc(alias = "RBX::GuiLabel::~GuiLabel()")]
// was: __ZN3RBX8GuiLabelD1Ev
// IDA 0x52e830: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_52e830() {
}

// 0x52e834 — __ZN3RBX8GuiLabelD0Ev
// type: void __fastcall(RBX::GuiLabel *__hidden this)
#[doc(alias = "__ZN3RBX8GuiLabelD0Ev")]
#[doc(alias = "RBX::GuiLabel::~GuiLabel()")]
// was: __ZN3RBX8GuiLabelD0Ev
// IDA 0x52e834: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52e834() {
}

// 0x52e8d4 — __ZNK3RBX17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEE12getClassNameEv
// IDA 0x52e8d4: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52e8d4() {
}

// 0x52e8fc — __ZThn32_N3RBX8GuiLabelD1Ev
// type: void __fastcall(RBX::GuiLabel *__hidden this)
#[doc(alias = "__ZThn32_N3RBX8GuiLabelD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiLabel::~GuiLabel()")]
// was: __ZThn32_N3RBX8GuiLabelD1Ev
// IDA 0x52e8fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52e8fc() {
}

// 0x52e904 — __ZThn32_N3RBX8GuiLabelD0Ev
// type: void __fastcall(RBX::GuiLabel *__hidden this)
#[doc(alias = "__ZThn32_N3RBX8GuiLabelD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiLabel::~GuiLabel()")]
// was: __ZThn32_N3RBX8GuiLabelD0Ev
// IDA 0x52e904: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52e904() {
}

// 0x52e9a8 — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEE12getClassNameEv
// IDA 0x52e9a8: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52e9a8() {
}

// 0x52e9d0 — __ZThn36_N3RBX8GuiLabelD1Ev
// type: void __fastcall(RBX::GuiLabel *__hidden this)
#[doc(alias = "__ZThn36_N3RBX8GuiLabelD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiLabel::~GuiLabel()")]
// was: __ZThn36_N3RBX8GuiLabelD1Ev
// IDA 0x52e9d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52e9d0() {
}

// 0x52e9d8 — __ZThn36_N3RBX8GuiLabelD0Ev
// type: void __fastcall(RBX::GuiLabel *__hidden this)
#[doc(alias = "__ZThn36_N3RBX8GuiLabelD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiLabel::~GuiLabel()")]
// was: __ZThn36_N3RBX8GuiLabelD0Ev
// IDA 0x52e9d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52e9d8() {
}

// 0x52ea7c — __ZN3RBX4Name13callDoDeclareILZNS_9sGuiLabelEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sGuiLabelEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sGuiLabelEEEEvv
// IDA 0x52ea7c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_52ea7c() {
}

// 0x52ea80 — __ZN3RBX4Name9doDeclareILZNS_9sGuiLabelEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGuiLabelEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sGuiLabelEEEERKS0_v
// IDA 0x52ea80: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52ea80() {
}

// 0x52eb60 — __ZN3RBX4Name13callDoDeclareILZNS_10sGuiButtonEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sGuiButtonEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_10sGuiButtonEEEEvv
// IDA 0x52eb60: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_52eb60() {
}

// 0x52eb64 — __ZN3RBX4Name9doDeclareILZNS_10sGuiButtonEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sGuiButtonEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10sGuiButtonEEEERKS0_v
// IDA 0x52eb64: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52eb64() {
}

// 0x52ec44 — __ZN3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x52ec44: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_52ec44() {
}

// 0x52ec48 — __ZN3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x52ec48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52ec48() {
}

// 0x52ece8 — __ZThn32_N3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x52ece8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52ece8() {
}

// 0x52ecf0 — __ZThn32_N3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x52ecf0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52ecf0() {
}

// 0x52ed94 — __ZThn36_N3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x52ed94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52ed94() {
}

// 0x52ed9c — __ZThn36_N3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x52ed9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52ed9c() {
}

// 0x52ee40 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>> const&)")]
// was: __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
// IDA 0x52ee40: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52ee40() {
}

// 0x52eeb4 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE23listenerConnectionAddedEv
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE23listenerConnectionAddedEv")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::listenerConnectionAdded(void)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE23listenerConnectionAddedEv
// IDA 0x52eeb4: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52eeb4() {
}

// 0x52ef00 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev
// IDA 0x52ef00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52ef00() {
}

// 0x52ef2c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev
// IDA 0x52ef2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52ef2c() {
}

// 0x52f000 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
// IDA 0x52f000: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f000() {
}

// 0x52f008 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
// IDA 0x52f008: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f008() {
}

// 0x52f010 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
// IDA 0x52f010: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f010() {
}

// 0x52f028 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev
// IDA 0x52f028: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52f028() {
}

// 0x52f054 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev
// IDA 0x52f054: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52f054() {
}

// 0x52f128 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>> const&)")]
// was: __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
// IDA 0x52f128: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f128() {
}

// 0x52f19c — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE23listenerConnectionAddedEv
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE23listenerConnectionAddedEv")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::listenerConnectionAdded(void)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE23listenerConnectionAddedEv
// IDA 0x52f19c: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f19c() {
}

// 0x52f1e8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev
// IDA 0x52f1e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52f1e8() {
}

// 0x52f214 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev
// IDA 0x52f214: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52f214() {
}

// 0x52f2e8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
// IDA 0x52f2e8: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f2e8() {
}

// 0x52f2f0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
// IDA 0x52f2f0: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f2f0() {
}

// 0x52f2f8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
// IDA 0x52f2f8: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f2f8() {
}

// 0x52f310 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev
// IDA 0x52f310: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52f310() {
}

// 0x52f33c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev
// IDA 0x52f33c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52f33c() {
}

// 0x52f410 — __ZN3rbx13remote_signalIFvvEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvvEEC2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(void)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvvEEC2Ev
// IDA 0x52f410: 122 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f410() {
}

// 0x52f55c — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE21connectSignalListenerEv
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE21connectSignalListenerEv")]
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>::connectSignalListener(void)")]
// was: __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE21connectSignalListenerEv
// IDA 0x52f55c: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f55c() {
}

// 0x52f650 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE
// IDA 0x52f650: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f650() {
}

// 0x52f6b8 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE25signalProducedIncrementedEii
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE25signalProducedIncrementedEii")]
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>::signalProducedIncremented(int,int)")]
// was: __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE25signalProducedIncrementedEii
// IDA 0x52f6b8: 9 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f6b8() {
}

// 0x52f6d0 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEii
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEii")]
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::replicateEvent(RBX::Reflection::EventSource *,int,int)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEii
// IDA 0x52f6d0: 137 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f6d0() {
}

// 0x52f83c — __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
// was: __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
// IDA 0x52f83c: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f83c() {
}

// 0x52f8b0 — __ZN3rbx7signals6signalIFviiEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE6insertEPNS3_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::insert(rbx::signals::signal<void ()(int,int)>::slot *)")]
// was: __ZN3rbx7signals6signalIFviiEE6insertEPNS3_4slotE
// IDA 0x52f8b0: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52f8b0() {
}

// 0x52fabc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiEE4slotEEaSEPS6_
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiEE4slotEEaSEPS6_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int)>::slot>::operator=(rbx::signals::signal<void ()(int,int)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiEE4slotEEaSEPS6_
// IDA 0x52fabc: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52fabc() {
}

// 0x52fae0 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev
// IDA 0x52fae0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52fae0() {
}

// 0x52fb0c — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev
// IDA 0x52fb0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52fb0c() {
}

// 0x52fbe0 — __ZN3rbx7signals6signalIFviiEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE4slot10disconnectEv")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFviiEE4slot10disconnectEv
// IDA 0x52fbe0: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52fbe0() {
}

// 0x52fcf0 — __ZNK3rbx7signals6signalIFviiEE4slot9connectedEv
#[doc(alias = "__ZNK3rbx7signals6signalIFviiEE4slot9connectedEv")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFviiEE4slot9connectedEv
// IDA 0x52fcf0: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52fcf0() {
}

// 0x52fcfc — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
// was: __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
// IDA 0x52fcfc: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52fcfc() {
}

// 0x52fd24 — __ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
// IDA 0x52fd24: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52fd24() {
}

// 0x52fd4c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiButtonEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiButtonEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiButtonEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x52fd4c: 14 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52fd4c() {
}

// 0x52fd74 — __ZN3rbx7signals6signalIFviiEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE6removeEPNS3_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::remove(rbx::signals::signal<void ()(int,int)>::slot *)")]
// was: __ZN3rbx7signals6signalIFviiEE6removeEPNS3_4slotE
// IDA 0x52fd74: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52fd74() {
}

// 0x52fe64 — __ZN3rbx7signals6signalIFviiEE4slot22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE4slot22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFviiEE4slot22safe_static_init_mutexEv
// IDA 0x52fe64: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_52fe64() {
}

// 0x52fe68 — __ZN3rbx7signals6signalIFviiEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE4slot24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFviiEE4slot24safe_static_do_get_mutexEv
// IDA 0x52fe68: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52fe68() {
}

// 0x52ff58 — __ZN3rbx7signals6signalIFviiEE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE4slotD1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFviiEE4slotD1Ev
// IDA 0x52ff58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52ff58() {
}

// 0x52ff84 — __ZN3rbx7signals6signalIFviiEE4slotD0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE4slotD0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFviiEE4slotD0Ev
// IDA 0x52ff84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52ff84() {
}

// 0x530058 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev
// IDA 0x530058: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_530058() {
}

// 0x530084 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev
// IDA 0x530084: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_530084() {
}

// 0x530158 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE21connectSignalListenerEv
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE21connectSignalListenerEv")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::connectSignalListener(void)")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE21connectSignalListenerEv
// IDA 0x530158: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_530158() {
}

// 0x53015c — __ZN3RBX19EventReplicatorImplILi0ENS_9GuiButtonEFvvEE21connectSignalListenerEv
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi0ENS_9GuiButtonEFvvEE21connectSignalListenerEv")]
#[doc(alias = "RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>::connectSignalListener(void)")]
// was: __ZN3RBX19EventReplicatorImplILi0ENS_9GuiButtonEFvvEE21connectSignalListenerEv
// IDA 0x53015c: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53015c() {
}
