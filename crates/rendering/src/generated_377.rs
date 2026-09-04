//! rendering shard 377 — 100 stubs 0x53bb28..0x546504 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 40960->41060 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x53bb28..0x546504 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x53bb28 — __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// IDA 0x53bb28: 7 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53bb28() {
}

// 0x53bb38 — __ZNK3RBX10Reflection13EventDescBaseINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x53bb38: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53bb38() {
}

// 0x53bb4c — __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*>::EventDesc(rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x53bb4c: 191 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53bb4c() {
}

// 0x53bd3c — __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// IDA 0x53bd3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53bd3c() {
}

// 0x53bd60 — __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiObject::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// IDA 0x53bd60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53bd60() {
}

// 0x53be14 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,bool>::PropDescriptor<bool (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(bool)>(char const*,char const*,bool (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x53be14: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53be14() {
}

// 0x53bf28 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEbED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEbED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEbED0Ev
// IDA 0x53bf28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53bf28() {
}

// 0x53bf54 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,bool>::GetSetImpl<bool (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// IDA 0x53bf54: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53bf54() {
}

// 0x53bf58 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,bool>::GetSetImpl<bool (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// IDA 0x53bf58: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53bf58() {
}

// 0x53bf5c — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,bool>::GetSetImpl<bool (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x53bf5c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53bf5c() {
}

// 0x53bf80 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,bool>::GetSetImpl<bool (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x53bf80: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53bf80() {
}

// 0x53bfa4 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,float>::PropDescriptor<float (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(float)>(char const*,char const*,float (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x53bfa4: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53bfa4() {
}

// 0x53c0b8 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEfED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEfED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEfED0Ev
// IDA 0x53c0b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53c0b8() {
}

// 0x53c0e4 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,float>::GetSetImpl<float (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(float)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
// IDA 0x53c0e4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c0e4() {
}

// 0x53c0e8 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,float>::GetSetImpl<float (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(float)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// IDA 0x53c0e8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c0e8() {
}

// 0x53c0ec — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,float>::GetSetImpl<float (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x53c0ec: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c0ec() {
}

// 0x53c10c — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,float>::GetSetImpl<float (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// IDA 0x53c10c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c10c() {
}

// 0x53c2dc — __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x53c2dc: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c2dc() {
}

// 0x53c3f0 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::BrickColor>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEED0Ev
// IDA 0x53c3f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53c3f0() {
}

// 0x53c41c — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::BrickColor)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x53c41c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c41c() {
}

// 0x53c420 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::BrickColor)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x53c420: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c420() {
}

// 0x53c424 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x53c424: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c424() {
}

// 0x53c44c — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x53c44c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c44c() {
}

// 0x53c470 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::EnumPropDescriptor<RBX::GuiObject::SizeConstraint (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::GuiObject::SizeConstraint)>(char const*,char const*,RBX::GuiObject::SizeConstraint (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::GuiObject::SizeConstraint),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x53c470: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c470() {
}

// 0x53c624 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEED0Ev
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEED0Ev
// IDA 0x53c624: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53c624() {
}

// 0x53c650 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10isReadOnlyEv
// IDA 0x53c650: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c650() {
}

// 0x53c660 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE11isWriteOnlyEv
// IDA 0x53c660: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c660() {
}

// 0x53c670 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE11equalValuesEPKNS0_13DescribedBaseES7_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x53c670: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c670() {
}

// 0x53c698 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x53c698: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c698() {
}

// 0x53c6bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x53c6bc: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c6bc() {
}

// 0x53c808 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x53c808: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c808() {
}

// 0x53c82c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE14hasStringValueEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE14hasStringValueEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE14hasStringValueEv
// IDA 0x53c82c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c82c() {
}

// 0x53c830 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE14getStringValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x53c830: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c830() {
}

// 0x53c854 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE14setStringValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x53c854: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c854() {
}

// 0x53c894 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x53c894: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c894() {
}

