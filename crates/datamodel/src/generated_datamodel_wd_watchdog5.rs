// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace (10215) exhausted — gap filler EA-sorted asc distinct not yet in crates/datamodel/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x47e900..0x48d480 | gap filler EA-sorted ascending next 120 after 0x47e8e8 (shard wd_watchdog4)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and ' stripped from alias
// Shard: wd_watchdog5 EA-sorted ascending continuation after wd_watchdog4 (global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x47e900 — __ZN3RBX13TaskScheduler7Arbiter8postStepEPNS0_3JobE
// type: _DWORD __fastcall(RBX::TaskScheduler::Arbiter *__hidden this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::TaskScheduler::Arbiter::postStep(RBX::TaskScheduler::Job *)")]
#[doc(alias = "__ZN3RBX13TaskScheduler7Arbiter8postStepEPNS0_3JobE")]
pub fn stub_47e900() -> ! {
    todo!("0x47e900 RBX::TaskScheduler::Arbiter::postStep(RBX::TaskScheduler::Job *)")
}

// 0x489784 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EEC2EMS2_FvbdEPKcS8_bS8_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, int, char, int, double, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::BoundFuncDesc(void (RBX::TaskSchedulerSettings::*)(bool,double),char const*,char const*,bool,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EEC2EMS2_FvbdEPKcS8_bS8_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_489784() -> ! {
    todo!("0x489784 RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::BoundFuncDesc(void (RBX::TaskSchedulerSettings::*)(bool,double),char const*,char const*,bool,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x4899b4 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_")]
pub fn stub_4899b4() -> ! {
    todo!("0x4899b4 RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x489a00 — __ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EED0Ev")]
pub fn stub_489a00() -> ! {
    todo!("0x489a00 RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::~BoundFuncDesc()")
}

// 0x489ae0 — __ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_489ae0() -> ! {
    todo!("0x489ae0 RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x489b38 — __ZN3RBX10Reflection9ArgHelper6getArgIdLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: __int64 __fastcall(int, int)
#[doc(alias = "double RBX::Reflection::ArgHelper::getArg<double,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgIdLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_489b38() -> ! {
    todo!("0x489b38 double RBX::Reflection::ArgHelper::getArg<double,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x489cd8 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::PropDescriptor<double (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,double (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_489cd8() -> ! {
    todo!("0x489cd8 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::PropDescriptor<double (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,double (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x489de4 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE10isReadOnlyEv")]
pub fn stub_489de4() -> ! {
    todo!("0x489de4 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")
}

// 0x489de8 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv")]
pub fn stub_489de8() -> ! {
    todo!("0x489de8 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")
}

// 0x489dec — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_489dec() -> ! {
    todo!("0x489dec RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x489e0c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd")]
pub fn stub_489e0c() -> ! {
    todo!("0x489e0c RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const")
}

// 0x489f2c — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiEC2IMS2_KFjvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::PropDescriptor<unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiEC2IMS2_KFjvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_489f2c() -> ! {
    todo!("0x489f2c RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::PropDescriptor<unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x48a038 — __ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiED0Ev")]
pub fn stub_48a038() -> ! {
    todo!("0x48a038 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::~PropDescriptor()")
}

// 0x48a064 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE10isReadOnlyEv")]
pub fn stub_48a064() -> ! {
    todo!("0x48a064 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")
}

// 0x48a068 — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE11isWriteOnlyEv")]
pub fn stub_48a068() -> ! {
    todo!("0x48a068 RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")
}

