//! rendering shard 457 — 100 stubs 0x6d82b4..0x6dd050 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn gap filler, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Global gap filler fallback EA asc not yet in rbx_rendering (48710->48810 distinct, fallback after 0x6d8294).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc Ogre tail + gap fallback not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x6d82b4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_6CameraEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Camera>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_6CameraEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_6CameraEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
pub fn stub_6d82b4() -> ! {
    todo!("0x6d82b4 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Camera>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0x6d8394 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_6CameraEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: 
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Camera>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_6CameraEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
// was: __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_6CameraEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
pub fn stub_6d8394() -> ! {
    todo!("0x6d8394 non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Camera>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0x6d83a0 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_6CameraEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Camera *>::GetSetImpl<RBX::Camera * (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(RBX::Camera *)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_6CameraEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_6CameraEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
pub fn stub_6d83a0() -> ! {
    todo!("0x6d83a0 RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Camera *>::GetSetImpl<RBX::Camera * (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(RBX::Camera *)>::isReadOnly(void)const")
}

// 0x6d83a4 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_6CameraEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Camera *>::GetSetImpl<RBX::Camera * (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(RBX::Camera *)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_6CameraEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_6CameraEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
pub fn stub_6d83a4() -> ! {
    todo!("0x6d83a4 RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Camera *>::GetSetImpl<RBX::Camera * (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(RBX::Camera *)>::isWriteOnly(void)const")
}

// 0x6d83a8 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_6CameraEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Camera *>::GetSetImpl<RBX::Camera * (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(RBX::Camera *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_6CameraEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_6CameraEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_6d83a8() -> ! {
    todo!("0x6d83a8 RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Camera *>::GetSetImpl<RBX::Camera * (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(RBX::Camera *)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6d83c8 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_6CameraEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Camera *>::GetSetImpl<RBX::Camera * (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(RBX::Camera *)>::setValue(RBX::Reflection::DescribedBase *,RBX::Camera * const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_6CameraEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_6CameraEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
pub fn stub_6d83c8() -> ! {
    todo!("0x6d83c8 RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Camera *>::GetSetImpl<RBX::Camera * (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(RBX::Camera *)>::setValue(RBX::Reflection::DescribedBase *,RBX::Camera * const&)const")
}

// 0x6d83ec — __ZN3RBX10Reflection7RefTypeIPNS_6CameraEED1Ev
// type: 
#[doc(alias = "RBX::Reflection::RefType<RBX::Camera *>::~RefType()")]
#[doc(alias = "__ZN3RBX10Reflection7RefTypeIPNS_6CameraEED1Ev")]
// was: __ZN3RBX10Reflection7RefTypeIPNS_6CameraEED1Ev
pub fn stub_6d83ec() -> ! {
    todo!("0x6d83ec RBX::Reflection::RefType<RBX::Camera *>::~RefType()")
}

// 0x6d83f0 — __ZN3RBX10Reflection4TypeC2IPNS_6CameraEEEPKcS6_PT_
// type: 
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Camera *>(char const*,char const*,RBX::Camera * *)")]
#[doc(alias = "__ZN3RBX10Reflection4TypeC2IPNS_6CameraEEEPKcS6_PT_")]
// was: __ZN3RBX10Reflection4TypeC2IPNS_6CameraEEEPKcS6_PT_
pub fn stub_6d83f0() -> ! {
    todo!("0x6d83f0 RBX::Reflection::Type::Type<RBX::Camera *>(char const*,char const*,RBX::Camera * *)")
}

// 0x6d849c — __ZN3RBX10Reflection7RefTypeIPNS_6CameraEED0Ev
// type: 
#[doc(alias = "RBX::Reflection::RefType<RBX::Camera *>::~RefType()")]
#[doc(alias = "__ZN3RBX10Reflection7RefTypeIPNS_6CameraEED0Ev")]
// was: __ZN3RBX10Reflection7RefTypeIPNS_6CameraEED0Ev
pub fn stub_6d849c() -> ! {
    todo!("0x6d849c RBX::Reflection::RefType<RBX::Camera *>::~RefType()")
}

// 0x6d84a4 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(void),0>::BoundFuncDesc(void (RBX::Workspace::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_6d84a4() -> ! {
    todo!("0x6d84a4 RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(void),0>::BoundFuncDesc(void (RBX::Workspace::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x6d85a8 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvvELi0EED0Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvvELi0EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvvELi0EED0Ev
pub fn stub_6d85a8() -> ! {
    todo!("0x6d85a8 RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(void),0>::~BoundFuncDesc()")
}

// 0x6d865c — __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_6d865c() -> ! {
    todo!("0x6d865c RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x6d867c — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(bool),1>::BoundFuncDesc(void (RBX::Workspace::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_6d867c() -> ! {
    todo!("0x6d867c RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(bool),1>::BoundFuncDesc(void (RBX::Workspace::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x6d87f4 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_6d87f4() -> ! {
    todo!("0x6d87f4 RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x6d8824 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvbELi1EED0Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(bool),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvbELi1EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvbELi1EED0Ev
pub fn stub_6d8824() -> ! {
    todo!("0x6d8824 RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(bool),1>::~BoundFuncDesc()")
}

// 0x6d88f8 — __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_6d88f8() -> ! {
    todo!("0x6d88f8 RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x6d8930 — __ZN3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: 
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::Workspace::*)(void)const,int>(char const*,char const*,RBX::Instance* (RBX::Workspace::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_6d8930() -> ! {
    todo!("0x6d8930 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::Workspace::*)(void)const,int>(char const*,char const*,RBX::Instance* (RBX::Workspace::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x6d89d4 — __ZN3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEED0Ev
// type: 
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEED0Ev")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEED0Ev
pub fn stub_6d89d4() -> ! {
    todo!("0x6d89d4 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::~RefPropDescriptor()")
}

// 0x6d8a04 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE10isReadOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE10isReadOnlyEv")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE10isReadOnlyEv
pub fn stub_6d8a04() -> ! {
    todo!("0x6d8a04 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::isReadOnly(void)const")
}

// 0x6d8a14 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11isWriteOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11isWriteOnlyEv")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11isWriteOnlyEv
pub fn stub_6d8a14() -> ! {
    todo!("0x6d8a14 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::isWriteOnly(void)const")
}

// 0x6d8a24 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: 
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
pub fn stub_6d8a24() -> ! {
    todo!("0x6d8a24 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x6d8a4c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
pub fn stub_6d8a4c() -> ! {
    todo!("0x6d8a4c RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x6d8b64 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: 
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_6d8b64() -> ! {
    todo!("0x6d8b64 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x6d8c2c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: 
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_6d8c2c() -> ! {
    todo!("0x6d8c2c RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x6d8c50 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: 
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_6d8c50() -> ! {
    todo!("0x6d8c50 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x6d8d24 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: 
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_6d8d24() -> ! {
    todo!("0x6d8d24 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x6d8d48 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
pub fn stub_6d8d48() -> ! {
    todo!("0x6d8d48 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6d8d5c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
pub fn stub_6d8d5c() -> ! {
    todo!("0x6d8d5c RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")
}

// 0x6d8dd8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// type: 
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
pub fn stub_6d8dd8() -> ! {
    todo!("0x6d8dd8 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")
}

// 0x6d8df8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
pub fn stub_6d8df8() -> ! {
    todo!("0x6d8df8 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0x6d8ed8 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: 
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
// was: __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
pub fn stub_6d8ed8() -> ! {
    todo!("0x6d8ed8 non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0x6d8ee0 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::Workspace::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
pub fn stub_6d8ee0() -> ! {
    todo!("0x6d8ee0 RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::Workspace::*)(void)const>::isReadOnly(void)const")
}

// 0x6d8ee4 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::Workspace::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
pub fn stub_6d8ee4() -> ! {
    todo!("0x6d8ee4 RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::Workspace::*)(void)const>::isWriteOnly(void)const")
}

// 0x6d8ee8 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::Workspace::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_6d8ee8() -> ! {
    todo!("0x6d8ee8 RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::Workspace::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6d8f08 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::Workspace::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEPNS_8InstanceEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
pub fn stub_6d8f08() -> ! {
    todo!("0x6d8f08 RBX::Reflection::PropDescriptor<RBX::Workspace,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::Workspace::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")
}

// 0x6d902c — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbELi3EEC2EMS2_FS7_S8_SF_bEPKcSL_SL_SL_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),3>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),char const*,char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,bool),3>::BoundFuncDesc(boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,bool),char const*,char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbELi3EEC2EMS2_FS7_S8_SF_bEPKcSL_SL_SL_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbELi3EEC2EMS2_FS7_S8_SF_bEPKcSL_SL_SL_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_6d902c() -> ! {
    todo!("0x6d902c RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),3>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),char const*,char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x6d9284 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbELi3EE16declareSignatureEPKcNS0_7VariantESJ_SK_SJ_SK_
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbELi3EE16declareSignatureEPKcNS0_7VariantESJ_SK_SJ_SK_")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbELi3EE16declareSignatureEPKcNS0_7VariantESJ_SK_SJ_SK_
pub fn stub_6d9284() -> ! {
    todo!("0x6d9284 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x6d92ec — __ZN5boost10scoped_ptrINS_10shared_ptrIKSt6vectorINS1_IN3RBX8InstanceEEESaIS5_EEEEED1Ev
// type: 
#[doc(alias = "boost::scoped_ptr<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::~scoped_ptr()")]
#[doc(alias = "boost::scoped_ptr<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrINS_10shared_ptrIKSt6vectorINS1_IN3RBX8InstanceEEESaIS5_EEEEED1Ev")]
// was: __ZN5boost10scoped_ptrINS_10shared_ptrIKSt6vectorINS1_IN3RBX8InstanceEEESaIS5_EEEEED1Ev
pub fn stub_6d92ec() -> ! {
    todo!("0x6d92ec boost::scoped_ptr<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::~scoped_ptr()")
}

// 0x6d9398 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbELi3EED0Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),3>::~BoundFuncDesc()")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,bool),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbELi3EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbELi3EED0Ev
pub fn stub_6d9398() -> ! {
    todo!("0x6d9398 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),3>::~BoundFuncDesc()")
}

// 0x6d94d8 — __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_6d94d8() -> ! {
    todo!("0x6d94d8 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x6d9604 — __ZN3RBX10Reflection11Call3HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbES8_SF_bS7_E4callEPS2_SH_RNS0_7VariantERKS8_RKSF_RKb
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::call(RBX::Workspace*,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),RBX::Reflection::Variant &,RBX::RbxRay const&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&,bool const&)")]
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,bool),RBX::RbxRay,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,bool,boost::shared_ptr<RBX::Reflection::Tuple const>>::call(RBX::Workspace*,boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,bool),RBX::Reflection::Variant &,RBX::RbxRay const&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&,bool const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call3HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbES8_SF_bS7_E4callEPS2_SH_RNS0_7VariantERKS8_RKSF_RKb")]
// was: __ZN3RBX10Reflection11Call3HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbES8_SF_bS7_E4callEPS2_SH_RNS0_7VariantERKS8_RKSF_RKb
pub fn stub_6d9604() -> ! {
    todo!("0x6d9604 RBX::Reflection::Call3Helper<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::call(RBX::Workspace*,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),RBX::Reflection::Variant &,RBX::RbxRay const&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&,bool const&)")
}

// 0x6d9770 — __ZN3RBX10Reflection9ArgHelper6getArgINS_6RbxRayELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: 
#[doc(alias = "RBX::RbxRay RBX::Reflection::ArgHelper::getArg<RBX::RbxRay,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::RbxRay> const&,boost::disable_if<boost::is_same<RBX::RbxRay,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "RBX::RbxRay RBX::Reflection::ArgHelper::getArg<RBX::RbxRay,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::RbxRay> const&,boost::disable_if<boost::is_same<RBX::RbxRay,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_6RbxRayELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_6RbxRayELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_6d9770() -> ! {
    todo!("0x6d9770 RBX::RbxRay RBX::Reflection::ArgHelper::getArg<RBX::RbxRay,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::RbxRay> const&,boost::disable_if<boost::is_same<RBX::RbxRay,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x6d9954 — __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISC_EEPNS3_10disable_ifINS3_7is_sameISC_NS4_IKNS0_5TupleEEEEEvE4typeE
// type: 
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> RBX::Reflection::ArgHelper::getArg<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>> const&,boost::disable_if<boost::is_same<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISC_EEPNS3_10disable_ifINS3_7is_sameISC_NS4_IKNS0_5TupleEEEEEvE4typeE")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISC_EEPNS3_10disable_ifINS3_7is_sameISC_NS4_IKNS0_5TupleEEEEEvE4typeE
pub fn stub_6d9954() -> ! {
    todo!("0x6d9954 rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x6d9b18 — __ZN3RBX10Reflection9ArgHelper6getArgIbLi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: 
#[doc(alias = "bool RBX::Reflection::ArgHelper::getArg<bool,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "bool RBX::Reflection::ArgHelper::getArg<bool,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgIbLi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIbLi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_6d9b18() -> ! {
    todo!("0x6d9b18 bool RBX::Reflection::ArgHelper::getArg<bool,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x6d9d2c — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEELi2EEC2EMS2_FbS3_SC_EPKcSI_SI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),2>::BoundFuncDesc(bool (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),2>::BoundFuncDesc(bool (RBX::Workspace::*)(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEELi2EEC2EMS2_FbS3_SC_EPKcSI_SI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEELi2EEC2EMS2_FbS3_SC_EPKcSI_SI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_6d9d2c() -> ! {
    todo!("0x6d9d2c RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),2>::BoundFuncDesc(bool (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x6d9ef8 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEELi2EE16declareSignatureEPKcNS0_7VariantESG_SH_
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEELi2EE16declareSignatureEPKcNS0_7VariantESG_SH_")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEELi2EE16declareSignatureEPKcNS0_7VariantESG_SH_
pub fn stub_6d9ef8() -> ! {
    todo!("0x6d9ef8 RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x6d9f44 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEELi2EED0Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),2>::~BoundFuncDesc()")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEELi2EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEELi2EED0Ev
pub fn stub_6d9f44() -> ! {
    todo!("0x6d9f44 RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),2>::~BoundFuncDesc()")
}

// 0x6da064 — __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_6da064() -> ! {
    todo!("0x6da064 RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x6da174 — __ZN3RBX10Reflection11Call2HelperINS_9WorkspaceEMS2_FbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEES3_SC_bE4callEPS2_SE_RNS0_7VariantERKS3_RKSC_
// type: int __fastcall(int, int, int, int, G3D::Matrix3 *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Workspace,bool (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool>::call(RBX::Workspace*,bool (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),RBX::Reflection::Variant &,RBX::Region3 const&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&)")]
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Workspace,bool (RBX::Workspace::*)(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,bool>::call(RBX::Workspace*,bool (RBX::Workspace::*)(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),RBX::Reflection::Variant &,RBX::Region3 const&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call2HelperINS_9WorkspaceEMS2_FbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEES3_SC_bE4callEPS2_SE_RNS0_7VariantERKS3_RKSC_")]
// was: __ZN3RBX10Reflection11Call2HelperINS_9WorkspaceEMS2_FbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEES3_SC_bE4callEPS2_SE_RNS0_7VariantERKS3_RKSC_
pub fn stub_6da174() -> ! {
    todo!("0x6da174 RBX::Reflection::Call2Helper<RBX::Workspace,bool (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool>::call(RBX::Workspace*,bool (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),RBX::Reflection::Variant &,RBX::Region3 const&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&)")
}

// 0x6da2bc — __ZN3RBX10Reflection9ArgHelper6getArgINS_7Region3ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: 
#[doc(alias = "RBX::Region3 RBX::Reflection::ArgHelper::getArg<RBX::Region3,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Region3> const&,boost::disable_if<boost::is_same<RBX::Region3,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "RBX::Region3 RBX::Reflection::ArgHelper::getArg<RBX::Region3,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Region3> const&,boost::disable_if<boost::is_same<RBX::Region3,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_7Region3ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_7Region3ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_6da2bc() -> ! {
    todo!("0x6da2bc RBX::Region3 RBX::Reflection::ArgHelper::getArg<RBX::Region3,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Region3> const&,boost::disable_if<boost::is_same<RBX::Region3,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x6da4e8 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iELi3EEC2EMS2_FSB_SC_SB_iEPKcSI_SI_SI_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),3>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),char const*,char const*,char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,int),3>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,int),char const*,char const*,char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iELi3EEC2EMS2_FSB_SC_SB_iEPKcSI_SI_SI_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iELi3EEC2EMS2_FSB_SC_SB_iEPKcSI_SI_SI_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_6da4e8() -> ! {
    todo!("0x6da4e8 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),3>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),char const*,char const*,char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x6da73c — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iELi3EE16declareSignatureEPKcNS0_7VariantESG_SH_SG_SH_
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iELi3EE16declareSignatureEPKcNS0_7VariantESG_SH_SG_SH_")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iELi3EE16declareSignatureEPKcNS0_7VariantESG_SH_SG_SH_
pub fn stub_6da73c() -> ! {
    todo!("0x6da73c RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x6da7a4 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iELi3EED0Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),3>::~BoundFuncDesc()")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,int),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iELi3EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iELi3EED0Ev
pub fn stub_6da7a4() -> ! {
    todo!("0x6da7a4 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),3>::~BoundFuncDesc()")
}

// 0x6da8d0 — __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_6da8d0() -> ! {
    todo!("0x6da8d0 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x6da9f8 — __ZN3RBX10Reflection11Call3HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iESC_SB_iSB_E4callEPS2_SE_RNS0_7VariantERKSC_RKSB_RKi
// type: int __fastcall(int, int, int, int, G3D::Matrix3 *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Workspace*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),RBX::Reflection::Variant &,RBX::Region3 const&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&,int const&)")]
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,int),RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,int,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::Workspace*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,int),RBX::Reflection::Variant &,RBX::Region3 const&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&,int const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call3HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iESC_SB_iSB_E4callEPS2_SE_RNS0_7VariantERKSC_RKSB_RKi")]
// was: __ZN3RBX10Reflection11Call3HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iESC_SB_iSB_E4callEPS2_SE_RNS0_7VariantERKSC_RKSB_RKi
pub fn stub_6da9f8() -> ! {
    todo!("0x6da9f8 RBX::Reflection::Call3Helper<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Workspace*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),RBX::Reflection::Variant &,RBX::Region3 const&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&,int const&)")
}

// 0x6dab64 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbELi3EEC2EMS2_FS7_S8_SA_bEPKcSG_SG_SA_SG_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),3>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),char const*,char const*,char const*,rbx_core::SharedPtr<RBX::Instance>,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,boost::shared_ptr<RBX::Instance>,bool),3>::BoundFuncDesc(boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,boost::shared_ptr<RBX::Instance>,bool),char const*,char const*,char const*,boost::shared_ptr<RBX::Instance>,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbELi3EEC2EMS2_FS7_S8_SA_bEPKcSG_SG_SA_SG_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbELi3EEC2EMS2_FS7_S8_SA_bEPKcSG_SG_SA_SG_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_6dab64() -> ! {
    todo!("0x6dab64 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),3>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),char const*,char const*,char const*,rbx_core::SharedPtr<RBX::Instance>,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x6dae38 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbELi3EE16declareSignatureEPKcNS0_7VariantESE_SF_SE_SF_
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,boost::shared_ptr<RBX::Instance>,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbELi3EE16declareSignatureEPKcNS0_7VariantESE_SF_SE_SF_")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbELi3EE16declareSignatureEPKcNS0_7VariantESE_SF_SE_SF_
pub fn stub_6dae38() -> ! {
    todo!("0x6dae38 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x6daea0 — __ZN5boost10scoped_ptrINS_10shared_ptrIN3RBX8InstanceEEEED1Ev
// type: 
#[doc(alias = "boost::scoped_ptr<rbx_core::SharedPtr<RBX::Instance>>::~scoped_ptr()")]
#[doc(alias = "boost::scoped_ptr<boost::shared_ptr<RBX::Instance>>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrINS_10shared_ptrIN3RBX8InstanceEEEED1Ev")]
// was: __ZN5boost10scoped_ptrINS_10shared_ptrIN3RBX8InstanceEEEED1Ev
pub fn stub_6daea0() -> ! {
    todo!("0x6daea0 boost::scoped_ptr<rbx_core::SharedPtr<RBX::Instance>>::~scoped_ptr()")
}

// 0x6daf4c — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbELi3EED0Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),3>::~BoundFuncDesc()")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,boost::shared_ptr<RBX::Instance>,bool),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbELi3EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbELi3EED0Ev
pub fn stub_6daf4c() -> ! {
    todo!("0x6daf4c RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),3>::~BoundFuncDesc()")
}

// 0x6db08c — __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,boost::shared_ptr<RBX::Instance>,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_6db08c() -> ! {
    todo!("0x6db08c RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x6db1b8 — __ZN3RBX10Reflection11Call3HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbES8_SA_bS7_E4callEPS2_SC_RNS0_7VariantERKS8_RKSA_RKb
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::call(RBX::Workspace*,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),RBX::Reflection::Variant &,RBX::RbxRay const&,rbx_core::SharedPtr<RBX::Instance> const&,bool const&)")]
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,boost::shared_ptr<RBX::Instance>,bool),RBX::RbxRay,boost::shared_ptr<RBX::Instance>,bool,boost::shared_ptr<RBX::Reflection::Tuple const>>::call(RBX::Workspace*,boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,boost::shared_ptr<RBX::Instance>,bool),RBX::Reflection::Variant &,RBX::RbxRay const&,boost::shared_ptr<RBX::Instance> const&,bool const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call3HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbES8_SA_bS7_E4callEPS2_SC_RNS0_7VariantERKS8_RKSA_RKb")]
// was: __ZN3RBX10Reflection11Call3HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbES8_SA_bS7_E4callEPS2_SC_RNS0_7VariantERKS8_RKSA_RKb
pub fn stub_6db1b8() -> ! {
    todo!("0x6db1b8 RBX::Reflection::Call3Helper<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::call(RBX::Workspace*,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::Workspace::*)(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),RBX::Reflection::Variant &,RBX::RbxRay const&,rbx_core::SharedPtr<RBX::Instance> const&,bool const&)")
}

// 0x6db328 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS2_FbS3_S7_EPKcSD_SD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(bool (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,rbx_core::SharedPtr<RBX::Instance>,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,boost::shared_ptr<RBX::Instance>),2>::BoundFuncDesc(bool (RBX::Workspace::*)(RBX::Region3,boost::shared_ptr<RBX::Instance>),char const*,char const*,char const*,boost::shared_ptr<RBX::Instance>,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS2_FbS3_S7_EPKcSD_SD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS2_FbS3_S7_EPKcSD_SD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_6db328() -> ! {
    todo!("0x6db328 RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(bool (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,rbx_core::SharedPtr<RBX::Instance>,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x6db574 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEELi2EE16declareSignatureEPKcNS0_7VariantESB_SC_
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,boost::shared_ptr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEELi2EE16declareSignatureEPKcNS0_7VariantESB_SC_")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEELi2EE16declareSignatureEPKcNS0_7VariantESB_SC_
pub fn stub_6db574() -> ! {
    todo!("0x6db574 RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x6db5c0 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev
pub fn stub_6db5c0() -> ! {
    todo!("0x6db5c0 RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")
}

// 0x6db6e0 — __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,boost::shared_ptr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_6db6e0() -> ! {
    todo!("0x6db6e0 RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x6db7f0 — __ZN3RBX10Reflection11Call2HelperINS_9WorkspaceEMS2_FbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEES3_S7_bE4callEPS2_S9_RNS0_7VariantERKS3_RKS7_
// type: int __fastcall(int, int, int, int, G3D::Matrix3 *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Workspace,bool (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,bool>::call(RBX::Workspace*,bool (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,RBX::Region3 const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Workspace,bool (RBX::Workspace::*)(RBX::Region3,boost::shared_ptr<RBX::Instance>),RBX::Region3,boost::shared_ptr<RBX::Instance>,bool>::call(RBX::Workspace*,bool (RBX::Workspace::*)(RBX::Region3,boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,RBX::Region3 const&,boost::shared_ptr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call2HelperINS_9WorkspaceEMS2_FbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEES3_S7_bE4callEPS2_S9_RNS0_7VariantERKS3_RKS7_")]
// was: __ZN3RBX10Reflection11Call2HelperINS_9WorkspaceEMS2_FbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEES3_S7_bE4callEPS2_S9_RNS0_7VariantERKS3_RKS7_
pub fn stub_6db7f0() -> ! {
    todo!("0x6db7f0 RBX::Reflection::Call2Helper<RBX::Workspace,bool (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,bool>::call(RBX::Workspace*,bool (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,RBX::Region3 const&,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x6db938 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iELi3EEC2EMS2_FSB_SC_S7_iEPKcSI_SI_S7_SI_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),3>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),char const*,char const*,char const*,rbx_core::SharedPtr<RBX::Instance>,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::Region3,boost::shared_ptr<RBX::Instance>,int),3>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,boost::shared_ptr<RBX::Instance>,int),char const*,char const*,char const*,boost::shared_ptr<RBX::Instance>,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iELi3EEC2EMS2_FSB_SC_S7_iEPKcSI_SI_S7_SI_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iELi3EEC2EMS2_FSB_SC_S7_iEPKcSI_SI_S7_SI_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_6db938() -> ! {
    todo!("0x6db938 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),3>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),char const*,char const*,char const*,rbx_core::SharedPtr<RBX::Instance>,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x6dbc08 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iELi3EE16declareSignatureEPKcNS0_7VariantESG_SH_SG_SH_
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::Region3,boost::shared_ptr<RBX::Instance>,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iELi3EE16declareSignatureEPKcNS0_7VariantESG_SH_SG_SH_")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iELi3EE16declareSignatureEPKcNS0_7VariantESG_SH_SG_SH_
pub fn stub_6dbc08() -> ! {
    todo!("0x6dbc08 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x6dbc70 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iELi3EED0Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),3>::~BoundFuncDesc()")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::Region3,boost::shared_ptr<RBX::Instance>,int),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iELi3EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iELi3EED0Ev
pub fn stub_6dbc70() -> ! {
    todo!("0x6dbc70 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),3>::~BoundFuncDesc()")
}

// 0x6dbd9c — __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::Region3,boost::shared_ptr<RBX::Instance>,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_6dbd9c() -> ! {
    todo!("0x6dbd9c RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x6dbec4 — __ZN3RBX10Reflection11Call3HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iESC_S7_iSB_E4callEPS2_SE_RNS0_7VariantERKSC_RKS7_RKi
// type: int __fastcall(int, int, int, int, G3D::Matrix3 *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Workspace*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),RBX::Reflection::Variant &,RBX::Region3 const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&)")]
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,boost::shared_ptr<RBX::Instance>,int),RBX::Region3,boost::shared_ptr<RBX::Instance>,int,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::Workspace*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,boost::shared_ptr<RBX::Instance>,int),RBX::Reflection::Variant &,RBX::Region3 const&,boost::shared_ptr<RBX::Instance> const&,int const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call3HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iESC_S7_iSB_E4callEPS2_SE_RNS0_7VariantERKSC_RKS7_RKi")]
// was: __ZN3RBX10Reflection11Call3HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iESC_S7_iSB_E4callEPS2_SE_RNS0_7VariantERKSC_RKS7_RKi
pub fn stub_6dbec4() -> ! {
    todo!("0x6dbec4 RBX::Reflection::Call3Helper<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Workspace*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),RBX::Reflection::Variant &,RBX::Region3 const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&)")
}

// 0x6dc030 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EEC2EMS2_FvSB_EPKcSH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::BoundFuncDesc(void (RBX::Workspace::*)(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),1>::BoundFuncDesc(void (RBX::Workspace::*)(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EEC2EMS2_FvSB_EPKcSH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EEC2EMS2_FvSB_EPKcSH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_6dc030() -> ! {
    todo!("0x6dc030 RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::BoundFuncDesc(void (RBX::Workspace::*)(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x6dc1ac — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_6dc1ac() -> ! {
    todo!("0x6dc1ac RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x6dc1dc — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EED0Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::~BoundFuncDesc()")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EED0Ev
pub fn stub_6dc1dc() -> ! {
    todo!("0x6dc1dc RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::~BoundFuncDesc()")
}

// 0x6dc2e4 — __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_6dc2e4() -> ! {
    todo!("0x6dc2e4 RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x6dc3c8 — __ZN3RBX10Reflection11Call1HelperINS_9WorkspaceEMS2_FvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEESB_vE4callEPS2_SD_RNS0_7VariantERKSB_
// type: 
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Workspace,void (RBX::Workspace::*)(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,void>::call(RBX::Workspace*,void (RBX::Workspace::*)(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&)")]
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Workspace,void (RBX::Workspace::*)(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,void>::call(RBX::Workspace*,void (RBX::Workspace::*)(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),RBX::Reflection::Variant &,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_9WorkspaceEMS2_FvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEESB_vE4callEPS2_SD_RNS0_7VariantERKSB_")]
// was: __ZN3RBX10Reflection11Call1HelperINS_9WorkspaceEMS2_FvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEESB_vE4callEPS2_SD_RNS0_7VariantERKSB_
pub fn stub_6dc3c8() -> ! {
    todo!("0x6dc3c8 RBX::Reflection::Call1Helper<RBX::Workspace,void (RBX::Workspace::*)(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,void>::call(RBX::Workspace*,void (RBX::Workspace::*)(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&)")
}

// 0x6dc4b0 — __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISC_EEPNS3_10disable_ifINS3_7is_sameISC_NS4_IKNS0_5TupleEEEEEvE4typeE
// type: 
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> RBX::Reflection::ArgHelper::getArg<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>> const&,boost::disable_if<boost::is_same<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISC_EEPNS3_10disable_ifINS3_7is_sameISC_NS4_IKNS0_5TupleEEEEEvE4typeE")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISC_EEPNS3_10disable_ifINS3_7is_sameISC_NS4_IKNS0_5TupleEEEEEvE4typeE
pub fn stub_6dc4b0() -> ! {
    todo!("0x6dc4b0 rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x6dc670 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EEC2EMS2_FSB_SC_EPKcSI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EEC2EMS2_FSB_SC_EPKcSI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EEC2EMS2_FSB_SC_EPKcSI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_6dc670() -> ! {
    todo!("0x6dc670 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x6dc7e8 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_6dc7e8() -> ! {
    todo!("0x6dc7e8 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x6dc818 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EED0Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EED0Ev
pub fn stub_6dc818() -> ! {
    todo!("0x6dc818 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()")
}

// 0x6dc8e4 — __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_6dc8e4() -> ! {
    todo!("0x6dc8e4 RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x6dca24 — __ZN3RBX10Reflection11Call1HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEESC_SB_E4callEPS2_SE_RNS0_7VariantERKSC_
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::ContentId),RBX::ContentId,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Workspace*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")]
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::ContentId),RBX::ContentId,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::Workspace*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEESC_SB_E4callEPS2_SE_RNS0_7VariantERKSC_")]
// was: __ZN3RBX10Reflection11Call1HelperINS_9WorkspaceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEESC_SB_E4callEPS2_SE_RNS0_7VariantERKSC_
pub fn stub_6dca24() -> ! {
    todo!("0x6dca24 RBX::Reflection::Call1Helper<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::ContentId),RBX::ContentId,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Workspace*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Workspace::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")
}

// 0x6dcbac — __ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::PropDescriptor<bool (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(bool)>(char const*,char const*,bool (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_6dcbac() -> ! {
    todo!("0x6dcbac RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::PropDescriptor<bool (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(bool)>(char const*,char const*,bool (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x6dccc4 — __ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEbED0Ev
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEbED0Ev")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEbED0Ev
pub fn stub_6dccc4() -> ! {
    todo!("0x6dccc4 RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::~PropDescriptor()")
}

// 0x6dccf4 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::GetSetImpl<bool (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
pub fn stub_6dccf4() -> ! {
    todo!("0x6dccf4 RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::GetSetImpl<bool (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(bool)>::isReadOnly(void)const")
}

// 0x6dccf8 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::GetSetImpl<bool (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
pub fn stub_6dccf8() -> ! {
    todo!("0x6dccf8 RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::GetSetImpl<bool (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(bool)>::isWriteOnly(void)const")
}

// 0x6dccfc — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::GetSetImpl<bool (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_6dccfc() -> ! {
    todo!("0x6dccfc RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::GetSetImpl<bool (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6dcd20 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::GetSetImpl<bool (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
pub fn stub_6dcd20() -> ! {
    todo!("0x6dcd20 RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::GetSetImpl<bool (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x6dcd44 — __ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,double>::PropDescriptor<double (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(double)>(char const*,char const*,double (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(double),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_6dcd44() -> ! {
    todo!("0x6dcd44 RBX::Reflection::PropDescriptor<RBX::Workspace,double>::PropDescriptor<double (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(double)>(char const*,char const*,double (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(double),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x6dce5c — __ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEdED0Ev
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,double>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEdED0Ev")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEdED0Ev
pub fn stub_6dce5c() -> ! {
    todo!("0x6dce5c RBX::Reflection::PropDescriptor<RBX::Workspace,double>::~PropDescriptor()")
}

// 0x6dce8c — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEdE10GetSetImplIMS2_KFdvEMS2_FvdEE10isReadOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,double>::GetSetImpl<double (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(double)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEdE10GetSetImplIMS2_KFdvEMS2_FvdEE10isReadOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEdE10GetSetImplIMS2_KFdvEMS2_FvdEE10isReadOnlyEv
pub fn stub_6dce8c() -> ! {
    todo!("0x6dce8c RBX::Reflection::PropDescriptor<RBX::Workspace,double>::GetSetImpl<double (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(double)>::isReadOnly(void)const")
}

// 0x6dce90 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEdE10GetSetImplIMS2_KFdvEMS2_FvdEE11isWriteOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,double>::GetSetImpl<double (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(double)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEdE10GetSetImplIMS2_KFdvEMS2_FvdEE11isWriteOnlyEv")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEdE10GetSetImplIMS2_KFdvEMS2_FvdEE11isWriteOnlyEv
pub fn stub_6dce90() -> ! {
    todo!("0x6dce90 RBX::Reflection::PropDescriptor<RBX::Workspace,double>::GetSetImpl<double (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(double)>::isWriteOnly(void)const")
}

// 0x6dce94 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,double>::GetSetImpl<double (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(double)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8getValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_6dce94() -> ! {
    todo!("0x6dce94 RBX::Reflection::PropDescriptor<RBX::Workspace,double>::GetSetImpl<double (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(double)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6dceb4 — __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8setValueEPNS0_13DescribedBaseERKd
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,double>::GetSetImpl<double (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(double)>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8setValueEPNS0_13DescribedBaseERKd")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9WorkspaceEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8setValueEPNS0_13DescribedBaseERKd
pub fn stub_6dceb4() -> ! {
    todo!("0x6dceb4 RBX::Reflection::PropDescriptor<RBX::Workspace,double>::GetSetImpl<double (RBX::Workspace::*)(void)const,void (RBX::Workspace::*)(double)>::setValue(RBX::Reflection::DescribedBase *,double const&)const")
}

// 0x6dcedc — __ZNK3RBX5NamedINS_16AdvArrowToolBaseELZNS_13sAdvArrowToolEEE7getNameEv
// type: 
#[doc(alias = "__ZNK3RBX5NamedINS_16AdvArrowToolBaseELZNS_13sAdvArrowToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_16AdvArrowToolBaseELZNS_13sAdvArrowToolEEE7getNameEv
pub fn stub_6dcedc() -> ! {
    todo!("0x6dcedc __ZNK3RBX5NamedINS_16AdvArrowToolBaseELZNS_13sAdvArrowToolEEE7getNameEv")
}

// 0x6dcee0 — __ZN3RBX12AdvArrowToolD1Ev
// type: void __fastcall(RBX::AdvArrowTool *__hidden this)
#[doc(alias = "RBX::AdvArrowTool::~AdvArrowTool()")]
#[doc(alias = "__ZN3RBX12AdvArrowToolD1Ev")]
// was: __ZN3RBX12AdvArrowToolD1Ev
pub fn stub_6dcee0() -> ! {
    todo!("0x6dcee0 RBX::AdvArrowTool::~AdvArrowTool()")
}

// 0x6dcee4 — __ZN3RBX12AdvArrowToolD0Ev
// type: void __fastcall(RBX::AdvArrowTool *__hidden this)
#[doc(alias = "RBX::AdvArrowTool::~AdvArrowTool()")]
#[doc(alias = "__ZN3RBX12AdvArrowToolD0Ev")]
// was: __ZN3RBX12AdvArrowToolD0Ev
pub fn stub_6dcee4() -> ! {
    todo!("0x6dcee4 RBX::AdvArrowTool::~AdvArrowTool()")
}

// 0x6dcf84 — __ZNK3RBX12AdvArrowTool8isStickyEv
// type: _DWORD __fastcall(RBX::AdvArrowTool *__hidden this)
#[doc(alias = "RBX::AdvArrowTool::isSticky(void)const")]
#[doc(alias = "__ZNK3RBX12AdvArrowTool8isStickyEv")]
// was: __ZNK3RBX12AdvArrowTool8isStickyEv
pub fn stub_6dcf84() -> ! {
    todo!("0x6dcf84 RBX::AdvArrowTool::isSticky(void)const")
}

// 0x6dd04c — __ZN3RBX16AdvArrowToolBase27getSelectedTargetPrimitivesERSt6vectorIPNS_9PrimitiveESaIS3_EE
// type: 
#[doc(alias = "RBX::AdvArrowToolBase::getSelectedTargetPrimitives(std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>> &)")]
#[doc(alias = "__ZN3RBX16AdvArrowToolBase27getSelectedTargetPrimitivesERSt6vectorIPNS_9PrimitiveESaIS3_EE")]
// was: __ZN3RBX16AdvArrowToolBase27getSelectedTargetPrimitivesERSt6vectorIPNS_9PrimitiveESaIS3_EE
pub fn stub_6dd04c() -> ! {
    todo!("0x6dd04c RBX::AdvArrowToolBase::getSelectedTargetPrimitives(std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>> &)")
}

// 0x6dd050 — __ZN3RBX12AdvArrowTool9setCursorESs
// type: 
#[doc(alias = "RBX::AdvArrowTool::setCursor(std::string)")]
#[doc(alias = "__ZN3RBX12AdvArrowTool9setCursorESs")]
// was: __ZN3RBX12AdvArrowTool9setCursorESs
pub fn stub_6dd050() -> ! {
    todo!("0x6dd050 RBX::AdvArrowTool::setCursor(std::string)")
}