// 0x53c8b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x53c8b4: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53c8b4() {
}

// 0x53caf4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE13getIndexValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x53caf4: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53caf4() {
}

// 0x53cb10 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE13setIndexValueEPNS0_13DescribedBaseEm")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x53cb10: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cb10() {
}

// 0x53cb44 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE12getEnumValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiObject,RBX::GuiObject::SizeConstraint>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9GuiObjectENS2_14SizeConstraintEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x53cb44: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53cb44() {
}

// 0x53d538 — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EEC2EMS2_FbS3_S4_S5_fbS7_EPKcSD_SD_S4_SD_S5_SD_fSD_bSD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EEC2EMS2_FbS3_S4_S5_fbS7_EPKcSD_SD_S4_SD_S5_SD_fSD_bSD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::BoundFuncDesc(bool (RBX::GuiObject::*)(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),char const*,char const*,char const*,RBX::GuiObject::TweenEasingDirection,char const*,RBX::GuiObject::TweenEasingStyle,char const*,float,char const*,bool,char const*,RBX::Lua::WeakFunctionRef,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EEC2EMS2_FbS3_S4_S5_fbS7_EPKcSD_SD_S4_SD_S5_SD_fSD_bSD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x53d538: 439 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53d538() {
}

// 0x53d980 — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_SB_SC_SB_SC_SB_SC_
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_SB_SC_SB_SC_SB_SC_")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_SB_SC_SB_SC_SB_SC_
// IDA 0x53d980: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53d980() {
}

// 0x53da40 — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EED0Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EED0Ev
// IDA 0x53da40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53da40() {
}

// 0x53dae0 — __ZNK3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x53dae0: 115 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53dae0() {
}

// 0x53dc24 — __ZN3RBX10Reflection11Call6HelperINS_9GuiObjectEMS2_FbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEES3_S4_S5_fbS7_bE4callEPS2_S9_RNS0_7VariantERKS3_RKS4_RKS5_RKfRKbRKS7_
#[doc(alias = "__ZN3RBX10Reflection11Call6HelperINS_9GuiObjectEMS2_FbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEES3_S4_S5_fbS7_bE4callEPS2_S9_RNS0_7VariantERKS3_RKS4_RKS5_RKfRKbRKS7_")]
#[doc(alias = "RBX::Reflection::Call6Helper<RBX::GuiObject,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef,bool>::call(RBX::GuiObject*,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),RBX::Reflection::Variant &,RBX::UDim2 const&,RBX::GuiObject::TweenEasingDirection const&,RBX::GuiObject::TweenEasingStyle const&,float const&,bool const&,RBX::Lua::WeakFunctionRef const&)")]
// was: __ZN3RBX10Reflection11Call6HelperINS_9GuiObjectEMS2_FbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEES3_S4_S5_fbS7_bE4callEPS2_S9_RNS0_7VariantERKS3_RKS4_RKS5_RKfRKbRKS7_
// IDA 0x53dc24: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53dc24() {
}

// 0x53e574 — __ZN3RBX10Reflection9ArgHelper6getArgINS_3Lua15WeakFunctionRefELi6EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_3Lua15WeakFunctionRefELi6EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
#[doc(alias = "RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,6>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_3Lua15WeakFunctionRefELi6EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x53e574: 178 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53e574() {
}

// 0x53e7f4 — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EEC2EMS2_FbS3_S3_S4_S5_fbS7_EPKcSD_SD_SD_S4_SD_S5_SD_fSD_bSD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EEC2EMS2_FbS3_S3_S4_S5_fbS7_EPKcSD_SD_SD_S4_SD_S5_SD_fSD_bSD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::BoundFuncDesc(bool (RBX::GuiObject::*)(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),char const*,char const*,char const*,char const*,RBX::GuiObject::TweenEasingDirection,char const*,RBX::GuiObject::TweenEasingStyle,char const*,float,char const*,bool,char const*,RBX::Lua::WeakFunctionRef,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EEC2EMS2_FbS3_S3_S4_S5_fbS7_EPKcSD_SD_SD_S4_SD_S5_SD_fSD_bSD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x53e7f4: 474 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53e7f4() {
}