// 0x48a06c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_48a06c() -> ! {
    todo!("0x48a06c RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x48a08c — __ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8setValueEPNS0_13DescribedBaseERKi
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiE7GetImplIMS2_KFjvEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_48a08c() -> ! {
    todo!("0x48a08c RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x48a1ac — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EEC2EMS2_FS7_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EEC2EMS2_FS7_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_48a1ac() -> ! {
    todo!("0x48a1ac RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x48a2b0 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED0Ev")]
pub fn stub_48a2b0() -> ! {
    todo!("0x48a2b0 RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()")
}

// 0x48a364 — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_48a364() -> ! {
    todo!("0x48a364 RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x48a388 — __ZN3RBX10Reflection11Call0HelperINS_13DebugSettingsEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEvES7_E4callEPS2_S9_RNS0_7VariantE
// type: void __fastcall(int, char *, int, _DWORD *)
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::call(RBX::DebugSettings*,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_13DebugSettingsEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEvES7_E4callEPS2_S9_RNS0_7VariantE")]
pub fn stub_48a388() -> ! {
    todo!("0x48a388 RBX::Reflection::Call0Helper<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::call(RBX::DebugSettings*,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),RBX::Reflection::Variant &)")
}

// 0x48a470 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKNS1_10Reflection5TupleEEEEERS3_RKT_
// type: _DWORD *__fastcall(_DWORD *, const shared_count *)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKNS1_10Reflection5TupleEEEEERS3_RKT_")]
pub fn stub_48a470() -> ! {
    todo!("0x48a470 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")
}

// 0x48a4d8 — __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEaSERKS5_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const>::operator=(rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEaSERKS5_")]
pub fn stub_48a4d8() -> ! {
    todo!("0x48a4d8 rbx_core::SharedPtr<RBX::Reflection::Tuple const>::operator=(rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")
}

// 0x48a510 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE14construct_funcEPKcPc
// type: const shared_count *__fastcall(const shared_count *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE14construct_funcEPKcPc")]
pub fn stub_48a510() -> ! {
    todo!("0x48a510 rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::construct_func(char const*,char *)")
}

// 0x48a538 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFlvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::PropDescriptor<long (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,long (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFlvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_48a538() -> ! {
    todo!("0x48a538 RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::PropDescriptor<long (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,long (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x48a644 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<long (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE10isReadOnlyEv")]
pub fn stub_48a644() -> ! {
    todo!("0x48a644 RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<long (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")
}

// 0x48a648 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<long (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE11isWriteOnlyEv")]
pub fn stub_48a648() -> ! {
    todo!("0x48a648 RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<long (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")
}

// 0x48a64c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<long (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_48a64c() -> ! {
    todo!("0x48a64c RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<long (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x48a66c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE8setValueEPNS0_13DescribedBaseERKi
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<long (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFlvEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_48a66c() -> ! {
    todo!("0x48a66c RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<long (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x48a78c — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::PropDescriptor<double (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,double (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_48a78c() -> ! {
    todo!("0x48a78c RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::PropDescriptor<double (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,double (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x48a898 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetImpl<double (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE10isReadOnlyEv")]
pub fn stub_48a898() -> ! {
    todo!("0x48a898 RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetImpl<double (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")
}

// 0x48a89c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetImpl<double (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv")]
pub fn stub_48a89c() -> ! {
    todo!("0x48a89c RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetImpl<double (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")
}

// 0x48a8a0 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetImpl<double (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_48a8a0() -> ! {
    todo!("0x48a8a0 RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetImpl<double (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x48a8c0 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetImpl<double (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd")]
pub fn stub_48a8c0() -> ! {
    todo!("0x48a8c0 RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::GetImpl<double (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const")
}

// 0x48a9e0 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::PropDescriptor<bool (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,bool (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_48a9e0() -> ! {
    todo!("0x48a9e0 RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::PropDescriptor<bool (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,bool (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x48aaec — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetImpl<bool (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE10isReadOnlyEv")]
pub fn stub_48aaec() -> ! {
    todo!("0x48aaec RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetImpl<bool (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")
}

// 0x48aaf0 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetImpl<bool (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv")]
pub fn stub_48aaf0() -> ! {
    todo!("0x48aaf0 RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetImpl<bool (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")
}

// 0x48aaf4 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetImpl<bool (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_48aaf4() -> ! {
    todo!("0x48aaf4 RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetImpl<bool (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x48ab18 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetImpl<bool (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_48ab18() -> ! {
    todo!("0x48ab18 RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::GetImpl<bool (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x48ac38 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::PropDescriptor<int (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,int (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_48ac38() -> ! {
    todo!("0x48ac38 RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::PropDescriptor<int (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,int (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x48ad44 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<int (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE10isReadOnlyEv")]
pub fn stub_48ad44() -> ! {
    todo!("0x48ad44 RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<int (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")
}

// 0x48ad48 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<int (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE11isWriteOnlyEv")]
pub fn stub_48ad48() -> ! {
    todo!("0x48ad48 RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<int (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")
}

// 0x48ad4c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<int (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_48ad4c() -> ! {
    todo!("0x48ad4c RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<int (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x48ad6c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<int (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_48ad6c() -> ! {
    todo!("0x48ad6c RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetImpl<int (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x48ae8c — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfEC2IMS2_KFfvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::PropDescriptor<float (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,float (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfEC2IMS2_KFfvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_48ae8c() -> ! {
    todo!("0x48ae8c RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::PropDescriptor<float (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,float (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x48b0bc — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfED0Ev")]
pub fn stub_48b0bc() -> ! {
    todo!("0x48b0bc RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::~PropDescriptor()")
}

// 0x48b0e8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIfE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIfE10isReadOnlyEv")]
pub fn stub_48b0e8() -> ! {
    todo!("0x48b0e8 RBX::Reflection::TypedPropertyDescriptor<float>::isReadOnly(void)const")
}

// 0x48b0f8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIfE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIfE11isWriteOnlyEv")]
pub fn stub_48b0f8() -> ! {
    todo!("0x48b0f8 RBX::Reflection::TypedPropertyDescriptor<float>::isWriteOnly(void)const")
}

// 0x48b108 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIfE11equalValuesEPKNS0_13DescribedBaseES5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIfE11equalValuesEPKNS0_13DescribedBaseES5_")]
pub fn stub_48b108() -> ! {
    todo!("0x48b108 RBX::Reflection::TypedPropertyDescriptor<float>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x48b140 — __ZN3rbx8any_castIRKfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "float const& rbx::any_cast<float const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_48b140() -> ! {
    todo!("0x48b140 float const& rbx::any_cast<float const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x48b228 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIfEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<float>(float const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIfEERS3_RKT_")]
pub fn stub_48b228() -> ! {
    todo!("0x48b228 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<float>(float const&)")
}

// 0x48b278 — __ZN3rbx14implementation12typed_holderIfE9singletonEv
// type: int(void)
#[doc(alias = "rbx::implementation::typed_holder<float>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIfE9singletonEv")]
pub fn stub_48b278() -> ! {
    todo!("0x48b278 rbx::implementation::typed_holder<float>::singleton(void)")
}

// 0x48b2e8 — __ZN3rbx14implementation12typed_holderIfE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<float>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIfE13destruct_funcEPc")]
pub fn stub_48b2e8() -> ! {
    todo!("0x48b2e8 rbx::implementation::typed_holder<float>::destruct_func(char *)")
}

// 0x48b2f0 — __ZN3RBX10Reflection23TypedPropertyDescriptorIfED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::~TypedPropertyDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorIfED0Ev")]
pub fn stub_48b2f0() -> ! {
    todo!("0x48b2f0 RBX::Reflection::TypedPropertyDescriptor<float>::~TypedPropertyDescriptor()")
}

// 0x48b31c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::GetImpl<float (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE10isReadOnlyEv")]
pub fn stub_48b31c() -> ! {
    todo!("0x48b31c RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::GetImpl<float (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")
}

// 0x48b320 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::GetImpl<float (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE11isWriteOnlyEv")]
pub fn stub_48b320() -> ! {
    todo!("0x48b320 RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::GetImpl<float (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")
}

// 0x48b324 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::GetImpl<float (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_48b324() -> ! {
    todo!("0x48b324 RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::GetImpl<float (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x48b344 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE8setValueEPNS0_13DescribedBaseERKf
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::GetImpl<float (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfE7GetImplIMS2_KFfvEE8setValueEPNS0_13DescribedBaseERKf")]
pub fn stub_48b344() -> ! {
    todo!("0x48b344 RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::GetImpl<float (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x48b464 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsEC2IMS2_KFSsvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::PropDescriptor<std::string (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,std::string (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsEC2IMS2_KFSsvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_48b464() -> ! {
    todo!("0x48b464 RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::PropDescriptor<std::string (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,std::string (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x48b694 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsED0Ev")]
pub fn stub_48b694() -> ! {
    todo!("0x48b694 RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::~PropDescriptor()")
}

// 0x48b6c0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorISsE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<std::string>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorISsE10isReadOnlyEv")]
pub fn stub_48b6c0() -> ! {
    todo!("0x48b6c0 RBX::Reflection::TypedPropertyDescriptor<std::string>::isReadOnly(void)const")
}

// 0x48b6d0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorISsE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<std::string>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorISsE11isWriteOnlyEv")]
pub fn stub_48b6d0() -> ! {
    todo!("0x48b6d0 RBX::Reflection::TypedPropertyDescriptor<std::string>::isWriteOnly(void)const")
}

// 0x48b6e0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorISsE11equalValuesEPKNS0_13DescribedBaseES5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<std::string>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorISsE11equalValuesEPKNS0_13DescribedBaseES5_")]
pub fn stub_48b6e0() -> ! {
    todo!("0x48b6e0 RBX::Reflection::TypedPropertyDescriptor<std::string>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x48b850 — __ZNK3RBX10Reflection23TypedPropertyDescriptorISsE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<std::string>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorISsE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_48b850() -> ! {
    todo!("0x48b850 RBX::Reflection::TypedPropertyDescriptor<std::string>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x48ba40 — __ZN3RBX10Reflection23TypedPropertyDescriptorISsED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<std::string>::~TypedPropertyDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorISsED1Ev")]
pub fn stub_48ba40() -> ! {
    todo!("0x48ba40 RBX::Reflection::TypedPropertyDescriptor<std::string>::~TypedPropertyDescriptor()")
}

// 0x48ba68 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::GetImpl<std::string (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE10isReadOnlyEv")]
pub fn stub_48ba68() -> ! {
    todo!("0x48ba68 RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::GetImpl<std::string (RBX::DebugSettings::*)(void)const>::isReadOnly(void)const")
}

// 0x48ba6c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::GetImpl<std::string (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE11isWriteOnlyEv")]
pub fn stub_48ba6c() -> ! {
    todo!("0x48ba6c RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::GetImpl<std::string (RBX::DebugSettings::*)(void)const>::isWriteOnly(void)const")
}

// 0x48ba70 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::GetImpl<std::string (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_48ba70() -> ! {
    todo!("0x48ba70 RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::GetImpl<std::string (RBX::DebugSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x48ba98 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::GetImpl<std::string (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsE7GetImplIMS2_KFSsvEE8setValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_48ba98() -> ! {
    todo!("0x48ba98 RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::GetImpl<std::string (RBX::DebugSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x48bbbc — __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::resize(unsigned long,RBX::Time::SampleMethod)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE6resizeEmS2_")]
pub fn stub_48bbbc() -> ! {
    todo!("0x48bbbc std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::resize(unsigned long,RBX::Time::SampleMethod)")
}

// 0x48bbf4 — __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::push_back(RBX::Time::SampleMethod const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE9push_backERKS2_")]
pub fn stub_48bbf4() -> ! {
    todo!("0x48bbf4 std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::push_back(RBX::Time::SampleMethod const&)")
}

// 0x48bc20 — __ZNSt3mapIPKN3RBX4NameENS0_4Time12SampleMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::Time::SampleMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_4Time12SampleMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_48bc20() -> ! {
    todo!("0x48bc20 std::map<RBX::Name const*,RBX::Time::SampleMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::operator[](RBX::Name const* const&)")
}

// 0x48bc78 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_48bc78() -> ! {
    todo!("0x48bc78 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")
}

// 0x48bd2c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_48bd2c() -> ! {
    todo!("0x48bd2c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")
}

// 0x48bd84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_48bd84() -> ! {
    todo!("0x48bd84 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")
}

// 0x48bdf0 — __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,RBX::Time::SampleMethod const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_48bdf0() -> ! {
    todo!("0x48bdf0 std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,RBX::Time::SampleMethod const&)")
}

// 0x48bed4 — __ZNSt12_Vector_baseIN3RBX4Time12SampleMethodESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX4Time12SampleMethodESaIS2_EE11_M_allocateEm")]
pub fn stub_48bed4() -> ! {
    todo!("0x48bed4 std::_Vector_base<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_allocate(unsigned long)")
}

// 0x48beec — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4Time12SampleMethodES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::Time::SampleMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Time::SampleMethod *,RBX::Time::SampleMethod *>(RBX::Time::SampleMethod *,RBX::Time::SampleMethod *,RBX::Time::SampleMethod *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4Time12SampleMethodES6_EET0_T_S8_S7_")]
pub fn stub_48beec() -> ! {
    todo!("0x48beec RBX::Time::SampleMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Time::SampleMethod *,RBX::Time::SampleMethod *>(RBX::Time::SampleMethod *,RBX::Time::SampleMethod *,RBX::Time::SampleMethod *)")
}

// 0x48bf2c — __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,unsigned long,RBX::Time::SampleMethod const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_48bf2c() -> ! {
    todo!("0x48bf2c std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,unsigned long,RBX::Time::SampleMethod const&)")
}

// 0x48c0c0 — __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::resize(unsigned long,RBX::EThrottle::EThrottleType)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE6resizeEmS2_")]
pub fn stub_48c0c0() -> ! {
    todo!("0x48c0c0 std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::resize(unsigned long,RBX::EThrottle::EThrottleType)")
}

// 0x48c0f4 — __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::push_back(RBX::EThrottle::EThrottleType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE9push_backERKS2_")]
pub fn stub_48c0f4() -> ! {
    todo!("0x48c0f4 std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::push_back(RBX::EThrottle::EThrottleType const&)")
}

// 0x48c11c — __ZNSt3mapIPKN3RBX4NameENS0_9EThrottle13EThrottleTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::EThrottle::EThrottleType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_9EThrottle13EThrottleTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_48c11c() -> ! {
    todo!("0x48c11c std::map<RBX::Name const*,RBX::EThrottle::EThrottleType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::operator[](RBX::Name const* const&)")
}

// 0x48c174 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_48c174() -> ! {
    todo!("0x48c174 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")
}

// 0x48c228 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_48c228() -> ! {
    todo!("0x48c228 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")
}

// 0x48c280 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_48c280() -> ! {
    todo!("0x48c280 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")
}

// 0x48c2e8 — __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,RBX::EThrottle::EThrottleType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_48c2e8() -> ! {
    todo!("0x48c2e8 std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,RBX::EThrottle::EThrottleType const&)")
}

// 0x48c3cc — __ZNSt12_Vector_baseIN3RBX9EThrottle13EThrottleTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9EThrottle13EThrottleTypeESaIS2_EE11_M_allocateEm")]
pub fn stub_48c3cc() -> ! {
    todo!("0x48c3cc std::_Vector_base<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_allocate(unsigned long)")
}

// 0x48c3e4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9EThrottle13EThrottleTypeES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::EThrottle::EThrottleType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *>(RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9EThrottle13EThrottleTypeES6_EET0_T_S8_S7_")]
pub fn stub_48c3e4() -> ! {
    todo!("0x48c3e4 RBX::EThrottle::EThrottleType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *>(RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *)")
}

// 0x48c420 — __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,unsigned long,RBX::EThrottle::EThrottleType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_48c420() -> ! {
    todo!("0x48c420 std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,unsigned long,RBX::EThrottle::EThrottleType const&)")
}

