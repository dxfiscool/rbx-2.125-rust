//! rendering shard 337 — 100 stubs 0x5c3c40..0x5c9188 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 36660->36760 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 36660 before -> 36760 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x5c3c40 (lowest remaining 0x5c3c40..0x5c9188, next lowest 0x5c9290 if exists)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5c3c40 — __ZN3RBX10Reflection14PropDescriptorINS_8LightingEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8LightingEbED0Ev
pub fn stub_5c3c40() -> ! {
    todo!("0x5c3c40 RBX::Reflection::PropDescriptor<RBX::Lighting,bool>::~PropDescriptor()")
}

// 0x5c3c6c — __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,bool>::GetSetImpl<bool (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
pub fn stub_5c3c6c() -> ! {
    todo!("0x5c3c6c RBX::Reflection::PropDescriptor<RBX::Lighting,bool>::GetSetImpl<bool (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(bool)>::isReadOnly(void)const")
}

// 0x5c3c70 — __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,bool>::GetSetImpl<bool (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
pub fn stub_5c3c70() -> ! {
    todo!("0x5c3c70 RBX::Reflection::PropDescriptor<RBX::Lighting,bool>::GetSetImpl<bool (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(bool)>::isWriteOnly(void)const")
}

// 0x5c3c74 — __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,bool>::GetSetImpl<bool (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_5c3c74() -> ! {
    todo!("0x5c3c74 RBX::Reflection::PropDescriptor<RBX::Lighting,bool>::GetSetImpl<bool (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x5c3c98 — __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,bool>::GetSetImpl<bool (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
pub fn stub_5c3c98() -> ! {
    todo!("0x5c3c98 RBX::Reflection::PropDescriptor<RBX::Lighting,bool>::GetSetImpl<bool (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x5c3e68 — __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFvdELi1EEC2EMS2_FvdEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Lighting,void ()(double),1>::BoundFuncDesc(void (RBX::Lighting::*)(double),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFvdELi1EEC2EMS2_FvdEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_5c3e68() -> ! {
    todo!("0x5c3e68 RBX::Reflection::BoundFuncDesc<RBX::Lighting,void ()(double),1>::BoundFuncDesc(void (RBX::Lighting::*)(double),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x5c3fe0 — __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFvdELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Lighting,void ()(double),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFvdELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_5c3fe0() -> ! {
    todo!("0x5c3fe0 RBX::Reflection::BoundFuncDesc<RBX::Lighting,void ()(double),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x5c4010 — __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFvdELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Lighting,void ()(double),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFvdELi1EED0Ev
pub fn stub_5c4010() -> ! {
    todo!("0x5c4010 RBX::Reflection::BoundFuncDesc<RBX::Lighting,void ()(double),1>::~BoundFuncDesc()")
}

// 0x5c40e4 — __ZNK3RBX10Reflection13BoundFuncDescINS_8LightingEFvdELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Lighting,void ()(double),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_8LightingEFvdELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_5c40e4() -> ! {
    todo!("0x5c40e4 RBX::Reflection::BoundFuncDesc<RBX::Lighting,void ()(double),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x5c4120 — __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFdvELi0EEC2EMS2_FdvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Lighting,double ()(void),0>::BoundFuncDesc(double (RBX::Lighting::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFdvELi0EEC2EMS2_FdvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_5c4120() -> ! {
    todo!("0x5c4120 RBX::Reflection::BoundFuncDesc<RBX::Lighting,double ()(void),0>::BoundFuncDesc(double (RBX::Lighting::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x5c4224 — __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFdvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Lighting,double ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFdvELi0EED0Ev
pub fn stub_5c4224() -> ! {
    todo!("0x5c4224 RBX::Reflection::BoundFuncDesc<RBX::Lighting,double ()(void),0>::~BoundFuncDesc()")
}

// 0x5c42d8 — __ZNK3RBX10Reflection13BoundFuncDescINS_8LightingEFdvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Lighting,double ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_8LightingEFdvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_5c42d8() -> ! {
    todo!("0x5c42d8 RBX::Reflection::BoundFuncDesc<RBX::Lighting,double ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x5c42fc — __ZN3RBX10Reflection11Call0HelperINS_8LightingEMS2_FdvEdE4callEPS2_S4_RNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Lighting,double (RBX::Lighting::*)(void),double>::call(RBX::Lighting*,double (RBX::Lighting::*)(void),RBX::Reflection::Variant &)")]
// was: __ZN3RBX10Reflection11Call0HelperINS_8LightingEMS2_FdvEdE4callEPS2_S4_RNS0_7VariantE
pub fn stub_5c42fc() -> ! {
    todo!("0x5c42fc RBX::Reflection::Call0Helper<RBX::Lighting,double (RBX::Lighting::*)(void),double>::call(RBX::Lighting*,double (RBX::Lighting::*)(void),RBX::Reflection::Variant &)")
}

// 0x5c4548 — __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFfvELi0EEC2EMS2_FfvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Lighting,float ()(void),0>::BoundFuncDesc(float (RBX::Lighting::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFfvELi0EEC2EMS2_FfvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_5c4548() -> ! {
    todo!("0x5c4548 RBX::Reflection::BoundFuncDesc<RBX::Lighting,float ()(void),0>::BoundFuncDesc(float (RBX::Lighting::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x5c464c — __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFfvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Lighting,float ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFfvELi0EED0Ev
pub fn stub_5c464c() -> ! {
    todo!("0x5c464c RBX::Reflection::BoundFuncDesc<RBX::Lighting,float ()(void),0>::~BoundFuncDesc()")
}

// 0x5c4700 — __ZNK3RBX10Reflection13BoundFuncDescINS_8LightingEFfvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Lighting,float ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_8LightingEFfvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_5c4700() -> ! {
    todo!("0x5c4700 RBX::Reflection::BoundFuncDesc<RBX::Lighting,float ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x5c4724 — __ZN3RBX10Reflection11Call0HelperINS_8LightingEMS2_FfvEfE4callEPS2_S4_RNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Lighting,float (RBX::Lighting::*)(void),float>::call(RBX::Lighting*,float (RBX::Lighting::*)(void),RBX::Reflection::Variant &)")]
// was: __ZN3RBX10Reflection11Call0HelperINS_8LightingEMS2_FfvEfE4callEPS2_S4_RNS0_7VariantE
pub fn stub_5c4724() -> ! {
    todo!("0x5c4724 RBX::Reflection::Call0Helper<RBX::Lighting,float (RBX::Lighting::*)(void),float>::call(RBX::Lighting*,float (RBX::Lighting::*)(void),RBX::Reflection::Variant &)")
}

// 0x5c4754 — __ZN3RBX10Reflection14PropDescriptorINS_8LightingEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,float>::PropDescriptor<float (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(float)>(char const*,char const*,float (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8LightingEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_5c4754() -> ! {
    todo!("0x5c4754 RBX::Reflection::PropDescriptor<RBX::Lighting,float>::PropDescriptor<float (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(float)>(char const*,char const*,float (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x5c4868 — __ZN3RBX10Reflection14PropDescriptorINS_8LightingEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8LightingEfED0Ev
pub fn stub_5c4868() -> ! {
    todo!("0x5c4868 RBX::Reflection::PropDescriptor<RBX::Lighting,float>::~PropDescriptor()")
}

// 0x5c4894 — __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,float>::GetSetImpl<float (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(float)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
pub fn stub_5c4894() -> ! {
    todo!("0x5c4894 RBX::Reflection::PropDescriptor<RBX::Lighting,float>::GetSetImpl<float (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(float)>::isReadOnly(void)const")
}

// 0x5c4898 — __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,float>::GetSetImpl<float (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(float)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
pub fn stub_5c4898() -> ! {
    todo!("0x5c4898 RBX::Reflection::PropDescriptor<RBX::Lighting,float>::GetSetImpl<float (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(float)>::isWriteOnly(void)const")
}

// 0x5c489c — __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,float>::GetSetImpl<float (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_5c489c() -> ! {
    todo!("0x5c489c RBX::Reflection::PropDescriptor<RBX::Lighting,float>::GetSetImpl<float (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x5c48bc — __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,float>::GetSetImpl<float (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8LightingEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
pub fn stub_5c48bc() -> ! {
    todo!("0x5c48bc RBX::Reflection::PropDescriptor<RBX::Lighting,float>::GetSetImpl<float (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x5c48e0 — __ZN3RBX10Reflection14PropDescriptorINS_8LightingESsEC2IMS2_KFSsvEMS2_FvRKSsEEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,std::string>::PropDescriptor<std::string (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(std::string const&)>(char const*,char const*,std::string (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(std::string const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8LightingESsEC2IMS2_KFSsvEMS2_FvRKSsEEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_5c48e0() -> ! {
    todo!("0x5c48e0 RBX::Reflection::PropDescriptor<RBX::Lighting,std::string>::PropDescriptor<std::string (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(std::string const&)>(char const*,char const*,std::string (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(std::string const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x5c49f4 — __ZN3RBX10Reflection14PropDescriptorINS_8LightingESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8LightingESsED0Ev
pub fn stub_5c49f4() -> ! {
    todo!("0x5c49f4 RBX::Reflection::PropDescriptor<RBX::Lighting,std::string>::~PropDescriptor()")
}

// 0x5c4a20 — __ZNK3RBX10Reflection14PropDescriptorINS_8LightingESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,std::string>::GetSetImpl<std::string (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(std::string const&)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8LightingESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE10isReadOnlyEv
pub fn stub_5c4a20() -> ! {
    todo!("0x5c4a20 RBX::Reflection::PropDescriptor<RBX::Lighting,std::string>::GetSetImpl<std::string (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(std::string const&)>::isReadOnly(void)const")
}

// 0x5c4a24 — __ZNK3RBX10Reflection14PropDescriptorINS_8LightingESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,std::string>::GetSetImpl<std::string (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(std::string const&)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8LightingESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE11isWriteOnlyEv
pub fn stub_5c4a24() -> ! {
    todo!("0x5c4a24 RBX::Reflection::PropDescriptor<RBX::Lighting,std::string>::GetSetImpl<std::string (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(std::string const&)>::isWriteOnly(void)const")
}

// 0x5c4a28 — __ZNK3RBX10Reflection14PropDescriptorINS_8LightingESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,std::string>::GetSetImpl<std::string (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(std::string const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8LightingESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_5c4a28() -> ! {
    todo!("0x5c4a28 RBX::Reflection::PropDescriptor<RBX::Lighting,std::string>::GetSetImpl<std::string (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(std::string const&)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x5c4a50 — __ZNK3RBX10Reflection14PropDescriptorINS_8LightingESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,std::string>::GetSetImpl<std::string (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(std::string const&)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8LightingESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE8setValueEPNS0_13DescribedBaseES8_
pub fn stub_5c4a50() -> ! {
    todo!("0x5c4a50 RBX::Reflection::PropDescriptor<RBX::Lighting,std::string>::GetSetImpl<std::string (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(std::string const&)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x5c4a74 — __ZN3RBX10Reflection9EventDescINS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Lighting,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Lighting::*>::EventDesc(rbx::signal<void ()(bool)> RBX::Lighting::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_5c4a74() -> ! {
    todo!("0x5c4a74 RBX::Reflection::EventDesc<RBX::Lighting,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Lighting::*>::EventDesc(rbx::signal<void ()(bool)> RBX::Lighting::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x5c4bf8 — __ZN3RBX10Reflection9EventDescINS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Lighting,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Lighting::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev
pub fn stub_5c4bf8() -> ! {
    todo!("0x5c4bf8 RBX::Reflection::EventDesc<RBX::Lighting,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Lighting::*>::~EventDesc()")
}

// 0x5c4cac — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Lighting,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Lighting::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_5c4cac() -> ! {
    todo!("0x5c4cac RBX::Reflection::EventDescImpl<1,RBX::Lighting,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Lighting::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x5c4e00 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Lighting,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Lighting::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
pub fn stub_5c4e00() -> ! {
    todo!("0x5c4e00 RBX::Reflection::EventDescImpl<1,RBX::Lighting,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Lighting::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x5c4e8c — __ZNK3RBX10Reflection13EventDescBaseINS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Lighting,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Lighting::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_5c4e8c() -> ! {
    todo!("0x5c4e8c RBX::Reflection::EventDescBase<RBX::Lighting,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Lighting::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x5c4ea0 — __ZN3RBX8LightingD2Ev
// type: void __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::~Lighting()")]
// was: __ZN3RBX8LightingD2Ev
pub fn stub_5c4ea0() -> ! {
    todo!("0x5c4ea0 RBX::Lighting::~Lighting()")
}

// 0x5c501c — __ZN5boost10posix_time21to_simple_string_typeIcEESbIT_St11char_traitsIS2_ESaIS2_EENS0_13time_durationE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "std::basic_string<char,std::char_traits<char>,std::allocator<char>> boost::posix_time::to_simple_string_type<char>(boost::posix_time::time_duration)")]
// was: __ZN5boost10posix_time21to_simple_string_typeIcEESbIT_St11char_traitsIS2_ESaIS2_EENS0_13time_durationE
pub fn stub_5c501c() -> ! {
    todo!("0x5c501c std::basic_string<char,std::char_traits<char>,std::allocator<char>> boost::posix_time::to_simple_string_type<char>(boost::posix_time::time_duration)")
}

// 0x5c5354 — __ZNK5boost9date_time11int_adapterIxE7compareERKS2_
// type: int(void)
#[doc(alias = "boost::date_time::int_adapter<long long>::compare(boost::date_time::int_adapter<long long> const&)const")]
// was: __ZNK5boost9date_time11int_adapterIxE7compareERKS2_
pub fn stub_5c5354() -> ! {
    todo!("0x5c5354 boost::date_time::int_adapter<long long>::compare(boost::date_time::int_adapter<long long> const&)const")
}

// 0x5c549c — __ZN5boost9date_time32str_from_delimited_time_durationINS_10posix_time13time_durationEcEET_RKSbIT0_St11char_traitsIS5_ESaIS5_EE
// type: int __fastcall(int)
#[doc(alias = "boost::posix_time::time_duration boost::date_time::str_from_delimited_time_duration<boost::posix_time::time_duration,char>(std::basic_string<char,std::char_traits<char>,std::allocator<char>> const&)")]
// was: __ZN5boost9date_time32str_from_delimited_time_durationINS_10posix_time13time_durationEcEET_RKSbIT0_St11char_traitsIS5_ESaIS5_EE
pub fn stub_5c549c() -> ! {
    todo!("0x5c549c boost::posix_time::time_duration boost::date_time::str_from_delimited_time_duration<boost::posix_time::time_duration,char>(std::basic_string<char,std::char_traits<char>,std::allocator<char>> const&)")
}

// 0x5c5d80 — __ZNK5boost9date_time11int_adapterIxEmlEi
// type: int(void)
#[doc(alias = "boost::date_time::int_adapter<long long>::operator*(int)const")]
// was: __ZNK5boost9date_time11int_adapterIxEmlEi
pub fn stub_5c5d80() -> ! {
    todo!("0x5c5d80 boost::date_time::int_adapter<long long>::operator*(int)const")
}

// 0x5c5de4 — __ZNK5boost9date_time11int_adapterIxE17mult_div_specialsERKi
// type: int(void)
#[doc(alias = "boost::date_time::int_adapter<long long>::mult_div_specials(int const&)const")]
// was: __ZNK5boost9date_time11int_adapterIxE17mult_div_specialsERKi
pub fn stub_5c5de4() -> ! {
    todo!("0x5c5de4 boost::date_time::int_adapter<long long>::mult_div_specials(int const&)const")
}

// 0x5c5e9c — __ZNK5boost14char_separatorIcSt11char_traitsIcEE7is_keptEc
// type: int(void)
#[doc(alias = "boost::char_separator<char,std::char_traits<char>>::is_kept(char)const")]
// was: __ZNK5boost14char_separatorIcSt11char_traitsIcEE7is_keptEc
pub fn stub_5c5e9c() -> ! {
    todo!("0x5c5e9c boost::char_separator<char,std::char_traits<char>>::is_kept(char)const")
}

// 0x5c5ecc — __ZN5boost16tokenizer_detail24traits_extension_detailsISt11char_traitsIcELi1EE7ispunctEc
// type: int(void)
#[doc(alias = "boost::tokenizer_detail::traits_extension_details<std::char_traits<char>,1>::ispunct(char)")]
// was: __ZN5boost16tokenizer_detail24traits_extension_detailsISt11char_traitsIcELi1EE7ispunctEc
pub fn stub_5c5ecc() -> ! {
    todo!("0x5c5ecc boost::tokenizer_detail::traits_extension_details<std::char_traits<char>,1>::ispunct(char)")
}

// 0x5c5efc — __ZN5boost16tokenizer_detail24traits_extension_detailsISt11char_traitsIcELi1EE7isspaceEc
// type: int(void)
#[doc(alias = "boost::tokenizer_detail::traits_extension_details<std::char_traits<char>,1>::isspace(char)")]
// was: __ZN5boost16tokenizer_detail24traits_extension_detailsISt11char_traitsIcELi1EE7isspaceEc
pub fn stub_5c5efc() -> ! {
    todo!("0x5c5efc boost::tokenizer_detail::traits_extension_details<std::char_traits<char>,1>::isspace(char)")
}

// 0x5c5f2c — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIxEEbRT_
// type: int(void)
#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long long>(long long &)")]
// was: __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIxEEbRT_
pub fn stub_5c5f2c() -> ! {
    todo!("0x5c5f2c bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long long>(long long &)")
}

// 0x5c5fb8 — __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEycEEbRT0_PKT1_S8_
// type: int(void)
#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long long,char>(unsigned long long &,char const*,char const*)")]
// was: __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEycEEbRT0_PKT1_S8_
pub fn stub_5c5fb8() -> ! {
    todo!("0x5c5fb8 bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long long,char>(unsigned long long &,char const*,char const*)")
}

// 0x5c641c — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE12shr_unsignedItEEbRT_
// type: int(void)
#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned short>(unsigned short &)")]
// was: __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE12shr_unsignedItEEbRT_
pub fn stub_5c641c() -> ! {
    todo!("0x5c641c bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned short>(unsigned short &)")
}

// 0x5c6460 — __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEtcEEbRT0_PKT1_S8_
// type: int(void)
#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned short,char>(unsigned short &,char const*,char const*)")]
// was: __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEtcEEbRT0_PKT1_S8_
pub fn stub_5c6460() -> ! {
    todo!("0x5c6460 bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned short,char>(unsigned short &,char const*,char const*)")
}

// 0x5c6848 — __GLOBAL__I_a_224
#[doc(alias = "global constructor keyed to_a_224")]
// was: __GLOBAL__I_a_224
pub fn stub_5c6848() -> ! {
    todo!("0x5c6848 global constructor keyed to_a_224")
}

// 0x5c7004 — __ZN3RBX13LocalBackpackC1Ev
// type: _DWORD __fastcall(RBX::LocalBackpack *__hidden this)
#[doc(alias = "RBX::LocalBackpack::LocalBackpack(void)")]
// was: __ZN3RBX13LocalBackpackC1Ev
pub fn stub_5c7004() -> ! {
    todo!("0x5c7004 RBX::LocalBackpack::LocalBackpack(void)")
}

// 0x5c7008 — __ZN3RBX13LocalBackpackC2Ev
// type: _DWORD __fastcall(RBX::LocalBackpack *__hidden this)
#[doc(alias = "RBX::LocalBackpack::LocalBackpack(void)")]
// was: __ZN3RBX13LocalBackpackC2Ev
pub fn stub_5c7008() -> ! {
    todo!("0x5c7008 RBX::LocalBackpack::LocalBackpack(void)")
}

// 0x5c7220 — __ZN3RBX13LocalBackpack17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::LocalBackpack *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::LocalBackpack::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX13LocalBackpack17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_5c7220() -> ! {
    todo!("0x5c7220 RBX::LocalBackpack::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x5c722c — __ZN3RBX13LocalBackpack20setOldSchoolBackpackEb
// type: _DWORD __fastcall(RBX::LocalBackpack *__hidden this, bool)
#[doc(alias = "RBX::LocalBackpack::setOldSchoolBackpack(bool)")]
// was: __ZN3RBX13LocalBackpack20setOldSchoolBackpackEb
pub fn stub_5c722c() -> ! {
    todo!("0x5c722c RBX::LocalBackpack::setOldSchoolBackpack(bool)")
}

// 0x5c7230 — __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFvbELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFvbELi1EED1Ev
pub fn stub_5c7230() -> ! {
    todo!("0x5c7230 RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,void ()(bool),1>::~BoundFuncDesc()")
}

// 0x5c7270 — __ZN3RBX13LocalBackpack20getOldSchoolBackpackEv
// type: _DWORD __fastcall(RBX::LocalBackpack *__hidden this)
#[doc(alias = "RBX::LocalBackpack::getOldSchoolBackpack(void)")]
// was: __ZN3RBX13LocalBackpack20getOldSchoolBackpackEv
pub fn stub_5c7270() -> ! {
    todo!("0x5c7270 RBX::LocalBackpack::getOldSchoolBackpack(void)")
}

// 0x5c7274 — __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFbvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,bool ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFbvELi0EED1Ev
pub fn stub_5c7274() -> ! {
    todo!("0x5c7274 RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,bool ()(void),0>::~BoundFuncDesc()")
}

// 0x5c7298 — __ZN3RBX13LocalBackpackD1Ev
// type: void __fastcall(RBX::LocalBackpack *__hidden this)
#[doc(alias = "RBX::LocalBackpack::~LocalBackpack()")]
// was: __ZN3RBX13LocalBackpackD1Ev
pub fn stub_5c7298() -> ! {
    todo!("0x5c7298 RBX::LocalBackpack::~LocalBackpack()")
}

// 0x5c729c — __ZN3RBX13LocalBackpackD0Ev
// type: void __fastcall(RBX::LocalBackpack *__hidden this)
#[doc(alias = "RBX::LocalBackpack::~LocalBackpack()")]
// was: __ZN3RBX13LocalBackpackD0Ev
pub fn stub_5c729c() -> ! {
    todo!("0x5c729c RBX::LocalBackpack::~LocalBackpack()")
}

// 0x5c733c — __ZNK3RBX13LocalBackpack12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::LocalBackpack *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::LocalBackpack::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX13LocalBackpack12askSetParentEPKNS_8InstanceE
pub fn stub_5c733c() -> ! {
    todo!("0x5c733c RBX::LocalBackpack::askSetParent(RBX::Instance const*)const")
}

// 0x5c7340 — __ZNK3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE12getClassNameEv
pub fn stub_5c7340() -> ! {
    todo!("0x5c7340 __ZNK3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE12getClassNameEv")
}

// 0x5c7368 — __ZThn32_N3RBX13LocalBackpackD1Ev
// type: void __fastcall(RBX::LocalBackpack *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LocalBackpack::~LocalBackpack()")]
// was: __ZThn32_N3RBX13LocalBackpackD1Ev
pub fn stub_5c7368() -> ! {
    todo!("0x5c7368 non-virtual thunk toRBX::LocalBackpack::~LocalBackpack()")
}

// 0x5c7370 — __ZThn32_N3RBX13LocalBackpackD0Ev
// type: void __fastcall(RBX::LocalBackpack *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LocalBackpack::~LocalBackpack()")]
// was: __ZThn32_N3RBX13LocalBackpackD0Ev
pub fn stub_5c7370() -> ! {
    todo!("0x5c7370 non-virtual thunk toRBX::LocalBackpack::~LocalBackpack()")
}

// 0x5c7414 — __ZThn32_NK3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE12getClassNameEv
pub fn stub_5c7414() -> ! {
    todo!("0x5c7414 __ZThn32_NK3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE12getClassNameEv")
}

// 0x5c743c — __ZThn36_N3RBX13LocalBackpackD1Ev
// type: void __fastcall(RBX::LocalBackpack *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LocalBackpack::~LocalBackpack()")]
// was: __ZThn36_N3RBX13LocalBackpackD1Ev
pub fn stub_5c743c() -> ! {
    todo!("0x5c743c non-virtual thunk toRBX::LocalBackpack::~LocalBackpack()")
}

// 0x5c7444 — __ZThn36_N3RBX13LocalBackpackD0Ev
// type: void __fastcall(RBX::LocalBackpack *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LocalBackpack::~LocalBackpack()")]
// was: __ZThn36_N3RBX13LocalBackpackD0Ev
pub fn stub_5c7444() -> ! {
    todo!("0x5c7444 non-virtual thunk toRBX::LocalBackpack::~LocalBackpack()")
}

// 0x5c74e8 — __ZN3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5c74e8() -> ! {
    todo!("0x5c74e8 __ZN3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5c74ec — __ZN3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5c74ec() -> ! {
    todo!("0x5c74ec __ZN3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5c758c — __ZThn32_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5c758c() -> ! {
    todo!("0x5c758c __ZThn32_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5c7594 — __ZThn32_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5c7594() -> ! {
    todo!("0x5c7594 __ZThn32_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5c7638 — __ZThn36_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5c7638() -> ! {
    todo!("0x5c7638 __ZThn36_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5c7640 — __ZThn36_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5c7640() -> ! {
    todo!("0x5c7640 __ZThn36_N3RBX10Reflection9DescribedINS_13LocalBackpackELZNS_14sLocalBackpackEENS_17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5c76e4 — __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFbvELi0EEC2EMS2_FbvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,bool ()(void),0>::BoundFuncDesc(bool (RBX::LocalBackpack::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFbvELi0EEC2EMS2_FbvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_5c76e4() -> ! {
    todo!("0x5c76e4 RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,bool ()(void),0>::BoundFuncDesc(bool (RBX::LocalBackpack::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x5c77e8 — __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFbvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,bool ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFbvELi0EED0Ev
pub fn stub_5c77e8() -> ! {
    todo!("0x5c77e8 RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,bool ()(void),0>::~BoundFuncDesc()")
}

// 0x5c789c — __ZNK3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFbvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,bool ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFbvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_5c789c() -> ! {
    todo!("0x5c789c RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,bool ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x5c78c0 — __ZN3RBX10Reflection11Call0HelperINS_13LocalBackpackEMS2_FbvEbE4callEPS2_S4_RNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::LocalBackpack,bool (RBX::LocalBackpack::*)(void),bool>::call(RBX::LocalBackpack*,bool (RBX::LocalBackpack::*)(void),RBX::Reflection::Variant &)")]
// was: __ZN3RBX10Reflection11Call0HelperINS_13LocalBackpackEMS2_FbvEbE4callEPS2_S4_RNS0_7VariantE
pub fn stub_5c78c0() -> ! {
    todo!("0x5c78c0 RBX::Reflection::Call0Helper<RBX::LocalBackpack,bool (RBX::LocalBackpack::*)(void),bool>::call(RBX::LocalBackpack*,bool (RBX::LocalBackpack::*)(void),RBX::Reflection::Variant &)")
}

// 0x5c78f0 — __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,void ()(bool),1>::BoundFuncDesc(void (RBX::LocalBackpack::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_5c78f0() -> ! {
    todo!("0x5c78f0 RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,void ()(bool),1>::BoundFuncDesc(void (RBX::LocalBackpack::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x5c7a68 — __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_5c7a68() -> ! {
    todo!("0x5c7a68 RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x5c7a98 — __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFvbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFvbELi1EED0Ev
pub fn stub_5c7a98() -> ! {
    todo!("0x5c7a98 RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,void ()(bool),1>::~BoundFuncDesc()")
}

// 0x5c7b6c — __ZNK3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13LocalBackpackEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_5c7b6c() -> ! {
    todo!("0x5c7b6c RBX::Reflection::BoundFuncDesc<RBX::LocalBackpack,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x5c7ba0 — __GLOBAL__I_a_225
#[doc(alias = "global constructor keyed to_a_225")]
// was: __GLOBAL__I_a_225
pub fn stub_5c7ba0() -> ! {
    todo!("0x5c7ba0 global constructor keyed to_a_225")
}

// 0x5c7e0c — __GLOBAL__I_a_226
#[doc(alias = "global constructor keyed to_a_226")]
// was: __GLOBAL__I_a_226
pub fn stub_5c7e0c() -> ! {
    todo!("0x5c7e0c global constructor keyed to_a_226")
}

// 0x5c7fa4 — __ZN3RBX7Message7setTextERKSs
// type: _DWORD __fastcall(RBX::Message *__hidden this, const std::string *)
#[doc(alias = "RBX::Message::setText(std::string const&)")]
// was: __ZN3RBX7Message7setTextERKSs
pub fn stub_5c7fa4() -> ! {
    todo!("0x5c7fa4 RBX::Message::setText(std::string const&)")
}

// 0x5c81b4 — __ZN3RBX7MessageC2Ev
// type: _DWORD __fastcall(RBX::Message *__hidden this)
#[doc(alias = "RBX::Message::Message(void)")]
// was: __ZN3RBX7MessageC2Ev
pub fn stub_5c81b4() -> ! {
    todo!("0x5c81b4 RBX::Message::Message(void)")
}

// 0x5c881c — __ZNK3RBX7Message7getTextEv
// type: _DWORD __fastcall(RBX::Message *__hidden this)
#[doc(alias = "RBX::Message::getText(void)const")]
// was: __ZNK3RBX7Message7getTextEv
pub fn stub_5c881c() -> ! {
    todo!("0x5c881c RBX::Message::getText(void)const")
}

// 0x5c8820 — __ZN3RBX10Reflection14PropDescriptorINS_7MessageESsED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Message,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7MessageESsED1Ev
pub fn stub_5c8820() -> ! {
    todo!("0x5c8820 RBX::Reflection::PropDescriptor<RBX::Message,std::string>::~PropDescriptor()")
}

// 0x5c8844 — __ZN3RBX7MessageD1Ev
// type: void __fastcall(RBX::Message *__hidden this)
#[doc(alias = "RBX::Message::~Message()")]
// was: __ZN3RBX7MessageD1Ev
pub fn stub_5c8844() -> ! {
    todo!("0x5c8844 RBX::Message::~Message()")
}

// 0x5c8938 — __ZN3RBX7MessageD0Ev
// type: void __fastcall(RBX::Message *__hidden this)
#[doc(alias = "RBX::Message::~Message()")]
// was: __ZN3RBX7MessageD0Ev
pub fn stub_5c8938() -> ! {
    todo!("0x5c8938 RBX::Message::~Message()")
}

// 0x5c8a3c — __ZNK3RBX7Message21getPersistentDataCostEv
// type: _DWORD __fastcall(RBX::Message *__hidden this)
#[doc(alias = "RBX::Message::getPersistentDataCost(void)const")]
// was: __ZNK3RBX7Message21getPersistentDataCostEv
pub fn stub_5c8a3c() -> ! {
    todo!("0x5c8a3c RBX::Message::getPersistentDataCost(void)const")
}

// 0x5c8a68 — __ZNK3RBX7Message12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Message *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Message::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX7Message12askSetParentEPKNS_8InstanceE
pub fn stub_5c8a68() -> ! {
    todo!("0x5c8a68 RBX::Message::askSetParent(RBX::Instance const*)const")
}

// 0x5c8a6c — __ZNK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E12getClassNameEv
pub fn stub_5c8a6c() -> ! {
    todo!("0x5c8a6c __ZNK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E12getClassNameEv")
}

// 0x5c8a80 — __ZThn32_N3RBX7MessageD1Ev
// type: void __fastcall(RBX::Message *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Message::~Message()")]
// was: __ZThn32_N3RBX7MessageD1Ev
pub fn stub_5c8a80() -> ! {
    todo!("0x5c8a80 non-virtual thunk toRBX::Message::~Message()")
}

// 0x5c8b74 — __ZThn32_N3RBX7MessageD0Ev
// type: void __fastcall(RBX::Message *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Message::~Message()")]
// was: __ZThn32_N3RBX7MessageD0Ev
pub fn stub_5c8b74() -> ! {
    todo!("0x5c8b74 non-virtual thunk toRBX::Message::~Message()")
}

// 0x5c8c7c — __ZThn32_NK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E12getClassNameEv
pub fn stub_5c8c7c() -> ! {
    todo!("0x5c8c7c __ZThn32_NK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E12getClassNameEv")
}

// 0x5c8c8c — __ZThn36_N3RBX7MessageD1Ev
// type: void __fastcall(RBX::Message *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Message::~Message()")]
// was: __ZThn36_N3RBX7MessageD1Ev
pub fn stub_5c8c8c() -> ! {
    todo!("0x5c8c8c non-virtual thunk toRBX::Message::~Message()")
}

// 0x5c8d7c — __ZThn36_N3RBX7MessageD0Ev
// type: void __fastcall(RBX::Message *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Message::~Message()")]
// was: __ZThn36_N3RBX7MessageD0Ev
pub fn stub_5c8d7c() -> ! {
    todo!("0x5c8d7c non-virtual thunk toRBX::Message::~Message()")
}

// 0x5c8e88 — __ZN3RBX4HintD1Ev
// type: void __fastcall(RBX::Hint *__hidden this)
#[doc(alias = "RBX::Hint::~Hint()")]
// was: __ZN3RBX4HintD1Ev
pub fn stub_5c8e88() -> ! {
    todo!("0x5c8e88 RBX::Hint::~Hint()")
}

// 0x5c8f7c — __ZN3RBX4HintD0Ev
// type: void __fastcall(RBX::Hint *__hidden this)
#[doc(alias = "RBX::Hint::~Hint()")]
// was: __ZN3RBX4HintD0Ev
pub fn stub_5c8f7c() -> ! {
    todo!("0x5c8f7c RBX::Hint::~Hint()")
}

// 0x5c9080 — __ZN3RBX4Hint15canClientCreateEv
// type: _DWORD __fastcall(RBX::Hint *__hidden this)
#[doc(alias = "RBX::Hint::canClientCreate(void)")]
// was: __ZN3RBX4Hint15canClientCreateEv
pub fn stub_5c9080() -> ! {
    todo!("0x5c9080 RBX::Hint::canClientCreate(void)")
}

// 0x5c9084 — __ZNK3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE12getClassNameEv
pub fn stub_5c9084() -> ! {
    todo!("0x5c9084 __ZNK3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE12getClassNameEv")
}

// 0x5c9094 — __ZThn32_N3RBX4HintD1Ev
// type: void __fastcall(RBX::Hint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Hint::~Hint()")]
// was: __ZThn32_N3RBX4HintD1Ev
pub fn stub_5c9094() -> ! {
    todo!("0x5c9094 non-virtual thunk toRBX::Hint::~Hint()")
}

// 0x5c9188 — __ZThn32_N3RBX4HintD0Ev
// type: void __fastcall(RBX::Hint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Hint::~Hint()")]
// was: __ZThn32_N3RBX4HintD0Ev
pub fn stub_5c9188() -> ! {
    todo!("0x5c9188 non-virtual thunk toRBX::Hint::~Hint()")
}