// 0x53ec8c — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_SB_SC_SB_SC_SB_SC_SB_SC_
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_SB_SC_SB_SC_SB_SC_SB_SC_")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_SB_SC_SB_SC_SB_SC_SB_SC_
// IDA 0x53ec8c: 84 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ec8c() {
}

// 0x53ed6c — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EED0Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EED0Ev
// IDA 0x53ed6c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53ed6c() {
}

// 0x53ee0c — __ZNK3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x53ee0c: 123 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ee0c() {
}

// 0x53ef64 — __ZN3RBX10Reflection11Call7HelperINS_9GuiObjectEMS2_FbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEES3_S3_S4_S5_fbS7_bE4callEPS2_S9_RNS0_7VariantERKS3_SF_RKS4_RKS5_RKfRKbRKS7_
#[doc(alias = "__ZN3RBX10Reflection11Call7HelperINS_9GuiObjectEMS2_FbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEES3_S3_S4_S5_fbS7_bE4callEPS2_S9_RNS0_7VariantERKS3_SF_RKS4_RKS5_RKfRKbRKS7_")]
#[doc(alias = "RBX::Reflection::Call7Helper<RBX::GuiObject,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef,bool>::call(RBX::GuiObject*,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),RBX::Reflection::Variant &,RBX::UDim2 const&,RBX::UDim2 const&,RBX::GuiObject::TweenEasingDirection const&,RBX::GuiObject::TweenEasingStyle const&,float const&,bool const&,RBX::Lua::WeakFunctionRef const&)")]
// was: __ZN3RBX10Reflection11Call7HelperINS_9GuiObjectEMS2_FbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEES3_S3_S4_S5_fbS7_bE4callEPS2_S9_RNS0_7VariantERKS3_SF_RKS4_RKS5_RKfRKbRKS7_
// IDA 0x53ef64: 131 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53ef64() {
}

// 0x53f8dc — __ZN3RBX10Reflection9ArgHelper6getArgINS_3Lua15WeakFunctionRefELi7EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_3Lua15WeakFunctionRefELi7EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
#[doc(alias = "RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,7>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_3Lua15WeakFunctionRefELi7EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x53f8dc: 178 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53f8dc() {
}

// 0x53fb5c — __ZN3RBX9GuiButtonD2Ev
#[doc(alias = "__ZN3RBX9GuiButtonD2Ev")]
#[doc(alias = "RBX::GuiButton::~GuiButton()")]
// was: __ZN3RBX9GuiButtonD2Ev
// IDA 0x53fb5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53fb5c() {
}

// 0x53fea0 — __ZN3rbx13remote_signalIFvvEED2Ev
#[doc(alias = "__ZN3rbx13remote_signalIFvvEED2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(void)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvvEED2Ev
// IDA 0x53fea0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53fea0() {
}

// 0x53ffec — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEED2Ev
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEED2Ev")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::~EventReplicatorBase()")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEED2Ev
// IDA 0x53ffec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_53ffec() {
}

// 0x54011c — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEED2Ev
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEED2Ev")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::~EventReplicatorBase()")]
// was: __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEED2Ev
// IDA 0x54011c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_54011c() {
}

// 0x54024c — __ZN5boost9function2IvPN3RBX9GuiObjectENS1_5UDim2EE5clearEv
#[doc(alias = "__ZN5boost9function2IvPN3RBX9GuiObjectENS1_5UDim2EE5clearEv")]
#[doc(alias = "boost::function2<void,RBX::GuiObject *,RBX::UDim2>::clear(void)")]
// was: __ZN5boost9function2IvPN3RBX9GuiObjectENS1_5UDim2EE5clearEv
// IDA 0x54024c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54024c() {
}