// 0x48c5b0 — __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::resize(unsigned long,RBX::DebugSettings::ErrorReporting)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE6resizeEmS2_")]
pub fn stub_48c5b0() -> ! {
    todo!("0x48c5b0 std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::resize(unsigned long,RBX::DebugSettings::ErrorReporting)")
}

// 0x48c5e4 — __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::push_back(RBX::DebugSettings::ErrorReporting const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE9push_backERKS2_")]
pub fn stub_48c5e4() -> ! {
    todo!("0x48c5e4 std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::push_back(RBX::DebugSettings::ErrorReporting const&)")
}

// 0x48c60c — __ZNSt3mapIPKN3RBX4NameENS0_13DebugSettings14ErrorReportingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::DebugSettings::ErrorReporting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_13DebugSettings14ErrorReportingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_48c60c() -> ! {
    todo!("0x48c60c std::map<RBX::Name const*,RBX::DebugSettings::ErrorReporting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::operator[](RBX::Name const* const&)")
}

// 0x48c664 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_48c664() -> ! {
    todo!("0x48c664 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")
}

// 0x48c718 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_48c718() -> ! {
    todo!("0x48c718 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")
}

// 0x48c770 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_48c770() -> ! {
    todo!("0x48c770 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")
}

// 0x48c7d8 — __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,RBX::DebugSettings::ErrorReporting const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_48c7d8() -> ! {
    todo!("0x48c7d8 std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,RBX::DebugSettings::ErrorReporting const&)")
}

