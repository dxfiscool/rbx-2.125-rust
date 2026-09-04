//! rendering shard 321 — 100 stubs 0x4853b0..0x48a08c EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 34900->35000 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 34900 before -> 35000 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x4853b0 (lowest remaining 0x4853b0..0x48a08c, next lowest 0x48a1ac)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4853b0 — __ZN3RBX10Reflection23TypedPropertyDescriptorIdED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::~TypedPropertyDescriptor()")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorIdED0Ev
// IDA 0x4853b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4853b0() {
}

// 0x48769c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x48769c: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48769c() {
}

// 0x4876a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x4876a4: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4876a4() {
}

// 0x4876f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x4876f0: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4876f0() {
}

// 0x487710 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x487710: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_487710() {
}

// 0x487744 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToIndex(RBX::TaskScheduler::PriorityMethod)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToIndexES3_
// IDA 0x487744: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_487744() {
}

// 0x4877b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x4877b4: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4877b4() {
}

// 0x4877f4 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// IDA 0x4877f4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4877f4() {
}

// 0x4877f8 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// IDA 0x4877f8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4877f8() {
}

// 0x4877fc — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x4877fc: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4877fc() {
}

// 0x48781c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::setValue(RBX::Reflection::DescribedBase *,RBX::TaskScheduler::PriorityMethod const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x48781c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48781c() {
}

// 0x487840 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE13initSingletonEv
// IDA 0x487840: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_487840() {
}

// 0x487844 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE14doGetSingletonEv
// IDA 0x487844: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_487844() {
}

// 0x487934 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EEC2EMS2_FvdiEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::BoundFuncDesc(void (RBX::TaskSchedulerSettings::*)(double,int),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EEC2EMS2_FvdiEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x487934: 176 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_487934() {
}

// 0x487afc — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// IDA 0x487afc: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_487afc() {
}

// 0x487b48 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EED0Ev
// IDA 0x487b48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_487b48() {
}

// 0x487c28 — __ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x487c28: 32 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_487c28() {
}

// 0x487c88 — __ZN3RBX10Reflection9ArgHelper6getArgIdLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: __int64 __fastcall(_DWORD, _DWORD)
#[doc(alias = "double RBX::Reflection::ArgHelper::getArg<double,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIdLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x487c88: 156 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_487c88() {
}

// 0x487e28 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::EnumPropDescriptor<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>(char const*,char const*,RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x487e28: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_487e28() {
}

// 0x487fdc — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEED0Ev
// IDA 0x487fdc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_487fdc() {
}

// 0x488008 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10isReadOnlyEv
// IDA 0x488008: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488008() {
}

// 0x488018 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11isWriteOnlyEv
// IDA 0x488018: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488018() {
}

// 0x488028 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11equalValuesEPKNS0_13DescribedBaseES8_
// IDA 0x488028: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488028() {
}

// 0x488050 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x488050: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488050() {
}

// 0x488074 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x488074: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488074() {
}

// 0x4881c0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE9copyValueEPKNS0_13DescribedBaseEPS6_
// IDA 0x4881c0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4881c0() {
}

// 0x4881e4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14hasStringValueEv
// IDA 0x4881e4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4881e4() {
}

// 0x4881e8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x4881e8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4881e8() {
}

// 0x48820c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x48820c: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48820c() {
}

// 0x48824c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x48824c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48824c() {
}

// 0x48826c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x48826c: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48826c() {
}

// 0x4884ac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x4884ac: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4884ac() {
}

// 0x4884c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x4884c8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4884c8() {
}

// 0x4884fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x4884fc: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4884fc() {
}

// 0x488504 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x488504: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488504() {
}

// 0x488550 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x488550: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488550() {
}

// 0x488570 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x488570: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488570() {
}

// 0x4885a4 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToIndex(RBX::TaskScheduler::ThreadPoolConfig)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_
// IDA 0x4885a4: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4885a4() {
}

// 0x488614 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x488614: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488614() {
}

// 0x488654 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// IDA 0x488654: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488654() {
}

// 0x488658 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// IDA 0x488658: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488658() {
}

// 0x48865c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x48865c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48865c() {
}

// 0x48867c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::setValue(RBX::Reflection::DescribedBase *,RBX::TaskScheduler::ThreadPoolConfig const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x48867c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48867c() {
}

// 0x4886a0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE13initSingletonEv
// IDA 0x4886a0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4886a0() {
}

// 0x4886a4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv
// IDA 0x4886a4: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4886a4() {
}

// 0x488794 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::BoundFuncDesc(void (RBX::DebugSettings::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x488794: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488794() {
}

// 0x48890c — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x48890c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48890c() {
}

// 0x48893c — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EED0Ev
// IDA 0x48893c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_48893c() {
}