// 0x540278 — __ZNK5boost9function1IvN3RBX9GuiObject11TweenStatusEEclES3_
#[doc(alias = "__ZNK5boost9function1IvN3RBX9GuiObject11TweenStatusEEclES3_")]
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::operator()(RBX::GuiObject::TweenStatus)const")]
// was: __ZNK5boost9function1IvN3RBX9GuiObject11TweenStatusEEclES3_
// IDA 0x540278: 67 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_540278() {
}

// 0x54033c — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EED2Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EED2Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ENS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi6EED2Ev
// IDA 0x54033c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_54033c() {
}

// 0x5404ac — __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EED2Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EED2Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9GuiObjectEFbNS_5UDim2ES3_NS2_20TweenEasingDirectionENS2_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefEELi7EED2Ev
// IDA 0x5404ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5404ac() {
}

// 0x540634 — __GLOBAL__I_a_207
#[doc(alias = "__GLOBAL__I_a_207")]
#[doc(alias = "global constructor keyed to_a_207")]
// was: __GLOBAL__I_a_207
// IDA 0x540634: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_540634() {
}

// 0x541c1c — __ZNK3RBX10GuiService20getModalDialogStatusEv
#[doc(alias = "__ZNK3RBX10GuiService20getModalDialogStatusEv")]
#[doc(alias = "RBX::GuiService::getModalDialogStatus(void)const")]
// was: __ZNK3RBX10GuiService20getModalDialogStatusEv
// IDA 0x541c1c: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_541c1c() {
}

// 0x541c98 — __ZN3RBX10GuiService6addKeyESs
#[doc(alias = "__ZN3RBX10GuiService6addKeyESs")]
#[doc(alias = "RBX::GuiService::addKey(std::string)")]
// was: __ZN3RBX10GuiService6addKeyESs
// IDA 0x541c98: 108 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_541c98() {
}

// 0x541de8 — __ZN3RBX10GuiService9removeKeyESs
#[doc(alias = "__ZN3RBX10GuiService9removeKeyESs")]
#[doc(alias = "RBX::GuiService::removeKey(std::string)")]
// was: __ZN3RBX10GuiService9removeKeyESs
// IDA 0x541de8: 109 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_541de8() {
}

// 0x541f3c — __ZN3RBX10GuiService13addSpecialKeyENS0_10SpecialKeyE
#[doc(alias = "__ZN3RBX10GuiService13addSpecialKeyENS0_10SpecialKeyE")]
#[doc(alias = "RBX::GuiService::addSpecialKey(RBX::GuiService::SpecialKey)")]
// was: __ZN3RBX10GuiService13addSpecialKeyENS0_10SpecialKeyE
// IDA 0x541f3c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_541f3c() {
}

// 0x541f8c — __ZN3RBX10GuiService15addNotificationESsSsSsiNS_3Lua15WeakFunctionRefE
#[doc(alias = "__ZN3RBX10GuiService15addNotificationESsSsSsiNS_3Lua15WeakFunctionRefE")]
#[doc(alias = "RBX::GuiService::addNotification(std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef)")]
// was: __ZN3RBX10GuiService15addNotificationESsSsSsiNS_3Lua15WeakFunctionRefE
// IDA 0x541f8c: 481 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_541f8c() {
}

// 0x5424e0 — __ZN3RBX10GuiService15addCenterDialogEN5boost10shared_ptrINS_8InstanceEEENS0_16CenterDialogTypeENS_3Lua15WeakFunctionRefES7_
#[doc(alias = "__ZN3RBX10GuiService15addCenterDialogEN5boost10shared_ptrINS_8InstanceEEENS0_16CenterDialogTypeENS_3Lua15WeakFunctionRefES7_")]
#[doc(alias = "RBX::GuiService::addCenterDialog(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef)")]
// was: __ZN3RBX10GuiService15addCenterDialogEN5boost10shared_ptrINS_8InstanceEEENS0_16CenterDialogTypeENS_3Lua15WeakFunctionRefES7_
// IDA 0x5424e0: 723 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5424e0() {
}