// 0x48c8bc — __ZNSt12_Vector_baseIN3RBX13DebugSettings14ErrorReportingESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX13DebugSettings14ErrorReportingESaIS2_EE11_M_allocateEm")]
pub fn stub_48c8bc() -> ! {
    todo!("0x48c8bc std::_Vector_base<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_allocate(unsigned long)")
}

// 0x48c8d4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DebugSettings14ErrorReportingES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::DebugSettings::ErrorReporting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *>(RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DebugSettings14ErrorReportingES6_EET0_T_S8_S7_")]
pub fn stub_48c8d4() -> ! {
    todo!("0x48c8d4 RBX::DebugSettings::ErrorReporting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *>(RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *)")
}

// 0x48c910 — __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,unsigned long,RBX::DebugSettings::ErrorReporting const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_48c910() -> ! {
    todo!("0x48c910 std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,unsigned long,RBX::DebugSettings::ErrorReporting const&)")
}

// 0x48caa0 — __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE6resizeEmS3_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::resize(unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE6resizeEmS3_")]
pub fn stub_48caa0() -> ! {
    todo!("0x48caa0 std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::resize(unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod)")
}

// 0x48cad4 — __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE9push_backERKS3_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::push_back(RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE9push_backERKS3_")]
pub fn stub_48cad4() -> ! {
    todo!("0x48cad4 std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::push_back(RBX::TaskScheduler::Job::SleepAdjustMethod const&)")
}