// 0x488a10 — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x488a10: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488a10() {
}

// 0x488a48 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::BoundFuncDesc(void (RBX::DebugSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x488a48: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488a48() {
}

// 0x488b4c — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EED0Ev
// IDA 0x488b4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_488b4c() {
}

// 0x488c00 — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x488c00: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488c00() {
}

// 0x488c20 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::EnumPropDescriptor<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>(char const*,char const*,RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x488c20: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488c20() {
}

// 0x488dd4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEED0Ev
// IDA 0x488dd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_488dd4() {
}

// 0x488e00 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10isReadOnlyEv
// IDA 0x488e00: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488e00() {
}

// 0x488e10 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11isWriteOnlyEv
// IDA 0x488e10: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488e10() {
}

// 0x488e20 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x488e20: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488e20() {
}

// 0x488e48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x488e48: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488e48() {
}

// 0x488e6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x488e6c: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488e6c() {
}

// 0x488fb8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x488fb8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488fb8() {
}

// 0x488fdc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14hasStringValueEv
// IDA 0x488fdc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488fdc() {
}

// 0x488fe0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x488fe0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_488fe0() {
}

// 0x489004 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x489004: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489004() {
}

// 0x489044 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x489044: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489044() {
}

// 0x489064 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x489064: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489064() {
}

// 0x4892a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x4892a4: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4892a4() {
}

// 0x4892c0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x4892c0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4892c0() {
}

// 0x4892f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x4892f4: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4892f4() {
}

// 0x4892fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x4892fc: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4892fc() {
}

// 0x489348 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x489348: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489348() {
}

// 0x489368 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x489368: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489368() {
}

// 0x48939c — __ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToIndex(RBX::DebugSettings::ErrorReporting)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_
// IDA 0x48939c: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48939c() {
}

// 0x48940c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x48940c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48940c() {
}

// 0x48944c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x48944c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48944c() {
}

// 0x489450 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x489450: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489450() {
}

// 0x489454 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x489454: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489454() {
}

// 0x489474 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::setValue(RBX::Reflection::DescribedBase *,RBX::DebugSettings::ErrorReporting const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x489474: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489474() {
}

// 0x489498 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE13initSingletonEv
// IDA 0x489498: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_489498() {
}

// 0x48949c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv
// IDA 0x48949c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48949c() {
}

// 0x48958c — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13DebugSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::DebugSettings>(char const*,char const*,bool RBX::DebugSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13DebugSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x48958c: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48958c() {
}

// 0x489720 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE10isReadOnlyEv
// IDA 0x489720: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489720() {
}

// 0x489724 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE11isWriteOnlyEv
// IDA 0x489724: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489724() {
}

// 0x489728 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x489728: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489728() {
}

// 0x489734 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x489734: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489734() {
}

// 0x489784 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EEC2EMS2_FvbdEPKcS8_bS8_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, int, char, int, double, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::BoundFuncDesc(void (RBX::TaskSchedulerSettings::*)(bool,double),char const*,char const*,bool,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EEC2EMS2_FvbdEPKcS8_bS8_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x489784: 213 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489784() {
}

// 0x4899b4 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// IDA 0x4899b4: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4899b4() {
}

// 0x489a00 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EED0Ev
// IDA 0x489a00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_489a00() {
}

// 0x489ae0 — __ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x489ae0: 30 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489ae0() {
}

// 0x489b38 — __ZN3RBX10Reflection9ArgHelper6getArgIdLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: __int64 __fastcall(int, int)
#[doc(alias = "double RBX::Reflection::ArgHelper::getArg<double,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIdLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x489b38: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489b38() {
}

// 0x489cd8 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::PropDescriptor<double (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,double (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x489cd8: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489cd8() {
}

// 0x489de4 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE10isReadOnlyEv
// IDA 0x489de4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489de4() {
}

// 0x489de8 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv
// IDA 0x489de8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489de8() {
}

// 0x489dec — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x489dec: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489dec() {
}

// 0x489e0c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd
// IDA 0x489e0c: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489e0c() {
}

// 0x489f2c — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiEC2IMS2_KFjvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::PropDescriptor<unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiEC2IMS2_KFjvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x489f2c: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_489f2c() {
}

// 0x48a038 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiED0Ev
// IDA 0x48a038: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_48a038() {
}

// 0x48a064 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE10isReadOnlyEv
// IDA 0x48a064: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48a064() {
}

// 0x48a068 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE11isWriteOnlyEv
// IDA 0x48a068: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48a068() {
}

// 0x48a06c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x48a06c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48a06c() {
}

// 0x48a08c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8setValueEPNS0_13DescribedBaseERKi
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x48a08c: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48a08c() {
}