// 0x542c84 — __ZN3RBX10GuiService18removeCenterDialogEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "__ZN3RBX10GuiService18removeCenterDialogEN5boost10shared_ptrINS_8InstanceEEE")]
#[doc(alias = "RBX::GuiService::removeCenterDialog(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX10GuiService18removeCenterDialogEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x542c84: 307 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_542c84() {
}

// 0x542fb4 — __ZN3RBX10GuiService17setGlobalGuiInsetEiiii
#[doc(alias = "__ZN3RBX10GuiService17setGlobalGuiInsetEiiii")]
#[doc(alias = "RBX::GuiService::setGlobalGuiInset(int,int,int,int)")]
// was: __ZN3RBX10GuiService17setGlobalGuiInsetEiiii
// IDA 0x542fb4: 31 insns (VMOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_542fb4() {
}

// 0x543028 — __ZN3RBX10GuiService23setShowLegacyPlayerListEb
#[doc(alias = "__ZN3RBX10GuiService23setShowLegacyPlayerListEb")]
#[doc(alias = "RBX::GuiService::setShowLegacyPlayerList(bool)")]
// was: __ZN3RBX10GuiService23setShowLegacyPlayerListEb
// IDA 0x543028: 9 insns (LDRB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_543028() {
}

// 0x543048 — __ZN3RBX10GuiService17openBrowserWindowESs
#[doc(alias = "__ZN3RBX10GuiService17openBrowserWindowESs")]
#[doc(alias = "RBX::GuiService::openBrowserWindow(std::string)")]
// was: __ZN3RBX10GuiService17openBrowserWindowESs
// IDA 0x543048: 191 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_543048() {
}

// 0x54366c — __ZN3RBX15StringConverterINS_10GuiService10SpecialKeyEE14convertToValueERKSsRS2_
#[doc(alias = "__ZN3RBX15StringConverterINS_10GuiService10SpecialKeyEE14convertToValueERKSsRS2_")]
#[doc(alias = "RBX::StringConverter<RBX::GuiService::SpecialKey>::convertToValue(std::string const&,RBX::GuiService::SpecialKey&)")]
// was: __ZN3RBX15StringConverterINS_10GuiService10SpecialKeyEE14convertToValueERKSsRS2_
// IDA 0x54366c: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54366c() {
}

// 0x5436b8 — __ZN3RBX15StringConverterINS_10GuiService16CenterDialogTypeEE14convertToValueERKSsRS2_
#[doc(alias = "__ZN3RBX15StringConverterINS_10GuiService16CenterDialogTypeEE14convertToValueERKSsRS2_")]
#[doc(alias = "RBX::StringConverter<RBX::GuiService::CenterDialogType>::convertToValue(std::string const&,RBX::GuiService::CenterDialogType&)")]
// was: __ZN3RBX15StringConverterINS_10GuiService16CenterDialogTypeEE14convertToValueERKSsRS2_
// IDA 0x5436b8: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5436b8() {
}

// 0x543704 — __ZN3RBX10GuiServiceC1Ev
#[doc(alias = "__ZN3RBX10GuiServiceC1Ev")]
#[doc(alias = "RBX::GuiService::GuiService(void)")]
// was: __ZN3RBX10GuiServiceC1Ev
// IDA 0x543704: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_543704() {
}

// 0x543708 — __ZN3RBX10GuiServiceC2Ev
#[doc(alias = "__ZN3RBX10GuiServiceC2Ev")]
#[doc(alias = "RBX::GuiService::GuiService(void)")]
// was: __ZN3RBX10GuiServiceC2Ev
// IDA 0x543708: 399 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_543708() {
}