// 0x48cafc — __ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler3Job17SleepAdjustMethodESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::Job::SleepAdjustMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler3Job17SleepAdjustMethodESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_")]
pub fn stub_48cafc() -> ! {
    todo!("0x48cafc std::map<RBX::Name const*,RBX::TaskScheduler::Job::SleepAdjustMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::operator[](RBX::Name const* const&)")
}

// 0x48cb54 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")]
pub fn stub_48cb54() -> ! {
    todo!("0x48cb54 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")
}

// 0x48cc08 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")]
pub fn stub_48cc08() -> ! {
    todo!("0x48cc08 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")
}

// 0x48cc60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler3Job17SleepAdjustMethodEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_")]
pub fn stub_48cc60() -> ! {
    todo!("0x48cc60 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")
}

// 0x48ccc8 — __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
pub fn stub_48ccc8() -> ! {
    todo!("0x48ccc8 std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")
}

// 0x48cdac — __ZNSt12_Vector_baseIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE11_M_allocateEm")]
pub fn stub_48cdac() -> ! {
    todo!("0x48cdac std::_Vector_base<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_allocate(unsigned long)")
}

// 0x48cdc4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler3Job17SleepAdjustMethodES7_EET0_T_S9_S8_
// type: int(void)
#[doc(alias = "RBX::TaskScheduler::Job::SleepAdjustMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *>(RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler3Job17SleepAdjustMethodES7_EET0_T_S9_S8_")]
pub fn stub_48cdc4() -> ! {
    todo!("0x48cdc4 RBX::TaskScheduler::Job::SleepAdjustMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *>(RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *)")
}

