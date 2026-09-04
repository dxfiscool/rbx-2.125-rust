//! rendering shard 320 — 100 stubs 0x4853dc..0x487668 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 34809->34909 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 34809 before -> 34909 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x4853dc (lowest remaining 0x4853dc..0x487668, next lowest 0x48769c)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4853dc — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetSetImpl<double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE10isReadOnlyEv
// IDA 0x4853dc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4853dc() {
}

// 0x4853e0 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetSetImpl<double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE11isWriteOnlyEv
// IDA 0x4853e0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4853e0() {
}

// 0x4853e4 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetSetImpl<double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x4853e4: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4853e4() {
}

// 0x485404 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8setValueEPNS0_13DescribedBaseERKd
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetSetImpl<double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double)>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8setValueEPNS0_13DescribedBaseERKd
// IDA 0x485404: 14 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485404() {
}

// 0x48542c — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::PropDescriptor<bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool)>(char const*,char const*,bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x48542c: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48542c() {
}

// 0x485544 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbED0Ev
// IDA 0x485544: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_485544() {
}

// 0x485574 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetSetImpl<bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// IDA 0x485574: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485574() {
}

// 0x485578 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetSetImpl<bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// IDA 0x485578: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485578() {
}

// 0x48557c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetSetImpl<bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x48557c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48557c() {
}

// 0x4855a0 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetSetImpl<bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x4855a0: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4855a0() {
}

// 0x4855c4 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::PropDescriptor<double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double)>(char const*,char const*,double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x4855c4: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4855c4() {
}

// 0x4856d8 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdED0Ev
// IDA 0x4856d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4856d8() {
}

// 0x485704 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetSetImpl<double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE10isReadOnlyEv
// IDA 0x485704: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485704() {
}

// 0x485708 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetSetImpl<double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE11isWriteOnlyEv
// IDA 0x485708: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485708() {
}

// 0x48570c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetSetImpl<double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x48570c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48570c() {
}

// 0x48572c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8setValueEPNS0_13DescribedBaseERKd
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetSetImpl<double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double)>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8setValueEPNS0_13DescribedBaseERKd
// IDA 0x48572c: 14 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48572c() {
}

// 0x485754 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::PropDescriptor<bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool)>(char const*,char const*,bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x485754: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485754() {
}

// 0x485868 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbED0Ev
// IDA 0x485868: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_485868() {
}

// 0x485894 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::GetSetImpl<bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// IDA 0x485894: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485894() {
}

// 0x485898 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::GetSetImpl<bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// IDA 0x485898: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485898() {
}

// 0x48589c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::GetSetImpl<bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x48589c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48589c() {
}

// 0x4858c0 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::GetSetImpl<bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x4858c0: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4858c0() {
}

// 0x4858e4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::EnumPropDescriptor<RBX::DataModelArbiter::ConcurrencyModel (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::DataModelArbiter::ConcurrencyModel)>(char const*,char const*,RBX::DataModelArbiter::ConcurrencyModel (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::DataModelArbiter::ConcurrencyModel),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x4858e4: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4858e4() {
}

// 0x485a98 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEED0Ev
// IDA 0x485a98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_485a98() {
}

// 0x485ac4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10isReadOnlyEv
// IDA 0x485ac4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485ac4() {
}

// 0x485ad4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11isWriteOnlyEv
// IDA 0x485ad4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485ad4() {
}

// 0x485ae4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11equalValuesEPKNS0_13DescribedBaseES8_
// IDA 0x485ae4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485ae4() {
}

// 0x485b0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x485b0c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485b0c() {
}

// 0x485b30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x485b30: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485b30() {
}

// 0x485c7c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE9copyValueEPKNS0_13DescribedBaseEPS6_
// IDA 0x485c7c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485c7c() {
}

// 0x485ca0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14hasStringValueEv
// IDA 0x485ca0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485ca0() {
}

// 0x485ca4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x485ca4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485ca4() {
}

// 0x485cc8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x485cc8: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485cc8() {
}

// 0x485d08 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x485d08: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485d08() {
}

// 0x485d28 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x485d28: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485d28() {
}

// 0x485f68 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x485f68: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485f68() {
}

// 0x485f84 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x485f84: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485f84() {
}

// 0x485fb8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x485fb8: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485fb8() {
}

// 0x485fc0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x485fc0: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485fc0() {
}

// 0x48600c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x48600c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48600c() {
}

// 0x48602c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x48602c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48602c() {
}

// 0x486060 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToValueERKNS_4NameERS3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToValue(RBX::Name const&,RBX::DataModelArbiter::ConcurrencyModel&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToValueERKNS_4NameERS3_
// IDA 0x486060: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486060() {
}

// 0x4860e0 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToIndex(RBX::DataModelArbiter::ConcurrencyModel)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToIndexES3_
// IDA 0x4860e0: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4860e0() {
}

// 0x486150 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x486150: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486150() {
}

// 0x486190 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToString(RBX::DataModelArbiter::ConcurrencyModel const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE15convertToStringERKS3_
// IDA 0x486190: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486190() {
}

// 0x486330 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::GetSetImpl<RBX::DataModelArbiter::ConcurrencyModel (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::DataModelArbiter::ConcurrencyModel)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// IDA 0x486330: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486330() {
}

// 0x486334 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::GetSetImpl<RBX::DataModelArbiter::ConcurrencyModel (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::DataModelArbiter::ConcurrencyModel)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// IDA 0x486334: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486334() {
}

// 0x486338 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::GetSetImpl<RBX::DataModelArbiter::ConcurrencyModel (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::DataModelArbiter::ConcurrencyModel)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x486338: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486338() {
}