// 0x543b40 — __ZNK3RBX10GuiService26shouldPreemptCurrentDialogEPNS0_13DialogWrapperE
#[doc(alias = "__ZNK3RBX10GuiService26shouldPreemptCurrentDialogEPNS0_13DialogWrapperE")]
#[doc(alias = "RBX::GuiService::shouldPreemptCurrentDialog(RBX::GuiService::DialogWrapper *)const")]
// was: __ZNK3RBX10GuiService26shouldPreemptCurrentDialogEPNS0_13DialogWrapperE
// IDA 0x543b40: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_543b40() {
}

// 0x543c10 — __ZNK3RBX10GuiService13getScreenSizeEv
#[doc(alias = "__ZNK3RBX10GuiService13getScreenSizeEv")]
#[doc(alias = "RBX::GuiService::getScreenSize(void)const")]
// was: __ZNK3RBX10GuiService13getScreenSizeEv
// IDA 0x543c10: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_543c10() {
}

// 0x543c3c — __ZN3RBX10GuiService18queueDialogWrapperEPNS0_13DialogWrapperEb
#[doc(alias = "__ZN3RBX10GuiService18queueDialogWrapperEPNS0_13DialogWrapperEb")]
#[doc(alias = "RBX::GuiService::queueDialogWrapper(RBX::GuiService::DialogWrapper *,bool)")]
// was: __ZN3RBX10GuiService18queueDialogWrapperEPNS0_13DialogWrapperEb
// IDA 0x543c3c: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_543c3c() {
}

// 0x543c90 — __ZN3RBXL14InvokeCallbackEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefEb
#[doc(alias = "__ZN3RBXL14InvokeCallbackEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefEb")]
#[doc(alias = "RBX::InvokeCallback(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool)")]
// was: __ZN3RBXL14InvokeCallbackEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefEb
// IDA 0x543c90: 400 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_543c90() {
}

// 0x5440c0 — __ZN3RBX10GuiService17showWaitingDialogENS0_16CenterDialogTypeE
#[doc(alias = "__ZN3RBX10GuiService17showWaitingDialogENS0_16CenterDialogTypeE")]
#[doc(alias = "RBX::GuiService::showWaitingDialog(RBX::GuiService::CenterDialogType)")]
// was: __ZN3RBX10GuiService17showWaitingDialogENS0_16CenterDialogTypeE
// IDA 0x5440c0: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5440c0() {
}

// 0x54416c — __ZN3RBX10GuiService11dispatchKeyENS0_10SpecialKeyE
#[doc(alias = "__ZN3RBX10GuiService11dispatchKeyENS0_10SpecialKeyE")]
#[doc(alias = "RBX::GuiService::dispatchKey(RBX::GuiService::SpecialKey)")]
// was: __ZN3RBX10GuiService11dispatchKeyENS0_10SpecialKeyE
// IDA 0x54416c: 134 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54416c() {
}

// 0x5442e4 — __ZN3RBX10GuiService14processKeyDownENS_8GuiEventE
#[doc(alias = "__ZN3RBX10GuiService14processKeyDownENS_8GuiEventE")]
#[doc(alias = "RBX::GuiService::processKeyDown(RBX::GuiEvent)")]
// was: __ZN3RBX10GuiService14processKeyDownENS_8GuiEventE
// IDA 0x5442e4: 347 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5442e4() {
}

// 0x5446a8 — __ZN3RBXL26InvokeNotificationCallbackEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefE
#[doc(alias = "__ZN3RBXL26InvokeNotificationCallbackEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefE")]
#[doc(alias = "RBX::InvokeNotificationCallback(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef)")]
// was: __ZN3RBXL26InvokeNotificationCallbackEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefE
// IDA 0x5446a8: 328 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5446a8() {
}

// 0x5449fc — __ZNK3RBX10GuiService10getVersionEv
#[doc(alias = "__ZNK3RBX10GuiService10getVersionEv")]
#[doc(alias = "RBX::GuiService::getVersion(void)const")]
// was: __ZNK3RBX10GuiService10getVersionEv
// IDA 0x5449fc: 3 insns (VMOV.F64..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5449fc() {
}