// 0x48ce00 — __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")]
pub fn stub_48ce00() -> ! {
    todo!("0x48ce00 std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")
}

// 0x48cf90 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::resize(unsigned long,RBX::TaskScheduler::PriorityMethod)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE6resizeEmS2_")]
pub fn stub_48cf90() -> ! {
    todo!("0x48cf90 std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::resize(unsigned long,RBX::TaskScheduler::PriorityMethod)")
}

// 0x48cfc4 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::push_back(RBX::TaskScheduler::PriorityMethod const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE9push_backERKS2_")]
pub fn stub_48cfc4() -> ! {
    todo!("0x48cfc4 std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::push_back(RBX::TaskScheduler::PriorityMethod const&)")
}

// 0x48cfec — __ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler14PriorityMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::PriorityMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler14PriorityMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_48cfec() -> ! {
    todo!("0x48cfec std::map<RBX::Name const*,RBX::TaskScheduler::PriorityMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::operator[](RBX::Name const* const&)")
}

// 0x48d044 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_48d044() -> ! {
    todo!("0x48d044 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")
}

// 0x48d0f8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_48d0f8() -> ! {
    todo!("0x48d0f8 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")
}

// 0x48d150 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13TaskScheduler14PriorityMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_48d150() -> ! {
    todo!("0x48d150 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")
}

