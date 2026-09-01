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
pub fn stub_4853b0() -> ! {
    todo!("0x4853b0 RBX::Reflection::TypedPropertyDescriptor<double>::~TypedPropertyDescriptor()")
}

// 0x48769c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE12getEnumValueEPKNS0_13DescribedBaseE
pub fn stub_48769c() -> ! {
    todo!("0x48769c RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4876a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE12setEnumValueEPNS0_13DescribedBaseEi
pub fn stub_4876a4() -> ! {
    todo!("0x4876a4 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x4876f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11getEnumItemEPKNS0_13DescribedBaseE
pub fn stub_4876f0() -> ! {
    todo!("0x4876f0 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x487710 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
pub fn stub_487710() -> ! {
    todo!("0x487710 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x487744 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToIndex(RBX::TaskScheduler::PriorityMethod)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToIndexES3_
pub fn stub_487744() -> ! {
    todo!("0x487744 RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToIndex(RBX::TaskScheduler::PriorityMethod)const")
}

// 0x4877b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11setIntValueEPNS0_13DescribedBaseEi
pub fn stub_4877b4() -> ! {
    todo!("0x4877b4 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x4877f4 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
pub fn stub_4877f4() -> ! {
    todo!("0x4877f4 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::isReadOnly(void)const")
}

// 0x4877f8 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
pub fn stub_4877f8() -> ! {
    todo!("0x4877f8 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::isWriteOnly(void)const")
}

// 0x4877fc — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_4877fc() -> ! {
    todo!("0x4877fc RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x48781c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::setValue(RBX::Reflection::DescribedBase *,RBX::TaskScheduler::PriorityMethod const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
pub fn stub_48781c() -> ! {
    todo!("0x48781c RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::GetSetImpl<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>::setValue(RBX::Reflection::DescribedBase *,RBX::TaskScheduler::PriorityMethod const&)const")
}

// 0x487840 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE13initSingletonEv
pub fn stub_487840() -> ! {
    todo!("0x487840 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod> const>::initSingleton(void)")
}

// 0x487844 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE14doGetSingletonEv
pub fn stub_487844() -> ! {
    todo!("0x487844 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod> const>::doGetSingleton(void)")
}

// 0x487934 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EEC2EMS2_FvdiEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::BoundFuncDesc(void (RBX::TaskSchedulerSettings::*)(double,int),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EEC2EMS2_FvdiEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_487934() -> ! {
    todo!("0x487934 RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::BoundFuncDesc(void (RBX::TaskSchedulerSettings::*)(double,int),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x487afc — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
pub fn stub_487afc() -> ! {
    todo!("0x487afc RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x487b48 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EED0Ev
pub fn stub_487b48() -> ! {
    todo!("0x487b48 RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::~BoundFuncDesc()")
}

// 0x487c28 — __ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_487c28() -> ! {
    todo!("0x487c28 RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x487c88 — __ZN3RBX10Reflection9ArgHelper6getArgIdLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: __int64 __fastcall(_DWORD, _DWORD)
#[doc(alias = "double RBX::Reflection::ArgHelper::getArg<double,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIdLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_487c88() -> ! {
    todo!("0x487c88 double RBX::Reflection::ArgHelper::getArg<double,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x487e28 — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::EnumPropDescriptor<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>(char const*,char const*,RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_487e28() -> ! {
    todo!("0x487e28 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::EnumPropDescriptor<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>(char const*,char const*,RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x487fdc — __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEED0Ev
pub fn stub_487fdc() -> ! {
    todo!("0x487fdc RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::~EnumPropDescriptor()")
}

// 0x488008 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10isReadOnlyEv
pub fn stub_488008() -> ! {
    todo!("0x488008 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::isReadOnly(void)const")
}

// 0x488018 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11isWriteOnlyEv
pub fn stub_488018() -> ! {
    todo!("0x488018 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::isWriteOnly(void)const")
}

// 0x488028 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11equalValuesEPKNS0_13DescribedBaseES8_
pub fn stub_488028() -> ! {
    todo!("0x488028 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x488050 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
pub fn stub_488050() -> ! {
    todo!("0x488050 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x488074 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_488074() -> ! {
    todo!("0x488074 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x4881c0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE9copyValueEPKNS0_13DescribedBaseEPS6_
pub fn stub_4881c0() -> ! {
    todo!("0x4881c0 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x4881e4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14hasStringValueEv
pub fn stub_4881e4() -> ! {
    todo!("0x4881e4 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::hasStringValue(void)const")
}

// 0x4881e8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14getStringValueEPKNS0_13DescribedBaseE
pub fn stub_4881e8() -> ! {
    todo!("0x4881e8 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x48820c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_48820c() -> ! {
    todo!("0x48820c RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x48824c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_48824c() -> ! {
    todo!("0x48824c RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x48826c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_48826c() -> ! {
    todo!("0x48826c RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x4884ac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE13getIndexValueEPKNS0_13DescribedBaseE
pub fn stub_4884ac() -> ! {
    todo!("0x4884ac RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4884c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE13setIndexValueEPNS0_13DescribedBaseEm
pub fn stub_4884c8() -> ! {
    todo!("0x4884c8 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x4884fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE12getEnumValueEPKNS0_13DescribedBaseE
pub fn stub_4884fc() -> ! {
    todo!("0x4884fc RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x488504 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE12setEnumValueEPNS0_13DescribedBaseEi
pub fn stub_488504() -> ! {
    todo!("0x488504 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x488550 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11getEnumItemEPKNS0_13DescribedBaseE
pub fn stub_488550() -> ! {
    todo!("0x488550 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x488570 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
pub fn stub_488570() -> ! {
    todo!("0x488570 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x4885a4 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToIndex(RBX::TaskScheduler::ThreadPoolConfig)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_
pub fn stub_4885a4() -> ! {
    todo!("0x4885a4 RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToIndex(RBX::TaskScheduler::ThreadPoolConfig)const")
}

// 0x488614 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11setIntValueEPNS0_13DescribedBaseEi
pub fn stub_488614() -> ! {
    todo!("0x488614 RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x488654 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
pub fn stub_488654() -> ! {
    todo!("0x488654 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::isReadOnly(void)const")
}

// 0x488658 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
pub fn stub_488658() -> ! {
    todo!("0x488658 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::isWriteOnly(void)const")
}

// 0x48865c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_48865c() -> ! {
    todo!("0x48865c RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x48867c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::setValue(RBX::Reflection::DescribedBase *,RBX::TaskScheduler::ThreadPoolConfig const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
pub fn stub_48867c() -> ! {
    todo!("0x48867c RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::setValue(RBX::Reflection::DescribedBase *,RBX::TaskScheduler::ThreadPoolConfig const&)const")
}

// 0x4886a0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE13initSingletonEv
pub fn stub_4886a0() -> ! {
    todo!("0x4886a0 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::initSingleton(void)")
}

// 0x4886a4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv
pub fn stub_4886a4() -> ! {
    todo!("0x4886a4 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::doGetSingleton(void)")
}

// 0x488794 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::BoundFuncDesc(void (RBX::DebugSettings::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_488794() -> ! {
    todo!("0x488794 RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::BoundFuncDesc(void (RBX::DebugSettings::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x48890c — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_48890c() -> ! {
    todo!("0x48890c RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x48893c — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EED0Ev
pub fn stub_48893c() -> ! {
    todo!("0x48893c RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::~BoundFuncDesc()")
}

// 0x488a10 — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_488a10() -> ! {
    todo!("0x488a10 RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x488a48 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::BoundFuncDesc(void (RBX::DebugSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_488a48() -> ! {
    todo!("0x488a48 RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::BoundFuncDesc(void (RBX::DebugSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x488b4c — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EED0Ev
pub fn stub_488b4c() -> ! {
    todo!("0x488b4c RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::~BoundFuncDesc()")
}

// 0x488c00 — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_488c00() -> ! {
    todo!("0x488c00 RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x488c20 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::EnumPropDescriptor<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>(char const*,char const*,RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_488c20() -> ! {
    todo!("0x488c20 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::EnumPropDescriptor<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>(char const*,char const*,RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x488dd4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEED0Ev
pub fn stub_488dd4() -> ! {
    todo!("0x488dd4 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::~EnumPropDescriptor()")
}

// 0x488e00 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10isReadOnlyEv
pub fn stub_488e00() -> ! {
    todo!("0x488e00 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::isReadOnly(void)const")
}

// 0x488e10 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11isWriteOnlyEv
pub fn stub_488e10() -> ! {
    todo!("0x488e10 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::isWriteOnly(void)const")
}

// 0x488e20 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11equalValuesEPKNS0_13DescribedBaseES7_
pub fn stub_488e20() -> ! {
    todo!("0x488e20 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x488e48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
pub fn stub_488e48() -> ! {
    todo!("0x488e48 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x488e6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_488e6c() -> ! {
    todo!("0x488e6c RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x488fb8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_488fb8() -> ! {
    todo!("0x488fb8 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x488fdc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14hasStringValueEv
pub fn stub_488fdc() -> ! {
    todo!("0x488fdc RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::hasStringValue(void)const")
}

// 0x488fe0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14getStringValueEPKNS0_13DescribedBaseE
pub fn stub_488fe0() -> ! {
    todo!("0x488fe0 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x489004 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_489004() -> ! {
    todo!("0x489004 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x489044 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_489044() -> ! {
    todo!("0x489044 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x489064 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_489064() -> ! {
    todo!("0x489064 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x4892a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13getIndexValueEPKNS0_13DescribedBaseE
pub fn stub_4892a4() -> ! {
    todo!("0x4892a4 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4892c0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE13setIndexValueEPNS0_13DescribedBaseEm
pub fn stub_4892c0() -> ! {
    todo!("0x4892c0 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x4892f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12getEnumValueEPKNS0_13DescribedBaseE
pub fn stub_4892f4() -> ! {
    todo!("0x4892f4 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4892fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE12setEnumValueEPNS0_13DescribedBaseEi
pub fn stub_4892fc() -> ! {
    todo!("0x4892fc RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x489348 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11getEnumItemEPKNS0_13DescribedBaseE
pub fn stub_489348() -> ! {
    todo!("0x489348 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x489368 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
pub fn stub_489368() -> ! {
    todo!("0x489368 RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x48939c — __ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToIndex(RBX::DebugSettings::ErrorReporting)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_
pub fn stub_48939c() -> ! {
    todo!("0x48939c RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToIndex(RBX::DebugSettings::ErrorReporting)const")
}

// 0x48940c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11setIntValueEPNS0_13DescribedBaseEi
pub fn stub_48940c() -> ! {
    todo!("0x48940c RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x48944c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_48944c() -> ! {
    todo!("0x48944c RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::isReadOnly(void)const")
}

// 0x489450 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_489450() -> ! {
    todo!("0x489450 RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::isWriteOnly(void)const")
}

// 0x489454 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_489454() -> ! {
    todo!("0x489454 RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x489474 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::setValue(RBX::Reflection::DescribedBase *,RBX::DebugSettings::ErrorReporting const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_489474() -> ! {
    todo!("0x489474 RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::GetSetImpl<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>::setValue(RBX::Reflection::DescribedBase *,RBX::DebugSettings::ErrorReporting const&)const")
}

// 0x489498 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE13initSingletonEv
pub fn stub_489498() -> ! {
    todo!("0x489498 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting> const>::initSingleton(void)")
}

// 0x48949c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv
pub fn stub_48949c() -> ! {
    todo!("0x48949c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting> const>::doGetSingleton(void)")
}

// 0x48958c — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13DebugSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::DebugSettings>(char const*,char const*,bool RBX::DebugSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13DebugSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_48958c() -> ! {
    todo!("0x48958c RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::DebugSettings>(char const*,char const*,bool RBX::DebugSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x489720 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE10isReadOnlyEv
pub fn stub_489720() -> ! {
    todo!("0x489720 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::isReadOnly(void)const")
}

// 0x489724 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE11isWriteOnlyEv
pub fn stub_489724() -> ! {
    todo!("0x489724 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::isWriteOnly(void)const")
}

// 0x489728 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_489728() -> ! {
    todo!("0x489728 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x489734 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13DebugSettingsEE8setValueEPNS0_13DescribedBaseERKb
pub fn stub_489734() -> ! {
    todo!("0x489734 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::DebugSettings>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x489784 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EEC2EMS2_FvbdEPKcS8_bS8_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, int, char, int, double, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::BoundFuncDesc(void (RBX::TaskSchedulerSettings::*)(bool,double),char const*,char const*,bool,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EEC2EMS2_FvbdEPKcS8_bS8_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_489784() -> ! {
    todo!("0x489784 RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::BoundFuncDesc(void (RBX::TaskSchedulerSettings::*)(bool,double),char const*,char const*,bool,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x4899b4 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
pub fn stub_4899b4() -> ! {
    todo!("0x4899b4 RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x489a00 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EED0Ev
pub fn stub_489a00() -> ! {
    todo!("0x489a00 RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::~BoundFuncDesc()")
}

// 0x489ae0 — __ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_489ae0() -> ! {
    todo!("0x489ae0 RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x489b38 — __ZN3RBX10Reflection9ArgHelper6getArgIdLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: __int64 __fastcall(int, int)
#[doc(alias = "double RBX::Reflection::ArgHelper::getArg<double,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIdLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_489b38() -> ! {
    todo!("0x489b38 double RBX::Reflection::ArgHelper::getArg<double,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x489cd8 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::PropDescriptor<double (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,double (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_489cd8() -> ! {
    todo!("0x489cd8 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::PropDescriptor<double (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,double (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x489de4 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE10isReadOnlyEv
pub fn stub_489de4() -> ! {
    todo!("0x489de4 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")
}

// 0x489de8 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv
pub fn stub_489de8() -> ! {
    todo!("0x489de8 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")
}

// 0x489dec — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_489dec() -> ! {
    todo!("0x489dec RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x489e0c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd
pub fn stub_489e0c() -> ! {
    todo!("0x489e0c RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const")
}

// 0x489f2c — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiEC2IMS2_KFjvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::PropDescriptor<unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiEC2IMS2_KFjvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_489f2c() -> ! {
    todo!("0x489f2c RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::PropDescriptor<unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x48a038 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiED0Ev
pub fn stub_48a038() -> ! {
    todo!("0x48a038 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::~PropDescriptor()")
}

// 0x48a064 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE10isReadOnlyEv
pub fn stub_48a064() -> ! {
    todo!("0x48a064 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")
}

// 0x48a068 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE11isWriteOnlyEv
pub fn stub_48a068() -> ! {
    todo!("0x48a068 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")
}

// 0x48a06c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_48a06c() -> ! {
    todo!("0x48a06c RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x48a08c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8setValueEPNS0_13DescribedBaseERKi
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8setValueEPNS0_13DescribedBaseERKi
pub fn stub_48a08c() -> ! {
    todo!("0x48a08c RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}