// 0x544a50 — __ZNK3RBX10GuiService12getIsWindowsEv
#[doc(alias = "__ZNK3RBX10GuiService12getIsWindowsEv")]
#[doc(alias = "RBX::GuiService::getIsWindows(void)const")]
// was: __ZNK3RBX10GuiService12getIsWindowsEv
// IDA 0x544a50: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_544a50() {
}

// 0x544a54 — __ZNK3RBX10GuiService13getUseLuaChatEv
#[doc(alias = "__ZNK3RBX10GuiService13getUseLuaChatEv")]
#[doc(alias = "RBX::GuiService::getUseLuaChat(void)const")]
// was: __ZNK3RBX10GuiService13getUseLuaChatEv
// IDA 0x544a54: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_544a54() {
}

// 0x544b48 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsSsSsiNS_3Lua15WeakFunctionRefEELi5EED1Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsSsSsiNS_3Lua15WeakFunctionRefEELi5EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string,std::string,std::string,int,RBX::Lua::WeakFunctionRef),5>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsSsSsiNS_3Lua15WeakFunctionRefEELi5EED1Ev
// IDA 0x544b48: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_544b48() {
}

// 0x544b4c — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EED1Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EED1Ev
// IDA 0x544b4c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_544b4c() {
}

// 0x544b50 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
// IDA 0x544b50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_544b50() {
}

// 0x544ca0 — __ZNK3RBX10GuiService23getShowLegacyPlayerListEv
#[doc(alias = "__ZNK3RBX10GuiService23getShowLegacyPlayerListEv")]
#[doc(alias = "RBX::GuiService::getShowLegacyPlayerList(void)const")]
// was: __ZNK3RBX10GuiService23getShowLegacyPlayerListEv
// IDA 0x544ca0: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_544ca0() {
}

// 0x545740 — __ZN3RBX15ServiceProvider6createINS_14CoreGuiServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_14CoreGuiServiceEEEPT_PKNS_8InstanceE")]
#[doc(alias = "RBX::CoreGuiService * RBX::ServiceProvider::create<RBX::CoreGuiService>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider6createINS_14CoreGuiServiceEEEPT_PKNS_8InstanceE
// IDA 0x545740: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_545740() {
}

// 0x545758 — __ZNSt3mapIN3RBX10GuiService16CenterDialogTypeESt4listIPNS1_13DialogWrapperESaIS5_EESt4lessIS2_ESaISt4pairIKS2_S7_EEEixERSB_
#[doc(alias = "__ZNSt3mapIN3RBX10GuiService16CenterDialogTypeESt4listIPNS1_13DialogWrapperESaIS5_EESt4lessIS2_ESaISt4pairIKS2_S7_EEEixERSB_")]
#[doc(alias = "std::map<RBX::GuiService::CenterDialogType,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::operator[](RBX::GuiService::CenterDialogType const&)")]
// was: __ZNSt3mapIN3RBX10GuiService16CenterDialogTypeESt4listIPNS1_13DialogWrapperESaIS5_EESt4lessIS2_ESaISt4pairIKS2_S7_EEEixERSB_
// IDA 0x545758: 134 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_545758() {
}

// 0x5458b0 — __ZN3RBX9weak_fromINS_9GuiObjectEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "__ZN3RBX9weak_fromINS_9GuiObjectEEEN5boost8weak_ptrIT_EEPS4_")]
#[doc(alias = "rbx_core::WeakPtr<RBX::GuiObject> RBX::weak_from<RBX::GuiObject>(RBX::GuiObject*)")]
// was: __ZN3RBX9weak_fromINS_9GuiObjectEEEN5boost8weak_ptrIT_EEPS4_
// IDA 0x5458b0: 188 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5458b0() {
}