// 0x48d1b8 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,RBX::TaskScheduler::PriorityMethod const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_48d1b8() -> ! {
    todo!("0x48d1b8 std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,RBX::TaskScheduler::PriorityMethod const&)")
}

// 0x48d29c — __ZNSt12_Vector_baseIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE11_M_allocateEm")]
pub fn stub_48d29c() -> ! {
    todo!("0x48d29c std::_Vector_base<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_allocate(unsigned long)")
}

// 0x48d2b4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler14PriorityMethodES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::TaskScheduler::PriorityMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *>(RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler14PriorityMethodES6_EET0_T_S8_S7_")]
pub fn stub_48d2b4() -> ! {
    todo!("0x48d2b4 RBX::TaskScheduler::PriorityMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *>(RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *)")
}

// 0x48d2f0 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,unsigned long,RBX::TaskScheduler::PriorityMethod const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_48d2f0() -> ! {
    todo!("0x48d2f0 std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,unsigned long,RBX::TaskScheduler::PriorityMethod const&)")
}

// 0x48d480 — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::resize(unsigned long,RBX::TaskScheduler::ThreadPoolConfig)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE6resizeEmS2_")]
pub fn stub_48d480() -> ! {
    todo!("0x48d480 std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::resize(unsigned long,RBX::TaskScheduler::ThreadPoolConfig)")
}