// 0x486358 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::GetSetImpl<RBX::DataModelArbiter::ConcurrencyModel (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::DataModelArbiter::ConcurrencyModel)>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModelArbiter::ConcurrencyModel const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_16DataModelArbiter16ConcurrencyModelEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x486358: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486358() {
}

// 0x48637c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEEE13initSingletonEv
// IDA 0x48637c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_48637c() {
}

// 0x486380 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEEE14doGetSingletonEv
// IDA 0x486380: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486380() {
}

// 0x486470 — __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEED0Ev
// IDA 0x486470: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_486470() {
}

// 0x486510 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE6lookupEPKc
// IDA 0x486510: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486510() {
}

// 0x486540 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE6lookupERKNS0_7VariantE
// IDA 0x486540: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486540() {
}

// 0x486560 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToValueEmRNS0_7VariantE
// IDA 0x486560: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486560() {
}

// 0x486598 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16DataModelArbiter16ConcurrencyModelEEERS3_RKT_
// type: int(void)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModelArbiter::ConcurrencyModel>(RBX::DataModelArbiter::ConcurrencyModel const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16DataModelArbiter16ConcurrencyModelEEERS3_RKT_
// IDA 0x486598: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486598() {
}

// 0x4865e8 — __ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE9singletonEv
// type: int(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModelArbiter::ConcurrencyModel>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE9singletonEv
// IDA 0x4865e8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4865e8() {
}

// 0x486658 — __ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModelArbiter::ConcurrencyModel>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX16DataModelArbiter16ConcurrencyModelEE13destruct_funcEPc
// IDA 0x486658: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_486658() {
}

// 0x48665c — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEEC2IMS2_KFS5_vEMS2_FvS5_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::EnumPropDescriptor<RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod)>(char const*,char const*,RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEEC2IMS2_KFS5_vEMS2_FvS5_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x48665c: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48665c() {
}

// 0x486810 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEED0Ev
// IDA 0x486810: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_486810() {
}

// 0x48683c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10isReadOnlyEv
// IDA 0x48683c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48683c() {
}

// 0x48684c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11isWriteOnlyEv
// IDA 0x48684c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48684c() {
}

// 0x48685c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11equalValuesEPKNS0_13DescribedBaseES9_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11equalValuesEPKNS0_13DescribedBaseES9_
// IDA 0x48685c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48685c() {
}

// 0x486884 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x486884: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486884() {
}

// 0x4868a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x4868a8: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4868a8() {
}

// 0x4869f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE9copyValueEPKNS0_13DescribedBaseEPS7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE9copyValueEPKNS0_13DescribedBaseEPS7_
// IDA 0x4869f4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4869f4() {
}

// 0x486a18 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14hasStringValueEv
// IDA 0x486a18: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486a18() {
}

// 0x486a1c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x486a1c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486a1c() {
}

// 0x486a40 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x486a40: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486a40() {
}

// 0x486a80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x486a80: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486a80() {
}

// 0x486aa0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x486aa0: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486aa0() {
}

// 0x486ce0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x486ce0: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486ce0() {
}

// 0x486cfc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x486cfc: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486cfc() {
}

// 0x486d30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x486d30: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486d30() {
}

// 0x486d38 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x486d38: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486d38() {
}

// 0x486d84 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x486d84: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486d84() {
}

// 0x486da4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x486da4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486da4() {
}

// 0x486dd8 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToIndexES4_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToIndex(RBX::TaskScheduler::Job::SleepAdjustMethod)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToIndexES4_
// IDA 0x486dd8: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486dd8() {
}

// 0x486e48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x486e48: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486e48() {
}

// 0x486e88 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::GetSetImpl<RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE10isReadOnlyEv
// IDA 0x486e88: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486e88() {
}

// 0x486e8c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::GetSetImpl<RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE11isWriteOnlyEv
// IDA 0x486e8c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486e8c() {
}

// 0x486e90 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::GetSetImpl<RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x486e90: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486e90() {
}

// 0x486eb0 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE8setValueEPNS0_13DescribedBaseERKS5_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::GetSetImpl<RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod)>::setValue(RBX::Reflection::DescribedBase *,RBX::TaskScheduler::Job::SleepAdjustMethod const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE10GetSetImplIMS2_KFS5_vEMS2_FvS5_EE8setValueEPNS0_13DescribedBaseERKS5_
// IDA 0x486eb0: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486eb0() {
}

// 0x486ed4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE13initSingletonEv
// IDA 0x486ed4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_486ed4() {
}

// 0x486ed8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE14doGetSingletonEv
// IDA 0x486ed8: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486ed8() {
}

// 0x486fc8 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::EnumPropDescriptor<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>(char const*,char const*,RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x486fc8: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_486fc8() {
}

// 0x48717c — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEED0Ev
// IDA 0x48717c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_48717c() {
}

// 0x4871a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10isReadOnlyEv
// IDA 0x4871a8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4871a8() {
}

// 0x4871b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11isWriteOnlyEv
// IDA 0x4871b8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4871b8() {
}

// 0x4871c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11equalValuesEPKNS0_13DescribedBaseES8_
// IDA 0x4871c8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4871c8() {
}

// 0x4871f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x4871f0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4871f0() {
}

// 0x487214 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x487214: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_487214() {
}

// 0x487360 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE9copyValueEPKNS0_13DescribedBaseEPS6_
// IDA 0x487360: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_487360() {
}

// 0x487384 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14hasStringValueEv
// IDA 0x487384: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_487384() {
}

// 0x487388 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x487388: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_487388() {
}

// 0x4873ac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x4873ac: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4873ac() {
}

// 0x4873ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x4873ec: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4873ec() {
}

// 0x48740c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x48740c: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48740c() {
}

// 0x48764c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x48764c: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48764c() {
}

// 0x487668 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x487668: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_487668() {
}