// 0x545ab8 — __ZN5boost8functionIFvvEEaSINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEbENS4_5list3INS4_5valueIS9_EENSF_ISB_EENSF_IbEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS2_E4typeESM_
#[doc(alias = "__ZN5boost8functionIFvvEEaSINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEbENS4_5list3INS4_5valueIS9_EENSF_ISB_EENSF_IbEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS2_E4typeESM_")]
// was: __ZN5boost8functionIFvvEEaSINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEbENS4_5list3INS4_5valueIS9_EENSF_ISB_EENSF_IbEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS2_E4typeESM_
// IDA 0x545ab8: 150 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_545ab8() {
}

// 0x545c48 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefEbS4_S6_bEENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefEbS4_S6_bEENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_")]
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool>::type> boost::bind<void,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool>(void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefEbS4_S6_bEENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_
// IDA 0x545c48: 187 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_545c48() {
}

// 0x545e28 — __ZNSt3mapIN5boost8weak_ptrIN3RBX9GuiObjectEEEPNS2_10GuiService13DialogWrapperESt4lessIS4_ESaISt4pairIKS4_S7_EEEixERSB_
#[doc(alias = "__ZNSt3mapIN5boost8weak_ptrIN3RBX9GuiObjectEEEPNS2_10GuiService13DialogWrapperESt4lessIS4_ESaISt4pairIKS4_S7_EEEixERSB_")]
#[doc(alias = "std::map<rbx_core::WeakPtr<RBX::GuiObject>,RBX::GuiService::DialogWrapper *,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::operator[](rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// was: __ZNSt3mapIN5boost8weak_ptrIN3RBX9GuiObjectEEEPNS2_10GuiService13DialogWrapperESt4lessIS4_ESaISt4pairIKS4_S7_EEEixERSB_
// IDA 0x545e28: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_545e28() {
}

// 0x545f38 — __ZN5boost4bindINS_8functionIFvvEEEEENS_3_bi6bind_tINS4_11unspecifiedET_NS4_5list0EEES7_
#[doc(alias = "__ZN5boost4bindINS_8functionIFvvEEEEENS_3_bi6bind_tINS4_11unspecifiedET_NS4_5list0EEES7_")]
#[doc(alias = "boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0> boost::bind<boost::function<void ()(void)>>(boost::function<void ()(void)>)")]
// was: __ZN5boost4bindINS_8functionIFvvEEEEENS_3_bi6bind_tINS4_11unspecifiedET_NS4_5list0EEES7_
// IDA 0x545f38: 68 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_545f38() {
}

// 0x545ffc — __ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE6removeERKS3_
#[doc(alias = "__ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE6removeERKS3_")]
#[doc(alias = "std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::remove(RBX::GuiService::DialogWrapper * const&)")]
// was: __ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE6removeERKS3_
// IDA 0x545ffc: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_545ffc() {
}

// 0x546034 — __ZN3rbx7signals16signal_with_argsILi2EFvN3RBX10GuiService10SpecialKeyESsEEclES4_Ss
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFvN3RBX10GuiService10SpecialKeyESsEEclES4_Ss")]
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(RBX::GuiService::SpecialKey,std::string)>::operator()(RBX::GuiService::SpecialKey,std::string)")]
// was: __ZN3rbx7signals16signal_with_argsILi2EFvN3RBX10GuiService10SpecialKeyESsEEclES4_Ss
// IDA 0x546034: 224 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_546034() {
}

// 0x5462a4 — __ZN3rbx7signals16signal_with_argsILi2EFvSsSsEEclESsSs
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFvSsSsEEclESsSs")]
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,std::string)>::operator()(std::string,std::string)")]
// was: __ZN3rbx7signals16signal_with_argsILi2EFvSsSsEEclESsSs
// IDA 0x5462a4: 216 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5462a4() {
}

// 0x546504 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_18NotificationObjectEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_18NotificationObjectEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationObject> RBX::Creatable<RBX::Instance>::create<RBX::NotificationObject>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_18NotificationObjectEEEN5boost10shared_ptrIT_EEv
// IDA 0x546504: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_546504() {
}